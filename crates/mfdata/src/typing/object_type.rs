use super::primitive_type::*;
use super::item_type::ItemType;
use super::struct_type::StructType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BitsWidth {
    // 1
    W8,
    // 2
    W16,
    // 4
    W32,
    // 8
    W64,
    // 16
    W128,
    // 32
    W256,
    // 64
    W512,
    // 128
    W1024,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayType {
    pub ty: Type,
    pub len: u32,
}

impl ArrayType {
    #[inline]
    #[must_use]
    pub fn new(ty: Type, len: u32) -> Self {
        Self {
            ty,
            len,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectType {
    /// `None` value.
    None,
    Primitive(PrimitiveType),
    String,
    Bytes,
    Bits(BitsWidth),
    Array(ArrayType),
    Struct(StructType),
    Item(ItemType),
    Abstract,
    Unknown,
}

impl From<PrimitiveType> for ObjectType {
    fn from(value: PrimitiveType) -> Self {
        Self::Primitive(value)
    }
}

impl From<BitsWidth> for ObjectType {
    fn from(value: BitsWidth) -> Self {
        Self::Bits(value)
    }
}

impl From<ArrayType> for ObjectType {
    fn from(value: ArrayType) -> Self {
        Self::Array(value)
    }
}

impl From<StructType> for ObjectType {
    fn from(value: StructType) -> Self {
        Self::Struct(value)
    }
}

impl From<ItemType> for ObjectType {
    fn from(value: ItemType) -> Self {
        Self::Item(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Type {
    pub(crate) inner_type: Box<ObjectType>,
}

impl Type {
    #[must_use]
    pub fn array(&self, len: u32) -> Type {
        Type {
            inner_type: Box::new(ObjectType::Array(ArrayType::new(self.clone(), len))),
        }
    }
}