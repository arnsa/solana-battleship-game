use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub struct ShipCoordinate {
    pub row: u8,
    pub col: u8,
    pub direction: ShipDirection,
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize)]
pub enum ShipDirection {
    Up,
    Right,
    Down,
    Left,
}