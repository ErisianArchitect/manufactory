
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
    let rotation = (packed >> 3) % 24;
    (Flip(flip), Rotation(rotation))
}

// verified (2025-12-28)
#[inline]
pub const fn wrap_angle(angle: i32) -> i32 {
    angle & Rotation::ANGLE_MASK_I32
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn orientation_test() {
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
        todo!()
    }
    
    
    use crate::orient_table::*;
    // I used this to generate the table in maptable.rs and I don't need it anymore, but I'm going
    // to keep it around just in case.
    fn map_face_coord_naive(orientation: Orientation, face: Direction) -> CoordMap {
        // First I will attempt a naive implementation, then I will use the naive implementation to generate code
        // for a more optimized implementation.
        // First get the source face
        let source_face = orientation.source_face(face);
        // next, get the up, right, down, and left for the source face and arg face.
        let face_up = face.up();
        let face_right = face.right();
        let src_up = source_face.up();
        let src_right = source_face.right();
        let src_down = source_face.down();
        let src_left = source_face.left();
        // Next, reface the src_dir faces
        let rsrc_up = orientation.reface(src_up);
        let rsrc_right = orientation.reface(src_right);
        let rsrc_down = orientation.reface(src_down);
        let rsrc_left = orientation.reface(src_left);
        // Now match up the faces
        // x_map and y_map must use right and up faces because the polarity is independent.
        let x_map = if face_right == rsrc_right { // PosX :facing: PosX, x maps to PosX (no change).
            AxisMap::PosX
        } else if face_right == rsrc_up { // PosX :facing: PosY, 1 turn counter-clockwise, NegY in place of PosX
            AxisMap::NegY
        } else if face_right == rsrc_left { // PosX :facing: NegX, x maps to NegX
            AxisMap::NegX
        } else { // PosX facing NegY, 1 clockwise turn, PosY is now in place of PosX
            AxisMap::PosY
        };
        
        let y_map = if face_up == rsrc_up {
            AxisMap::PosY
        } else if face_up == rsrc_left {
            AxisMap::PosX
        } else if face_up == rsrc_down {
            AxisMap::NegY
        } else {
            AxisMap::NegX
        };
        CoordMap {
            x: x_map,
            y: y_map
        }
    }

    fn source_face_coord_naive(orientation: Orientation, face: Direction) -> CoordMap {
        // First I will attempt a naive implementation, then I will use the naive implementation to generate code
        // for a more optimized implementation.
        // First get the source face
        let source_face = orientation.source_face(face);
        // next, get the up, right, down, and left for the source face and arg face.
        let src_up = source_face.up();
        let src_right = source_face.right();
        let face_up = face.up();
        let face_right = face.right();
        let face_down = face.down();
        let face_left = face.left();
        // Next, reface the src_dir faces
        let rsrc_up = orientation.reface(src_up);
        let rsrc_right = orientation.reface(src_right);
        // Now match up the faces
        let x_map = if rsrc_right == face_right {
            AxisMap::PosX
        } else if rsrc_right == face_down {
            AxisMap::PosY
        } else if rsrc_right == face_left {
            AxisMap::NegX
        } else {
            AxisMap::NegY
        };
        let y_map = if rsrc_up == face_up {
            AxisMap::PosY
        } else if rsrc_up == face_right {
            AxisMap::NegX
        } else if rsrc_up == face_down {
            AxisMap::NegY
        } else {
            AxisMap::PosX
        };
        CoordMap {
            x: x_map,
            y: y_map
        }
    }
    
        #[test]
    fn map_coord_gencode() {
        let output = {
            use std::fmt::Write;
            let mut output = String::new();
            let mut count = 0usize;
            for flipi in 0..8 { // flip
                for roti in 0..24 { // rotation
                    Direction::iter_discriminant_order().for_each(|face| {
                        count += 1;
                        let map = map_face_coord_naive(Orientation::new(Rotation(roti as u8), Flip(flipi as u8)), face);
                        writeln!(output, "CoordMap::new(AxisMap::{:?}, AxisMap::{:?}),", map.x, map.y).unwrap();
                    });
                }
            }
            output
        };
        use std::io::{Write, BufWriter};
        use std::fs::File;
        let mut writer = BufWriter::new(File::create("ignore/map_coord_table.rs").expect("Failed to open file"));
        writer.write_all(output.as_bytes()).expect("Failed to write file.");
        println!("Wrote the output to file at ./ignore/map_coord_table.rs");
    }
    #[test]
    fn source_coord_gencode() {
        let output = {
            use std::fmt::Write;
            let mut output = String::new();
            let mut count = 0usize;
            for flipi in 0..8 { // flip
                for roti in 0..24 { // rotation
                    Direction::iter_discriminant_order().for_each(|face| {
                        count += 1;
                        let map = source_face_coord_naive(Orientation::new(Rotation(roti as u8), Flip(flipi as u8)), face);
                        writeln!(output, "CoordMap::new(AxisMap::{:?}, AxisMap::{:?}),", map.x, map.y);
                    });
                }
            }
            output
        };
        use std::io::{Write, BufWriter};
        use std::fs::File;
        let mut writer = BufWriter::new(File::create("ignore/source_face_coord_table.rs").expect("Failed to open file"));
        writer.write_all(output.as_bytes());
        println!("Wrote the output to file at ./ignore/source_face_coord_table.rs");
    }
}