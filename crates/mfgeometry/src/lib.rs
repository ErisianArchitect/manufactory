
/*====================================================================================*\
||I find that sometimes, using big lookup tables is the best solution to your problem.||
||All of the functionality in this library can be done logically, but I think it will ||
||probably execute faster with the lookup tables and such. The logic to construct     ||
||these tables was fairly complicated.                                                ||
\*====================================================================================*/

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

// this code feels like cheating.

// verified (2025-12-28)
// This packing format should remain consistent, and should be considered permanent.
// Field
// Flip    : 0..3 (3 bits)
//      X: 0
//      Y: 1
//      Z: 2
// Rotation: 3..8 (5 bits)
//      angle: 3..5 (2 bits)
//      up   : 5..8 (3 bits)
#[inline]
pub const fn pack_flip_and_rotation(flip: Flip, rotation: Rotation) -> u8 {
    flip.0 | (rotation.0 << 3)
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
        // macro_rules! test_rot {
        //     ($(
        //         $up:ident($rot:literal) => $dir:ident -> $expect:ident
        //     ),*$(,)?) => {
        //         $(
        //             {
        //                 let orient = Orientation::new(Rotation::new(Direction::$up, $rot), Flip::NONE);
        //                 let dir = Direction::$dir;
        //                 let dir_rot = orient.reface(dir);
        //                 assert_eq!(dir_rot, Direction::$expect, stringify!(($up, $rot) => $dir -> $expect));
        //             }
        //         )*
        //     };
        // }
        for dir in Direction::iter() {
            let orient = Orientation::new(Rotation::new(dir, 1), Flip::NONE);
            
            let orient_face = orient.rotation().up();
            assert_eq!(orient.forward(), orient_face.left(), "forward: {dir}");
            
            assert_eq!(
                orient.reface(Direction::FORWARD),
                orient_face.left(),
                "{dir}: Forward -> Left"
            );
            assert_eq!(
                orient.reface(Direction::LEFT),
                orient_face.down(),
                "{dir}: Left -> Down"
            );
            assert_eq!(
                orient.reface(Direction::BACKWARD),
                orient_face.right(),
                "{dir}: Back -> Right"
            );
            assert_eq!(
                orient.reface(Direction::RIGHT),
                orient_face.up(),
                "{dir}: Right -> Up"
            );
        }
    }
    
    /// verifies [Rotation::reface] function. By extension, also verifies [Rotation::up], [Rotation::down], [Rotation::left], [Rotation::right], [Rotation::forward], and [Rotation::backward].
    #[test]
    fn orientation_query_test() {
        // Since all of the functions used in this function are verified, this is
        // another way to rotate faces. This is used to verify the `reface` function.
        fn rotate_world(up: Direction, angle: i32, world: Direction) -> Direction {
            match world {
                Direction::NegX => up.left_at_angle(angle),
                Direction::NegY => up.invert(),
                Direction::NegZ => up.up_at_angle(angle),
                Direction::PosX => up.right_at_angle(angle),
                Direction::PosY => up,
                Direction::PosZ => up.down_at_angle(angle),
            }
        }
        for angle in 0..4 {
            for up in Direction::iter() {
                for world in Direction::iter() {
                    let rotation = Rotation::new(up, angle);
                    let rot_world = rotation.reface(world);
                    let rot_world_alt = rotate_world(up, angle, world);
                    assert_eq!(rot_world, rot_world_alt, "(rot: [up: {up}, angle: {angle}], world: {world})");
                }
            }
        }
    }
    
    // verifies `source_face` function as well as symmetry between `reface` and `source_face`.
    #[test]
    fn reface_sourceface_symmetry_test() {
        let start_time = std::time::Instant::now();
        for angle in 0..4 {
            for up in Direction::iter() {
                let rotation = Rotation::new(up, angle);
                for world in Direction::iter() {
                    let refaced = rotation.reface(world);
                    let source = rotation.source_face(refaced);
                    assert_eq!(source, world);
                }
            }
        }
        let elapsed = start_time.elapsed();
        println!("Elapsed Time: {elapsed:.3?}");
    }
    
    #[test]
    fn face_angle_test() {
        for angle in 0..4 {
            for up in Direction::iter() {
                let rotation = Rotation::new(up, angle);
            }
        }
    }
}