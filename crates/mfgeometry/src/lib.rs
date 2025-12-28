pub mod axis;
pub mod cardinal;
pub mod direction;
pub mod flip;
pub mod orient_table;
pub mod orientation;
pub mod rotation;

pub use axis::Axis;
pub use direction::Direction;
pub use flip::Flip;
pub use orientation::Orientation;
pub use rotation::Rotation;

#[inline]
pub const fn pack_flip_and_rotation(flip: Flip, rotation: Rotation) -> u8 {
    flip.0 | rotation.0 << 3
}

#[inline]
pub const fn unpack_flip_and_rotation(packed: u8) -> (Flip, Rotation) {
    let flip = packed & 0b111;
    let rotation = packed >> 3;
    (Flip(flip), Rotation(rotation))
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn orientation_test() {
        macro_rules! test_rot {
            ($(
                $up:ident($rot:literal) => $dir:ident -> $expect:ident
            ),*$(,)?) => {
                $(
                    {
                        let orient = Orientation::new(Rotation::new(Direction::$up, $rot), Flip::NONE);
                        let dir = Direction::$dir;
                        let dir_rot = orient.reface(dir);
                        assert_eq!(dir_rot, Direction::$expect, stringify!(($up, $rot) => $dir -> $expect));
                    }
                )*
            };
        }
        test_rot!(
            UP(1) => FORWARD -> LEFT,
            BACKWARD(2) => UP -> BACKWARD,
        );
        let orient = Orientation::new(Rotation::new(Direction::PosY, 1), Flip::NONE);
        let fwd = Direction::FORWARD;
        let fwd_rot = orient.reface(fwd);
        assert_eq!(fwd_rot, Direction::LEFT);
    }
}