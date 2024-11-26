// Game configuration
pub const GAME_ACCOUNT_SEED: &[u8] = b"battleship-game";
pub const SHIPS: [u8; 5] = [5, 4, 3, 3, 2];
pub const GRID_SIZE: usize = 10;

// Solana account size constants
pub const DISCRIMINATOR_LENGTH: usize = 8;
pub const UINT8_SIZE: usize = 1;
pub const ENUM_SIZE: usize = 1;
pub const VEC_PREFIX_SIZE: usize = 4;