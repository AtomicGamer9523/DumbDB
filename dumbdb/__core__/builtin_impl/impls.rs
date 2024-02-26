use super::*;

impl<K, V> DumbDataBaseReadHandler<K, V> for DumbDBHandler<K, V> where
    K: DumbKey,
    V: DumbValue,
{
    #[inline]
    fn get(&self, key: K) -> Option<&V> {
        unsafe { &*self.0 }.get(key).ok()?
    }
    #[inline]
    fn contains(&self, key: K) -> bool {
        unsafe { &*self.0 }.contains(key).unwrap_or(false)
    }
}

impl<K, V> DumbDataBaseReadHandler<K, V> for DumbDBWriteHandler<K, V> where
    K: DumbKey,
    V: DumbValue,
{
    #[inline]
    fn get(&self, key: K) -> Option<&V> {
        unsafe { &*self.0 }.get(key).ok()?
    }
    #[inline]
    fn contains(&self, key: K) -> bool {
        unsafe { &*self.0 }.contains(key).unwrap_or(false)
    }
}

impl<K, V> DumbDataBaseWriteHandler<K, V> for DumbDBWriteHandler<K, V> where
    K: DumbKey,
    V: DumbValue,
{
    #[inline]
    fn set(&self, key: K, value: V) {
        let _ = unsafe { &*self.0 }.set(key, value);
    }
    #[inline]
    fn delete(&self, key: K) {
        let _ = unsafe { &*self.0 }.delete(key);
    }
}
unsafe impl<K, V> Send for DumbDBHandler<K, V> where
    K: DumbKey,
    V: DumbValue,
{ }
unsafe impl<K, V> Send for DumbDBWriteHandler<K, V> where
    K: DumbKey,
    V: DumbValue,
{ }

impl<K, V> DumbDataBase<K, V> for DumbDB<K, V> where
    K: DumbKey,
    V: DumbValue,
{
    type Handler = DumbDBHandler<K, V>;
    type WriteHandler = DumbDBWriteHandler<K, V>;
    #[inline]
    fn new_handler(&self) -> Self::Handler {
        DumbDBHandler(self)
    }
    #[inline]
    fn new_write_handler(&self) -> Self::WriteHandler {
        DumbDBWriteHandler(self as *const _ as *mut _)
    }
}

macro_rules! impl_key_for {
    ($($t:ty),*) => {
        $(
            impl DumbKey for $t { }
        )*
    };
}

fn digits_u32(n: u32) -> Vec<u8> {
    fn x_inner(n: u32, xs: &mut Vec<u8>) {
        if n >= 10 {
            x_inner(n / 10, xs);
        }
        xs.push(n as u8 % 10 + b'0');
    }
    let mut xs = Vec::new();
    x_inner(n, &mut xs);
    xs
}

impl DumbValue for u32 {
    fn deserialize(bytes: &Vec<u8>) -> DumbResult<Self> {
        let mut n = 0;
        for byte in bytes {
            if byte < &b'0' || byte > &b'9' {
                return Err(DumbError::ParsingError("invalid digit"));
            }
            let number_from_byte = byte - b'0';
            n = n * 10 + number_from_byte as u32;
        }
        Ok(n)
    }
    fn serialize(&self) -> Vec<u8> {
        digits_u32(*self)
    }
}
