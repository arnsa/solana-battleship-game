use anchor_lang::prelude::*;

use crate::CreateGameAccount;

pub fn get_random_number(ctx: &Context<CreateGameAccount>, counter: &mut usize) -> Result<u8> {
    let blockhash_data = ctx
        .accounts
        .recent_blockhashes
        .get(*counter % 32)
        .ok_or(ProgramError::InvalidAccountData)?;
    let random = (blockhash_data.blockhash.to_bytes()[0] % 10) as u8;

    *counter += 1;

    Ok(random)
}
