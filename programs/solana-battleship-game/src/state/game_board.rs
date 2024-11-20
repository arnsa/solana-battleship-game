use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct GameBoard {
    pub player_grid: [[TileState; 10]; 10],
    pub target_grid: [[TileState; 10]; 10],
}

impl GameBoard {
    pub fn place_ship() {

    }
}

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub enum TileState {
    Empty,
    Ship,
    Hit,
    Miss,
}
