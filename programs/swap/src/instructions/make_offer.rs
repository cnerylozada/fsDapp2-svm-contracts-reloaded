use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MakeOffer {}

pub fn handler(_ctx: Context<MakeOffer>) -> Result<()> {
    Ok(())
}
