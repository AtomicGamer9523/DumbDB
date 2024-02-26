#[cfg(feature = "export-builtin-impl")]
use ::alloc::collections::BTreeMap as Map;
use ::core::result::Result as R;
use ::core::fmt;

extern crate alloc;

#[cfg(feature = "pyo3")]
mod python_bindings;

mod error;
#[cfg(feature = "export-builtin-impl")]
mod builtin_impl;

pub use error::{DumbError, DumbResult};
#[cfg(feature = "export-builtin-impl")]
pub use builtin_impl::DumbDB;

#[doc(hidden)]
pub mod internal_handlers {
    pub use super::builtin_impl::{DumbDBHandler, DumbDBWriteHandler};
}

pub trait DumbDataBase<K, V> where
    K: DumbKey,
    V: DumbValue,
{
    type Handler: DumbDataBaseReadHandler<K, V>;
    type WriteHandler: DumbDataBaseWriteHandler<K, V>;
    fn new_handler(&self) -> Self::Handler;
    fn new_write_handler(&self) -> Self::WriteHandler;
}

pub trait DumbDataBaseReadHandler<K, V> where
    K: DumbKey,
    V: DumbValue
{
    fn get(&self, key: K) -> DumbResult<Option<&V>>;
    fn contains(&self, key: K) -> DumbResult<bool>;
}

pub trait DumbDataBaseWriteHandler<K, V> where
    Self: DumbDataBaseReadHandler<K, V>,
    K: DumbKey,
    V: DumbValue
{
    fn set(&self, key: K, value: V) -> DumbResult;
    fn delete(&self, key: K) -> DumbResult;
}

pub trait DumbValue: Sized {
    fn deserialize(vec: &Vec<u8>) -> DumbResult<Self>;
    fn serialize(&self) -> Vec<u8>;
}
pub trait DumbKey: DumbValue + Ord { }

impl<V> DumbKey for V where
    V: DumbValue + Ord
{ }
