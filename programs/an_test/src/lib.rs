use anchor_lang::prelude::*;

declare_id!("BuvS4T7L1kFGRiQXiKD76khfLUxq9EqyeX3nH3CEYPPK");

#[program]
pub mod an_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
