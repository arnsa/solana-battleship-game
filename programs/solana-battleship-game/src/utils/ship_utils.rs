use anchor_lang::prelude::Result;
use crate::{error::BattleshipError, state::ShipDirection};

fn check_new_coordinates(coordinates: (u8, u8)) -> Result<()> {
    if coordinates.0 > 9 || coordinates.1 > 9 {
        return Err(BattleshipError::CoordinatesNotOnBoardError.into())
    }

    Ok(())
}

pub fn get_ship_coordinates(
    start_point: (u8, u8),
    size: u8,
    direction: &ShipDirection,
) -> Result<Vec<(u8, u8)>> {
    let mut result = Vec::with_capacity(size as usize);

    for i in 0..size {
        match direction {
            ShipDirection::Down => {
                let x = start_point.0;
                let y = start_point.1 + i;

                check_new_coordinates((x, y))?;
                result.push((x, y))
            }
            ShipDirection::Up => {
                let x = start_point.0;
                let y = start_point.1 - i;

                check_new_coordinates((x, y))?;
                result.push((x, y))
            }
            ShipDirection::Left => {
                let x = start_point.0 - i;
                let y = start_point.1;

                check_new_coordinates((x, y))?;
                result.push((x, y))
            }
            ShipDirection::Right => {
                let x = start_point.0 + i;
                let y = start_point.1;

                check_new_coordinates((x, y))?;
                result.push((x, y))
            }
        };
    }

    return Ok(result);
}
