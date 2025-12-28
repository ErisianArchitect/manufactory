use crate::geometry::Face;



pub struct CubeSides<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
    pub front: T,
    pub back: T,
}

impl<T> CubeSides<T> {
    pub const fn get_side(&self, face: Face) -> &T {
        match face {
            Face::PosX => &self.right,
            Face::PosY => &self.top,
            Face::PosZ => &self.back,
            Face::NegX => &self.left,
            Face::NegY => &self.bottom,
            Face::NegZ => &self.front,
        }
    }
}