#![cfg(feature = "export-builtin-impl")]

use std::cell::UnsafeCell;
use std::fs::File;
use crate::*;

mod memory;
mod impls;

use memory::Memory;

#[derive(Debug)]
struct DumbDBInner<K: DumbKey, V: DumbValue> {
    file_handle: File,
    memory: Memory<K, V>,
}

pub struct DumbDB<K: DumbKey, V: DumbValue> {
    inner: UnsafeCell<Option<DumbDBInner<K, V>>>,
    name: &'static str,
}

unsafe impl<K: DumbKey, V: DumbValue> Send for DumbDB<K, V> {}
unsafe impl<K: DumbKey, V: DumbValue> Sync for DumbDB<K, V> {}

impl<K: DumbKey, V: DumbValue> DumbDB<K, V> {
    #[inline]
    fn is_init(&self) -> bool {
        unsafe { &*self.inner.get() }.is_some()
    }
    pub const fn new(name: &'static str) -> Self {
        Self {
            inner: UnsafeCell::new(None),
            name,
        }
    }
    pub fn init(&self) -> DumbResult {
        let name = format!("{}.dumb.db", self.name);
        if self.is_init() {
            return Err(DumbError::DBAlreadyInitialized);
        }
        let mut memory = Memory::new();
        let mut open_options = File::options();
        open_options.read(true);
        open_options.write(false);
        open_options.create(false);
        open_options.truncate(false);
        if let Ok(mut f) = open_options.open(&name) {
            memory.load(&mut f)?;
        }
        open_options.write(true);
        open_options.create(true);
        open_options.truncate(true);
        let file_handle = match open_options.open(&name) {
            Err(_) => return Err(DumbError::DBOpenError),
            Ok(f) => f,
        };
        let inner = DumbDBInner { memory, file_handle };
        unsafe { *self.inner.get() = Some(inner) };
        Ok(())
    }
    pub fn save(&self) -> DumbResult {
        if !self.is_init() {
            return Err(DumbError::DBNotInitialized);
        }
        let inner = unsafe {
            (&mut *self.inner.get()).as_mut().unwrap_unchecked()
        };
        inner.memory.save(&mut inner.file_handle)?;
        Ok(())
    }
    pub fn get(&self, key: K) -> DumbResult<Option<&V>> {
        unsafe { &*self.inner.get() }.as_ref()
            .map(|v| v.memory.get(key))
            .ok_or(DumbError::DBNotInitialized)
    }
    pub fn contains(&self, key: K) -> DumbResult<bool> {
        unsafe { &*self.inner.get() }.as_ref()
            .map(|v| v.memory.contains(key))
            .ok_or(DumbError::DBNotInitialized)
    }
    pub fn set(&self, key: K, value: V) -> DumbResult {
        unsafe { &mut *self.inner.get() }.as_mut()
            .map(|v| v.memory.set(key, value))
            .ok_or(DumbError::DBNotInitialized)
    }
    pub fn delete(&self, key: K) -> DumbResult {
        unsafe { &mut *self.inner.get() }.as_mut()
            .map(|v| v.memory.delete(key))
            .ok_or(DumbError::DBNotInitialized)
    }
    pub fn clear(&self) -> DumbResult {
        unsafe { &mut *self.inner.get() }.as_mut()
            .map(|v| v.memory.clear())
            .ok_or(DumbError::DBNotInitialized)
    }
}

use ::core::fmt;

impl<K, V> fmt::Debug for DumbDB<K, V> where
    K: DumbKey + fmt::Debug,
    V: DumbValue + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_init() {
            let memory = unsafe { (&*self.inner.get()).as_ref().unwrap_unchecked() };
            f.debug_struct("DumbDB")
                .field("name", &self.name)
                .field("memory", memory)
                .finish()
        } else {
            f.debug_struct("DumbDB")
                .field("name", &self.name)
                .finish()
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DumbDBHandler<K: DumbKey, V: DumbValue>(*const DumbDB<K, V>);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DumbDBWriteHandler<K: DumbKey, V: DumbValue>(*mut DumbDB<K, V>);
