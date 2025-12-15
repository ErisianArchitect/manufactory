pub mod context;
pub mod crafting;
pub mod player;
pub mod world;

use world::World;
use player::Player;

pub struct Game {
    pub(crate) world: World,
    pub(crate) player: Player,
}