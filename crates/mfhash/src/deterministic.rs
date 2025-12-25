use std::{any::TypeId, ffi::{CStr, CString}};

pub trait DeterministicHasher {
    fn write(&mut self, input: &[u8]);
    
    #[inline]
    fn write_u8(&mut self, input: u8) {
        self.write(&[input]);
    }
    
    #[inline]
    fn write_u16(&mut self, input: u16) {
        let bytes = input.to_le_bytes();
        self.write(&bytes);
    }
    
    #[inline]
    fn write_u32(&mut self, input: u32) {
        let bytes = input.to_le_bytes();
        self.write(&bytes);
    }
    
    #[inline]
    fn write_u64(&mut self, input: u64) {
        let bytes = input.to_le_bytes();
        self.write(&bytes);
    }
    
    #[inline]
    fn write_u128(&mut self, input: u128) {
        let bytes = input.to_le_bytes();
        self.write(&bytes);
    }
    
    #[inline]
    fn write_usize(&mut self, input: usize) {
        self.write_u64(input as u64);
    }
    
    #[inline]
    fn write_i8(&mut self, input: i8) {
        self.write(&[input.cast_unsigned()])
    }
    
    #[inline]
    fn write_i16(&mut self, input: i16) {
        let bytes = input.to_le_bytes();
        self.write(&bytes);
    }
    
    #[inline]
    fn write_i32(&mut self, input: i32) {
        let bytes = input.to_le_bytes();
        self.write(&bytes);
    }
    
    #[inline]
    fn write_i64(&mut self, input: i64) {
        let bytes = input.to_le_bytes();
        self.write(&bytes);
    }
    
    #[inline]
    fn write_i128(&mut self, input: i128) {
        let bytes = input.to_le_bytes();
        self.write(&bytes);
    }
    
    #[inline]
    fn write_isize(&mut self, input: isize) {
        self.write_i64(input as i64);
    }
    
    #[inline]
    fn write_bool(&mut self, input: bool) {
        self.write_u8(if input {
            1
        } else {
            0
        })
    }
    
    #[inline]
    fn write_char(&mut self, input: char) {
        self.write_u32(input as u32)
    }
    
    #[inline]
    fn write_str(&mut self, s: &str) {
        self.write(s.as_bytes());
    }
    
    #[inline]
    fn write_cstr(&mut self, cs: &CStr) {
        self.write(cs.to_bytes());
    }
    
    fn finish(&self) -> [u8; 32];
}

pub trait DeterministicHash {
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H);
}

macro_rules! impl_hash {
    ($func:ident($type:ty $(as $as_type:ty)?)) => {
        impl DeterministicHash for $type {
            #[inline]
            fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
                hasher.$func(*self $(as $as_type)?);
            }
        }
    };
    ($(
        $func:ident($type:ty $(as $as_type:ty)?)
    ),+$(,)?) => {
        $(
            impl_hash!($func($type $(as $as_type)?));
        )*
    };
}

impl_hash!(
    write_u8(u8),
    write_u16(u16),
    write_u32(u32),
    write_u64(u64),
    write_u128(u128),
    write_usize(usize),
    write_i8(i8),
    write_i16(i16),
    write_i32(i32),
    write_i64(i64),
    write_isize(isize),
    write_bool(bool),
    write_char(char),
);

impl DeterministicHash for &str {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.len().deterministic_hash(hasher);
        hasher.write(self.as_bytes());
    }
}

impl DeterministicHash for str {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.len().deterministic_hash(hasher);
        hasher.write(self.as_bytes());
    }
}

impl DeterministicHash for String {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.len().deterministic_hash(hasher);
        hasher.write(self.as_bytes());
    }
}

impl DeterministicHash for &CStr {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        hasher.write(self.to_bytes());
    }
}

impl DeterministicHash for CStr {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        hasher.write(self.to_bytes());
    }
}

impl DeterministicHash for CString {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        hasher.write(self.as_bytes());
    }
}

impl<T: DeterministicHash + 'static, const LEN: usize> DeterministicHash for [T; LEN] {
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.len().deterministic_hash(hasher);
        if ::core::mem::size_of::<T>() == 1 && (
            TypeId::of::<T>() == TypeId::of::<u8>()
            || TypeId::of::<T>() == TypeId::of::<i8>()
        ) {
            let bytes: &[u8] = unsafe {
                ::core::slice::from_raw_parts(self.as_ptr().cast(), self.len())
            };
            hasher.write(bytes);
        } else {
            for value in self.iter() {
                value.deterministic_hash(hasher);
            }
        }
    }
}

impl<T: DeterministicHash + 'static> DeterministicHash for [T] {
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.len().deterministic_hash(hasher);
        if ::core::mem::size_of::<T>() == 1 && (
            TypeId::of::<T>() == TypeId::of::<u8>()
            || TypeId::of::<T>() == TypeId::of::<i8>()
        ) {
            let bytes: &[u8] = unsafe {
                ::core::slice::from_raw_parts(self.as_ptr().cast(), self.len())
            };
            hasher.write(bytes);
        } else {
            for value in self.iter() {
                value.deterministic_hash(hasher);
            }
        }
    }
}

impl<'a, T: DeterministicHash + 'static> DeterministicHash for &'a [T] {
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.len().deterministic_hash(hasher);
        if ::core::mem::size_of::<T>() == 1 && (
            TypeId::of::<T>() == TypeId::of::<u8>()
            || TypeId::of::<T>() == TypeId::of::<i8>()
        ) {
            let bytes: &[u8] = unsafe {
                ::core::slice::from_raw_parts(self.as_ptr().cast(), self.len())
            };
            hasher.write(bytes);
        } else {
            for value in self.iter() {
                value.deterministic_hash(hasher);
            }
        }
    }
}

impl<'a, 'b, T: DeterministicHash + 'b> DeterministicHash for &'a T {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        (*self).deterministic_hash(hasher);
    }
}

impl<T: DeterministicHash> DeterministicHash for Option<T> {
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        if let Some(inner) = self {
            (1u8).deterministic_hash(hasher);
            inner.deterministic_hash(hasher);
        } else {
            (0u8).deterministic_hash(hasher);
        }
    }
}

impl<T: DeterministicHash, E: DeterministicHash> DeterministicHash for Result<T, E> {
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        match self {
            Ok(ok) => {
                (1u8).deterministic_hash(hasher);
                ok.deterministic_hash(hasher);
            },
            Err(err) => {
                (0u8).deterministic_hash(hasher);
                err.deterministic_hash(hasher);
            },
        }
    }
}

impl<T: DeterministicHash> DeterministicHash for Box<T> {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.as_ref().deterministic_hash(hasher);
    }
}

impl<T: DeterministicHash + 'static> DeterministicHash for Vec<T> {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.as_slice().deterministic_hash(hasher);
    }
}

impl<T: DeterministicHash> DeterministicHash for ::std::rc::Rc<T> {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.as_ref().deterministic_hash(hasher);
    }
}

impl<T: DeterministicHash> DeterministicHash for ::std::sync::Arc<T> {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        self.as_ref().deterministic_hash(hasher);
    }
}

impl DeterministicHash for () {
    #[inline]
    fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
        0u8.deterministic_hash(hasher);
    }
}

macro_rules! impl_deterministic_hash_tuples {
    (tuple: $($generic:ident),*$(,)?) => {
        impl<$($generic: DeterministicHash),*> DeterministicHash for ($($generic,)*) {
            #[allow(non_snake_case)]
            fn deterministic_hash<H: DeterministicHasher>(&self, hasher: &mut H) {
                let (
                    $(
                        $generic,
                    )*
                ) = self;
                $(
                    $generic.deterministic_hash(hasher);
                )*
            }
        }
    };
    ($(
        ($($generic:ident),*$(,)?)
    )*) => {
        $(
            impl_deterministic_hash_tuples!(tuple: $($generic),*);
        )*
    };
}

impl_deterministic_hash_tuples!(
    (T0)
    (T0, T1)
    (T0, T1, T2)
    (T0, T1, T2, T3)
    (T0, T1, T2, T3, T4)
    (T0, T1, T2, T3, T4, T5)
    (T0, T1, T2, T3, T4, T5, T6)
    (T0, T1, T2, T3, T4, T5, T6, T7)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61)
    (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32, T33, T34, T35, T36, T37, T38, T39, T40, T41, T42, T43, T44, T45, T46, T47, T48, T49, T50, T51, T52, T53, T54, T55, T56, T57, T58, T59, T60, T61, T62)
);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn deterministic_hash_test() {
        // (0u32).deterministic_hash(hasher);
        let mut hasher = crate::Blake3Hasher::new();
        ([1u128, 2u128, 3u128], (true, 1, b"test"), [0u8; 4].as_slice()).deterministic_hash(&mut hasher);
        let hash = hasher.finalize();
        println!("Hash: {hash}");
    }
}