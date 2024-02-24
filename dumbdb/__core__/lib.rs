#![no_std]

use ::alloc::collections::BTreeMap as Map;
use ::core::result::Result as R;
use ::alloc::string::ToString;
use ::core::str::FromStr;
use ::core::fmt;

extern crate alloc;

mod error;
pub use error::{Error, Result};

pub trait DumbDataBase<'a, V> {
    fn set(&mut self, key: &'a str, value: V);
    fn get(&self, key: &'a str) -> Option<&V>;
    fn delete(&mut self, key: &'a str);
    fn contains(&self, key: &'a str) -> bool;
    fn clear(&mut self);
}

pub trait DumbDBValue: FromStr + ToString {}
impl<T: FromStr + ToString> DumbDBValue for T {}

pub struct DumbInMemoryStorage<'a, V>(Map<&'a str, V>);

impl<'a, V> DumbInMemoryStorage<'a, V> {
    #[inline]
    pub fn new() -> Self {
        Self(Map::new())
    }
}

impl<'a, V> DumbInMemoryStorage<'a, V> where
    V: DumbDBValue
{
    pub fn construct(s: &'a str) -> Result<Self> {
        let mut map = Map::new();
        for line in s.lines() {
            // Ensure there is ONLY one '=' in the line
            let mut count = 0;
            for c in line.chars() {
                if c == '=' {
                    count += 1;
                }
            }
            if count != 1 {
                return Err(Error::ParsingError("Line contains more than one '=' character"));
            }

            // Split the line into key and value
            let mut parts = line.splitn(2, '=');

            // Get the key
            let key = parts.next().ok_or(Error::ParsingError("Line doesn't contain a key"))?.trim();

            // Get & Validate the value
            let value = parts.next().ok_or(Error::ParsingError("Line doesn't contain a value"))?.trim();
            let value = V::from_str(value).map_err(|_| Error::ParsingError("Failed to parse value"))?;

            // Insert the key-value pair into the map
            map.insert(key, value);
        }
        Ok(Self(map))
    }
}

impl<'a, V> DumbDataBase<'a, V> for DumbInMemoryStorage<'a, V> {
    #[inline]
    fn set(&mut self, key: &'a str, value: V) {
        self.0.insert(key, value);
    }
    #[inline]
    fn get(&self, key: &'a str) -> Option<&V> {
        self.0.get(key)
    }
    #[inline]
    fn delete(&mut self, key: &'a str) {
        self.0.remove(key);
    }
    #[inline]
    fn contains(&self, key: &'a str) -> bool {
        self.0.contains_key(key)
    }
    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
}

// impl ::core::fmt::Debug for DumbDBBase<()> {
//     fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//         f.debug_struct("DumbDBBase")
//             .field("name", &self.name)
//             .field("items", &self.data)
//             .finish()
//     }
// }

// impl<V> DumbDBBase<V> where
//     V: DumbDBValue
// {
//     pub fn new<Name: ToString>(name: Name) -> Result<DumbDBBase<V>> {
//         // Opening / Creating database file
//         let file;
//         let filename = format!("{}.dumb.db", name.to_string());
//         match File::open(filename.clone()) {
//             Err(e) => {
//                 if e.kind() != std::io::ErrorKind::NotFound {
//                     return Err(Error::DBOpenError);
//                 }
//                 match File::create(filename) {
//                     Err(_) => return Err(Error::DBCreateError),
//                     Ok(f) => file = f,
//                 }
//             },
//             Ok(f) => file = f,
//         }
// 
//         // Attempt to load data from file
// 
// 
//         Ok(DumbDBBase {
//             data: Map::new(),
//             file_handle: file,
//             name: name.to_string(),
//         })
//     }
// }
