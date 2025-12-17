use ::core::hash::Hash;
use mfhash::*;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeedContext(&'static str);

impl SeedContext {
    pub const ROOT: Self = Self("__ROOT__");
    pub const WORLD: Self = Self("manufactory/world-seed (v1.0.0)");
    
    #[must_use]
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seed(HashSeed256);

impl Seed {
    #[inline]
    #[must_use]
    pub const fn with_hashseed(seed: HashSeed256) -> Self {
        Self(seed)
    }
    
    #[inline]
    #[must_use]
    pub fn reversible_hash<T: Hash>(&self, value: T) -> [u8; 32] {
        self.0.hash(value)
    }
    
    /// Uses blake3 to cryptographically derive a key (hash) from the given value hashed through the insecure hasher.
    pub fn derive_seed<T: Hash>(&self, value: T, context: Option<SeedContext>) -> [u8; 32] {
        let pre_hash = self.reversible_hash(value);
        let context = context.unwrap_or(SeedContext::ROOT);
        blake3::derive_key(context.as_str(), &pre_hash)
    }
    
    pub fn derive_rng<T: Hash>(&self, value: T, context: Option<SeedContext>) -> ChaCha20Rng {
        let seed = self.derive_seed(value, context);
        ChaCha20Rng::from_seed(seed)
    }
    
    pub fn derive_new<T: Hash>(&self, value: T, context: Option<SeedContext>) -> Self {
        Self(HashSeed256::new(self.derive_seed(value, context)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_chacha::ChaCha20Rng;
    use rand::{Rng, SeedableRng};
    
    #[repr(transparent)]
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Cell(char);
    
    struct Map {
        cells: Box<[Cell]>,
    }
    
    impl Map {
        pub const WIDTH: usize = 64;
        pub const HEIGHT: usize = Self::WIDTH;
        pub const CELL_COUNT: usize = Self::WIDTH * Self::HEIGHT;
        pub const DEFAULT_CELL: Cell = Cell(' ');
        
        pub fn new() -> Self {
            Self {
                cells: Box::from_iter((0..Self::CELL_COUNT).map(|_| Self::DEFAULT_CELL)),
            }
        }
        
        fn index_at(x: i32, y: i32) -> usize {
            let x = x.rem_euclid(Self::WIDTH as i32) as usize;
            let y = y.rem_euclid(Self::HEIGHT as i32) as usize;
            (y * Self::WIDTH) + x
        }
        
        pub fn set(&mut self, x: i32, y: i32, value: Cell) -> Cell {
            let index = Self::index_at(x, y);
            ::core::mem::replace(&mut self.cells[index], value)
        }
        
        pub fn get(&self, x: i32, y: i32) -> Cell {
            let index = Self::index_at(x, y);
            self.cells[index]
        }
        
    }
    
    fn circle_points<F: FnMut([i32; 2], [i32; 2])>(x: i32, y: i32, radius: u8, mut segment: F) {
        macro_rules! draw_points {
            ($xc:expr, $yc:expr, $x:expr, $y:expr) => {
                {
                    let xc = $xc;
                    let yc = $yc;
                    let x = $x;
                    let y = $y;
                    // put(xc + x, yc + y);
                    // put(xc - x, yc + y);
                    // put(xc + x, yc - y);
                    // put(xc - x, yc - y);
                    // put(xc + y, yc + x);
                    // put(xc - y, yc + x);
                    // put(xc + y, yc - x);
                    // put(xc - y, yc - x);
                    segment(
                        [xc - x, yc + y],
                        [xc + x, yc + y],
                    );
                    segment(
                        [xc - x, yc - y],
                        [xc + x, yc - y],
                    );
                    segment(
                        [xc - y, yc + x],
                        [xc + y, yc + x],
                    );
                    segment(
                        [xc - y, yc - x],
                        [xc + y, yc - x],
                    );
                }
            };
        }
        let xc = x;
        let yc = y;
        let mut x = 0;
        let mut y = radius as i32;
        let mut d = 3 - 2 * radius as i32;
        draw_points!(xc, yc, x, y);
        while y >= x {
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
            x += 1;
            draw_points!(xc, yc, x, y);
        }
    }
    
    fn circle<F: FnMut(i32, i32)>(x: i32, y: i32, radius: u8, mut put: F) {
        circle_points(x, y, radius, move |[x1, y1], [x2, y2]| {
            put(x1, y1);
            put(x2, y2);
        });
    }
    
    fn fill_circle<F: FnMut(i32, i32)>(x: i32, y: i32, radius: u8, mut put: F) {
        circle_points(x, y, radius, move |[x1, y1], [x2, y2]| {
            for x in x1..=x2 {
                put(x, y1);
                put(x, y2);
            }
        });
    }
    
    #[test]
    fn circle_test() {
        let mut map = Map::new();
        fill_circle(15, 15, 13, |x, y| {map.set(x, y, Cell('X'));});
        for y in 0..Map::HEIGHT {
            for x in 0..Map::WIDTH {
                print!("{}", map.get(x as i32, y as i32).0);
            }
            println!();
        }
    }
    
    #[test]
    fn proc_genie() {
        use rand::SeedableRng;
        use rand_chacha::ChaCha20Rng;
        const CHARS: [char; 94] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];
        let mut map = Map::new();
        let seed = Seed::with_hashseed(HashSeed256::from_u64(0xDEADBEEFu64));
        let mut world_rng = seed.derive_rng(2, None);
        let radii = Box::from_iter((0..world_rng.random_range(3..64)).map(|_| {
            world_rng.random_range(4..13u8)
        }));
        let positions = Box::from_iter((0..world_rng.random_range(8192..16384)).map(|_| {
            let x = world_rng.random::<i32>();
            let y = world_rng.random::<i32>();
            (x, y)
        }));
        for (i, (x, y)) in positions.iter().copied().enumerate() {
            let chr = CHARS[world_rng.random_range(0..CHARS.len())];
            let radius = radii[i % radii.len()];
            if world_rng.random_bool(0.1) {
                circle(x, y, radius, |x, y| {
                    map.set(x, y, Cell(chr));
                });
            } else {
                fill_circle(x, y, radius, |x, y| {
                    map.set(x, y, Cell(chr));
                });
            }
        }
        for y in 0..Map::HEIGHT {
            for x in 0..Map::WIDTH {
                print!("{}", map.get(x as i32, y as i32).0);
            }
            println!();
        }
    }
    
    #[test]
    fn gen_test() {
        let world_seed = HashSeed256::from_u64(0xDEADBEEFu64);
        let world_seed = Seed::with_hashseed(world_seed);
        let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'];
        let mut char_map: Box<[char]> = Box::from_iter((0..64*64).map(|_| ' '));
        fn index_at(x: u32, y: u32) -> usize {
            (y as usize * 64) + x as usize
        }
        let mut put = |x: i32, y: i32, chr: char| {
            char_map[index_at(x.rem_euclid(64) as u32, y.rem_euclid(64) as u32)] = chr;
        };
        for y in (0..64).step_by(8) {
            for x in (0..64).step_by(8) {
                let mut rng = ChaCha20Rng::from_seed(world_seed.derive_seed(&([x, y], "terrain"), None));
                for sy in (0..8).map(move |n| n + y) {
                    for sx in (0..8).map(move |n| n + x) {
                        let chri = rng.random_range(0..chars.len());
                        put(sx, sy, chars[chri]);
                    }
                }
            }
        }
        for y in 0..64 {
            for x in 0..64 {
                let index = index_at(x, y);
                print!("{}", char_map[index]);
            }
            println!();
        }
    }
}