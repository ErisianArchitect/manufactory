pub mod deterministic;
// use blake3::Hash;
use deterministic::DeterministicHasher;

use crate::deterministic::DeterministicHash;

// #[macro_export]
// macro_rules! int_writers {
//     () => {
//         $crate::int_writers!(
//             write_u8(u8);
//             write_u16(u16);
//             write_u32(u32);
//             write_u64(u64);
//             write_u128(u128);
//             write_i8(i8);
//             write_i16(i16);
//             write_i32(i32);
//             write_i64(i64);
//             write_i128(i128);
//         );
//     };
//     ($func_name:ident($type:ty)) => {
//         fn $func_name(&mut self, i: $type) {
//             let bytes = i.to_le_bytes();
//             self.update(&bytes);
//         }
//     };
//     ($(
//         $func_name:ident($type:ty);
//     )+) => {
//         $(
//             $crate::int_writers!($func_name($type));
//         )*
//     };
// }

// #[inline]
// #[must_use]
// #[cfg(target_endian = "little")]
// pub(crate) const fn lanes_to_bytes(lanes: [u64; 4]) -> [u8; 32] {
//     // since we want little endian bytes, and the target platform
//     // is little endian, we can just transmute.
//     unsafe {
//         ::core::mem::transmute(lanes)
//     }
// }

// #[inline]
// #[must_use]
// #[cfg(target_endian = "big")]
// pub(crate) const fn lanes_to_bytes(lanes: [u64; 4]) -> [u8; 32] {
//     let mut bytes = [0u8; 32];
//     macro_rules! write_lane {
//         ($index:literal) => {
//             {
//                 let index: usize = $index;
//                 let offset = (index * 8);
//                 // must ALWAYS use `to_le_bytes` to ensure compatibility across platforms.
//                 let lane_bytes = lanes[index].to_le_bytes();
//                 unsafe {
//                     ::core::ptr::copy_nonoverlapping(lane_bytes.as_ptr(), bytes.as_mut_ptr().add(offset), 8);
//                 }
//             }
//         };
//     }
//     write_lane!(0);
//     write_lane!(1);
//     write_lane!(2);
//     write_lane!(3);
//     bytes
// }

#[inline]
#[must_use]
pub(crate) const fn bytes_to_lanes(input: &[u8; 32]) -> [u64; 4] {
    let mut lanes = [0u64; 4];
    let mut index = 0;
    while index < 4 {
        let mut bytes = [0u8; 8];
        unsafe {
            ::core::ptr::copy_nonoverlapping(input.as_ptr().add(index * 8), bytes.as_mut_ptr(), 8);
        }
        lanes[index] = u64::from_le_bytes(bytes);
        index += 1;
    }
    lanes
}

#[inline]
#[must_use]
pub(crate) const fn mix_lanes(input: [u64; 4]) -> u64 {
    #[inline(always)]
    const fn mix_two(a: u64, b: u64) -> u64 {
        a ^ b.rotate_left(23)
    }
    
    mix_two(mix_two(input[0], input[1]), mix_two(input[2], input[3]))
}

pub const GOLDEN_RATIO_64: u64 = 0x9e3779b97f4a7c15;
pub const DEADBEEF_64: u64 = 0xDEADBEEF;

pub struct Blake3Hasher {
    pub hasher: blake3::Hasher,
}

impl Blake3Hasher {
    #[inline]
    #[must_use]
    pub fn from_hasher(hasher: blake3::Hasher) -> Self {
        Self {
            hasher,
        }
    }
    
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::from_hasher(blake3::Hasher::new())
    }
    
    #[inline]
    #[must_use]
    pub fn new_keyed(key: &[u8; 32]) -> Self {
        Self::from_hasher(blake3::Hasher::new_keyed(key))
    }
    
    #[inline]
    #[must_use]
    pub fn new_derive_key(context: &str) -> Self {
        Self::from_hasher(blake3::Hasher::new_derive_key(context))
    }
    
    pub fn update(&mut self, input: &[u8]) -> &mut Self {
        self.hasher.update(input);
        self
    }
    
    pub fn finalize(&self) -> blake3::Hash {
        self.hasher.finalize()
    }
    
    pub fn finalize_xof(&self) -> blake3::OutputReader {
        self.hasher.finalize_xof()
    }
    
    pub fn finalize_bytes<const LEN: usize>(&self) -> [u8; LEN] {
        let mut bytes = [0u8; LEN];
        let mut reader = self.hasher.finalize_xof();
        reader.fill(&mut bytes);
        bytes
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_u128(&self) -> u128 {
        u128::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_u64(&self) -> u64 {
        u64::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_u32(&self) -> u32 {
        u32::from_be_bytes(self.finalize_bytes())
        
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_u16(&self) -> u16 {
        u16::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_u8(&self) -> u8 {
        u8::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_i128(&self) -> i128 {
        i128::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_i64(&self) -> i64 {
        i64::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_i32(&self) -> i32 {
        i32::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_i16(&self) -> i16 {
        i16::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_i8(&self) -> i8 {
        i8::from_be_bytes(self.finalize_bytes())
    }
    
    #[inline]
    #[must_use]
    pub fn finalize_bool(&self) -> bool {
        let byte = self.finalize_u8();
        let step1 = (byte ^ (byte >> 4)) & 0xF;
        let step2 = (step1 ^ (step1 >> 2)) & 0x3;
        let step3 = (step2 ^ (step2 >> 1)) & 0b1;
        step3 == 1
    }
}

impl DeterministicHasher for Blake3Hasher {
    fn write(&mut self, input: &[u8]) {
        self.update(input);
    }
    
    fn finish(&self) -> [u8; 32] {
        self.finalize_bytes()
    }
}

#[must_use]
pub fn deterministic_hash<T: DeterministicHash>(value: T) -> Blake3Hasher {
    let mut hasher = Blake3Hasher::new();
    value.deterministic_hash(&mut hasher);
    hasher
}

#[inline]
#[must_use]
pub fn deterministic_hash_xof<T: DeterministicHash>(value: T) -> blake3::OutputReader {
    deterministic_hash(value).finalize_xof()
}

#[inline]
#[must_use]
pub fn deterministic_hash_bytes<T: DeterministicHash, const LEN: usize>(value: T) -> [u8; LEN] {
    deterministic_hash(value).finalize_bytes()
}

#[inline]
#[must_use]
pub fn deterministic_hash_bytes_into<T: DeterministicHash>(value: T, buf: &mut [u8]) {
    let mut reader = deterministic_hash_xof(value);
    reader.fill(buf);
}

#[inline]
#[must_use]
pub fn deterministic_hash256<T: DeterministicHash>(value: T) -> [u8; 32] {
    deterministic_hash_bytes(value)
}

#[inline]
#[must_use]
pub fn deterministic_hash_u128<T: DeterministicHash>(value: T) -> u128 {
    deterministic_hash(value).finalize_u128()
}

#[inline]
#[must_use]
pub fn deterministic_hash_u64<T: DeterministicHash>(value: T) -> u64 {
    deterministic_hash(value).finalize_u64()
}

#[inline]
#[must_use]
pub fn deterministic_hash_u32<T: DeterministicHash>(value: T) -> u32 {
    deterministic_hash(value).finalize_u32()
}

#[inline]
#[must_use]
pub fn deterministic_hash_u16<T: DeterministicHash>(value: T) -> u16 {
    deterministic_hash(value).finalize_u16()
}

#[inline]
#[must_use]
pub fn deterministic_hash_u8<T: DeterministicHash>(value: T) -> u8 {
    deterministic_hash(value).finalize_u8()
}

#[inline]
#[must_use]
pub fn deterministic_hash_bool<T: DeterministicHash>(value: T) -> bool {
    deterministic_hash(value).finalize_bool()
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use std::collections::{HashMap, HashSet};
    use ::core::hash::Hash;
    use crate::deterministic::DeterministicHash;

    use super::*;
    
    #[test]
    fn deterministic_hasher_test() {
        let value = (
            [420, 69, 0xDEADBEEFu128],
            (
                true,
                'A',
                ['A', 'B'],
                "test",
                b"test",
            )
        );
        
        let hash_512: [u8; 64] = deterministic_hash_bytes(&value);
        let hash256: [u8; 32] = deterministic_hash_bytes(&value);
        let hash64 = deterministic_hash_u64(&value);
        let hash32 = deterministic_hash_u32(&value);
        let hash16 = deterministic_hash_u16(&value);
        let hash8 = deterministic_hash_u8(&value);
        let hash_bool = deterministic_hash_bool(&value);
        
        println!(" 512: {hash_512:x?}");
        println!(" 256: {hash256:x?}");
        println!("  64: {hash64:016x}");
        println!("  32: {hash32:08x}");
        println!("  16: {hash16:04x}");
        println!("   8: {hash8:02x}");
        println!("bool: {hash_bool}");
        println!(" i32: {}", deterministic_hash(&value).finalize_i32());
    }
    
    #[test]
    fn hash_test() {
        const ITERATIONS: usize = 10usize.pow(9);
        let mut hashes = HashSet::<u64>::with_capacity(ITERATIONS);
        let mut collision_counts = HashMap::<u64, usize>::new();
        let mut total_collisions: usize = 0;
        let mut track_collision = {
            let hashes = &mut hashes;
            let collision_counts = &mut collision_counts;
            let total_collisions = &mut total_collisions;
            move |hash: u64| {
                if !hashes.insert(hash) {
                    *total_collisions += 1;
                    *collision_counts.entry(hash).or_insert(0) += 1;
                }
            }
        };
        for i in 0..ITERATIONS {
            let mut hasher = Blake3Hasher::new();
            (1, 2, 3, i).deterministic_hash(&mut hasher);
            let hash = hasher.finalize_u64();
            track_collision(hash);
        }
        let collision_ratio = total_collisions as f64 / ITERATIONS as f64;
        println!("Collision Count: {total_collisions} / {ITERATIONS} (r: {collision_ratio})");
        let mut max_collisions = 0;
        let mut max_collided = 0;
        for (&hash, &count) in collision_counts.iter() {
            if count > max_collisions {
                max_collisions = count;
                max_collided = hash;
            }
        }
        println!("Max Collisions: {max_collisions} ({max_collided})");
    }
    
    fn bit_stats(n: u64) -> (u32, [u8; 64]) {
        const BIT_COUNTS: [[u8; 8]; 256] = {
            let mut counts = [[0u8; 8]; 256];
            const fn get_bit(n: u32, i: u32) -> u8 {
                ((n >> i) & 1) as u8
            }
            let mut i = 0;
            while i < 256 {
                let mut bits = [0u8; 8];
                let mut bit = 0;
                while bit < 8 {
                    bits[bit as usize] = get_bit(i, bit);
                    bit += 1;
                }
                counts[i as usize] = bits;
                i += 1;
            }
            counts
        };
        const fn get_bit_counts(n: u8) -> [u8; 8] {
            BIT_COUNTS[n as usize]
        }
        let bit_count = n.count_ones();
        let mut bit_counts = [0u8; 64];
        for i in (0..64).step_by(8) {
            let byte = ((n >> i) & 0xFF) as u8;
            let sub_counts = get_bit_counts(byte);
            for (i, bit) in (i..i+8).enumerate() {
                bit_counts[bit] = sub_counts[i];
            }
        }
        (bit_count, bit_counts)
    }
    
    fn add_bit_counts(count: &[u8; 64], out: &mut [u32; 64]) {
        for i in 0..64 {
            out[i] += count[i] as u32;
        }
    }
    
    // #[test]
    // fn print_salts_binary() {
    //     let mut counts = [0u32; 64];
    //     let mut total_bits = 0u32;
    //     let mut greatest_bit_deviation = 0f64;
    //     let mut set = HashSet::<u64>::new();
    //     let mut collisions = HashMap::<u64, usize>::new();
    //     let mut collision_count = 0usize;
    //     let mut track_collisions = |hash: u64| {
    //         if !set.insert(hash) {
    //             collision_count += 1;
    //             *collisions.entry(hash).or_insert(0) += 1;
    //         }
    //     };
    //     const ITERATIONS: usize = 1000;
    //     println!("===[Hashes]===");
    //     for i in 0..ITERATIONS {
    //         // let mix = mix64(i + 1);
    //         // let mix = SALTS[i as usize];
    //         let mix = FastHash::hash_one_with_seed(0xDEADBEEFu64, (1, 2, i));
    //         track_collisions(mix);
    //         let (bit_count, bits_counts) = bit_stats(mix);
    //         add_bit_counts(&bits_counts, &mut counts);
    //         total_bits += bit_count;
    //         println!("{mix:064b} : 0x{mix:X}");
    //     }
    //     let bit_count_ratio = (total_bits as f64) / ((64 * ITERATIONS) as f64);
    //     println!("===[Bit Count Ratios]===");
    //     for i in 0..64 {
    //         let count = counts[i];
    //         let count_ratio = (count as f64) / (ITERATIONS as f64);
    //         // let ratio_deviation = count_ratio.max(0.5) - (0.5f64).min(count_ratio);
    //         let ratio_deviation = (0.5 - count_ratio).abs();
    //         greatest_bit_deviation = greatest_bit_deviation.max(ratio_deviation);
    //         println!("Ratio of {i}: {count_ratio:.3} ({count}, deviation: {ratio_deviation:.3})");
    //     }
    //     println!("===[Statistic Information]===");
    //     println!("Bit Count Ratio: {bit_count_ratio:.3}");
    //     println!("Greatest Bit Deviation: {greatest_bit_deviation:.3}");
    //     let collision_rate = collision_count as f64 / ITERATIONS as f64;
    //     let collision_rate_percent = collision_rate * 100.0;
    //     println!("Collision Rate: {collision_rate_percent:.4}% ({collision_count})");
    //     let mut max_collisions = 0usize;
    //     let mut max_collided = 0u64;
    //     for (&hash, &count) in collisions.iter() {
    //         if count > max_collisions {
    //             max_collided = hash;
    //             max_collisions = count;
    //         }
    //     }
    //     println!("Max Collisions: {max_collisions}");
    //     println!("Max Collided: {max_collided}");
    // }
}