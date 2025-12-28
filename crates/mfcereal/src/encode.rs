use ::core::ffi::CStr;
use std::any::TypeId;

struct Counter {
    count: u64,
}

impl Counter {
    #[inline(always)]
    #[must_use]
    pub const fn new() -> Self {
        Self { count: 0 }
    }
    
    #[inline(always)]
    pub fn incr<E>(&mut self, result: Result<u64, E>) -> Result<(), E> {
        self.count += result?;
        Ok(())
    }
    
    #[inline(always)]
    pub fn ok<E>(self) -> Result<u64, E> {
        Ok(self.count)
    }
}

type EncRes<E> = Result<u64, E>;

/// Simply a wrapper around [::core::mem::transmute]. Meant to ensure
/// that it is an slice that is being transmuted.
#[inline(always)]
#[must_use]
unsafe fn cast_slice<'a, Src, Dst>(array: &'a [Src]) -> &'a [Dst] {
    const {
        use ::core::mem::{align_of, size_of};
        if !(
            size_of::<Src>() == size_of::<Dst>()
            && align_of::<Src>() == align_of::<Dst>()
        ) {
            panic!("Size and Alignment of `Src` and `Dst` must be equal.");
        }
    }
    unsafe {
        ::core::mem::transmute(array)
    }
}

pub trait Encoder {
    type Error;
    
    fn write_exact(&mut self, bytes: &[u8]) -> EncRes<Self::Error>;
    
    fn write_u8(&mut self, value: u8) -> EncRes<Self::Error> {
        self.write_exact(&[value])
    }
    
    fn write_u16(&mut self, value: u16) -> EncRes<Self::Error> {
        let bytes = value.to_be_bytes();
        self.write_exact(&bytes)
    }
    
    fn write_u32(&mut self, value: u32) -> EncRes<Self::Error> {
        let bytes = value.to_be_bytes();
        self.write_exact(&bytes)
    }
    
    fn write_u64(&mut self, value: u64) -> EncRes<Self::Error> {
        let bytes = value.to_be_bytes();
        self.write_exact(&bytes)
    }
    
    fn write_u128(&mut self, value: u128) -> EncRes<Self::Error> {
        let bytes = value.to_be_bytes();
        self.write_exact(&bytes)
    }
    
    fn write_usize(&mut self, value: usize) -> EncRes<Self::Error> {
        self.write_u64(value as u64)
    }
    
    fn write_i8(&mut self, value: i8) -> EncRes<Self::Error> {
        self.write_u8(value.cast_unsigned())
    }
    
    fn write_i16(&mut self, value: i16) -> EncRes<Self::Error> {
        self.write_u16(value.cast_unsigned())
    }
    
    fn write_i32(&mut self, value: i32) -> EncRes<Self::Error> {
        self.write_u32(value.cast_unsigned())
    }
    
    fn write_i64(&mut self, value: i64) -> EncRes<Self::Error> {
        self.write_u64(value.cast_unsigned())
    }
    
    fn write_i128(&mut self, value: i128) -> EncRes<Self::Error> {
        self.write_u128(value.cast_unsigned())
    }
    
    fn write_isize(&mut self, value: isize) -> EncRes<Self::Error> {
        self.write_usize(value.cast_unsigned())
    }
    
    fn write_bool(&mut self, value: bool) -> EncRes<Self::Error> {
        self.write_u8(if value {
            1
        } else {
            0
        })
    }
    
    fn write_char(&mut self, value: char) -> EncRes<Self::Error> {
        self.write_u32(value as u32)
    }
    
    fn write_str(&mut self, value: &str) -> EncRes<Self::Error> {
        self.write_u8_slice(value.as_bytes(), true)
    }
    
    fn write_cstr(&mut self, value: &CStr) -> EncRes<Self::Error> {
        self.write_u8_slice(value.to_bytes_with_nul(), true)
    }
    
    fn write_u8_slice(&mut self, slice: &[u8], with_len: bool) -> EncRes<Self::Error> {
        if with_len {
            Ok(
                self.write_usize(slice.len())?
                + self.write_exact(slice)?
            )
        } else {
            self.write_exact(slice)
        }
    }
    
    fn write_u16_slice(&mut self, slice: &[u16], with_len: bool) -> EncRes<Self::Error> {
        let mut counter = Counter::new();
        if with_len {
            counter.incr(self.write_usize(slice.len()))?;
        }
        for elem in slice.iter().copied() {
            counter.incr(self.write_u16(elem))?;
        }
        counter.ok()
    }
    
    fn write_u32_slice(&mut self, slice: &[u32], with_len: bool) -> EncRes<Self::Error> {
        let mut counter = Counter::new();
        if with_len {
            counter.incr(self.write_usize(slice.len()))?;
        }
        for elem in slice.iter().copied() {
            counter.incr(self.write_u32(elem))?;
        }
        counter.ok()
    }
    
    fn write_u64_slice(&mut self, slice: &[u64], with_len: bool) -> EncRes<Self::Error> {
        let mut count = Counter::new();
        if with_len {
            count.incr(self.write_usize(slice.len()))?;
        }
        for elem in slice.iter().copied() {
            count.incr(self.write_u64(elem))?;
        }
        count.ok()
    }
    
    fn write_u128_slice(&mut self, slice: &[u128], with_len: bool) -> EncRes<Self::Error> {
        let mut count = Counter::new();
        if with_len {
            count.incr(self.write_usize(slice.len()))?;
        }
        for elem in slice.iter().copied() {
            count.incr(self.write_u128(elem))?;
        }
        count.ok()
    }
    
    fn write_usize_slice(&mut self, slice: &[usize], with_len: bool) -> EncRes<Self::Error> {
        let mut count = Counter::new();
        if with_len {
            count.incr(self.write_usize(slice.len()))?;
        }
        for elem in slice.iter().copied() {
            count.incr(self.write_usize(elem))?;
        }
        count.ok()
    }
    
    fn write_i8_slice(&mut self, slice: &[i8], with_len: bool) -> EncRes<Self::Error> {
        unsafe {
            self.write_u8_slice(cast_slice(slice), with_len)
        }
    }
    
    fn write_i16_slice(&mut self, slice: &[i16], with_len: bool) -> EncRes<Self::Error> {
        unsafe {
            self.write_u16_slice(cast_slice(slice), with_len)
        }
    }
    
    fn write_i32_slice(&mut self, slice: &[i32], with_len: bool) -> EncRes<Self::Error> {
        unsafe {
            self.write_u32_slice(cast_slice(slice), with_len)
        }
    }
    
    fn write_i64_slice(&mut self, slice: &[i64], with_len: bool) -> EncRes<Self::Error> {
        unsafe {
            self.write_u64_slice(cast_slice(slice), with_len)
        }
    }
    
    fn write_i128_slice(&mut self, slice: &[i128], with_len: bool) -> EncRes<Self::Error> {
        unsafe {
            self.write_u128_slice(cast_slice(slice), with_len)
        }
    }
    
    fn write_isize_slice(&mut self, slice: &[isize], with_len: bool) -> EncRes<Self::Error> {
        unsafe {
            self.write_usize_slice(cast_slice(slice), with_len)
        }
    }
    
    fn write_bool_slice(&mut self, slice: &[bool], with_len: bool) -> EncRes<Self::Error> {
        let mut count = Counter::new();
        if with_len {
            count.incr(self.write_usize(slice.len()))?;
        }
        for elem in slice.iter().copied() {
            count.incr(self.write_bool(elem))?;
        }
        count.ok()
    }
    
    fn write_char_slice(&mut self, slice: &[char], with_len: bool) -> EncRes<Self::Error> {
        let mut count = Counter::new();
        if with_len {
            count.incr(self.write_usize(slice.len()))?;
        }
        for elem in slice.iter().copied() {
            count.incr(self.write_char(elem))?;
        }
        count.ok()
    }
}

pub trait Encode {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<u64, E::Error>;
}

macro_rules! primitive_encode_impls {
    ($(
        $encode_fn:ident($impl_ty:ty)
    )+) => {
        $(
            impl Encode for $impl_ty {
                #[inline]
                fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<u64, E::Error> {
                    encoder.$encode_fn(*self)
                }
            }
        )*
    };
}

primitive_encode_impls!(
    write_u8(u8)
    write_u16(u16)
    write_u32(u32)
    write_u64(u64)
    write_u128(u128)
    write_usize(usize)
    write_i8(i8)
    write_i16(i16)
    write_i32(i32)
    write_i64(i64)
    write_i128(i128)
    write_isize(isize)
    write_bool(bool)
    write_char(char)
);

fn encode_sized_slice<T: Encode + 'static, E: Encoder>(slice: &[T], encoder: &mut E) -> Result<u64, E::Error> {
    if
        TypeId::of::<T>() == TypeId::of::<u8>()
        || TypeId::of::<T>() == TypeId::of::<i8>()
    {
        let slice: &[u8] = unsafe { cast_slice(slice) };
        encoder.write_u8_slice(slice, true)
    } else {
        let mut counter = Counter::new();
        counter.incr(encoder.write_usize(slice.len()))?;
        for item in slice.iter() {
            counter.incr(item.encode(encoder))?;
        }
        counter.ok()
    }
}

impl<T: Encode + 'static> Encode for Vec<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<u64, E::Error> {
        encode_sized_slice(self.as_slice(), encoder)
    }
}

impl<T: Encode + 'static> Encode for Box<[T]> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<u64, E::Error> {
        encode_sized_slice(self.as_ref(), encoder)
    }
}