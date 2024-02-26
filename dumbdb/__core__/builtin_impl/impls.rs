use super::*;

impl<K, V> DumbDataBaseReadHandler<K, V> for DumbDBHandler<K, V> where
    K: DumbKey,
    V: DumbValue,
{
    #[inline]
    fn get(&self, key: K) -> DumbResult<Option<&V>> {
        unsafe { &*self.0 }.get(key)
    }
    #[inline]
    fn contains(&self, key: K) -> DumbResult<bool> {
        unsafe { &*self.0 }.contains(key)
    }
}

impl<K, V> DumbDataBaseReadHandler<K, V> for DumbDBWriteHandler<K, V> where
    K: DumbKey,
    V: DumbValue,
{
    #[inline]
    fn get(&self, key: K) -> DumbResult<Option<&V>> {
        unsafe { &*self.0 }.get(key)
    }
    #[inline]
    fn contains(&self, key: K) -> DumbResult<bool> {
        unsafe { &*self.0 }.contains(key)
    }
}

impl<K, V> DumbDataBaseWriteHandler<K, V> for DumbDBWriteHandler<K, V> where
    K: DumbKey,
    V: DumbValue,
{
    #[inline]
    fn set(&self, key: K, value: V) -> DumbResult {
        unsafe { &*self.0 }.set(key, value)
    }
    #[inline]
    fn delete(&self, key: K) -> DumbResult {
        unsafe { &*self.0 }.delete(key)
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

macro_rules! impl_key_for_num {
    (
        $func:ident -> $type:ty
    ) => (
        fn $func(n: $type) -> Vec<u8> {
            fn inner(n: $type, xs: &mut Vec<u8>) {
                if n >= 10 {
                    inner(n / 10, xs);
                }
                xs.push(n as u8 % 10 + b'0');
            }
            let mut xs = Vec::new();
            inner(n, &mut xs);
            xs
        }
        
        impl DumbValue for $type {
            fn deserialize(bytes: &Vec<u8>) -> DumbResult<Self> {
                let mut n = 0;
                for byte in bytes {
                    if byte < &b'0' || byte > &b'9' {
                        return Err(DumbError::ParsingError("invalid digit"));
                    }
                    let number_from_byte = byte - b'0';
                    n = n * 10 + number_from_byte as $type;
                }
                Ok(n)
            }
            #[inline]
            fn serialize(&self) -> Vec<u8> {
                $func(*self)
            }
        }
    );
}

impl_key_for_num!(serialize_u8 -> u8);
impl_key_for_num!(serialize_u16 -> u16);
impl_key_for_num!(serialize_u32 -> u32);
impl_key_for_num!(serialize_u64 -> u64);
impl_key_for_num!(serialize_u128 -> u128);
impl_key_for_num!(serialize_usize -> usize);
