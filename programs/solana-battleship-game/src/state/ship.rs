use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum ShipDirection {
    Up,
    Down,
    Left,
    Right,
}
