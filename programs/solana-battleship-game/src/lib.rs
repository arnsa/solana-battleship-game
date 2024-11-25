mod constants;
mod error;
mod state;
mod utils;

use anchor_lang::prelude::*;

use anchor_lang::solana_program::sysvar::recent_blockhashes::RecentBlockhashes;
use constants::GAME_ACCOUNT_SEED;
use state::GameState;
use state::ShipCoordinate;

declare_id!("2wgGg9s2kpGTk8du1ccZ9E1v5PW3TUz7aSQyeX4drqNE");

#[program]
pub mod solana_battleship_game {
    use super::*;

    pub fn initialize(
        ctx: Context<CreateGameAccount>,
        ships_coordinates: Vec<ShipCoordinate>,
    ) -> Result<()> {
        let game = GameState::initialize_game(&ctx, &ships_coordinates)?;

        ctx.accounts.game_account.game_boards = game.game_boards;
        ctx.accounts.game_account.status = game.status;
        ctx.accounts.game_account.current_turn = game.current_turn;
        ctx.accounts.game_account.rounds_played = game.rounds_played;

        Ok(())
    }

    pub fn close_game(_ctx: Context<CloseGameAccount>) -> Result<()> {
        msg!("Closing game account");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGameAccount<'info> {
    #[account(
        init,
        payer = player,
        // TODO: calculate correct space
        space = 10240,
        // space = 100000 + 8 + // discriminator
        //     32 + // player pubkey
        //     32 + // system program
        //     1 + // game status
        //     1 + // current turn
        //     1 + // rounds played
        //     ((1 + (10 * 10)) * 2),
        seeds = [GAME_ACCOUNT_SEED, player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameState>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub recent_blockhashes: Sysvar<'info, RecentBlockhashes>,
}

#[derive(Accounts)]
pub struct CloseGameAccount<'info> {
    #[account(
        mut,
        close = payer,
    )]
    pub game_account: Account<'info, GameState>,

    #[account(mut)]
    pub payer: Signer<'info>,
}