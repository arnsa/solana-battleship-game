mod board;
mod game_state;

use anchor_lang::prelude::*;
use game_state::GameState;

declare_id!("FWADG4FNxH7Sx8DyZ4VXuZtHfBPuekWBewoyWKuUPNsz");

#[program]
pub mod solana_battleship_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateGameAccount<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + 32 + 32 + 8,
        seeds = [b"battleship-game", player.key().as_ref()],
        bump
    )]
    pub game_account: Account<'info, GameState>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub system_program: Program<'info, System>,
}
