// Last Reviewed: (2025-12-28)

use crate::wrap_angle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Cardinal {
    /// -Z
    North = 0,
    /// -X
    West  = 1,
    /// +Z
    South = 2,
    /// +X
    East  = 3,
}

impl Cardinal {
    pub const FORWARD: Cardinal = Cardinal::North;
    pub const LEFT: Cardinal = Cardinal::West;
    pub const BACKWARD: Cardinal = Cardinal::South;
    pub const RIGHT: Cardinal = Cardinal::East;
    /// Ordered: West, South, East, North
    pub const ALL: [Cardinal; 4] = [
        Cardinal::North,
        Cardinal::West,
        Cardinal::South,
        Cardinal::East,
    ];

    pub const FLOOD_NORTH_EAST: [Cardinal; 4] = [
        Cardinal::North,
        Cardinal::South,
        Cardinal::East,
        Cardinal::West,
    ];

    pub const FLOOD_NORTH_WEST: [Cardinal; 4] = [
        Cardinal::North,
        Cardinal::South,
        Cardinal::West,
        Cardinal::East,
    ];

    pub const FLOOD_SOUTH_EAST: [Cardinal; 4] = [
        Cardinal::South,
        Cardinal::North,
        Cardinal::East,
        Cardinal::West,
    ];

    pub const FLOOD_SOUTH_WEST: [Cardinal; 4] = [
        Cardinal::South,
        Cardinal::North,
        Cardinal::West,
        Cardinal::East,
    ];
    
    #[inline]
    pub const fn at_angle(angle: i32) -> Self {
        Self::ALL[wrap_angle(angle) as usize]
    }

    /// Rotates the [Cardinal] direction counter-clockwise by `rotation`.
    #[inline]
    pub const fn rotate(self, rotation: i32) -> Self {
        let new_index = ((rotation as i64 + self.angle() as i64) & 3) as usize;
        Self::ALL[new_index]
    }

    /// Inverts the [Cardinal] to the opposite direction.
    #[inline]
    pub const fn invert(self) -> Self {
        match self {
            Cardinal::North => Cardinal::South,
            Cardinal::West => Cardinal::East,
            Cardinal::South => Cardinal::North,
            Cardinal::East => Cardinal::West,
        }
    }

    /// Gets the [Cardinal] as a single bit based on discriminant.
    #[inline]
    pub const fn bit(self) -> u8 {
        1 << self as u8
    }

    #[inline]
    pub const fn discriminant(self) -> u8 {
        self as u8
    }
    
    #[inline]
    pub const fn angle(self) -> i32 {
        match self {
            Cardinal::North => 0,
            Cardinal::West => 1,
            Cardinal::South => 2,
            Cardinal::East => 3,
        }
    }

    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }

    #[inline]
    pub fn iter_ne() -> impl Iterator<Item = Self> {
        Self::FLOOD_NORTH_EAST.into_iter()
    }

    #[inline]
    pub fn iter_nw() -> impl Iterator<Item = Self> {
        Self::FLOOD_NORTH_WEST.into_iter()
    }

    #[inline]
    pub fn iter_se() -> impl Iterator<Item = Self> {
        Self::FLOOD_SOUTH_EAST.into_iter()
    }

    #[inline]
    pub fn iter_sw() -> impl Iterator<Item = Self> {
        Self::FLOOD_SOUTH_WEST.into_iter()
    }

    // #[inline]
    // pub const fn to_ivec2(self) -> glam::IVec2 {
    //     match self {
    //         Cardinal::West => glam::ivec2(-1, 0),
    //         Cardinal::North => glam::ivec2(0, -1),
    //         Cardinal::East => glam::ivec2(1, 0),
    //         Cardinal::South => glam::ivec2(0, 1),
    //     }
    // }

    #[inline]
    pub const fn to_ituple2(self) -> (i32, i32) {
        match self {
            Cardinal::North => (0, -1),
            Cardinal::West => (-1, 0),
            Cardinal::South => (0, 1),
            Cardinal::East => (1, 0),
        }
    }

    // #[inline]
    // pub const fn to_vec2(self) -> glam::Vec2 {
    //     match self {
    //         Cardinal::West => glam::vec2(-1., 0.),
    //         Cardinal::North => glam::vec2(0., -1.),
    //         Cardinal::East => glam::vec2(1., 0.),
    //         Cardinal::South => glam::vec2(0., 1.),
    //     }
    // }

    #[inline]
    pub const fn to_ftuple2(self) -> (f32, f32) {
        match self {
            Cardinal::North => (0., -1.),
            Cardinal::West => (-1., 0.),
            Cardinal::South => (0., 1.),
            Cardinal::East => (1., 0.),
        }
    }

    // #[inline]
    // pub const fn to_ivec3(self) -> glam::IVec3 {
    //     match self {
    //         Cardinal::West => glam::ivec3(-1, 0, 0),
    //         Cardinal::North => glam::ivec3(0, 0, -1),
    //         Cardinal::East => glam::ivec3(1, 0, 0),
    //         Cardinal::South => glam::ivec3(0, 0, 1),
    //     }
    // }

    #[inline]
    pub const fn to_ituple3(self) -> (i32, i32, i32) {
        match self {
            Cardinal::North => (0, 0, -1),
            Cardinal::West => (-1, 0, 0),
            Cardinal::South => (0, 0, 1),
            Cardinal::East => (1, 0, 0),
        }
    }

    // #[inline]
    // pub const fn to_vec3(self) -> glam::Vec3 {
    //     match self {
    //         Cardinal::West => glam::vec3(-1., 0., 0.),
    //         Cardinal::North => glam::vec3(0., 0., -1.),
    //         Cardinal::East => glam::vec3(1., 0., 0.),
    //         Cardinal::South => glam::vec3(0., 0., 1.),
    //     }
    // }

    #[inline]
    pub const fn to_ftuple3(self) -> (f32, f32, f32) {
        match self {
            Cardinal::North => (0., 0., -1.),
            Cardinal::West => (-1., 0., 0.),
            Cardinal::South => (0., 0., 1.),
            Cardinal::East => (1., 0., 0.),
        }
    }
}

// impl Into<glam::IVec2> for Cardinal {
//     fn into(self) -> glam::IVec2 {
//         self.to_ivec2()
//     }
// }

// impl Into<glam::IVec3> for Cardinal {
//     fn into(self) -> glam::IVec3 {
//         self.to_ivec3()
//     }
// }

// impl Into<glam::Vec2> for Cardinal {
//     fn into(self) -> glam::Vec2 {
//         self.to_vec2()
//     }
// }

// impl Into<glam::Vec3> for Cardinal {
//     fn into(self) -> glam::Vec3 {
//         self.to_vec3()
//     }
// }

impl Into<(i32, i32)> for Cardinal {
    fn into(self) -> (i32, i32) {
        self.to_ituple2()
    }
}

impl Into<(i32, i32, i32)> for Cardinal {
    fn into(self) -> (i32, i32, i32) {
        self.to_ituple3()
    }
}

impl Into<(f32, f32)> for Cardinal {
    fn into(self) -> (f32, f32) {
        self.to_ftuple2()
    }
}

impl Into<(f32, f32, f32)> for Cardinal {
    fn into(self) -> (f32, f32, f32) {
        self.to_ftuple3()
    }
}

impl Into<[i32; 2]> for Cardinal {
    fn into(self) -> [i32; 2] {
        let (x, y) = self.to_ituple2();
        [x, y]
    }
}

impl Into<[i32; 3]> for Cardinal {
    fn into(self) -> [i32; 3] {
        let (x, y, z) = self.to_ituple3();
        [x, y, z]
    }
}

impl Into<[f32; 2]> for Cardinal {
    fn into(self) -> [f32; 2] {
        let (x, y) = self.to_ftuple2();
        [x, y]
    }
}

impl Into<[f32; 3]> for Cardinal {
    fn into(self) -> [f32; 3] {
        let (x, y, z) = self.to_ftuple3();
        [x, y, z]
    }
}