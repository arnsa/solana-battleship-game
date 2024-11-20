use anchor_lang::prelude::*;

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
