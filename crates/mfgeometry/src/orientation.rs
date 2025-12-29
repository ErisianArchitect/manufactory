use crate::{
    flip::Flip,
    rotation::Rotation,
    orient_table,
    pack_flip_and_rotation,
    direction::Direction,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeconstructedOrientation {
    pub flip_x: bool,
    pub flip_y: bool,
    pub flip_z: bool,
    pub angle: u8,
    pub up: Direction,
}

impl DeconstructedOrientation {
    #[inline]
    pub const fn construct(self) -> Orientation {
        Orientation::new(
            Rotation::new(
                self.up,
                self.angle as i32,
            ),
            Flip::new(
                self.flip_x,
                self.flip_y,
                self.flip_z,
            ),
        )
    }
}

// Field     : Bit Range
// Flip      : 0..3
//      X    : 0
//      Y    : 1
//      Z    : 2
// Rotation  : 3..8
//      angle: 3..5
//      up   : 5..8
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Orientation(pub(crate) u8);

impl Orientation {
    pub(crate) const TOTAL_ORIENTATION_COUNT: u8 = /* Flip */ 8 * /* Angle */ 4 * /* Up */ 6;
    pub const UNORIENTED: Orientation = Orientation::new(Rotation::new(Direction::PosY, 0), Flip::NONE);
    pub const ROTATE_X: Orientation = Rotation::ROTATE_X.orientation();
    pub const ROTATE_Y: Orientation = Rotation::ROTATE_Y.orientation();
    pub const ROTATE_Z: Orientation = Rotation::ROTATE_Z.orientation();
    pub const X_ROTATIONS: [Orientation; 4] = Self::ROTATE_X.angles();
    pub const Y_ROTATIONS: [Orientation; 4] = Self::ROTATE_Y.angles();
    pub const Z_ROTATIONS: [Orientation; 4] = Self::ROTATE_Z.angles();

    // FIXME: Make consistent with Rotation::CORNER_ROTATIONS_MATRIX
    pub const CORNER_ORIENTATIONS_MATRIX: [[[[Orientation; 3]; 2]; 2]; 2] = [
        [
            [Rotation::new(Direction::PosX, 2).orientation().corner_angles(), Rotation::new(Direction::PosZ, 3).orientation().corner_angles()],
            [Rotation::new(Direction::NegZ, 1).orientation().corner_angles(), Rotation::new(Direction::NegX, 0).orientation().corner_angles()]
        ],
        [
            [Rotation::new(Direction::NegZ, 3).orientation().corner_angles(), Rotation::new(Direction::PosX, 0).orientation().corner_angles()],
            [Rotation::new(Direction::NegX, 2).orientation().corner_angles(), Rotation::new(Direction::PosZ, 1).orientation().corner_angles()]
        ],
    ];
    
    // I think this is wrong (2025-12-28)
    pub const FACE_ORIENTATIONS: [[Orientation; 4]; 6] = [
        Self::Y_ROTATIONS, // PosY
        Self::X_ROTATIONS, // PosX
        Self::Z_ROTATIONS, // PosZ
        Self::ROTATE_Y.invert().angles(), // NegY
        Self::ROTATE_X.invert().angles(), // NegX
        Self::ROTATE_Z.invert().angles(), // NegZ
    ];
    
    /// An orientation that you can orient an orientation by to rotate around a face by angle. That was a mouthful.  
    /// If angle == 0, orientation is default orientation.
    #[inline]
    pub const fn face_orientation(face: Direction, angle: i32) -> Self {
        Self::FACE_ORIENTATIONS[face as usize][(angle & 3) as usize]
    }

    pub const fn corner_orientation(x: i32, y: i32, z: i32, angle: i32) -> Orientation {
        let x = if x <= 0 {
            0
        } else {
            1
        } as usize;
        let y = if y <= 0 {
            0
        } else {
            1
        } as usize;
        let z = if z <= 0 {
            0
        } else {
            1
        } as usize;
        let angle = angle.rem_euclid(3) as usize;
        Self::CORNER_ORIENTATIONS_MATRIX[y][z][x][angle]
    }

    #[inline]
    pub const fn new(rotation: Rotation, flip: Flip) -> Self {
        Self(pack_flip_and_rotation(flip, rotation))
    }
    
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        if value > Self::TOTAL_ORIENTATION_COUNT {
            return None;
        }
        Some(Self(value))
    }
    
    #[inline]
    pub const fn to_u8(self) -> u8 {
        self.0
    }
    
    #[inline]
    pub const fn deconstruct(self) -> DeconstructedOrientation {
        DeconstructedOrientation {
            flip_x: self.flip().x(),
            flip_y: self.flip().y(),
            flip_z: self.flip().z(),
            angle: (self.rotation().angle() & Rotation::ANGLE_MASK_I32) as u8,
            up: self.rotation().up(),
        }
    }
    
    // verified (2025-12-28)
    /// A helper function to create 4 orientations for an orientation group.  
    /// An orientation group is a series of "contiguous" orientations. That is, the orientations are logically sequential.
    /// An example would be rotations around an axis, or around a face, where there are 4 orientations possible.
    /// The first orientation is unoriented, the second orientation is the target orientation
    /// applied once, the third orientation is the target orientation applied twice,
    /// and the fourth orientation is the target orientation applied three times.
    pub const fn angles(self) -> [Orientation; 4] {
        let angle1 = self;
        let angle2 = angle1.reorient(angle1);
        let angle3 = angle2.reorient(angle1);
        [
            Orientation::UNORIENTED,
            angle1,
            angle2,
            angle3,
        ]
    }

    // verified (2025-12-28)
    /// A helper function to create 3 orientations for a corner orientation group.
    /// The first orientation is unoriented, the second orientation is the target orientation,
    /// and the third orientation is the target orientation applied to itself.
    pub const fn corner_angles(self) -> [Orientation; 3] {
        let angle1 = self;
        let angle2 = angle1.reorient(angle1);
        [
            Orientation::UNORIENTED,
            angle1,
            angle2,
        ]
    }

    #[inline]
    pub const fn flip(self) -> Flip {
        Flip(self.0 & 0b111)
    }

    #[inline]
    pub const fn rotation(self) -> Rotation {
        Rotation(self.0 >> 3)
    }

    #[inline]
    pub fn set_flip(&mut self, flip: Flip) {
        self.0 = (self.0 & 0b11111000) | flip.0
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.0 = (self.0 & 0b111) | rotation.0 << 3;
    }

    #[inline]
    pub fn set_up(&mut self, up: Direction) {
        let mut rot = self.rotation();
        rot.set_up(up);
        self.set_rotation(rot);
    }

    #[inline]
    pub fn set_angle(&mut self, angle: i32) {
        let mut rot = self.rotation();
        rot.set_angle(angle);
        self.set_rotation(rot);
    }
    
    pub fn cycle(self) -> Self {
        // Here, we assume that `self` has a valid bit representation.
        Self((self.0 + 1) % Self::TOTAL_ORIENTATION_COUNT)
    }
    
    /// This will cycle through the 8 [Flip] states before cycling the 24 [Rotation] states.
    /// 
    /// If you would like a version that cycles the rotations before cycling the flips, use [Orientation::iter_rotation_order].
    #[inline]
    pub fn iter(self) -> impl Iterator<Item = Self> {
        (0..Self::TOTAL_ORIENTATION_COUNT).map(move |i| Self(i))
    }
    
    #[inline]
    pub fn iter_rotation_order(self) -> RotationFirstOrientationIterator {
        RotationFirstOrientationIterator::START
    }

    /// `reface` can be used to determine where a face will end up after orientation.
    /// First rotates and then flips the face.
    #[inline]
    pub const fn reface(self, face: Direction) -> Direction {
        let rotated = self.rotation().reface(face);
        rotated.flip(self.flip())
    }

    /// This determines which face was oriented to `face`.
    #[inline]
    pub const fn source_face(self, face: Direction) -> Direction {
        let flipped = face.flip(self.flip());
        self.rotation().source_face(flipped)
    }

    /// Gets the direction that [Direction::PosY] is pointing towards.
    #[inline]
    pub const fn up(self) -> Direction {
        self.reface(Direction::PosY)
    }

    /// Gets the direction that [Direction::NegY] is pointing towards.
    #[inline]
    pub const fn down(self) -> Direction {
        self.reface(Direction::NegY)
    }

    /// Gets the direction that [Direction::NegZ] is pointing towards.
    #[inline]
    pub const fn forward(self) -> Direction {
        self.reface(Direction::NegZ)
    }

    /// Gets the direction that [Direction::PosZ] is pointing towards.
    #[inline]
    pub const fn backward(self) -> Direction {
        self.reface(Direction::PosZ)
    }

    /// Gets the direction that [Direction::NegX] is pointing towards.
    #[inline]
    pub const fn left(self) -> Direction {
        self.reface(Direction::NegX)
    }

    /// Gets the direction that [Direction::PosX] is pointing towards.
    #[inline]
    pub const fn right(self) -> Direction {
        self.reface(Direction::PosX)
    }

    /// If you're using this method to transform mesh vertices, make sure that you 
    /// reverse your indices if the face will be flipped (for backface culling). To
    /// determine if your indices need to be inverted, simply XOR each axis of the [Orientation]'s [Flip].
    /// This method will rotate and then flip the coordinate.
    #[inline]
    pub fn transform<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T, T)> + From<(T, T, T)>>(self, point: C) -> C {
        let rotated = self.rotation().rotate_coord(point);
        self.flip().flip_coord(rotated)
    }

    /// This method can tell you where on the target face a source UV is.
    /// To get the most benefit out of this, it is advised that you center your coords around (0, 0).
    /// So if you're trying to map a coord within a rect of size (16, 16), you would subtract 8 from the
    /// x and y of the coord, then pass that offset coord to this function, then add 8 back to the x and y
    /// to get your final coord.
    #[inline]
    pub fn map_face_coord<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T)> + From<(T, T)>>(self, face: Direction, uv: C) -> C {
        let table_index = orient_table::table_index(self.rotation(), self.flip(), face);
        let coordmap = orient_table::MAP_COORD_TABLE[table_index];
        coordmap.map(uv)
    }

    /// This method can tell you where on the source face a target UV is.
    /// To get the most benefit out of this, it is advised that you center your coords around (0, 0).
    /// So if you're trying to map a coord within a rect of size (16, 16), you would subtract 8 from the
    /// x and y of the coord, then pass that offset coord to this function, then add 8 back to the x and y
    /// to get your final coord.
    #[inline]
    pub fn source_face_coord<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T)> + From<(T, T)>>(self, face: Direction, uv: C) -> C {
        let table_index = orient_table::table_index(self.rotation(), self.flip(), face);
        let coordmap = orient_table::SOURCE_FACE_COORD_TABLE[table_index];
        coordmap.map(uv)
    }

    /// Apply an orientation to an orientation.
    pub const fn reorient(self, orientation: Orientation) -> Self {
        let up = self.up();
        let fwd = self.forward();
        let reup = orientation.reface(up);
        let refwd = orientation.reface(fwd);
        let flip = self.flip().flip(orientation.flip());
        let flipup = reup.flip(flip);
        let flipfwd = refwd.flip(flip);
        let Some(rot) = Rotation::from_up_and_forward(flipup, flipfwd) else {
            unreachable!()
        };
        Orientation::new(rot, flip)
    }

    /// Remove an orientation from an orientation.
    /// This is the inverse operation to [Orientation::reorient].
    pub const fn deorient(self, orientation: Orientation) -> Self {
        let up = self.up();
        let fwd = self.forward();
        let reup = orientation.source_face(up);
        let refwd = orientation.source_face(fwd);
        let flip = self.flip().flip(orientation.flip());
        let flipup = reup.flip(flip);
        let flipfwd = refwd.flip(flip);
        let Some(rot) = Rotation::from_up_and_forward(flipup, flipfwd) else {
            unreachable!()
        };
        Orientation::new(rot, flip)
    }
    
    /// Returns the orientation that can be applied to deorient by [self].
    #[inline]
    pub const fn invert(self) -> Self {
        Orientation::UNORIENTED.deorient(self)
    }

    #[inline]
    pub const fn flip_x(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_x())
    }

    #[inline]
    pub const fn flip_y(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_y())
    }

    #[inline]
    pub const fn flip_z(self) -> Self {
        Orientation::new(self.rotation(), self.flip().flip_z())
    }

    #[inline]
    pub const fn rotate_x(self, angle: i32) -> Self {
        self.reorient(Orientation::X_ROTATIONS[(angle & 3) as usize])
    }

    #[inline]
    pub const fn rotate_y(self, angle: i32) -> Self {
        self.reorient(Orientation::Y_ROTATIONS[(angle & 3) as usize])
    }

    #[inline]
    pub const fn rotate_z(self, angle: i32) -> Self {
        self.reorient(Orientation::Z_ROTATIONS[(angle & 3) as usize])
    }

    /// Rotate `face` clockwise by `angle`. Use a negative `angle` to rotate counter-clockwise.
    #[inline]
    pub const fn rotate_face(self, face: Direction, angle: i32) -> Self {
        let orient = Self::face_orientation(face, angle);
        self.reorient(orient)
    }

    #[inline]
    pub const fn rotate_corner(self, x: i32, y: i32, z: i32, angle: i32) -> Self {
        let orient = Self::corner_orientation(x, y, z, angle);
        self.reorient(orient)
    }

    // #[inline]
    // pub fn to_matrix(self) -> glam::Mat4 {
    //     let flip = self.flip();
    //     let rotation = self.rotation();
    //     let scale = flip.to_scale();
    //     // scale.x = -scale.x;
    //     let up = rotation.reface(Direction::PosY).to_vec3();
    //     let forward = rotation.reface(Direction::PosZ).to_vec3();
    //     let right = self.reface(Direction::PosX).to_vec3();

    //     // let right = glam::vec3(right.x * scale.x, right.y, right.z);
    //     // let up = glam::vec3(up.x, up.y * scale.y, up.z);
    //     // let forward = glam::vec3(forward.x, forward.y, forward.z * scale.z);

    //     glam::Mat4::from_cols(
    //         (right * scale).extend(0.0),
    //         (up * scale).extend(0.0),
    //         (forward * scale).extend(0.0),
    //         glam::Vec3::ZERO.extend(1.0),
    //     )
    // }
}

impl From<u8> for Orientation {
    #[inline]
    fn from(value: u8) -> Self {
        Orientation(value)
    }
}

impl Into<u8> for Orientation {
    #[inline]
    fn into(self) -> u8 {
        self.0
    }
}

impl From<Rotation> for Orientation {
    #[inline]
    fn from(value: Rotation) -> Self {
        Orientation::new(value, Flip::NONE)
    }
}

impl From<Flip> for Orientation {
    #[inline]
    fn from(value: Flip) -> Self {
        Orientation::new(Rotation::default(), value)
    }
}

impl std::fmt::Display for Orientation {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Orientation({},{})", self.flip(), self.rotation())
    }
}

// #[cfg(test)]
// mod testing_sandbox {
//     // TODO: Remove this sandbox when it is no longer in use.
//     use super::*;
//     #[test]
//     fn sandbox() {
//         let orientation = Orientation::new(Rotation::new(Direction::PosZ, 1), Flip::XYZ);
//         // let orientation = Orientation::UNORIENTED;
//         println!("Forward: {}", orientation.rotation().forward());
//         let mat = orientation.to_matrix();
//         let pos_y = mat.transform_point3(glam::Vec3::Y);
//         let neg_z = mat.transform_point3(glam::Vec3::NEG_Z);
//         println!("PosY: {pos_y:?}");
//         println!("NegZ: {neg_z:?}");
//         let flip = Flip::XYZ;
//         let mat = flip.to_matrix();
//         let posx = mat.transform_point3(glam::Vec3::X);
//         let posy = mat.transform_point3(glam::Vec3::Y);
//         let posz = mat.transform_point3(glam::Vec3::Z);
//         println!("!PosX: {posx:?}");
//         println!("!PosY: {posy:?}");
//         println!("!PosZ: {posz:?}");
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RotationFirstOrientationIterator {
    flip: u8,
    rotation: u8,
}

impl RotationFirstOrientationIterator {
    pub(crate) const START: Self = Self { flip: 0, rotation: 0 };
    #[inline]
    pub const fn new() -> Self {
        Self::START
    }
    
    #[inline]
    pub const fn start_at(orientation: Orientation) -> Self {
        Self {
            flip: orientation.flip().0,
            rotation: orientation.rotation().0,
        }
    }
    
    /// Gets the current element without advancing the iterator.
    #[inline]
    pub const fn current(self) -> Option<Orientation> {
        if self.flip == 8 {
            return None;
        }
        Some(Orientation::new(Rotation(self.rotation), Flip(self.flip)))
    }
}

impl Iterator for RotationFirstOrientationIterator {
    type Item = Orientation;
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        const SIZE: usize = Orientation::TOTAL_ORIENTATION_COUNT as usize;
        let total = (self.flip as usize * 24) + self.rotation as usize;
        (SIZE - total, Some(SIZE - total))
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.flip == 8 {
            return None;
        }
        let result = Some(Orientation::new(Rotation(self.rotation), Flip(self.flip)));
        self.rotation += 1;
        if self.rotation == 24 {
            self.flip += 1;
        }
        result
    }
}