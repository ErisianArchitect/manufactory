
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Face {
    PosX = 0,
    PosY = 1,
    PosZ = 2,
    NegX = 3,
    NegY = 4,
    NegZ = 5,
}

impl Face {
    pub const RIGHT: Face = Face::PosX;
    pub const TOP: Face = Face::PosY;
    pub const BACK: Face = Face::PosZ;
    pub const LEFT: Face = Face::NegX;
    pub const BOTTOM: Face = Face::NegY;
    pub const FRONT: Face = Face::NegZ;
    
    
}