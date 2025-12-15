use super::Type;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructType {
    pub fields: Box<[Type]>,
}