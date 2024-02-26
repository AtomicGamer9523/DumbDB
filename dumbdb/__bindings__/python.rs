use pyo3::prelude::*;
use pyo3::PyAny;

use ddb::{
    internal_handlers::{
        DumbDBWriteHandler as WriteHandler,
        DumbDBHandler as Handler,
    },
    DumbDataBaseWriteHandler,
    DumbDataBaseReadHandler,
    DumbDataBase,
    DumbDB as D,
};
type Key = u32;
type Value = Py<PyAny>;
type Inner = D<Key, Value>;
type WriteH = WriteHandler<Key, Value>;
type H = Handler<Key, Value>;

macro_rules! obj {
    ($name:ident, $inner:ty) => {
        #[pyclass]
        #[doc(hidden)]
        #[repr(transparent)]
        pub struct $name($inner);

        impl From<$inner> for $name {
            #[inline]
            fn from(inner: $inner) -> Self {
                $name(inner)
            }
        }

        impl From<$name> for $inner {
            #[inline]
            fn from(obj: $name) -> Self {
                obj.0
            }
        }
    };
}

obj!(DumbDB, Inner);
obj!(DumbDBWriteHandler, WriteH);
obj!(DumbDBHandler, H);

#[pymethods]
impl DumbDB {
    #[new]
    #[inline]
    #[doc(hidden)]
    pub fn new(name: String) -> PyResult<Self> {
        let this = Inner::new();
        this.init(name)?;
        Ok(this.into())
    }
    #[inline]
    pub fn save(&self) -> PyResult<()> {
        self.0.save().map_err(Into::into)
    }
    #[inline]
    pub fn get(&self, key: Key) -> PyResult<Option<&Py<PyAny>>> {
        self.0.get(key).map_err(Into::into)
    }
    #[inline]
    pub fn contains(&self, key: Key) -> PyResult<bool> {
        self.0.contains(key).map_err(Into::into)
    }
    #[inline]
    pub fn set(&self, key: Key, value: Py<PyAny>) -> PyResult<()> {
        self.0.set(key, value).map_err(Into::into)
    }
    #[inline]
    pub fn delete(&self, key: Key) -> PyResult<()> {
        self.0.delete(key).map_err(Into::into)
    }
    #[inline]
    pub fn clear(&self) -> PyResult<()> {
        self.0.clear().map_err(Into::into)
    }
    #[inline]
    pub fn new_handler(&self) -> DumbDBHandler {
        self.0.new_handler().into()
    }
    #[inline]
    pub fn new_write_handler(&self) -> DumbDBWriteHandler {
        self.0.new_write_handler().into()
    }
}

#[pymethods]
impl DumbDBHandler {
    #[inline]
    pub fn get(&self, key: Key) -> PyResult<Option<&Py<PyAny>>> {
        self.0.get(key).map_err(Into::into)
    }
    #[inline]
    pub fn contains(&self, key: Key) -> PyResult<bool> {
        self.0.contains(key).map_err(Into::into)
    }
}

#[pymethods]
impl DumbDBWriteHandler {
    #[inline]
    pub fn get(&self, key: Key) -> PyResult<Option<&Py<PyAny>>> {
        self.0.get(key).map_err(Into::into)
    }
    #[inline]
    pub fn contains(&self, key: Key) -> PyResult<bool> {
        self.0.contains(key).map_err(Into::into)
    }
    #[inline]
    pub fn set(&self, key: Key, value: Py<PyAny>) -> PyResult<()> {
        self.0.set(key, value).map_err(Into::into)
    }
    #[inline]
    pub fn delete(&self, key: Key) -> PyResult<()> {
        self.0.delete(key).map_err(Into::into)
    }
}

#[pymodule]
pub fn dumbdb(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DumbDB>()?;
    m.add_class::<DumbDBHandler>()?;
    m.add_class::<DumbDBWriteHandler>()?;
    Ok(())
}
