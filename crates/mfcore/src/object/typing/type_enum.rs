use super::int_type::*;
use super::item_type::*;

pub enum Type {
    None,
    Bool,
    Char,
    Int(IntType),
    String,
    Bytes,
    Item(ItemType),
    UnknownId(u32),
    Unknown,
}