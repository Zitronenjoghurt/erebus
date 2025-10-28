use crate::crypto::password::Password;
use crate::database::entity::{Entity, MultiEntity};
use crate::database::pw_verify::PasswordVerifier;
use crate::error::{ErebusError, ErebusResult};
use redb::{ReadableDatabase, ReadableMultimapTable, ReadableTable, ReadableTableMetadata};
use std::path::Path;

pub mod entity;
mod pw_verify;

pub struct Database {
    db: redb::Database,
    password: Password,
}

impl Database {
    pub fn initialize(path: &Path) -> ErebusResult<Self> {
        let create_new = !path.exists();
        let password =
            Password::from_env("DATABASE_PASSWORD").ok_or(ErebusError::DatabasePassword)?;

        let redb = redb::Database::create(path)?;
        let db = Self { db: redb, password };

        if create_new {
            db.create_password_verifier()?;
        }
        db.verify_password()?;

        Ok(db)
    }

    fn create_password_verifier(&self) -> ErebusResult<()> {
        let pw_verifier = PasswordVerifier::new();
        self.save(&pw_verifier)?;
        Ok(())
    }

    fn verify_password(&self) -> ErebusResult<()> {
        let Ok(Some(pw_verifier)) =
            self.find::<PasswordVerifier>(PasswordVerifier::PW_VERIFY_STRING.to_string())
        else {
            return Err(ErebusError::DatabasePassword);
        };

        if !pw_verifier.verify() {
            Err(ErebusError::DatabasePassword)
        } else {
            Ok(())
        }
    }

    fn open_table_or_empty<E: Entity>(
        &self,
        txn: &redb::ReadTransaction,
    ) -> ErebusResult<Option<redb::ReadOnlyTable<E::Id, Vec<u8>>>> {
        match txn.open_table(E::table_def()) {
            Ok(table) => Ok(Some(table)),
            Err(redb::TableError::TableDoesNotExist(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    fn open_multimap_or_empty<E: MultiEntity>(
        &self,
        txn: &redb::ReadTransaction,
    ) -> ErebusResult<Option<redb::ReadOnlyMultimapTable<E::Id, &[u8]>>> {
        match txn.open_multimap_table(E::multimap_table_def()) {
            Ok(table) => Ok(Some(table)),
            Err(redb::TableError::TableDoesNotExist(_)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn save<E: Entity>(&self, entity: &E) -> ErebusResult<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(E::table_def())?;
            let bytes = entity.encode(&self.password)?;
            table.insert(entity.id(), &bytes)?;
        }
        txn.commit()?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn save_multi<E: MultiEntity>(&self, entity: &E) -> ErebusResult<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_multimap_table(E::multimap_table_def())?;
            let bytes = entity.encode(&self.password)?;
            table.insert(entity.id(), &*bytes)?;
        }
        txn.commit()?;
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn find<E: Entity>(&self, id: E::Id) -> ErebusResult<Option<E>> {
        let txn = self.db.begin_read()?;
        let Some(table) = self.open_table_or_empty::<E>(&txn)? else {
            return Ok(None);
        };

        let Some(data) = table.get(id)?.map(|guard| guard.value()) else {
            return Ok(None);
        };

        let entity = E::decode(&data, &self.password)?;
        Ok(Some(entity))
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn find_multi<E: MultiEntity>(&self, id: E::Id) -> ErebusResult<Vec<E>> {
        let txn = self.db.begin_read()?;
        let table = match txn.open_multimap_table(E::multimap_table_def()) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(Vec::new()),
            Err(e) => return Err(e.into()),
        };

        let mut results = Vec::new();
        for item in table.get(id)? {
            let guard = item?;
            let bytes = guard.value().to_vec();
            let entity = E::decode(&bytes, &self.password)?;
            results.push(entity);
        }

        Ok(results)
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn for_each<E: Entity, F>(&self, f: F) -> ErebusResult<()>
    where
        F: Fn(E) -> ErebusResult<()>,
    {
        let txn = self.db.begin_read()?;
        let Some(table) = self.open_table_or_empty::<E>(&txn)? else {
            return Ok(());
        };

        for result in table.iter()? {
            let (_key, guard) = result?;
            let entity = E::decode(&guard.value(), &self.password)?;
            f(entity)?;
        }

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn for_each_multi<E: MultiEntity, F>(&self, f: F) -> ErebusResult<()>
    where
        F: Fn(E) -> ErebusResult<()>,
    {
        let txn = self.db.begin_read()?;
        let table = match txn.open_multimap_table(E::multimap_table_def()) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(()),
            Err(e) => return Err(e.into()),
        };

        for result in table.iter()? {
            let (_key, values) = result?;
            for value_result in values {
                let value = value_result?;
                let bytes = value.value();
                let entity = E::decode(bytes, &self.password)?;
                f(entity)?;
            }
        }

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn count<E: Entity>(&self) -> ErebusResult<u64> {
        let txn = self.db.begin_read()?;

        let Some(table) = self.open_table_or_empty::<E>(&txn)? else {
            return Ok(0);
        };

        Ok(table.len().unwrap_or(0))
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn count_multi<E: MultiEntity>(&self) -> ErebusResult<u64> {
        let txn = self.db.begin_read()?;

        let table = match txn.open_multimap_table(E::multimap_table_def()) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(0),
            Err(e) => return Err(e.into()),
        };

        Ok(table.len().unwrap_or(0))
    }
}
