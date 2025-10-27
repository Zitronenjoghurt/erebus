use crate::database::entity::{Entity, MultiEntity};
use crate::error::ErebusResult;
use redb::{ReadableDatabase, ReadableMultimapTable, ReadableTable};
use std::path::Path;

pub mod entity;

#[derive(Debug)]
pub struct Database {
    db: redb::Database,
}

impl Database {
    pub fn initialize(directory: &Path) -> ErebusResult<Self> {
        let db = redb::Database::create(directory)?;
        Ok(Self { db })
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
            let bytes = entity.encode()?;
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
            let bytes = entity.encode()?;
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

        let entity = E::decode(&data)?;
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
            let entity = E::decode(&bytes)?;
            results.push(entity);
        }

        Ok(results)
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn map<E: Entity, F>(&self, mut f: F) -> ErebusResult<()>
    where
        F: FnMut(E) -> ErebusResult<()>,
    {
        let txn = self.db.begin_read()?;
        let Some(table) = self.open_table_or_empty::<E>(&txn)? else {
            return Ok(());
        };

        for result in table.iter()? {
            let (_key, guard) = result?;
            let entity = E::decode(&guard.value())?;
            f(entity)?;
        }

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip_all)]
    pub fn map_multi<E: MultiEntity, F>(&self, mut f: F) -> ErebusResult<()>
    where
        F: FnMut(E) -> ErebusResult<()>,
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
                let entity = E::decode(bytes)?;
                f(entity)?;
            }
        }

        Ok(())
    }
}
