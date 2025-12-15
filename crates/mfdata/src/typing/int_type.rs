#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IntType {
    U8, U16, U32, U64,
    I8, I16, I32, I64,
}