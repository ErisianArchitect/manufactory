use ::core::hash::{
    BuildHasher,
    Hash,
    Hasher,
};

pub const GOLDEN_RATIO_64: u64 = 0x9e3779b97f4a7c15;
pub const DEADBEEF_64: u64 = 0xDEADBEEF;
pub const PREMIX: u64 = DEADBEEF_64.wrapping_mul(GOLDEN_RATIO_64);
pub const SALTS_LEN: usize = 128;

/// Simple deterministic algorithm for deriving salts that are (hopefully) good
/// enough for simple deterministic hashing.
const fn fill_derived_salts(salts: &mut [u64]) {
    let mut index = 0usize;
    while index < salts.len() {
        salts[index] = GOLDEN_RATIO_64.wrapping_mul(
            ((index as u64) + 1)
                .wrapping_pow(3)
                .wrapping_mul(GOLDEN_RATIO_64)
                .wrapping_add(DEADBEEF_64)
        );
        index += 1;
    }
}

/// Simple deterministic algorithm for deriving salts that are (hopefully) good
/// enough for simple deterministic hashing.
#[must_use]
const fn derive_salts<const LEN: usize>() -> [u64; LEN] {
    let mut salts = [0u64; LEN];
    fill_derived_salts(&mut salts);
    salts
}

// This is commented out because it is not being used, but it should
// be used at least once if the length of the salts changes.
// /// This function will panic if the salts are not unique.
// #[must_use]
// const fn derive_unique_salts<const LEN: usize>() -> [u64; LEN] {
//     // salts might not be ideal if there are repeats, so this function
//     // can be used to check if there are repeats.
//     // In practice, there aren't likely to be any repeats with the derive
//     // algorithm, at least for many iterations, so this shouldn't be
//     // necessary.
//     #[inline(always)]
//     const fn has_repeat(arr: &[u64]) -> bool {
//         let mut x = 0usize;
//         while x < arr.len() {
//             let mut y = x + 1usize;
//             while y < arr.len() {
//                 if arr[x] == arr[y] {
//                     return true;
//                 }
//                 y += 1;
//             }
//             x += 1;
//         }
//         false
//     }
//     let mut salts = [0u64; LEN];
//     fill_derived_salts(&mut salts);
//     if has_repeat(&salts) {
//         panic!("Repeat found in salts sequence. Tough luck.")
//     }
//     salts
// }

const SALTS: [u64; SALTS_LEN] = derive_salts();

#[derive(Debug, Default, Clone, Copy)]
pub struct Accumulator {
    accum: u64,
    count: u32,
}

impl Accumulator {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            accum: 0,
            count: 0,
        }
    }
    
    pub const fn push(&mut self, byte: u8) -> Option<u64> {
        let cur_count = self.count;
        let shifted = (byte as u64) << (cur_count * 8);
        self.accum |= shifted;
        if cur_count == 7 {
            self.count = 0;
            Some(self.accum)
        } else {
            self.count = cur_count + 1;
            None
        }
    }
    
    #[inline]
    #[must_use]
    pub const fn finish(self) -> u64 {
        self.accum
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FastHash {
    hash: u64,
    accum: Accumulator,
    index: usize,
}

/// Effectively scatters the bits of `value` deterministically.
#[inline]
#[must_use]
pub const fn fast_mix(value: u64) -> u64 {
    value.wrapping_add(PREMIX)
        .wrapping_mul(GOLDEN_RATIO_64)
        .rotate_left(23)
}

#[inline]
#[must_use]
pub const fn get_salt(index: usize) -> u64 {
    SALTS[index % SALTS.len()]
}

/// Get a mixed salt. This also works as a pseudo-random number
/// generator coincidentally.
#[inline]
#[must_use]
pub const fn get_mixed_salt(index: usize) -> u64 {
    fast_mix(get_salt(index) ^ fast_mix(index as u64))
}

impl FastHash {
    
    pub const ZERO: Self = Self::with_seed(0);
    
    /// Simply initializes the [FastHasher] with the starting hash set
    /// to `seed`.
    #[inline]
    #[must_use]
    pub const fn with_seed(seed: u64) -> Self {
        Self {
            hash: seed,
            accum: Accumulator::new(),
            index: 0,
        }
    }
    
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self::with_seed(0)
    }
    
    #[inline]
    #[must_use]
    pub const fn new_deadbeef() -> Self {
        Self::with_seed(DEADBEEF_64)
    }
    
    /// Update the hasher with the given bytes then return the hash state.
    pub const fn update(&mut self, data: &[u8]) -> &mut Self {
        let mut index = self.index;
        let mut byte_index = 0usize;
        while byte_index < data.len() {
            let byte = data[byte_index];
            if let Some(accum) = self.accum.push(byte) {
                let salt = get_mixed_salt(index);
                self.hash ^= fast_mix(accum ^ salt);
                index += 1;
            }
            byte_index += 1;
        }
        self.index = index;
        self
    }
    
    /// Computes the final hash.
    /// This does not mutate the hasher state.
    pub const fn finish(&self) -> u64 {
        if self.accum.count != 0 {
            let accum = self.accum.finish();
            let salt = get_mixed_salt(self.index);
            self.hash ^ fast_mix(accum ^ salt)
        } else {
            self.hash
        }
    }
    
    /// The same as calling [FastHasher::finish].
    pub const fn hash(&self) -> u64 {
        self.finish()
    }
    
    pub fn seeded_oneshot(seed: u64, data: &[u8]) -> u64 {
        let mut hasher = Self::with_seed(seed);
        hasher.update(data);
        hasher.finish()
    }
    
    pub fn oneshot(data: &[u8]) -> u64 {
        Self::seeded_oneshot(0, data)
    }
    
    pub fn deadbeef_oneshot(data: &[u8]) -> u64 {
        Self::seeded_oneshot(0xDEADBEEF, data)
    }
    
    pub fn hash_one_with_seed<T: Hash>(seed: u64, value: T) -> u64 {
        let mut hasher = Self::with_seed(seed);
        value.hash(&mut hasher);
        hasher.finish()
    }
    
    pub fn hash_one<T: Hash>(value: T) -> u64 {
        Self::hash_one_with_seed(0, value)
    }
}

macro_rules! int_writers {
    () => {
        int_writers!(
            write_u8(u8);
            write_u16(u16);
            write_u32(u32);
            write_u64(u64);
            write_u128(u128);
            write_i8(i8);
            write_i16(i16);
            write_i32(i32);
            write_i64(i64);
            write_i128(i128);
        );
    };
    ($func_name:ident($type:ty)) => {
        fn $func_name(&mut self, i: $type) {
            let bytes = i.to_le_bytes();
            self.update(&bytes);
        }
    };
    ($(
        $func_name:ident($type:ty);
    )+) => {
        $(
            int_writers!($func_name($type));
        )*
    };
}

impl Hasher for FastHash {
    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
    
    int_writers!();
    
    fn write_isize(&mut self, i: isize) {
        // for maximum compatibility, we will convert to i128.
        self.write_i128(i as i128);
    }
    
    fn write_usize(&mut self, i: usize) {
        self.write_u128(i as u128);
    }
    
    fn finish(&self) -> u64 {
        self.finish()
    }
}

/// A 64-bit seed used for hashing with [FastHasher].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HashSeed64(u64);

impl HashSeed64 {
    pub const ZERO: Self = HashSeed64(0);
    pub const DEADBEEF: Self = HashSeed64(DEADBEEF_64);
    pub const GOLDEN_RATIO: Self = HashSeed64(GOLDEN_RATIO_64);
    
    #[inline]
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        Self(seed)
    }
    
    #[inline]
    #[must_use]
    pub fn hash<T: Hash>(self, source: &T) -> u64 {
        seeded_hash(self.0, source)
    }
    
    #[inline]
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl BuildHasher for HashSeed64 {
    type Hasher = FastHash;
    
    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FastHash::with_seed(self.0)
    }
    
    fn hash_one<T: Hash>(&self, x: T) -> u64
        where
            Self: Sized,
            Self::Hasher: Hasher, {
        FastHash::hash_one_with_seed(self.0, x)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HashSeed256([u64; 4]);

#[inline]
const fn scramble_lanes(lanes: &mut [u64; 4]) {
    let mut index = 0usize;
    while index < 4 {
        lanes[index] = fast_mix(lanes[index] ^ get_mixed_salt(index));
        index += 1;
    }
}

impl HashSeed256 {
    
    #[inline]
    pub const fn scramble(&mut self) {
        scramble_lanes(&mut self.0);
    }
    
    #[inline]
    #[must_use]
    pub const fn scrambled(mut self) -> Self {
        self.scramble();
        self
    }
    
    #[inline(always)]
    #[must_use]
    pub const fn new(seed: [u8; 32]) -> Self {
        Self(seed_to_lanes(seed))
    }
    
    #[inline(always)]
    #[must_use]
    pub const fn from_lanes(lanes: [u64; 4]) -> Self {
        Self(lanes)
    }
    
    /// This will do some basic hashing of the 64-bit seed to derive
    /// the 256-bit seed.
    #[inline(always)]
    #[must_use]
    pub const fn from_u64(seed: u64) -> Self {
        Self([
            fast_mix(seed ^ get_mixed_salt(0)),
            fast_mix(seed ^ get_mixed_salt(1)),
            fast_mix(seed ^ get_mixed_salt(2)),
            fast_mix(seed ^ get_mixed_salt(3)),
        ])
    }
    
    pub fn hash<T: Hash>(&self, value: T) -> [u8; 32] {
        let mut hasher = FastHash256::with_seed_lanes(self.0);
        value.hash(&mut hasher);
        hasher.finish()
    }
    
    pub fn hash64<T: Hash>(&self, value: T) -> u64 {
        let mut hasher = FastHash256::with_seed_lanes(self.0);
        value.hash(&mut hasher);
        hasher.finish_u64()
    }
    
    #[inline(always)]
    #[must_use]
    pub const fn seed_bytes(self) -> [u8; 32] {
        lanes_to_bytes(self.0)
    }
    
    #[inline(always)]
    #[must_use]
    pub const fn seed_lanes(self) -> [u64; 4] {
        self.0
    }
    
    pub const fn build_hasher(&self) -> FastHash256 {
        FastHash256::with_seed_lanes(self.0)
    }
}

impl BuildHasher for HashSeed256 {
    type Hasher = FastHash256;
    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        FastHash256::with_seed_lanes(self.0)
    }
    
    fn hash_one<T: Hash>(&self, x: T) -> u64
        where
            Self: Sized,
            Self::Hasher: Hasher, {
        let mut hasher = self.build_hasher();
        x.hash(&mut hasher);
        hasher.finish_u64()
    }
}

#[inline]
#[must_use]
#[cfg(target_endian = "little")]
const fn lanes_to_bytes(lanes: [u64; 4]) -> [u8; 32] {
    // since we want little endian bytes, and the target platform
    // is little endian, we can just transmute.
    unsafe {
        ::core::mem::transmute(lanes)
    }
}

#[inline]
#[must_use]
#[cfg(target_endian = "big")]
const fn lanes_to_bytes(lanes: [u64; 4]) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    macro_rules! write_lane {
        ($index:literal) => {
            {
                let index: usize = $index;
                let offset = (index * 8);
                // must ALWAYS use `to_le_bytes` to ensure compatibility across platforms.
                let lane_bytes = lanes[index].to_le_bytes();
                unsafe {
                    ::core::ptr::copy_nonoverlapping(lane_bytes.as_ptr(), bytes.as_mut_ptr().add(offset), 8);
                }
            }
        };
    }
    write_lane!(0);
    write_lane!(1);
    write_lane!(2);
    write_lane!(3);
    bytes
}

#[inline]
#[must_use]
const fn seed_to_lanes(seed: [u8; 32]) -> [u64; 4] {
    let mut lanes = [0u64; 4];
    let mut index = 0;
    while index < 4 {
        let mut bytes = [0u8; 8];
        unsafe {
            ::core::ptr::copy_nonoverlapping(seed.as_ptr().add(index * 8), bytes.as_mut_ptr(), 8);
        }
        lanes[index] = u64::from_le_bytes(bytes);
        index += 1;
    }
    lanes
}

#[inline]
const fn mix_lanes(lanes: &mut [u64; 4], index: usize, accum: u64) {
    lanes[0] ^= fast_mix(accum ^ get_mixed_salt(index));
    lanes[1] ^= fast_mix(accum ^ get_mixed_salt(index + 1));
    lanes[2] ^= fast_mix(accum ^ get_mixed_salt(index + 2));
    lanes[3] ^= fast_mix(accum ^ get_mixed_salt(index + 3));
}

pub struct FastHash256 {
    lanes: [u64; 4],
    accum: Accumulator,
    index: usize,
}

impl FastHash256 {
    #[inline]
    #[must_use]
    pub const fn with_seed(seed: [u8; 32]) -> Self {
        Self::with_seed_lanes(seed_to_lanes(seed))
    }
    
    #[must_use]
    pub const fn with_seed_lanes(lanes: [u64; 4]) -> Self {
        Self {
            lanes,
            accum: Accumulator::new(),
            index: 0,
        }
    }
    
    #[inline]
    #[must_use]
    pub const fn with_seed_u64(seed: u64) -> Self {
        HashSeed256::from_u64(seed).build_hasher()
    }
    
    #[inline]
    pub const fn scramble_hash(&mut self) {
        scramble_lanes(&mut self.lanes);
    }
    
    #[inline]
    #[must_use]
    pub const fn with_scrambled_hash(mut self) -> Self {
        self.scramble_hash();
        self
    }
    
    pub const fn update(&mut self, data: &[u8]) -> &mut Self {
        let mut index = self.index;
        let mut byte_index = 0usize;
        while byte_index < data.len() {
            let byte = data[byte_index];
            if let Some(accum) = self.accum.push(byte) {
                mix_lanes(&mut self.lanes, index, accum);
                index = index.wrapping_add(4);
            }
            byte_index += 1;
        }
        self.index = index;
        self
    }
    
    #[must_use]
    pub const fn finish(&self) -> [u8; 32] {
        let mut lanes = self.lanes;
        if self.accum.count != 0 {
            mix_lanes(&mut lanes, self.index, self.accum.finish());
        }
        lanes_to_bytes(lanes)
    }
    
    #[must_use]
    pub const fn finish_u64(&self) -> u64 {
        if self.accum.count != 0 {
            let mut lanes = self.lanes;
            mix_lanes(&mut lanes, self.index, self.accum.finish());
            lanes[0] ^ lanes[1] ^ lanes[2] ^ lanes[3]
        } else {
            self.lanes[0] ^ self.lanes[1] ^ self.lanes[2] ^ self.lanes[3]
        }
    }
    
    #[must_use]
    pub const fn new() -> Self {
        Self::with_seed([0u8; 32])
    }
}

impl Hasher for FastHash256 {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
    
    int_writers!();
    
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u128(i as u128);
    }
    
    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write_i128(i as i128);
    }
    
    #[inline]
    fn finish(&self) -> u64 {
        self.finish_u64()
    }
}

/// Oneshot hash using [XxHash64] with the given `seed`.
/// 
/// Note: The results of this function will differ from the results
/// of the same slice being passed to the [hash] or [seeded_hash] functions.
#[inline]
#[must_use]
pub fn seeded_oneshot(seed: u64, data: &[u8]) -> u64 {
    FastHash::seeded_oneshot(seed, data)
}

/// Oneshot hash using [XxHash64] with a seed of `0`.
/// 
/// Note: The results of this function will differ from the results
/// of the same slice being passed to the [hash] or [seeded_hash] functions.
#[inline]
#[must_use]
pub fn oneshot(data: &[u8]) -> u64 {
    FastHash::seeded_oneshot(0, data)
}

#[inline]
#[must_use]
pub fn hash_str_with_seed(seed: u64, s: &str) -> u64 {
    FastHash::seeded_oneshot(seed, s.as_bytes())
}

#[inline]
#[must_use]
pub fn hash_str(s: &str) -> u64 {
    FastHash::seeded_oneshot(0, s.as_bytes())
}

/// Get 64-bit hash using [XxHash64] as a hasher with the given `seed`.
#[inline]
#[must_use]
pub fn seeded_hash<T: Hash>(seed: u64, source: T) -> u64 {
    FastHash::hash_one_with_seed(seed, source)
}

/// Get a 64-bit hash using [XxHash64] as a hasher with a seed of `0`.
#[inline]
#[must_use]
pub fn hash<T: Hash>(source: T) -> u64 {
    seeded_hash(0, source)
}

/*
These salts were generated using some basic Python code:
```
import secrets

def token_u64()->int:
    secure_bytes = secrets.token_bytes(8)
    return int.from_bytes(secure_bytes, signed = False)

def token_u64_hex()->str:
    token = token_u64()
    return f'0x{token:016X}'

def salt_row(width: int)->str:
    return ' '.join((f'{token_u64_hex()},' for _ in range(width)))

def salt_table(cols: int, rows: int, indent: str | int = '')->str:
    if isinstance(indent, int):
        indent = ' ' * indent
    return '\n'.join(f'{indent}{salt_row(cols)}' for _ in range(rows))
```
*/
// const SALTS: [u64; 64] = [
//     0xC23FFE61CD4929A5, 0x179207BC8A78BB38, 0x7242DF53DDFD8269, 0x1ABBCCC2FA757D5C,
//     0x1CB95F5BDD01276E, 0x7EB3E6E23322CD5F, 0xDE8A83183670E5C0, 0x77A3E36A1EC33DF6,
//     0xDE123EBEB6A19D15, 0x989DD146929281E9, 0xF4D970CF6DE21350, 0xCDC40951E2557CCD,
//     0xBDB2572CA2F86B62, 0x46B6E1A8DCF61BA5, 0x313D19D223DBE8D5, 0x10079D83B0BB730C,
//     0xC42635B49B75D502, 0x28D4276B4316A6D6, 0x24F74AF3C6E95B3A, 0x5169D9512B576B93,
//     0x2581D1E09610D323, 0x82D7685F8391C3A4, 0x9785DFFA4BCB32C6, 0x525E3C9C0E9A847D,
//     0xF8D1374BEFA0D6B7, 0x78E2A5F569C3149B, 0x3FF3242A828B412D, 0x9017ED3071D3B19F,
//     0xEA42775FF3E3B35E, 0xF4B271DD3602B8D2, 0x8DD1AB5C74348E27, 0xEBC0252F2DAD9E81,
//     0x3513511BA9EF9DAA, 0x6AFD6E9140A494BB, 0xA2E992851BD2CAA7, 0xF1320FF5EC28DAF1,
//     0xEB82E56C5F9E8C73, 0x1E540B165A9C29F3, 0x941AB4BED9F725D2, 0x4BCE6BBD9BD0F0B6,
//     0x86D9F5FFA0811354, 0x6E92BD345BAB5F9F, 0x7BD7065BDC0570F0, 0xCE6B7D1B3FBDD725,
//     0x5F0DD68B9C8C1DA7, 0xD89CA06E4E00FEFF, 0x4C16833C569110B0, 0x6E199F043FF610CC,
//     0x8F4C5FA3F303F244, 0x9573BFEC70DD032D, 0x0EA87A342E692910, 0xB1A82BB2F8B73236,
//     0x191BA0E0C3E1A5E7, 0x1477E1797D03016F, 0xD7E766BCFF3F29F1, 0x41A7E7E82C5DCFEB,
//     0x3E788E097835B305, 0xFB4B6EF8980CF5E1, 0x6BF5BD4B84085473, 0x287BE1E206399AFA,
//     0x293213F7C50B0FB0, 0xFDA6F93146E7538D, 0xF0C415A21D125915, 0xDE5BD05AE74AF9AC,
// ];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    
    #[test]
    fn mixed_salt_test() {
        let mut set = HashSet::new();
        for i in 0..100000 {
            let salt = get_mixed_salt(i);
            if !set.insert(salt) {
                println!("{salt}");
            }
        }
    }
    
    #[test]
    fn fast_hasher_test() {
        use std::time::*;
        let start = Instant::now();
        let mut hasher1 = FastHash::new();
        let mut hasher2 = FastHash::new();
        for _ in 0..10000 {
            let hash1 = hasher1.update(b"The quick brown fox jumps over the lazy dog.").hash();
            let hash2 = hasher2.update(b"The quick brown fox jumps over the lazy dog.").hash();
            assert_eq!(hash1, hash2);
        }
        let hash1 = hasher1.finish();
        let hash2 = hasher2.finish();
        let elapsed = start.elapsed();
        assert_eq!(hash1, hash2);
        println!("Hash: {hash1}\nIn {elapsed:.3?}");
    }

    #[test]
    fn arbitrary_hash_test() {
        let hash1 = FastHash::hash_one(&(0xDEADBEEFu64, 45i8, "Hello, world"));
        let hash2 = FastHash::hash_one(&(0xDEADBEEFu64, 45i8, "Hello, world"));
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn fast_hash256_test() {
        let mut hasher = FastHash256::new();
        hasher.update(b"The quick brown fox jumps over the lazy dog.");
        let hash1 = hasher.finish();
        println!("{hash1:?}");
    }
}