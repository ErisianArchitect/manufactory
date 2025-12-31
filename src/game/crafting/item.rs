use std::{marker::PhantomData, num::NonZeroU32};

// pub trait ItemLike {
//     fn name() -> &'static str;
// }

// #[repr(transparent)]
// pub struct I<T: ItemLike>(PhantomData<T>);

// impl<T: ItemLike> I<T> {
//     #[allow(unused)]
//     #[inline]
//     #[must_use]
//     pub(crate) const fn create() -> Self {
//         Self(PhantomData)
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ItemId(pub(crate) u32);

impl ItemId {
    #[allow(unused)]
    #[inline]
    #[must_use]
    pub(crate) fn new(value: u32) -> Self {
        Self(value)
    }
    
    #[inline]
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

macro_rules! make_item_type {
    (
        $(
            #[$attr:meta]
        )*
        pub enum ItemType {
            $(
                $(
                    #[$var_attr:meta]
                )*
                $variant:ident {
                    text: $display:literal,
                    id: $id:expr,
                }
            ),*$(,)?
        }
    ) => {
        $(
            #[$attr]
        )*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum ItemType {
            $(
                $variant,
            )*
        }
        
        impl ItemType {
            pub const fn id(self) -> ItemId {
                ItemId(match self {
                    $(
                        ItemType::$variant => $id,
                    )*
                })
            }
            
            pub const fn display(self) -> &'static str {
                match self {
                    $(
                        ItemType::$variant => $display,
                    )*
                }
            }
        }
    };
}

/// Resource Sub-type index
macro_rules! res_sub {
    (Ore) => { 0 };
    (IngotPrecursor) => { 1 };
    (Ingot) => { 2 };
    (Cube) => { 3 };
    (KiloCube) => { 4 };
    (MegaCube) => { 5 };
    (GigaCube) => { 6 };
    // Leave room for more cubes.
    (Rod) => { 16 };
    (Screws) => { 17 };
    (Sheet) => { 18 };
    (Plate) => { 19 };
    ($other:expr) => { $other };
}

macro_rules! res_type {
    (Iron) => { 16 };
    (Steel) => { 17 };
    (Copper) => { 18 };
    (Alluminum) => { 19 };
    (Gold) => { 20 };
    (Bronze) => { 21 };
    (Lead) => { 22 };
    // Crystals start at 1024
    (Quartz) => { 1024 };
    (0) => { compile_error!("Cannot have resource id of 0."); };
    ($other:expr) => { $other };
}

const RESOURCE_SECTION_SIZE: u32 = 1024;
const RESOURCES_START: u32 = 0;

macro_rules! res_id {
    ($section:tt, $index:tt) => {
        const { ((res_type!($section)) * RESOURCE_SECTION_SIZE) + RESOURCES_START + res_sub!($index) }
    };
}

make_item_type!(
    pub enum ItemType {
        // IronOre starts at id=4096
        IronOre { 
            text: "Iron Ore",
            id: res_id!(Iron, Ore),
        },
        IronOreCrushed {
            text: "Iron Ore (Crushed)",
            id: res_id!(Iron, IngotPrecursor),
        },
        IronIngot {
            text: "Iron Ingot",
            id: res_id!(Iron, Ingot),
        },
        /// A way to compress 64 Ingots a single cube. Useful for saving stack size, as they can
        /// easily be decomposed back into a stack of 64 Ingots.
        IronCube {
            text: "Iron Cube",
            id: res_id!(Iron, Cube),
        },
        /// 64 Cubes (64^2)
        IronKiloCube {
            text: "Iron KiloCube",
            id: res_id!(Iron, KiloCube),
        },
        /// 64 KiloCubes (64^3)
        IronMegaCube {
            text: "Iron MegaCube",
            id: res_id!(Iron, MegaCube),
        },
        /// 64 MegaCubes (64^4)
        IronGigaCube {
            text: "Iron GigaCube",
            id: res_id!(Iron, GigaCube),
        },
        // leave room for more NCubes
        IronRod {
            text: "Iron Rod",
            id: res_id!(Iron, Rod),
        },
        IronScrews {
            text: "Iron Screws",
            id: res_id!(Iron, Screws),
        },
        IronSheet {
            text: "Iron Sheet",
            id: res_id!(Iron, Sheet),
        },
        IronPlate {
            text: "Iron Plate",
            id: res_id!(Iron, Plate),
        },
        
        SteelIngot {
            text: "Steel Ingot",
            id: res_id!(Steel, Ingot),
        },
        SteelCube {
            text: "Steel Cube",
            id: res_id!(Steel, Cube),
        },
        SteelKiloCube {
            text: "Steel KiloCube",
            id: res_id!(Steel, KiloCube),
        },
        SteelMegaCube {
            text: "Steel MegaCube",
            id: res_id!(Steel, MegaCube),
        },
        SteelGigaCube {
            text: "Steel GigaCube",
            id: res_id!(Steel, GigaCube),
        },
        SteelRod {
            text: "Steel Rod",
            id: res_id!(Steel, Rod),
        },
        SteelScrews {
            text: "Steel Screws",
            id: res_id!(Steel, Screws),
        },
        SteelSheet {
            text: "Steel Sheet",
            id: res_id!(Steel, Sheet),
        },
        SteelPlate {
            text: "Steel Plate",
            id: res_id!(Steel, Plate),
        },
        
        CopperOre {
            text: "Copper Ore",
            id: res_id!(Copper, Ore),
        },
        CopperOreCrushed {
            text: "Copper Ore (Crushed)",
            id: res_id!(Copper, IngotPrecursor),
        },
        // TODO: CopperIngot precursor 
        CopperIngot {
            text: "Copper Ingot",
            id: res_id!(Copper, Ingot),
        },
        CopperCube {
            text: "Copper Cube",
            id: res_id!(Copper, Cube),
        },
        CopperKiloCube {
            text: "Copper KiloCube",
            id: res_id!(Copper, KiloCube),
        },
        CopperMegaCube {
            text: "Copper MegaCube",
            id: res_id!(Copper, MegaCube),
        },
        CopperGigaCube {
            text: "Copper GigaCube",
            id: res_id!(Copper, GigaCube),
        },
        CopperRod {
            text: "Copper Rod",
            id: res_id!(Copper, Rod),
        },
        CopperScrews {
            text: "Copper Screws",
            id: res_id!(Copper, Screws),
        },
        CopperSheet {
            text: "Copper Sheet",
            id: res_id!(Copper, Sheet),
        },
        CopperPlate {
            text: "Copper Plate",
            id: res_id!(Copper, Plate),
        },
        
        /// Alluminum Ore
        Bauxite {
            text: "Bauxite",
            id: res_id!(Alluminum, Ore),
        },
        Allumina {
            text: "Allumina",
            id: res_id!(Alluminum, IngotPrecursor),
        },
        AlluminumIngot {
            text: "Alluminum Ingot",
            id: res_id!(Alluminum, Ingot),
        },
        AlluminumCube {
            text: "Alluminum Cube",
            id: res_id!(Alluminum, Cube),
        },
        AlluminumKiloCube {
            text: "Alluminum KiloCube",
            id: res_id!(Alluminum, KiloCube),
        },
        AlluminumMegaCube {
            text: "Alluminum MegaCube",
            id: res_id!(Alluminum, MegaCube),
        },
        AlluminumGigaCube {
            text: "Alluminum GigaCube",
            id: res_id!(Alluminum, GigaCube),
        },
        AlluminumRod {
            text: "Alluminum Rod",
            id: res_id!(Alluminum, Rod),
        },
        AlluminumScrews {
            text: "Allumium Screws",
            id: res_id!(Alluminum, Screws),
        },
        AlluminumSheet {
            text: "Alluminum Sheet",
            id: res_id!(Alluminum, Sheet),
        },
        AlluminumPlate {
            text: "Alluminum Plate",
            id: res_id!(Alluminum, Plate),
        },
        
        // Crystals begin at section 1024
        Quartz {
            text: "Quartz",
            id: res_id!(Quartz, Ore),
        },
        QuartzPowder {
            text: "Quartz Powder",
            id: res_id!(Quartz, IngotPrecursor),
        },
        QuartzIngot {
            text: "Quartz Ingot",
            id: res_id!(Quartz, Ingot),
        },
        QuartzCube {
            text: "Quartz Cube",
            id: res_id!(Quartz, Cube),
        },
        QuartzKiloCube {
            text: "Quartz KiloCube",
            id: res_id!(Quartz, KiloCube),
        },
        QuartzMegaCube {
            text: "Quartz MegaCube",
            id: res_id!(Quartz, MegaCube),
        },
        QuartzGigaCube {
            text: "Quartz GigaCube",
            id: res_id!(Quartz, GigaCube),
        },
    }
);

pub struct ItemData {
    pub(crate) item_type: ItemType,
}

impl ItemData {
    #[inline]
    #[must_use]
    pub const fn item_type(&self) -> ItemType {
        self.item_type
    }
    
    #[inline]
    #[must_use]
    pub const fn text(&self) -> &'static str {
        self.item_type().display()
    }
    
    #[inline]
    #[must_use]
    pub const fn id(&self) -> ItemId {
        self.item_type().id()
    }
}