use std::io::{Write, Read};
use super::*;

#[derive(Debug)]
pub(crate) struct Memory<K: DumbKey, V: DumbValue>(Map<K, V>);

const KEY_VALUE_SEPARATOR: u8 = 0x0002;
const VALUE_END: u8 = 0x0003;

impl<K: DumbKey, V: DumbValue> Memory<K, V> {
    #[inline]
    pub(crate) fn new() -> Self {
        Self(Map::new())
    }
    #[inline]
    pub(crate) fn get(&self, key: K) -> Option<&V> {
        self.0.get(&key)
    }
    #[inline]
    pub(crate) fn contains(&self, key: K) -> bool {
        self.0.contains_key(&key)
    }
    #[inline]
    pub(crate) fn set(&mut self, key: K, value: V) {
        self.0.insert(key, value);
    }
    #[inline]
    pub(crate) fn delete(&mut self, key: K) {
        self.0.remove(&key);
    }
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.0.clear();
    }

    pub(crate) fn load<W: Read>(&mut self, mut reader: W) -> DumbResult {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        let mut key = Vec::new();
        let mut value = Vec::new();
        let mut is_key = true;
        for byte in buffer {
            if byte == KEY_VALUE_SEPARATOR {
                is_key = false;
                continue;
            }
            if byte == VALUE_END {
                self.0.insert(K::deserialize(&key)?, V::deserialize(&value)?);
                key.clear();
                value.clear();
                is_key = true;
                continue;
            }
            if is_key {
                key.push(byte);
            } else {
                value.push(byte);
            }
        }

        Ok(())
    }

    pub(crate) fn save<W>(&self, mut writer: W) -> DumbResult
    where
        W: Write,
    {
        for (k, v) in &self.0 {
            writer.write_all(&k.serialize())?;
            writer.write_all(&[KEY_VALUE_SEPARATOR])?;
            writer.write_all(&v.serialize())?;
            writer.write_all(&[VALUE_END])?;
        }
        Ok(())
    }
}
