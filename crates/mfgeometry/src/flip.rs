// Last Reviewed: 2025-12-28
use paste::paste;

use crate::{direction::Direction};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Flip(pub(crate) u8);

macro_rules! flip_axes {
    ($(
        {const $const_name:ident = $bin:literal; fn $fn_name:ident}
    )*) => {
        $(
            paste!{
                // pub const X: Self = Self(0b001);
                pub const $const_name: Self = Self($bin);
                // pub const fn x(self) -> bool {
                //     self.0 & Self::X.0 == Self::X.0
                // }
                #[inline]
                pub const fn $fn_name(self) -> bool {
                    self.0 & Self::$const_name.0 == Self::$const_name.0
                }
                
                // pub const fn set_x(&mut self, value: bool) -> bool {
                //     let old = self.x();
                //     if value {
                //         self.0 |= Self::X.0;
                //     } else {
                //         self.0 &= const { Self::X.invert().0 };
                //     }
                //     old
                // }
                #[inline]
                pub const fn [<set_ $fn_name>](&mut self, value: bool) -> bool {
                    let old = self.$fn_name();
                    if value {
                        self.0 |= Self::$const_name.0;
                    } else {
                        self.0 &= const { Self::$const_name.invert().0 };
                    }
                    old
                }
                
                // pub const fn with_x(mut self, value: bool) -> Self {
                //     self.set_x(value);
                //     self
                // }
                #[inline]
                pub const fn [<with_ $fn_name>](mut self, value: bool) -> Self {
                    self.[<set_ $fn_name>](value);
                    self
                }
                
                // pub const fn flip_x(self) -> Self {
                //     Self(self.0 ^ Self::X.0)
                // }
                #[inline]
                pub const fn [<flip_ $fn_name>](self) -> Self {
                    Self(self.0 ^ Self::$const_name.0)
                }
            }
        )*
    };
}

type Tup3<T> = (T, T, T);

macro_rules! flip_coord_impls {
    ($(
        $type:ty
    ),*$(,)?) => {
        $(
            paste!{
                pub const fn [<flip_coord_ $type>](self, coord: Tup3<$type>) -> Tup3<$type> {
                    let (x, y, z) = coord;
                    match self {
                        Self::NONE => (x, y, z),
                        Self::X => (-x, y, z),
                        Self::Y => (x, -y, z),
                        Self::XY => (-x, -y, z),
                        Self::Z => (x, y, -z),
                        Self::XZ => (-x, y, -z),
                        Self::YZ => (x, -y, -z),
                        Self::XYZ => (-x, -y, -z),
                        // SAFETY: Any other state is inconstructible.
                        _ => unsafe { ::core::hint::unreachable_unchecked() },
                    }
                }
            }
        )*
    };
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlipState {
    None = 0b000,
    X = 0b001,
    Y = 0b010,
    Z = 0b100,
    XY = 0b011,
    XZ = 0b101,
    YZ = 0b110,
    XYZ = 0b111,
}

impl FlipState {
    #[inline]
    pub const fn to_flip(self) -> Flip {
        match self {
            FlipState::None => Flip::NONE,
            FlipState::X => Flip::X,
            FlipState::Y => Flip::Y,
            FlipState::Z => Flip::Z,
            FlipState::XY => Flip::XY,
            FlipState::XZ => Flip::XZ,
            FlipState::YZ => Flip::YZ,
            FlipState::XYZ => Flip::XYZ,
        }
    }
    
    pub const fn from_flip(flip: Flip) -> Self {
        match Flip(flip.0 & Flip::ALL.0) {
            Flip::NONE => Self::None,
            Flip::X => Self::X,
            Flip::Y => Self::Y,
            Flip::Z => Self::Z,
            Flip::XY => Self::XY,
            Flip::XZ => Self::XZ,
            Flip::YZ => Self::YZ,
            Flip::XYZ => Self::XYZ,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl Flip {
    flip_axes!(
        {const X   = 0b001; fn x  } // 1
        {const XY  = 0b011; fn xy } // 3
        {const XZ  = 0b101; fn xz } // 5
        {const Y   = 0b010; fn y  } // 2
        {const YZ  = 0b110; fn yz } // 6
        {const Z   = 0b100; fn z  } // 4
        {const XYZ = 0b111; fn xyz} // 7
    );
    pub const ALL: Flip = Flip::XYZ;
    pub const NONE: Flip = Flip(0b000);

    #[inline]
    pub const fn new(x: bool, y: bool, z: bool) -> Self {
        Self((x as u8) | ((y as u8) << 1) | ((z as u8) << 2))
    }
    
    /// `bits` must be no greater than `0b111` (7).
    /// If a higher value is passed in, the behavior is undefined.
    #[inline]
    pub const unsafe fn from_u8_unchecked(bits: u8) -> Self {
        Self(bits)
    }
    
    #[inline]
    pub const fn from_u8(bits: u8) -> Option<Self> {
        if bits > Flip::ALL.0 {
            return None;
        }
        // SAFETY: Guard clause ensures that u8 is valid
        Some(unsafe { Self::from_u8_unchecked(bits) })
    }
    
    #[inline]
    pub const fn to_u8(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn flip(self, flip: Flip) -> Self {
        Self(self.0 ^ flip.0)
    }
    
    #[inline]
    pub const fn invert(self) -> Self {
        Self(self.0 ^ Self::ALL.0)
    }

    /// Xors all the bits.
    pub const fn bits_xor(self) -> bool {
        self.x() ^ self.y() ^ self.z()
    }

    pub fn flip_coord<T: Copy + std::ops::Neg<Output = T>, C: Into<(T, T, T)> + From<(T, T, T)>>(self, value: C) -> C {
        let (mut x, mut y, mut z): (T, T, T) = value.into();
        if self.x() {
            x = -x;
        }
        if self.y() {
            y = -y;
        }
        if self.z() {
            z = -z;
        }
        C::from((x, y, z))
    }
    
    flip_coord_impls!(
        i8,
        i16,
        i32,
        i64,
        i128,
        f32,
        f64,
    );

    // I don't know how useful this would be, but the code is already written.
    /// Determines if a face is on an axis that is flipped.
    pub const fn is_flipped(self, face: Direction) -> bool {
        if self.0 == 0 {
            return false;
        }
        use Direction::*;
        match face {
            NegX | PosX if self.x() => true,
            NegY | PosY if self.y() => true,
            NegZ | PosZ if self.z() => true,
            _ => false,
        }
    }
    
    #[inline]
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..8).map(Self)
    }

    // /// If the [Flip] is being used to flip vertices, this method determines if the indices need to be reversed.
    // #[inline]
    // pub const fn reverse_indices(self) -> bool {
    //     self.x() ^ self.y() ^ self.z()
    // }

    // #[inline]
    // pub fn to_scale(self) -> glam::Vec3 {
    //     fn select_scale(flipped: bool) -> f32 {
    //         if flipped {
    //             -1.0
    //         } else {
    //             1.0
    //         }
    //     }
    //     glam::vec3(
    //         select_scale(self.x()),
    //         select_scale(self.y()),
    //         select_scale(self.z()),
    //     )
    // }

    // #[inline]
    // pub fn to_matrix(self) -> glam::Mat4 {
    //     let scale = self.to_scale();
    //     glam::Mat4::from_scale(scale)
    // }
}

impl std::ops::BitOr<Flip> for Flip {
    type Output = Self;
    
    #[inline]
    fn bitor(self, rhs: Flip) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign<Flip> for Flip {
    #[inline]
    fn bitor_assign(&mut self, rhs: Flip) {
        *self = *self | rhs;
    }
}

impl std::ops::BitAnd<Flip> for Flip {
    type Output = Self;
    
    #[inline]
    fn bitand(self, rhs: Flip) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign<Flip> for Flip {
    #[inline]
    fn bitand_assign(&mut self, rhs: Flip) {
        self.0 &= rhs.0
    }
}

impl std::ops::Add<Flip> for Flip {
    type Output = Flip;
    #[inline]
    fn add(self, rhs: Flip) -> Self::Output {
        self | rhs
    }
}

impl std::ops::AddAssign<Flip> for Flip {
    #[inline]
    fn add_assign(&mut self, rhs: Flip) {
        self.0 |= rhs.0;
    }
}

impl std::ops::Sub<Flip> for Flip {
    type Output = Flip;
    
    #[inline]
    fn sub(self, rhs: Flip) -> Self::Output {
        self & !rhs
    }
}

impl std::ops::SubAssign<Flip> for Flip {
    #[inline]
    fn sub_assign(&mut self, rhs: Flip) {
        *self = *self & !rhs;
    }
}

impl std::ops::Not for Flip {
    type Output = Self;
    
    #[inline]
    fn not(self) -> Self::Output {
        Self(self.0 ^ 0b111)
    }
}

impl std::fmt::Display for Flip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Flip(")?;
        let mut sep = false;
        if self.x() {
            write!(f, "X")?;
            sep = true;
        }
        if self.y() {
            if sep {
                write!(f, "|")?;
            }
            write!(f, "Y")?;
        }
        if self.z() {
            if sep {
                write!(f, "|")?;
            }
            write!(f, "Z")?;
        }
        write!(f, ")")
    }
}