use mfhash::{HashSeed, deterministic::DeterministicHash};
use mffmt::hex::HexBytes as Hex;

fn main() {
    let seed = HashSeed::derived("This is a test.");
    let mut hasher = seed.build_hasher();
    let value = ([1, 2, 3], "apple");
    value.deterministic_hash(&mut hasher);
    let hash = hasher.finalize_u128();
    let hash_bytes: [u8; 16] = hasher.finalize_bytes();
    println!("{hash:032x}");
    println!("{}", Hex(&hash_bytes));
}
