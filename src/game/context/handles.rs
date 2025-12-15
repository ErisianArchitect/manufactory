

use std::num::NonZeroU32;

/// Cheaply copyable handle for use as a key since Ids are not copyable.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle(pub(crate) NonZeroU32);

impl Handle {
    #[allow(unused)]
    #[inline]
    #[must_use]
    pub(crate) const fn new(value: NonZeroU32) -> Self {
        Self(value)
    }
    
    /// Returns the value of the inner [NonZeroU32].
    #[inline]
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0.get()
    }
    
    #[inline]
    #[must_use]
    pub const fn base_index(self) -> u32 {
        self.value() - 1
    }
    
    /// Returns the inner [NonZeroU32].
    #[inline]
    #[must_use]
    pub const fn inner(self) -> NonZeroU32 {
        self.0
    }
}

macro_rules! handle_types {
    ($(
        $(
            #[$attr:meta]
        )*
        $type_name:ident
    )*) => {
        $(
            $(
                #[$attr]
            )*
            #[repr(transparent)]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $type_name {
                pub(crate) handle: Handle,
            }
            
            impl $type_name {
                #[allow(unused)]
                #[inline]
                #[must_use]
                pub(crate) const fn new(value: NonZeroU32) -> Self {
                    Self {
                        handle: Handle::new(value),
                    }
                }
                
                #[inline]
                #[must_use]
                pub const fn handle(&self) -> Handle {
                    self.handle
                }
                
                #[inline]
                #[must_use]
                pub const fn value(&self) -> u32 {
                    self.handle().value()
                }
                
                #[inline]
                #[must_use]
                pub const fn base_index(&self) -> u32 {
                    self.handle().base_index()
                }
                
                #[inline]
                #[must_use]
                pub const fn raw(&self) -> NonZeroU32 {
                    self.handle().inner()
                }
            }
        )*
    };
}

handle_types!(
    ItemId
    TypeId
    FnId
    RecipeId
);