use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

use super::ShipDirection;
use crate::constants::SHIPS;
use crate::error::BattleshipError;
use crate::utils::get_ship_coordinates;

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct GameBoard {
    pub player_grid: [[TileState; 10]; 10],
    pub target_grid: [[TileState; 10]; 10],
}

impl GameBoard {
    pub fn initialize_game_board() -> Self {
        let player_grid: [[TileState; 10]; 10] = [[TileState::Empty; 10]; 10];
        let target_grid: [[TileState; 10]; 10] = [[TileState::Empty; 10]; 10];

        Self {
            player_grid,
            target_grid,
        }
    }

    pub fn initiate_board_with_ships_from_input(
        &mut self,
        ships_coordinates: &Vec<(u8, u8, ShipDirection)>,
    ) -> Result<()> {
        for (&ship_size, &(col, row, direction)) in SHIPS.iter().zip(ships_coordinates.iter()) {
            self.place_ship((col, row), ship_size, &direction)?;
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

                if self.place_ship((col, row), ship_size, &direction).is_ok() {
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
            for &(row, col) in ship_coordinates.iter() {
                self.target_grid[row as usize][col as usize] = TileState::Ship;
            }

            return Ok(());
        }

        return err!(BattleshipError::ShipPlacementError);
    }

    fn can_place_ship(&self, new_ship_coordinates: &Vec<(u8, u8)>) -> bool {
        new_ship_coordinates
            .iter()
            .any(|&(row, col)| self.target_grid[row as usize][col as usize] == TileState::Ship)
    }

    fn parse_user_input(input: &str) -> Result<(u8, u8, ShipDirection)> {
        let mut chars = input.chars();
        let row = chars.next().ok_or(BattleshipError::InputFormatError)?;
        let col = chars
            .by_ref()
            .take_while(|num| num.is_digit(10))
            .collect::<String>()
            .parse::<u8>()
            .map_err(|_| BattleshipError::InputFormatError)?;
        let direction = match chars.next().ok_or(BattleshipError::InputFormatError) {
            Ok('U') => ShipDirection::Up,
            Ok('R') => ShipDirection::Right,
            Ok('D') => ShipDirection::Down,
            Ok('L') => ShipDirection::Left,
            _ => return Err(BattleshipError::InputFormatError.into()),
        };

        if row < 'A' || row > 'J' || col < 1 || col > 10 {
            return Err(BattleshipError::InputFormatError.into());
        };

        return Ok(((row as u8) - b'A', col - 1, direction));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum TileState {
    Empty,
    Ship,
    Hit,
    Miss,
}
