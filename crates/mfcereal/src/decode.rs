use std::ffi::CString;


#[inline(always)]
const fn size_align_eq<L, R>() -> bool {
    const {
        size_of::<L>() == size_of::<R>()
        && align_of::<L>() == align_of::<R>()
    }
}

#[inline(always)]
#[track_caller]
const unsafe fn cast_mut_slice<'a, Src, Dst>(src: &'a mut [Src]) -> &'a mut [Dst] {
    const {
        if !size_align_eq::<Src, Dst>() {
            panic!("Size and Alignment of Src and Dst must match.");
        }
    }
    unsafe {
        ::core::mem::transmute(src)
    }
}

/// Read value of type `T` from `decoder` using transformer
/// function `f` which takes an array of `LEN` bytes.
#[inline(always)]
pub fn decoder_read_value<
    const LEN: usize,
    D: Decoder,
    T,
    F: FnOnce([u8; LEN]) -> T,
>(
    decoder: &mut D,
    f: F,
) -> Result<T, D::Error> {
    let mut buf = [0u8; LEN];
    decoder.read_exact(&mut buf)?;
    Ok(f(buf))
}

/// Read [Vec<T>] from `decoder` ([Decoder]) using function `f`
/// to read individual values.
#[inline(always)]
pub fn decoder_read_vec<
    D: Decoder,
    T,
    F: Fn(&mut D) -> Result<T, D::Error>,
>(
    decoder: &mut D,
    f: F,
) -> Result<Vec<T>, D::Error> {
    let capacity = decoder.read_usize()?;
    let mut buf = Vec::with_capacity(capacity);
    for _ in 0..capacity {
        buf.push(f(decoder)?);
    }
    Ok(buf)
}

#[inline(always)]
pub fn decoder_read_slice<
    D: Decoder,
    E,
    T,
    F: Fn(&mut D) -> Result<T, E>,
>(
    decoder: &mut D,
    f: F,
    output: &mut [T],
) -> Result<(), E> {
    output.iter_mut().try_for_each(move |value| {
        *value = f(decoder)?;
        Ok(())
    })
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError<E> {
    #[error("Invalid unicode {0}")]
    InvalidChar(u32),
    #[error("Utf-8 Error: {0}")]
    Utf8Error(#[from] ::std::string::FromUtf8Error),
    #[error("From Vec With Nul Error: {0}")]
    FromVecWithNul(#[from] ::std::ffi::FromVecWithNulError),
    #[error("Decoder Error: {0}")]
    DecoderError(E),
}

impl<E> DecodeError<E> {
    #[inline(always)]
    pub fn map<T>(result: Result<T, E>) -> Result<T, Self> {
        result.map_err(Self::DecoderError)
    }
}

pub trait Decoder: Sized {
    type Error;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
    
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        decoder_read_value(self, u8::from_be_bytes)
    }
    
    fn read_u16(&mut self) -> Result<u16, Self::Error> {
        decoder_read_value(self, u16::from_be_bytes)
    }
    
    fn read_u32(&mut self) -> Result<u32, Self::Error> {
        decoder_read_value(self, u32::from_be_bytes)
    }
    
    fn read_u64(&mut self) -> Result<u64, Self::Error> {
        decoder_read_value(self, u64::from_be_bytes)
    }
    
    fn read_u128(&mut self) -> Result<u128, Self::Error> {
        decoder_read_value(self, u128::from_be_bytes)
    }
    
    fn read_usize(&mut self) -> Result<usize, Self::Error> {
        decoder_read_value(self, |bytes: [u8; 8]| u64::from_be_bytes(bytes) as usize)
    }
    
    fn read_i8(&mut self) -> Result<i8, Self::Error> {
        decoder_read_value(self, i8::from_be_bytes)
    }
    
    fn read_i16(&mut self) -> Result<i16, Self::Error> {
        decoder_read_value(self, i16::from_be_bytes)
    }
    
    fn read_i32(&mut self) -> Result<i32, Self::Error> {
        decoder_read_value(self, i32::from_be_bytes)
    }
    
    fn read_i64(&mut self) -> Result<i64, Self::Error> {
        decoder_read_value(self, i64::from_be_bytes)
    }
    
    fn read_i128(&mut self) -> Result<i128, Self::Error> {
        decoder_read_value(self, i128::from_be_bytes)
    }
    
    fn read_isize(&mut self) -> Result<isize, Self::Error> {
        decoder_read_value(self, |bytes: [u8; 8]| i64::from_be_bytes(bytes) as isize)
    }
    
    fn read_bool(&mut self) -> Result<bool, Self::Error> {
        decoder_read_value(self, |[byte]| byte != 0)
    }
    
    fn read_char(&mut self) -> Result<char, DecodeError<Self::Error>> {
        let mut bytes = [0u8; 4];
        if let Err(err) = self.read_exact(&mut bytes) {
            return Err(DecodeError::DecoderError(err));
        }
        let code = u32::from_be_bytes(bytes);
        char::from_u32(code)
            .ok_or_else(move || DecodeError::InvalidChar(code))
    }
    
    fn read_u16_slice(&mut self, output: &mut [u16]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_u16, output)
    }
    
    fn read_u32_slice(&mut self, output: &mut [u32]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_u32, output)
    }
    
    fn read_u64_slice(&mut self, output: &mut [u64]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_u64, output)
    }
    
    fn read_u128_slice(&mut self, output: &mut [u128]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_u128, output)
    }
    
    fn read_usize_slice(&mut self, output: &mut [usize]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_usize, output)
    }
    
    fn read_i8_slice(&mut self, output: &mut [i8]) -> Result<(), Self::Error> {
        self.read_exact(unsafe { cast_mut_slice(output) })
    }
    
    fn read_i16_slice(&mut self, output: &mut [i16]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_i16, output)
    }
    
    fn read_i32_slice(&mut self, output: &mut [i32]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_i32, output)
    }
    
    fn read_i64_slice(&mut self, output: &mut [i64]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_i64, output)
    }
    
    fn read_i128_slice(&mut self, output: &mut [i128]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_i128, output)
    }
    
    fn read_isize_slice(&mut self, output: &mut [isize]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_isize, output)
    }
    
    fn read_bool_slice(&mut self, output: &mut [bool]) -> Result<(), Self::Error> {
        decoder_read_slice(self, Self::read_bool, output)
    }
    
    fn read_char_slice(&mut self, output: &mut [char]) -> Result<(), DecodeError<Self::Error>> {
        decoder_read_slice(self, Self::read_char, output)
    }
    
    fn read_u8_vec(&mut self) -> Result<Vec<u8>, Self::Error> {
        let len = self.read_usize()?;
        let mut buf = vec![0u8; len];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }
    
    fn read_u16_vec(&mut self) -> Result<Vec<u16>, Self::Error> {
        decoder_read_vec(self, Self::read_u16)
    }
    
    fn read_u32_vec(&mut self) -> Result<Vec<u32>, Self::Error> {
        decoder_read_vec(self, Self::read_u32)
    }
    
    fn read_u64_vec(&mut self) -> Result<Vec<u64>, Self::Error> {
        decoder_read_vec(self, Self::read_u64)
    }
    
    fn read_u128_vec(&mut self) -> Result<Vec<u128>, Self::Error> {
        decoder_read_vec(self, Self::read_u128)
    }
    
    fn read_usize_vec(&mut self) -> Result<Vec<usize>, Self::Error> {
        decoder_read_vec(self, Self::read_usize)
    }
    
    fn read_i8_vec(&mut self) -> Result<Vec<i8>, Self::Error> {
        let bytes: Vec<u8> = self.read_u8_vec()?;
        // SAFETY: Vec<i8> has same size and alignment as Vec<u8>
        //         i8 has same size and alignment as u8
        unsafe {
            Ok(::core::mem::transmute(bytes))
        }
    }
    
    fn read_i16_vec(&mut self) -> Result<Vec<i16>, Self::Error> {
        decoder_read_vec(self, Self::read_i16)
    }
    
    fn read_i32_vec(&mut self) -> Result<Vec<i32>, Self::Error> {
        decoder_read_vec(self, Self::read_i32)
    }
    
    fn read_i64_vec(&mut self) -> Result<Vec<i64>, Self::Error> {
        decoder_read_vec(self, Self::read_i64)
    }
    
    fn read_i128_vec(&mut self) -> Result<Vec<i128>, Self::Error> {
        decoder_read_vec(self, Self::read_i128)
    }
    
    fn read_isize_vec(&mut self) -> Result<Vec<isize>, Self::Error> {
        decoder_read_vec(self, Self::read_isize)
    }
    
    fn read_bool_vec(&mut self) -> Result<Vec<bool>, Self::Error> {
        decoder_read_vec(self, Self::read_bool)
    }
    
    fn read_char_vec(&mut self) -> Result<Vec<char>, DecodeError<Self::Error>> {
        let len = DecodeError::map(self.read_usize())?;
        let mut output = Vec::with_capacity(len);
        for _ in 0..len {
            output.push(self.read_char()?);
        }
        Ok(output)
    }
    
    fn read_str(&mut self) -> Result<String, DecodeError<Self::Error>> {
        let string_bytes = DecodeError::map(self.read_u8_vec())?;
        Ok(String::from_utf8(string_bytes)?)
    }
    
    fn read_cstr(&mut self) -> Result<CString, DecodeError<Self::Error>> {
        let string_bytes = DecodeError::map(self.read_u8_vec())?;
        Ok(CString::from_vec_with_nul(string_bytes)?)
    }
}

#[inline(always)]
fn map_err<T, E>(result: Result<T, E>) -> Result<T, DecodeError<E>> {
    DecodeError::map(result)
}

pub trait Decode: Sized {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError<D::Error>>;
}

macro_rules! int_decode_impls {
    ($(
        $decoder_fn:ident -> $for_ty:ty
    )+) => {
        $(
            impl Decode for $for_ty {
                fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError<D::Error>> {
                    map_err(decoder.$decoder_fn())
                }
            }
        )*
    };
}

int_decode_impls!(
    read_u8 -> u8
    read_u16 -> u16
    read_u32 -> u32
    read_u64 -> u64
    read_u128 -> u128
    read_usize -> usize
    
    read_i8 -> i8
    read_i16 -> i16
    read_i32 -> i32
    read_i64 -> i64
    read_i128 -> i128
    read_isize -> isize
    
    
);

impl<T> Decode for (T,)
where
    T: Decode
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError<D::Error>> {
        Ok((T::decode(decoder)?,))
    }
}

impl<T0, T1> Decode for (T0, T1)
where
    T0: Decode,
    T1: Decode
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError<D::Error>> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
        ))
    }
}

impl<
    T0,
    T1,
    T2
> Decode for (
    T0,
    T1,
    T2
)
where
    T0: Decode,
    T1: Decode,
    T2: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError<D::Error>> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
        ))
    }
}

impl<
    T0,
    T1,
    T2,
    T3,
> Decode for (
    T0,
    T1,
    T2,
    T3,
)
where
    T0: Decode,
    T1: Decode,
    T2: Decode,
    T3: Decode,
{
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError<D::Error>> {
        Ok((
            T0::decode(decoder)?,
            T1::decode(decoder)?,
            T2::decode(decoder)?,
            T3::decode(decoder)?,
        ))
    }
}

fn decode<T: Decode, D: Decoder>(decoder: &mut D) -> Result<T, DecodeError<D::Error>> {
    T::decode(decoder)
}

#[test]
fn foo() {
    struct Dec;
    impl Decoder for Dec {
        type Error = ();
        fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
            buf.iter_mut().for_each(|v| *v = 0);
            Ok(())
        }
    }
    let value: (i32, i8, u64) = decode(&mut Dec).unwrap();
}