use anchor_lang::prelude::*;

#[error_code]
pub enum BattleshipError {
    #[msg("Can't place ship at those coordinates: ships are overlapping")]
    ShipPlacementError,

    #[msg("Wrong input format. Input example: A5 D")]
    InputFormatError,

    #[msg("Provided coordinates are not on board")]
    CoordinatesNotOnBoardError,
}
