use anchor_lang::prelude::*;

declare_id!("FRqafj3DiwKbsnkCF3KRDEDdBRm1yrXymp3r8fBq6Lbp");

#[program]
pub mod staking_in_anchor {
    use super::*;

    pub fn create_pda_account(ctx: Context<CreatePdaAccount>) -> Result<()> {
        msg!("Creating a pda account");
        let pda_account = &mut ctx.accounts.pda_account;
        let clock = Clock::get()?;

        pda_account.owner = ctx.accounts.payer.key();
        pda_account.staked_amount = 0;
        pda_account.total_points = 0;
        pda_account.last_updated_time = clock.unix_timestamp;
        pda_account.bump = ctx.bumps.pda_account;

        msg!("Pda account successfully created");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreatePdaAccount<'info> {
    #[account(mut)]
    pub payer : Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + 32 + 8 + 8 + 8 + 1,
        seeds = [b"client1",payer.key().as_ref()],
        bump
    )]
    pub pda_account : Account<'info,StakeAccount>,
    pub system_program: Program<'info,System>
}

#[account]
pub struct StakeAccount{
    pub owner: Pubkey,             //   32 bytes 
    pub staked_amount: u64,        //   8 bytes
    pub total_points: u64,         //   8 bytes
    pub last_updated_time: i64,    //   8 bytes
    pub bump: u8                   //   1 byte
}