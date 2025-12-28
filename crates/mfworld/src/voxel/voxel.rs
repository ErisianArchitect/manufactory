use super::id::VoxelId;
use crate::geometry::Face;

// This should always be u16.
/// Defines the egress (ability to enter/exit) for each side of the voxel cube.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoxelEgress(u16);

/// Defines whether a Voxel can be entered or exited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Egress {
    /// Voxel can be entered.
    pub enter: bool,
    /// Voxel can be exited.
    pub exit: bool,
}

impl Egress {
    #[inline(always)]
    pub const fn new(enter: bool, exit: bool) -> Self {
        Self {
            enter,
            exit,
        }
    }
    
    #[inline]
    pub const fn bits(self) -> u16 {
        (self.enter as u16) | ((self.exit as u16) << 1)
    }
    
    pub const fn with_enterable(mut self, enterable: bool) -> Self {
        self.enter = enterable;
        self
    }
    
    pub const fn with_exitable(mut self, exitable: bool) -> Self {
        self.exit = exitable;
        self
    }
}

impl VoxelEgress {
    pub const CLOSED: Self = Self(0);
    
    pub const fn from_sides(top: Egress, bottom: Egress, left: Egress, right: Egress, front: Egress, back: Egress) {
        
    }
    
    pub const fn face_bit_start(face: Face) -> u32 {
        match face {
            Face::PosX => 0,
            Face::PosY => 2,
            Face::PosZ => 4,
            Face::NegX => 6,
            Face::NegY => 8,
            Face::NegZ => 10,
        }
    }
    
    pub const fn set_egress(&mut self, face: Face, egress: Egress) {
        let bits = egress.bits();
        let start = Self::face_bit_start(face);
        let remove_mask = !(0b11u16 << start);
        let bits = bits << start;
        self.0 = (self.0 & remove_mask) | bits;
    }
    
    pub const fn get_egress(&mut self, face: Face) -> Egress {
        const EGRESSES: [Egress; 4] = [
            Egress::new(false, false),
            Egress::new(true, false),
            Egress::new(false, true),
            Egress::new(true, true),
        ];
        let start = Self::face_bit_start(face);
        let mask = 0b11u16 << start;
        EGRESSES[((self.0 & mask) >> start) as usize]
    }
    
    #[inline]
    pub const fn get_enterable(&mut self, face: Face) -> bool {
        self.get_egress(face).enter
    }
    
    #[inline]
    pub const fn get_exitable(&mut self, face: Face) -> bool {
        self.get_egress(face).enter
    }
    
    #[inline]
    pub const fn set_enterable(&mut self, face: Face, enterable: bool) {
        let egress = self.get_egress(face);
        self.set_egress(face, egress.with_enterable(enterable));
    }
    
    #[inline]
    pub const fn set_exitable(&mut self, face: Face, exitable: bool) {
        let egress = self.get_egress(face);
        self.set_egress(face, egress.with_exitable(exitable));
    }
}

// 64 bytes max
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Voxel {
    id: VoxelId, // 4 bytes
    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn egress_test() {
        let mut egress = VoxelEgress(0b111111111111);
        egress.set_enterable(Face::PosY, false);
        let expected = Egress::new(false, true);
        assert_eq!(egress.get_egress(Face::PosY), expected);
    }
}