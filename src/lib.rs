pub use mfcore;
pub use mfhash as hash;
pub mod game;
/*
World:
Grid
[3; i64] cells
Cell(Block):
    
Player:

Inventory
Hotbar

Notes:
    16-bits for each side (6 sides * 16 bits = 96 bits or 12 bytes)
        A turtle can only move forward and backward along its face axis.
*/