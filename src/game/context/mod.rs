use std::{cell::RefCell, rc::Rc};

use crate::game::crafting::item::ItemData;

pub mod handles;

/*
The Context stores game data such as Types, Functions, Recipes, etc.
Data within the context can be accessed via handles, which are 32 bit NonZero values.
*/

pub(crate) struct Containers {
    pub items: Vec<ItemData>,
    pub types: Vec<()>,
    pub functions: Vec<()>,
    pub recipes: Vec<()>,
}

pub(crate) struct ContextInner {
    pub seed: u64,
    pub containers: Containers,
}

#[derive(Clone)]
pub struct Context {
    pub(crate) inner: Rc<ContextInner>,
}

impl Context {
    // pub fn seeded(seed: u64) -> Self {
        
    // }
}