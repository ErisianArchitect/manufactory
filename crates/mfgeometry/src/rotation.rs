/*---------------------------------------------------------------------------------------**
||Angles increase in a counter-clockwise direction.                                      ||
||To get the angle in degress, multiply `angle.rem_euclid(4)` or `(angle & 0b11)` by 90. ||
||So an angle of 1 would be 90 degress, 2 would be 180, and 3 would be 270               ||
**---------------------------------------------------------------------------------------*/

use crate::{
    direction::Direction,
    orientation::Orientation,
    wrap_angle,
};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rotation(pub(crate) u8);

impl Rotation {
    pub const MIN: Self = Self(0);
    // only 24 valid rotations (6 sides * 4 angles)
    pub const MAX: Self = Self(23);
    pub(crate) const ANGLE_MASK: u8 = 0b00000011;
    pub(crate) const ANGLE_MASK_I32: i32 = Self::ANGLE_MASK as i32;
    /// ((up << UP_SHIFT) & UP_MASK) | (angle & ANGLE_MASK)
    pub(crate) const UP_SHIFT: u32 = 2;
    
    pub const UNROTATED: Rotation = Rotation::new(Direction::PosY, 0);
    pub const ROTATE_X: Rotation = Rotation::new(Direction::PosZ, 0);
    pub const ROTATE_X_CCW: Rotation = Self::ROTATE_X;
    pub const ROTATE_X_CW: Rotation = Self::ROTATE_X.invert();
    pub const ROTATE_Y: Rotation = Rotation::new(Direction::PosY, 1);
    pub const ROTATE_Y_CCW: Rotation = Self::ROTATE_Y;
    pub const ROTATE_Y_CW: Rotation = Self::ROTATE_Y.invert();
    pub const ROTATE_Z: Rotation = Rotation::new(Direction::NegX, 1);
    pub const ROTATE_Z_CCW: Rotation = Self::ROTATE_Z;
    pub const ROTATE_Z_CW: Rotation = Self::ROTATE_Z.invert();
    pub const X_ROTATIONS: [Rotation; 4] = Self::ROTATE_X.angles();
    pub const Y_ROTATIONS: [Rotation; 4] = Self::ROTATE_Y.angles();
    pub const Z_ROTATIONS: [Rotation; 4] = Self::ROTATE_Z.angles();// verified

    // verified (2025-12-28)
    pub const CORNER_ROTATIONS_MATRIX: [[[[Rotation; 3]; 2]; 2]; 2] = [
        [
            [Rotation::new(Direction::PosX, 2).corner_angles(), Rotation::new(Direction::PosZ, 3).corner_angles()],
            [Rotation::new(Direction::NegZ, 1).corner_angles(), Rotation::new(Direction::NegX, 0).corner_angles()],
        ],
        [
            [Rotation::new(Direction::NegZ, 3).corner_angles(), Rotation::new(Direction::PosX, 0).corner_angles()],
            [Rotation::new(Direction::NegX, 2).corner_angles(), Rotation::new(Direction::PosZ, 1).corner_angles()],
        ],
    ];

    // verified (2025-12-28)
    pub const FACE_ROTATIONS: [[Rotation; 4]; 6] = [
        Self::Y_ROTATIONS, // PosY
        Self::X_ROTATIONS, // PosX
        Self::Z_ROTATIONS, // PosZ
        Self::ROTATE_Y.invert().angles(), // NegY
        Self::ROTATE_X.invert().angles(), // NegX
        Self::ROTATE_Z.invert().angles(), // NegZ
    ];

    // verified (2025-12-28)
    #[inline]
    pub const fn face_rotation(face: Direction, angle: i32) -> Self {
        Self::FACE_ROTATIONS[face.rotation_discriminant() as usize][wrap_angle(angle) as usize]
    }
    
    // verified (2025-12-28)
    pub const fn corner_rotation(x: i32, y: i32, z: i32, angle: i32) -> Rotation {
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
        let angle = wrap_angle(angle) as usize;
        Self::CORNER_ROTATIONS_MATRIX[y][z][x][angle]
    }
    
    #[inline]
    pub const fn new(up: Direction, angle: i32) -> Self {
        let up = up as u8;
        let angle = wrap_angle(angle) as u8;
        Self(angle | (up << Self::UP_SHIFT))
    }
    
    #[inline]
    pub const fn from_up(up: Direction) -> Self {
        Self::new(up, 0)
    }
    
    #[inline]
    pub const unsafe fn from_u8_unchecked(value: u8) -> Self {
        Self(value)
    }
    
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Self> {
        // Only first 24 values are valid rotations.
        if value > Self::MAX.0 {
            return None;
        }
        // SAFETY: Guard clause ensures that u8 is valid value.
        Some(unsafe { Self::from_u8_unchecked(value) })
    }
    
    #[inline]
    pub const fn to_u8(self) -> u8 {
        self.0
    }
    
    /// Creates a new [Rotation] with [Direction::NegX] as the up direction.
    #[inline]
    pub const fn neg_x(angle: i32) -> Self {
        Self::new(Direction::NegX, angle)
    }
    
    /// Creates a new [Rotation] with [Direction::NegY] as the up direction.
    #[inline]
    pub const fn neg_y(angle: i32) -> Self {
        Self::new(Direction::NegY, angle)
    }
    
    /// Creates a new [Rotation] with [Direction::NegZ] as the up direction.
    #[inline]
    pub const fn neg_z(angle: i32) -> Self {
        Self::new(Direction::NegZ, angle)
    }
    
    /// Creates a new [Rotation] with [Direction::PosX] as the up direction.
    #[inline]
    pub const fn pos_x(angle: i32) -> Self {
        Self::new(Direction::PosX, angle)
    }
    
    /// Creates a new [Rotation] with [Direction::PosY] as the up direction.
    #[inline]
    pub const fn pos_y(angle: i32) -> Self {
        Self::new(Direction::PosY, angle)
    }
    
    /// Creates a new [Rotation] with [Direction::PosZ] as the up direction.
    #[inline]
    pub const fn pos_z(angle: i32) -> Self {
        Self::new(Direction::PosZ, angle)
    }
    
    // verified (2025-12-28)
    /// A helper function to create 4 rotations for a rotation group.  
    /// A rotation group is a series of "contiguous" rotations. That is, the rotations are logically sequential.
    /// An example would be rotations around an axis, or around a face, where there are 4 rotations possible.
    /// The first rotation is unrotated, the second rotation is the target rotation, the
    /// third rotation is the target rotation applied twice, and
    /// the fourth rotation is the target rotation applied three times.
    pub const fn angles(self) -> [Self; 4] {
        let angle1 = self;
        let angle2 = angle1.reorient(angle1);
        let angle3 = angle2.reorient(angle1);
        [
            Self::UNROTATED,
            angle1,
            angle2,
            angle3,
        ]
    }
    
    // verified (2025-12-28)
    /// A helper function to create 3 rotations for a corner rotation group.
    /// The first rotation is unrotated, the second rotation is the target rotation,
    /// and the third rotation is the target rotation applied to itself.
    pub const fn corner_angles(self) -> [Self; 3] {
        let angle1 = self;
        let angle2 = angle1.reorient(angle1);
        [
            Self::UNROTATED,
            angle1,
            angle2
        ]
    }

    // verified (2025-12-28)
    #[inline]
    pub const fn with_flip(self, flip: super::Flip) -> Orientation {
        Orientation::new(self, flip)
    }

    // verified (2025-12-28)
    #[inline]
    pub const fn orientation(self) -> Orientation {
        self.with_flip(super::Flip::NONE)
    }
    
    // verified (2025-12-28)
    pub const fn from_up_and_forward(up: Direction, forward: Direction) -> Option<Rotation> {
        use Direction::*;
        Some(Rotation::new(up, match (up, forward) {
            (PosY, PosX) => 3,
            (PosY, PosZ) => 2,
            (PosY, NegX) => 1,
            (PosY, NegZ) => 0,
            (PosX, PosY) => 0,
            (PosX, PosZ) => 1,
            (PosX, NegY) => 2,
            (PosX, NegZ) => 3,
            (PosZ, PosY) => 0,
            (PosZ, PosX) => 3,
            (PosZ, NegY) => 2,
            (PosZ, NegX) => 1,
            (NegY, PosX) => 3,
            (NegY, PosZ) => 0,
            (NegY, NegX) => 1,
            (NegY, NegZ) => 2,
            (NegX, PosY) => 0,
            (NegX, PosZ) => 3,
            (NegX, NegY) => 2,
            (NegX, NegZ) => 1,
            (NegZ, PosY) => 0,
            (NegZ, PosX) => 1,
            (NegZ, NegY) => 2,
            (NegZ, NegX) => 3,
            _ => return None,
        }))
    }

    // Yes, this method works. I checked.
    /// Cycle through rotations (24 in total).
    #[inline]
    #[must_use]
    pub const fn cycle(self, offset: i32) -> Rotation {
        let index = self.0 as i32;
        // Don't use wrapping_add here, as tempting as it seems. It would be incorrect because 2**32 is not a multiple of 24.
        let new_index = (index as i64 + offset as i64).rem_euclid(24) as u8;
        Rotation(new_index)
    }

    #[inline]
    pub const fn angle(self) -> i32 {
        (self.0 & Self::ANGLE_MASK) as i32
    }

    #[inline]
    pub fn set_up(&mut self, up: Direction) {
        const ANGLE_ISOLATE_MASK: u8 = 0b00000011;
        self.0 = (self.0 & ANGLE_ISOLATE_MASK) | (up.rotation_discriminant() << Self::UP_SHIFT);
    }

    // verified (2025-12-28)
    #[inline]
    pub fn set_angle(&mut self, angle: i32) {
        const UP_ISOLATE_MASK: u8 = 0b11111100;
        self.0 = (self.0 & UP_ISOLATE_MASK) | wrap_angle(angle) as u8;
    }
    
    #[inline]
    pub fn iter() -> RotationIterator {
        RotationIterator::START
    }
    
    // verified (2025-12-28)
    pub const fn up(self) -> Direction {
        let up = self.0 >> Self::UP_SHIFT;
        match up {
            0 => Direction::PosY,
            1 => Direction::PosX,
            2 => Direction::PosZ,
            3 => Direction::NegY,
            4 => Direction::NegX,
            5 => Direction::NegZ,
            _ => unreachable!(),
        }
    }

    // verified (2025-12-28)
    pub const fn down(self) -> Direction {
        let up = self.0 >> Self::UP_SHIFT;
        match up {
            4 => Direction::PosX,
            3 => Direction::PosY,
            5 => Direction::PosZ,
            1 => Direction::NegX,
            0 => Direction::NegY,
            2 => Direction::NegZ,
            _ => unreachable!(),
        }
    }

    // verified (2025-12-28)
    pub const fn left(self) -> Direction {
        use Direction::*;
        match (self.angle(), self.up()) {
            (0, PosY) => NegX,
            (0, PosX) => PosZ,
            (0, PosZ) => NegX,
            (0, NegY) => NegX,
            (0, NegX) => NegZ,
            (0, NegZ) => PosX,
            (1, PosY) => PosZ,
            (1, PosX) => NegY,
            (1, PosZ) => NegY,
            (1, NegY) => NegZ,
            (1, NegX) => NegY,
            (1, NegZ) => NegY,
            (2, PosY) => PosX,
            (2, PosX) => NegZ,
            (2, PosZ) => PosX,
            (2, NegY) => PosX,
            (2, NegX) => PosZ,
            (2, NegZ) => NegX,
            (3, PosY) => NegZ,
            (3, PosX) => PosY,
            (3, PosZ) => PosY,
            (3, NegY) => PosZ,
            (3, NegX) => PosY,
            (3, NegZ) => PosY,
            _ => unreachable!(),
        }
    }

    // verified (2025-12-28)
    pub const fn right(self) -> Direction {
        use Direction::*;
        match (self.angle(), self.up()) {
            (0, PosY) => PosX,
            (0, PosX) => NegZ,
            (0, PosZ) => PosX,
            (0, NegY) => PosX,
            (0, NegX) => PosZ,
            (0, NegZ) => NegX,
            (1, PosY) => NegZ,
            (1, PosX) => PosY,
            (1, PosZ) => PosY,
            (1, NegY) => PosZ,
            (1, NegX) => PosY,
            (1, NegZ) => PosY,
            (2, PosY) => NegX,
            (2, PosX) => PosZ,
            (2, PosZ) => NegX,
            (2, NegY) => NegX,
            (2, NegX) => NegZ,
            (2, NegZ) => PosX,
            (3, PosY) => PosZ,
            (3, PosX) => NegY,
            (3, PosZ) => NegY,
            (3, NegY) => NegZ,
            (3, NegX) => NegY,
            (3, NegZ) => NegY,
            _ => unreachable!(),
        }
    }

    // verified (2025-12-28)
    pub const fn forward(self) -> Direction {
        use Direction::*;
        match (self.angle(), self.up()) {
            (0, PosY) => NegZ,
            (0, PosX) => PosY,
            (0, PosZ) => PosY,
            (0, NegY) => PosZ,
            (0, NegX) => PosY,
            (0, NegZ) => PosY,
            (1, PosY) => NegX,
            (1, PosX) => PosZ,
            (1, PosZ) => NegX,
            (1, NegY) => NegX,
            (1, NegX) => NegZ,
            (1, NegZ) => PosX,
            (2, PosY) => PosZ,
            (2, PosX) => NegY,
            (2, PosZ) => NegY,
            (2, NegY) => NegZ,
            (2, NegX) => NegY,
            (2, NegZ) => NegY,
            (3, PosY) => PosX,
            (3, PosX) => NegZ,
            (3, PosZ) => PosX,
            (3, NegY) => PosX,
            (3, NegX) => PosZ,
            (3, NegZ) => NegX,
            _ => unreachable!(),
        }
    }

    // verified (2025-12-28)
    pub const fn backward(self) -> Direction {
        // self.forward().invert()
        use Direction::*;
        match (self.angle(), self.up()) {
            (0, PosY) => PosZ,
            (0, PosX) => NegY,
            (0, PosZ) => NegY,
            (0, NegY) => NegZ,
            (0, NegX) => NegY,
            (0, NegZ) => NegY,
            (1, PosY) => PosX,
            (1, PosX) => NegZ,
            (1, PosZ) => PosX,
            (1, NegY) => PosX,
            (1, NegX) => PosZ,
            (1, NegZ) => NegX,
            (2, PosY) => NegZ,
            (2, PosX) => PosY,
            (2, PosZ) => PosY,
            (2, NegY) => PosZ,
            (2, NegX) => PosY,
            (2, NegZ) => PosY,
            (3, PosY) => NegX,
            (3, PosX) => PosZ,
            (3, PosZ) => NegX,
            (3, NegY) => NegX,
            (3, NegX) => NegZ,
            (3, NegZ) => PosX,
            _ => unreachable!(),
        }        
    }

    // unverified (2025-12-28)
    /// Rotates `coord`.
    pub fn rotate_coord<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T, T)> + From<(T, T, T)>>(self, coord: C) -> C {
        let (x, y, z): (T, T, T) = coord.into();
        // In case this code breaks, I'm leaving the old implementation.
        // use Direction::*;
        // C::from(match (self.angle(), self.up()) {
        //     (0, PosY) => (x, y, z), // Default rotation, no change.
        //     (0, PosX) => (y, -z, -x),
        //     (0, PosZ) => (x, -z, y),
        //     (0, NegY) => (x, -y, -z),
        //     (0, NegX) => (-y, -z, x),
        //     (0, NegZ) => (-x, -z, -y),
        //     (1, PosY) => (-z, y, x),
        //     (1, PosX) => (y, -x, z),
        //     (1, PosZ) => (-z, -x, y),
        //     (1, NegY) => (-z, -y, -x),
        //     (1, NegX) => (-y, -x, -z),
        //     (1, NegZ) => (z, -x, -y),
        //     (2, PosY) => (-x, y, -z),
        //     (2, PosX) => (y, z, x),
        //     (2, PosZ) => (-x, z, y),
        //     (2, NegY) => (-x, -y, z),
        //     (2, NegX) => (-y, z, -x),
        //     (2, NegZ) => (x, z, -y),
        //     (3, PosY) => (z, y, -x),
        //     (3, PosX) => (y, x, -z),
        //     (3, PosZ) => (z, x, y),
        //     (3, NegY) => (z, -y, x),
        //     (3, NegX) => (-y, x, z),
        //     (3, NegZ) => (-z, x, -y),
        // })
        C::from(match self.0 {
            00 /* (0, PosY) */ => (x, y, z), // Default rotation, no change.
            01 /* (1, PosY) */ => (z, y, -x),
            02 /* (2, PosY) */ => (-x, y, -z),
            03 /* (3, PosY) */ => (-z, y, x),
            04 /* (0, PosX) */ => (y, -z, -x),
            05 /* (1, PosX) */ => (y, x, -z),
            06 /* (2, PosX) */ => (y, z, x),
            07 /* (3, PosX) */ => (y, -x, z),
            08 /* (0, PosZ) */ => (x, -z, y),
            09 /* (1, PosZ) */ => (z, x, y),
            10 /* (2, PosZ) */ => (-x, z, y),
            11 /* (3, PosZ) */ => (-z, -x, y),
            12 /* (0, NegY) */ => (x, -y, -z),
            13 /* (1, NegY) */ => (z, -y, x),
            14 /* (2, NegY) */ => (-x, -y, z),
            15 /* (3, NegY) */ => (-z, -y, -x),
            16 /* (0, NegX) */ => (-y, -z, x),
            17 /* (1, NegX) */ => (-y, x, z),
            18 /* (2, NegX) */ => (-y, z, -x),
            19 /* (3, NegX) */ => (-y, -x, -z),
            
            20 /* (0, NegZ) */ => (-x, -z, -y),
            21 /* (1, NegZ) */ => (-z, x, -y),
            22 /* (2, NegZ) */ => (x, z, -y),
            23 /* (3, NegZ) */ => (z, -x, -y),
            _  => unreachable!(),
        })
    }
    
    // verified (2025-12-28): reface and source_face are symmetrical.
    /// Rotates direction.
    pub const fn reface(self, direction: Direction) -> Direction {
        match direction {
            Direction::NegX => self.left(),
            Direction::NegY => self.down(),
            Direction::NegZ => self.forward(),
            Direction::PosX => self.right(),
            Direction::PosY => self.up(),
            Direction::PosZ => self.backward(),
        }
    }

    // verified (2025-12-28): source_face and reface are symmetrical.
    /// Tells which [Direction] rotated to `destination`.
    pub const fn source_face(self, destination: Direction) -> Direction {
        // This code was bootstrap generated. I wrote a naive solution,
        // then generated this code with the naive solution.
        // Besides maybe if you rearrange the order of matching,
        // this should theoretically be the optimal solution.
        use Direction::*;
        match ((self.angle(), self.up()), destination) {
            ((0, PosY), PosY) => PosY,
            ((0, PosY), PosX) => PosX,
            ((0, PosY), PosZ) => PosZ,
            ((0, PosY), NegY) => NegY,
            ((0, PosY), NegX) => NegX,
            ((0, PosY), NegZ) => NegZ,
            ((0, PosX), PosY) => NegZ,
            ((0, PosX), PosX) => PosY,
            ((0, PosX), PosZ) => NegX,
            ((0, PosX), NegY) => PosZ,
            ((0, PosX), NegX) => NegY,
            ((0, PosX), NegZ) => PosX,
            ((0, PosZ), PosY) => NegZ,
            ((0, PosZ), PosX) => PosX,
            ((0, PosZ), PosZ) => PosY,
            ((0, PosZ), NegY) => PosZ,
            ((0, PosZ), NegX) => NegX,
            ((0, PosZ), NegZ) => NegY,
            ((0, NegY), PosY) => NegY,
            ((0, NegY), PosX) => PosX,
            ((0, NegY), PosZ) => NegZ,
            ((0, NegY), NegY) => PosY,
            ((0, NegY), NegX) => NegX,
            ((0, NegY), NegZ) => PosZ,
            ((0, NegX), PosY) => NegZ,
            ((0, NegX), PosX) => NegY,
            ((0, NegX), PosZ) => PosX,
            ((0, NegX), NegY) => PosZ,
            ((0, NegX), NegX) => PosY,
            ((0, NegX), NegZ) => NegX,
            ((0, NegZ), PosY) => NegZ,
            ((0, NegZ), PosX) => NegX,
            ((0, NegZ), PosZ) => NegY,
            ((0, NegZ), NegY) => PosZ,
            ((0, NegZ), NegX) => PosX,
            ((0, NegZ), NegZ) => PosY,
            ((1, PosY), PosY) => PosY,
            ((1, PosY), PosX) => PosZ,
            ((1, PosY), PosZ) => NegX,
            ((1, PosY), NegY) => NegY,
            ((1, PosY), NegX) => NegZ,
            ((1, PosY), NegZ) => PosX,
            ((1, PosX), PosY) => PosX,
            ((1, PosX), PosX) => PosY,
            ((1, PosX), PosZ) => NegZ,
            ((1, PosX), NegY) => NegX,
            ((1, PosX), NegX) => NegY,
            ((1, PosX), NegZ) => PosZ,
            ((1, PosZ), PosY) => PosX,
            ((1, PosZ), PosX) => PosZ,
            ((1, PosZ), PosZ) => PosY,
            ((1, PosZ), NegY) => NegX,
            ((1, PosZ), NegX) => NegZ,
            ((1, PosZ), NegZ) => NegY,
            ((1, NegY), PosY) => NegY,
            ((1, NegY), PosX) => PosZ,
            ((1, NegY), PosZ) => PosX,
            ((1, NegY), NegY) => PosY,
            ((1, NegY), NegX) => NegZ,
            ((1, NegY), NegZ) => NegX,
            ((1, NegX), PosY) => PosX,
            ((1, NegX), PosX) => NegY,
            ((1, NegX), PosZ) => PosZ,
            ((1, NegX), NegY) => NegX,
            ((1, NegX), NegX) => PosY,
            ((1, NegX), NegZ) => NegZ,
            ((1, NegZ), PosY) => PosX,
            ((1, NegZ), PosX) => NegZ,
            ((1, NegZ), PosZ) => NegY,
            ((1, NegZ), NegY) => NegX,
            ((1, NegZ), NegX) => PosZ,
            ((1, NegZ), NegZ) => PosY,
            ((2, PosY), PosY) => PosY,
            ((2, PosY), PosX) => NegX,
            ((2, PosY), PosZ) => NegZ,
            ((2, PosY), NegY) => NegY,
            ((2, PosY), NegX) => PosX,
            ((2, PosY), NegZ) => PosZ,
            ((2, PosX), PosY) => PosZ,
            ((2, PosX), PosX) => PosY,
            ((2, PosX), PosZ) => PosX,
            ((2, PosX), NegY) => NegZ,
            ((2, PosX), NegX) => NegY,
            ((2, PosX), NegZ) => NegX,
            ((2, PosZ), PosY) => PosZ,
            ((2, PosZ), PosX) => NegX,
            ((2, PosZ), PosZ) => PosY,
            ((2, PosZ), NegY) => NegZ,
            ((2, PosZ), NegX) => PosX,
            ((2, PosZ), NegZ) => NegY,
            ((2, NegY), PosY) => NegY,
            ((2, NegY), PosX) => NegX,
            ((2, NegY), PosZ) => PosZ,
            ((2, NegY), NegY) => PosY,
            ((2, NegY), NegX) => PosX,
            ((2, NegY), NegZ) => NegZ,
            ((2, NegX), PosY) => PosZ,
            ((2, NegX), PosX) => NegY,
            ((2, NegX), PosZ) => NegX,
            ((2, NegX), NegY) => NegZ,
            ((2, NegX), NegX) => PosY,
            ((2, NegX), NegZ) => PosX,
            ((2, NegZ), PosY) => PosZ,
            ((2, NegZ), PosX) => PosX,
            ((2, NegZ), PosZ) => NegY,
            ((2, NegZ), NegY) => NegZ,
            ((2, NegZ), NegX) => NegX,
            ((2, NegZ), NegZ) => PosY,
            ((3, PosY), PosY) => PosY,
            ((3, PosY), PosX) => NegZ,
            ((3, PosY), PosZ) => PosX,
            ((3, PosY), NegY) => NegY,
            ((3, PosY), NegX) => PosZ,
            ((3, PosY), NegZ) => NegX,
            ((3, PosX), PosY) => NegX,
            ((3, PosX), PosX) => PosY,
            ((3, PosX), PosZ) => PosZ,
            ((3, PosX), NegY) => PosX,
            ((3, PosX), NegX) => NegY,
            ((3, PosX), NegZ) => NegZ,
            ((3, PosZ), PosY) => NegX,
            ((3, PosZ), PosX) => NegZ,
            ((3, PosZ), PosZ) => PosY,
            ((3, PosZ), NegY) => PosX,
            ((3, PosZ), NegX) => PosZ,
            ((3, PosZ), NegZ) => NegY,
            ((3, NegY), PosY) => NegY,
            ((3, NegY), PosX) => NegZ,
            ((3, NegY), PosZ) => NegX,
            ((3, NegY), NegY) => PosY,
            ((3, NegY), NegX) => PosZ,
            ((3, NegY), NegZ) => PosX,
            ((3, NegX), PosY) => NegX,
            ((3, NegX), PosX) => NegY,
            ((3, NegX), PosZ) => NegZ,
            ((3, NegX), NegY) => PosX,
            ((3, NegX), NegX) => PosY,
            ((3, NegX), NegZ) => PosZ,
            ((3, NegZ), PosY) => NegX,
            ((3, NegZ), PosX) => PosZ,
            ((3, NegZ), PosZ) => NegY,
            ((3, NegZ), NegY) => PosX,
            ((3, NegZ), NegX) => NegZ,
            ((3, NegZ), NegZ) => PosY,
            _ => unreachable!(),
        }
    }

    // verified (2025-12-28)
    // double verified (2025-12-29)
    /// Gets the angle of the face oriented to `world_face`.
    pub fn face_angle(self, world_face: Direction) -> u8 {
        use Direction::*;
        match (self.angle(), self.up(), world_face) {
            (0, NegX, NegX) => 0,
            (0, NegX, NegY) => 1,
            (0, NegX, NegZ) => 3,
            (0, NegX, PosX) => 2,
            (0, NegX, PosY) => 1,
            (0, NegX, PosZ) => 1,
            (0, NegY, NegX) => 2,
            (0, NegY, NegY) => 0,
            (0, NegY, NegZ) => 2,
            (0, NegY, PosX) => 2,
            (0, NegY, PosY) => 0,
            (0, NegY, PosZ) => 2,
            (0, NegZ, NegX) => 1,
            (0, NegZ, NegY) => 2,
            (0, NegZ, NegZ) => 0,
            (0, NegZ, PosX) => 3,
            (0, NegZ, PosY) => 0,
            (0, NegZ, PosZ) => 2,
            (0, PosX, NegX) => 2,
            (0, PosX, NegY) => 3,
            (0, PosX, NegZ) => 1,
            (0, PosX, PosX) => 0,
            (0, PosX, PosY) => 3,
            (0, PosX, PosZ) => 3,
            (0, PosY, NegX) => 0,
            (0, PosY, NegY) => 0,
            (0, PosY, NegZ) => 0,
            (0, PosY, PosX) => 0,
            (0, PosY, PosY) => 0,
            (0, PosY, PosZ) => 0,
            (0, PosZ, NegX) => 3,
            (0, PosZ, NegY) => 0,
            (0, PosZ, NegZ) => 2,
            (0, PosZ, PosX) => 1,
            (0, PosZ, PosY) => 2,
            (0, PosZ, PosZ) => 0,
            (1, NegX, NegX) => 1,
            (1, NegX, NegY) => 1,
            (1, NegX, NegZ) => 3,
            (1, NegX, PosX) => 1,
            (1, NegX, PosY) => 1,
            (1, NegX, PosZ) => 1,
            (1, NegY, NegX) => 2,
            (1, NegY, NegY) => 1,
            (1, NegY, NegZ) => 2,
            (1, NegY, PosX) => 2,
            (1, NegY, PosY) => 3,
            (1, NegY, PosZ) => 2,
            (1, NegZ, NegX) => 1,
            (1, NegZ, NegY) => 2,
            (1, NegZ, NegZ) => 1,
            (1, NegZ, PosX) => 3,
            (1, NegZ, PosY) => 0,
            (1, NegZ, PosZ) => 1,
            (1, PosX, NegX) => 1,
            (1, PosX, NegY) => 3,
            (1, PosX, NegZ) => 1,
            (1, PosX, PosX) => 1,
            (1, PosX, PosY) => 3,
            (1, PosX, PosZ) => 3,
            (1, PosY, NegX) => 0,
            (1, PosY, NegY) => 3,
            (1, PosY, NegZ) => 0,
            (1, PosY, PosX) => 0,
            (1, PosY, PosY) => 1,
            (1, PosY, PosZ) => 0,
            (1, PosZ, NegX) => 3,
            (1, PosZ, NegY) => 0,
            (1, PosZ, NegZ) => 1,
            (1, PosZ, PosX) => 1,
            (1, PosZ, PosY) => 2,
            (1, PosZ, PosZ) => 1,
            (2, NegX, NegX) => 2,
            (2, NegX, NegY) => 1,
            (2, NegX, NegZ) => 3,
            (2, NegX, PosX) => 0,
            (2, NegX, PosY) => 1,
            (2, NegX, PosZ) => 1,
            (2, NegY, NegX) => 2,
            (2, NegY, NegY) => 2,
            (2, NegY, NegZ) => 2,
            (2, NegY, PosX) => 2,
            (2, NegY, PosY) => 2,
            (2, NegY, PosZ) => 2,
            (2, NegZ, NegX) => 1,
            (2, NegZ, NegY) => 2,
            (2, NegZ, NegZ) => 2,
            (2, NegZ, PosX) => 3,
            (2, NegZ, PosY) => 0,
            (2, NegZ, PosZ) => 0,
            (2, PosX, NegX) => 0,
            (2, PosX, NegY) => 3,
            (2, PosX, NegZ) => 1,
            (2, PosX, PosX) => 2,
            (2, PosX, PosY) => 3,
            (2, PosX, PosZ) => 3,
            (2, PosY, NegX) => 0,
            (2, PosY, NegY) => 2,
            (2, PosY, NegZ) => 0,
            (2, PosY, PosX) => 0,
            (2, PosY, PosY) => 2,
            (2, PosY, PosZ) => 0,
            (2, PosZ, NegX) => 3,
            (2, PosZ, NegY) => 0,
            (2, PosZ, NegZ) => 0,
            (2, PosZ, PosX) => 1,
            (2, PosZ, PosY) => 2,
            (2, PosZ, PosZ) => 2,
            (3, NegX, NegX) => 3,
            (3, NegX, NegY) => 1,
            (3, NegX, NegZ) => 3,
            (3, NegX, PosX) => 3,
            (3, NegX, PosY) => 1,
            (3, NegX, PosZ) => 1,
            (3, NegY, NegX) => 2,
            (3, NegY, NegY) => 3,
            (3, NegY, NegZ) => 2,
            (3, NegY, PosX) => 2,
            (3, NegY, PosY) => 1,
            (3, NegY, PosZ) => 2,
            (3, NegZ, NegX) => 1,
            (3, NegZ, NegY) => 2,
            (3, NegZ, NegZ) => 3,
            (3, NegZ, PosX) => 3,
            (3, NegZ, PosY) => 0,
            (3, NegZ, PosZ) => 3,
            (3, PosX, NegX) => 3,
            (3, PosX, NegY) => 3,
            (3, PosX, NegZ) => 1,
            (3, PosX, PosX) => 3,
            (3, PosX, PosY) => 3,
            (3, PosX, PosZ) => 3,
            (3, PosY, NegX) => 0,
            (3, PosY, NegY) => 1,
            (3, PosY, NegZ) => 0,
            (3, PosY, PosX) => 0,
            (3, PosY, PosY) => 3,
            (3, PosY, PosZ) => 0,
            (3, PosZ, NegX) => 3,
            (3, PosZ, NegY) => 0,
            (3, PosZ, NegZ) => 3,
            (3, PosZ, PosX) => 1,
            (3, PosZ, PosY) => 2,
            (3, PosZ, PosZ) => 3,
            _ => unreachable!(),
        }
    }

    // verified (2025-12-28)
    /// Rotate a [Rotation] by another [Rotation].
    pub const fn reorient(self, rotation: Self) -> Self {
        // What??? I know I wrote this code, but this is kinda nuts.
        let up = self.up();
        let fwd = self.forward();
        let rot_up = rotation.reface(up);
        let rot_fwd = rotation.reface(fwd);
        // Pattern matching is used here because it's a const fn and unwrap()
        // won't work.
        let Some(rot) = Self::from_up_and_forward(rot_up, rot_fwd) else {
            unreachable!()
        };
        rot
    }

    // verified (2025-12-28)
    /// Rotate a [Rotation] by the inverse of another [Rotation].
    pub const fn deorient(self, rotation: Self) -> Self {
        let up = self.up();
        let fwd = self.forward();
        let rot_up = rotation.source_face(up);
        let rot_fwd = rotation.source_face(fwd);
        // Pattern matching is used here because it's a const fn and unwrap()
        // won't work.
        let Some(rot) = Self::from_up_and_forward(rot_up, rot_fwd) else {
            unreachable!()
        };
        rot
    }
    
    // verified (2025-12-28)
    /// Creates a [Rotation] that when rotated by the original will create the base [Rotation].
    #[inline]
    pub const fn invert(self) -> Self {
        Self::UNROTATED.deorient(self)
    }

    #[inline]
    pub const fn rotate_x(self, angle: i32) -> Self {
        self.reorient(Self::X_ROTATIONS[wrap_angle(angle) as usize])
    }

    #[inline]
    pub const fn rotate_y(self, angle: i32) -> Self {
        self.reorient(Self::Y_ROTATIONS[wrap_angle(angle) as usize])
    }

    #[inline]
    pub const fn rotate_z(self, angle: i32) -> Self {
        self.reorient(Self::Z_ROTATIONS[wrap_angle(angle) as usize])
    }

    /// Rotate `face` counter-clockwise by `angle`. Use a negative `angle` to rotate clockwise.
    #[inline]
    pub const fn rotate_face(self, face: Direction, angle: i32) -> Self {
        let rot = Self::face_rotation(face, angle);
        self.reorient(rot)
    }

    /// Rotate corner clockwise by `angle`.
    #[inline]
    pub const fn rotate_corner(self, x: i32, y: i32, z: i32, angle: i32) -> Self {
        let rot = Self::corner_rotation(x, y, z, angle);
        self.reorient(rot)
    }

    // #[inline]
    // pub fn to_matrix(self) -> glam::Mat4 {
    //     let up = self.reface(Direction::PosY).to_vec3();
    //     let forward = self.reface(Direction::PosZ).to_vec3();
    //     let right = self.reface(Direction::NegX).to_vec3();
    //     glam::Mat4::from_cols(
    //         right.extend(0.0),
    //         up.extend(0.0),
    //         forward.extend(0.0),
    //         glam::Vec3::ZERO.extend(1.0),
    //     )
    // }
}

// #[cfg(test)]
// mod testing_sandbox {
//     // TODO: Remove this sandbox when it is no longer in use.
//     use super::*;
//     #[test]
//     fn sandbox() {
//         let rot = Rotation::new(Direction::NegX, 3);
//         let mat = rot.to_matrix();
//         let trans = mat.transform_point3(glam::Vec3::NEG_Z);
//         println!("{trans:?}");
//         println!("{}", rot.source_face(Direction::NegZ));
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RotationIterator {
    rotation: u8,
}

impl RotationIterator {
    pub const START: Self = Self { rotation: 0 };
    
    #[inline]
    pub const fn start_at(rotation: Rotation) -> Self {
        Self { rotation: rotation.0 }
    }
    
    #[inline]
    pub const fn new() -> Self {
        Self::START
    }
    
    pub const fn current(self) -> Option<Rotation> {
        if self.rotation == 24 {
            return None;
        }
        Some(Rotation(self.rotation))
    }
}

impl Iterator for RotationIterator {
    type Item = Rotation;
    
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        const MAX: u8 = 24;
        let remain = (MAX - self.rotation) as usize;
        (remain, Some(remain))
    }
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.rotation == 24 {
            return None;
        }
        let result = Some(Rotation(self.rotation));
        self.rotation += 1;
        result
    }
}

impl From<Direction> for Rotation {
    #[inline]
    fn from(value: Direction) -> Self {
        Self::new(value, 0)
    }
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rotation(up={},forward={},angle={})", self.up(), self.forward(), self.angle())
    }
}