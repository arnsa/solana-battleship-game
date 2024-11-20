mod state;
mod constants;

use anchor_lang::prelude::*;
use constants::GAME_ACCOUNT_SEED;
use state::GameState;

declare_id!("FWADG4FNxH7Sx8DyZ4VXuZtHfBPuekWBewoyWKuUPNsz");

#[program]
pub mod solana_battleship_game {
    use super::*;

    pub fn initialize(ctx: Context<CreateGameAccount>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateGameAccount<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + 32 + 32 + 8,
        seeds = [GAME_ACCOUNT_SEED, player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameState>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}