use std::time::Instant;

use mfhash::{HashSeed, deterministic::DeterministicHash};
use mffmt::hex::HexBytes as Hex;

#[allow(unused)]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum U7Niche {
    V0 = 0,
    V1 = 1,
    V2 = 2,
    V3 = 3,
    V4 = 4,
    V5 = 5,
    V6 = 6,
    V7 = 7,
    V8 = 8,
    V9 = 9,
    V10 = 10,
    V11 = 11,
    V12 = 12,
    V13 = 13,
    V14 = 14,
    V15 = 15,
    V16 = 16,
    V17 = 17,
    V18 = 18,
    V19 = 19,
    V20 = 20,
    V21 = 21,
    V22 = 22,
    V23 = 23,
    V24 = 24,
    V25 = 25,
    V26 = 26,
    V27 = 27,
    V28 = 28,
    V29 = 29,
    V30 = 30,
    V31 = 31,
    V32 = 32,
    V33 = 33,
    V34 = 34,
    V35 = 35,
    V36 = 36,
    V37 = 37,
    V38 = 38,
    V39 = 39,
    V40 = 40,
    V41 = 41,
    V42 = 42,
    V43 = 43,
    V44 = 44,
    V45 = 45,
    V46 = 46,
    V47 = 47,
    V48 = 48,
    V49 = 49,
    V50 = 50,
    V51 = 51,
    V52 = 52,
    V53 = 53,
    V54 = 54,
    V55 = 55,
    V56 = 56,
    V57 = 57,
    V58 = 58,
    V59 = 59,
    V60 = 60,
    V61 = 61,
    V62 = 62,
    V63 = 63,
    V64 = 64,
    V65 = 65,
    V66 = 66,
    V67 = 67,
    V68 = 68,
    V69 = 69,
    V70 = 70,
    V71 = 71,
    V72 = 72,
    V73 = 73,
    V74 = 74,
    V75 = 75,
    V76 = 76,
    V77 = 77,
    V78 = 78,
    V79 = 79,
    V80 = 80,
    V81 = 81,
    V82 = 82,
    V83 = 83,
    V84 = 84,
    V85 = 85,
    V86 = 86,
    V87 = 87,
    V88 = 88,
    V89 = 89,
    V90 = 90,
    V91 = 91,
    V92 = 92,
    V93 = 93,
    V94 = 94,
    V95 = 95,
    V96 = 96,
    V97 = 97,
    V98 = 98,
    V99 = 99,
    V100 = 100,
    V101 = 101,
    V102 = 102,
    V103 = 103,
    V104 = 104,
    V105 = 105,
    V106 = 106,
    V107 = 107,
    V108 = 108,
    V109 = 109,
    V110 = 110,
    V111 = 111,
    V112 = 112,
    V113 = 113,
    V114 = 114,
    V115 = 115,
    V116 = 116,
    V117 = 117,
    V118 = 118,
    V119 = 119,
    V120 = 120,
    V121 = 121,
    V122 = 122,
    V123 = 123,
    V124 = 124,
    V125 = 125,
    V126 = 126,
    V127 = 127,
}

#[repr(C, align(1))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Align1;

#[repr(C, align(2))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Align2;

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Align4;

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Align8;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Align16;

#[cfg(target_endian = "big")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NichePacking<const LEN: usize, A> {
    _align: A,
    _niche: U7Niche,
    _mem: [u8; LEN],
}


#[cfg(target_endian = "little")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NichePacking<const LEN: usize, A> {
    _align: A,
    _mem: [u8; LEN],
    _niche: U7Niche,
}

#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct u31(NichePacking<3, Align4>);

impl u31 {
    pub(crate) const MIN_U32: u32 = 0u32;
    pub(crate) const MAX_U32: u32 = 0x_7f_ff_ff_ff;
    pub const MIN: Self = unsafe { Self::new_unchecked(Self::MIN_U32) };
    pub const MAX: Self = unsafe { Self::new_unchecked(Self::MAX_U32) };
    
    #[inline(always)]
    pub const fn get(self) -> u32 {
        unsafe { ::core::mem::transmute(self) }
    }

    #[inline(always)]
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        debug_assert!(value <= Self::MAX_U32);
        unsafe { ::core::mem::transmute(value) }
    }

    #[inline(always)]
    pub const fn new_truncated(value: u32) -> Self {
        unsafe { Self::new_unchecked(value & Self::MAX_U32) }
    }

    #[inline(always)]
    pub const fn new(value: u32) -> Option<Self> {
        if value > Self::MAX.get() {
            return None;
        }
        Some(unsafe { ::core::mem::transmute(value) })
    }
}

// const SIZE: usize = size_of::<Option<u31>>();
macro_rules! jank {($($tokens:tt)*) => {};}

const FOO_MAGIC: &'static [u8] = b"_foofoo_";

const PAGE_SIZE: u64 = 4096;


jank!(
    // `jank` uses a DSL to define a file type and associated data.
    // Here, we define an associated type for this file type.
    // This is going to be a `File` type, which means that the content inside the braces will use
    // file type declaration syntax.
    struct Foo(File) {
        // setting the `size` to `4096` enforces that the total size cannot exceed 4096 bytes, and that
        // pad bytes will be added to make the total size 4096 if it is less than 4096 bytes.
        // content inside `${}`, `$()`, `${}`, or `$<>` are treated as special syntax.
        // - `$()`: Rust Eval. Inner tokens are treated as a Rust expression.
        // - `${}`: jank eval. Inner tokens are treated as a jank expression.
        layout {
            header[size=$(PAGE_SIZE)] {
                // const is used to insert exact data into the file. This data must be known at compile time.
                // Common acceptable types:
                // - byte array
                // - string
                // - 
                const[size=8] b"_header_";
                const[size=8] $(FOO_MAGIC);
                // using `reserve` is like a contract saying that this space might eventually be used for something.
                // The space is reserved so that future versions of the engine can still be compatible with old versions of the file.
                reserve(16);
                // In this example, the checksum would be calculated by the user. I can't currently think of how I could handle a checksum system.
                priv checksum: [u8; 32] = ${
                    build_checksum(
                        
                    )
                },
            }
            footer[size=$(PAGE_SIZE)] {
                
            }
        }
    }
);

fn main() {
    let mut items = vec![1234u64; 4096];
    let expected_len = items.len() + 1;
    let timer = Instant::now();
    items.insert(0, 42069u64);
    let elapsed = timer.elapsed();
    assert_eq!(items.len(), expected_len);
    println!("Elapsed: {elapsed:.3?}");
    // let value = u31::new_truncated(123456789);
    // let opt = Some(value);
    // let val_u32: u32 = unsafe { ::core::mem::transmute(opt) };
    // println!("{}\n{val_u32}", value.get());
    // assert_eq!(val_u32, value.get());
    // assert_eq!(
    //     size_of::<u32>(),
    //     size_of::<u31>(),
    // );
    // assert_eq!(
    //     size_of::<u31>(),
    //     size_of::<Option<u31>>(),
    // );
    // assert_eq!(
    //     size_of::<u31>(),
    //     size_of::<Option<Option<Option<Option<u31>>>>>(),
    // );
    // let seed = HashSeed::derived("This is a test.");
    // let mut hasher = seed.build_hasher();
    // let value = ([1, 2, 3], "apple");
    // value.deterministic_hash(&mut hasher);
    // let hash = hasher.finalize_u128();
    // let hash_bytes: [u8; 16] = hasher.finalize_bytes();
    // println!("{hash:032x}");
    // println!("{}", Hex(&hash_bytes));
}
