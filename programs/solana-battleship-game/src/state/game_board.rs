use std::io::{self, Write};

use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use anyhow::{anyhow, Result};

use super::ShipDirection;
use crate::constants::SHIPS;
use crate::utils::get_ship_coordinates;

#[derive(Debug, Clone, AnchorDeserialize, AnchorSerialize)]
pub struct GameBoard {
    pub player_grid: [[TileState; 10]; 10],
    pub target_grid: [[TileState; 10]; 10],
}

impl GameBoard {
    pub fn new() -> Self {
        let player_grid: [[TileState; 10]; 10] = [[TileState::Empty; 10]; 10];
        let target_grid: [[TileState; 10]; 10] = [[TileState::Empty; 10]; 10];

        Self {
            player_grid,
            target_grid,
        }
    }

    pub fn initiate_board_with_ships_from_input(&mut self) -> Result<()> {
        for ship_size in SHIPS {
            loop {
                let mut input = String::new();

                print!("Place ship (size: {}): ", ship_size);
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin().read_line(&mut input).expect("Failed to read input");

                let (col, row, direction) = GameBoard::parse_user_input(&input)?;

                match self.place_ship((col, row), ship_size, &direction) {
                    Ok(_) => break,
                    Err(err) => println!("ERR: {}", err),
                }
            }
        }

        Ok(())
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

        return Err(anyhow!(
            "Can't place ship at those coordinates: ships are overlapping"
        ));
    }

    fn can_place_ship(&self, new_ship_coordinates: &Vec<(u8, u8)>) -> bool {
        new_ship_coordinates
            .iter()
            .any(|&(row, col)| self.target_grid[row as usize][col as usize] == TileState::Ship)
    }

    fn parse_user_input(input: &str) -> Result<(u8, u8, ShipDirection)> {
        const WRONG_FORMAT_ERROR_MESSAGE: &str = "Wrong input format. Input example: A5 D";
        let mut chars = input.chars();
        let row = chars.next().ok_or(anyhow!(WRONG_FORMAT_ERROR_MESSAGE))?;
        let col = chars
            .by_ref()
            .take_while(|num| num.is_digit(10))
            .collect::<String>()
            .parse::<u8>()
            .map_err(|_| anyhow!(WRONG_FORMAT_ERROR_MESSAGE))?;
        let direction = match chars.next().ok_or(WRONG_FORMAT_ERROR_MESSAGE) {
            Ok('U') => ShipDirection::Up,
            Ok('R') => ShipDirection::Right,
            Ok('D') => ShipDirection::Down,
            Ok('L') => ShipDirection::Left,
            Ok(_) => return Err(anyhow!(WRONG_FORMAT_ERROR_MESSAGE)),
            Err(_) => return Err(anyhow!(WRONG_FORMAT_ERROR_MESSAGE)),
        };

        if row < 'A' || row > 'J' || col < 1 || col > 10 {
            return Err(anyhow!(WRONG_FORMAT_ERROR_MESSAGE));
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
