use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use crate::CreateGameAccount;

use super::{GameBoard, ShipCoordinate};

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum GameStatus {
    InProgress,
    Over,
}

#[account]
#[derive(Debug)]
pub struct GameState {
    pub game_boards: (GameBoard, GameBoard),
    pub status: GameStatus,
    pub current_turn: u8,
    pub rounds_played: u8,
}

impl GameState {
    pub fn initialize_game(
        ctx: &Context<CreateGameAccount>,
        ships_coordinates: &Vec<ShipCoordinate>,
    ) -> Result<Self> {
        let mut p1_game_board = GameBoard::initialize_game_board();
        let mut p2_game_board = GameBoard::initialize_game_board();

        p1_game_board.initiate_board_with_ships_from_input(ships_coordinates)?;
        p2_game_board.initiate_board_with_ships_at_random(ctx)?;

        Ok(Self {
            game_boards: (p1_game_board, p2_game_board),
            status: GameStatus::InProgress,
            current_turn: 0,
            rounds_played: 0,
        })
    }
}
