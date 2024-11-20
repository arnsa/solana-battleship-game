use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use super::{ShipCoordinate, ShipDirection};
use crate::constants::SHIPS;
use crate::error::BattleshipError;
use crate::utils::get_ship_coordinates;

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct GameBoard {
    pub player_grid: Vec<Vec<TileState>>,
    pub target_grid: Vec<Vec<TileState>>,
}

impl GameBoard {
    pub fn initialize_game_board() -> Self {
        let player_grid = vec![vec![TileState::Empty; 10]; 10];
        let target_grid = vec![vec![TileState::Empty; 10]; 10];

        Self {
            player_grid,
            target_grid,
        }
    }

    pub fn initiate_board_with_ships_from_input(
        &mut self,
        ships_coordinates: &Vec<ShipCoordinate>,
    ) -> Result<()> {
        for (&ship_size, coords) in SHIPS.iter().zip(ships_coordinates.iter()) {
            self.place_ship((coords.col, coords.row), ship_size, &coords.direction)?;
        }

        Ok(())
    }

    pub fn initiate_board_with_ships_at_random(&mut self) {
        for ship_size in SHIPS {
            loop {
                let row = (Clock::get().unwrap().unix_timestamp % 10) as u8;
                let col = (Clock::get().unwrap().unix_timestamp % 10) as u8;
                let direction_num = Clock::get().unwrap().unix_timestamp % 4;
                let direction = match direction_num {
                    0 => ShipDirection::Up,
                    1 => ShipDirection::Right,
                    2 => ShipDirection::Down,
                    3 => ShipDirection::Left,
                    _ => unreachable!(),
                };

                if self.place_ship((row, col), ship_size, &direction).is_ok() {
                    break;
                }
            }
        }
    }

    fn place_ship(
        &mut self,
        start_point: (u8, u8),
        size: u8,
        direction: &ShipDirection,
    ) -> Result<()> {
        let ship_coordinates = get_ship_coordinates(start_point, size, direction)?;

        if self.can_place_ship(&ship_coordinates) {
            for &(col, row) in ship_coordinates.iter() {
                self.target_grid[row as usize][col as usize] = TileState::Ship;
            }

            return Ok(());
        }

        return err!(BattleshipError::ShipsOverlappingError);
    }

    fn can_place_ship(&self, new_ship_coordinates: &Vec<(u8, u8)>) -> bool {
        !new_ship_coordinates
            .iter()
            .any(|&(col, row)| self.target_grid[row as usize][col as usize] == TileState::Ship)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum TileState {
    Empty,
    Ship,
    Hit,
    Miss,
}
