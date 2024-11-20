use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::board::GameBoard;

#[account]
#[derive(Debug)]
pub struct GameState {
    pub game_board: GameBoard,
    pub status: GameStatus,
    pub current_turn: u8,
    pub rounds_played: u8,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum GameStatus {
    InProgress,
    Over,
}