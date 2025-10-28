use crate::crypto::password::Password;
use crate::error::ErebusResult;
use redb::{MultimapTableDefinition, TableDefinition};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait Entity: Sized + Serialize + for<'de> Deserialize<'de> {
    type Id: redb::Key
        + for<'a> std::borrow::Borrow<<Self::Id as redb::Value>::SelfType<'a>>
        + 'static;

    fn id(&self) -> Self::Id;
    fn table_name() -> &'static str;

    fn table_def() -> TableDefinition<'static, Self::Id, Vec<u8>> {
        TableDefinition::new(Self::table_name())
    }

    fn encode(&self, password: &Password) -> ErebusResult<Vec<u8>> {
        password.encrypt(&rmp_serde::to_vec_named(self)?)
    }

    fn decode(bytes: &[u8], password: &Password) -> ErebusResult<Self> {
        Ok(rmp_serde::from_slice(&password.decrypt(bytes)?)?)
    }
}

pub trait MultiEntity: Sized + Serialize + for<'de> Deserialize<'de> + Debug {
    type Id: redb::Key
        + for<'a> std::borrow::Borrow<<Self::Id as redb::Value>::SelfType<'a>>
        + 'static;

    fn id(&self) -> Self::Id;
    fn multimap_table_name() -> &'static str;

    fn multimap_table_def() -> MultimapTableDefinition<'static, Self::Id, &'static [u8]> {
        MultimapTableDefinition::new(Self::multimap_table_name())
    }

    fn encode(&self, password: &Password) -> ErebusResult<Vec<u8>> {
        password.encrypt(&rmp_serde::to_vec_named(self)?)
    }

    fn decode(bytes: &[u8], password: &Password) -> ErebusResult<Self> {
        Ok(rmp_serde::from_slice(&password.decrypt(bytes)?)?)
    }
}
