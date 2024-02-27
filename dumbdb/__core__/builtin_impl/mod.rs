#![cfg(feature = "export-builtin-impl")]

use std::cell::UnsafeCell;
use std::fs::File;
use ::core::fmt;
use crate::*;

mod memory;
mod impls;

use memory::Memory;
use std::path::PathBuf;

#[derive(Debug)]
struct DumbDBInner<K: DumbKey, V: DumbValue> {
    memory: Memory<K, V>,
    file_path: PathBuf,
}

#[repr(transparent)]
pub struct DumbDB<K, V>(UnsafeCell<Option<DumbDBInner<K, V>>>) where
    K: DumbKey,
    V: DumbValue;

unsafe impl<K: DumbKey, V: DumbValue> Send for DumbDB<K, V> {}
unsafe impl<K: DumbKey, V: DumbValue> Sync for DumbDB<K, V> {}

impl<K: DumbKey, V: DumbValue> DumbDB<K, V> {
    #[inline]
    fn is_init(&self) -> bool {
        unsafe { &*self.0.get() }.is_some()
    }
    #[inline]
    pub const fn new() -> Self {
        Self(UnsafeCell::new(None))
    }
    pub fn init<Name>(&self, name: Name) -> DumbResult where
        Name: fmt::Display
    {
        let file_path: PathBuf = format!("{name}.dumb.db").into();
        if self.is_init() {
            return Err(DumbError::DBAlreadyInitialized);
        }
        let mut memory = Memory::new();
        let mut open_options = File::options();
        open_options.read(true);
        open_options.write(false);
        open_options.create(false);
        open_options.truncate(false);
        if let Ok(mut f) = open_options.open(&file_path) {
            memory.load(&mut f)?;
        }
        let inner = DumbDBInner { memory, file_path };
        unsafe { *self.0.get() = Some(inner) };
        Ok(())
    }
    pub fn save(&self) -> DumbResult {
        if !self.is_init() {
            return Err(DumbError::DBNotInitialized);
        }
        let inner = unsafe {
            (&mut *self.0.get()).as_mut().unwrap_unchecked()
        };
        let mut open_options = File::options();
        open_options.read(true);
        open_options.write(true);
        open_options.create(true);
        open_options.truncate(true);
        let mut file_handle = match open_options.open(&inner.file_path) {
            Err(_) => return Err(DumbError::DBOpenError),
            Ok(f) => f,
        };
        inner.memory.save(&mut file_handle)?;
        Ok(())
    }
    pub fn get(&self, key: K) -> DumbResult<Option<&V>> {
        unsafe { &*self.0.get() }.as_ref()
            .map(|v| v.memory.get(key))
            .ok_or(DumbError::DBNotInitialized)
    }
    pub fn contains(&self, key: K) -> DumbResult<bool> {
        unsafe { &*self.0.get() }.as_ref()
            .map(|v| v.memory.contains(key))
            .ok_or(DumbError::DBNotInitialized)
    }
    pub fn set(&self, key: K, value: V) -> DumbResult {
        unsafe { &mut *self.0.get() }.as_mut()
            .map(|v| v.memory.set(key, value))
            .ok_or(DumbError::DBNotInitialized)
    }
    pub fn delete(&self, key: K) -> DumbResult {
        unsafe { &mut *self.0.get() }.as_mut()
            .map(|v| v.memory.delete(key))
            .ok_or(DumbError::DBNotInitialized)
    }
    pub fn clear(&self) -> DumbResult {
        unsafe { &mut *self.0.get() }.as_mut()
            .map(|v| v.memory.clear())
            .ok_or(DumbError::DBNotInitialized)
    }
}

impl<K, V> fmt::Debug for DumbDB<K, V> where
    K: DumbKey + fmt::Debug,
    V: DumbValue + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_init() {
            let inner = unsafe { (&*self.0.get()).as_ref().unwrap_unchecked() };
            f.debug_struct("DumbDB")
                .field("initilized", &true)
                .field("name", &inner.file_path)
                .finish()
        } else {
            f.debug_struct("DumbDB")
                .field("initilized", &false)
                .finish()
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DumbDBHandler<K, V>(*const DumbDB<K, V>) where
    K: DumbKey,
    V: DumbValue;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DumbDBWriteHandler<K, V>(*mut DumbDB<K, V>) where
    K: DumbKey,
    V: DumbValue;
