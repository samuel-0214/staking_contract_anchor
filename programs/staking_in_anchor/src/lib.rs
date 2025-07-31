use anchor_lang::prelude::*;
use anchor_lang::system_program;

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

    pub fn stake(ctx: Context<Stake>, amount:u64) -> Result<()>{
        msg!("Staking initialized");
        require!(amount > 0, StakeError::InvalidAmount);

        let pda_account = &mut ctx.accounts.pda_account;
        let clock = Clock::get()?;

        update_points(pda_account,clock.unix_timestamp);

        // Transferring sol from user to pda account
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer{
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.pda_account.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount)?;

        pda_account.staked_amount = pda_account.staked_amount.checked_add(amount).ok_or(StakeError::Overflow)?;

        msg!("Staked {} lamports. Total staked: {}, Total points: {}", 
             amount, pda_account.staked_amount, pda_account.total_points / 1_000_000);
        Ok(())
    }
}

fn update_points(pda_account : &mut StakeAccount,current_time: i64) -> Result<()>{
    
    let time_passed = current_time.checked_sub(pda_account.last_updated_time).ok_or(StakeError::InvalidTimeStamp)? as u64;

    if time_passed > 0 && pda_account.staked_amount > 0{
        let new_points = calculate_points(pda_account.staked_amount, time_passed)?;
        pda_account.total_points = pda_account.total_points.checked_add(new_points).ok_or(StakeError::Overflow)?;
    }

    pda_account.last_updated_time = current_time;
    Ok(())
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

#[derive(Accounts)]
pub struct Stake<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"client1",user.key().as_ref()],
        bump = pda_account.bump,
        constraint = pda_account.owner == user.key() @ StakeError::UnauthorizedOwner
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

#[error_code]
pub enum StakeError{
    #[msg("Unauthorized access")]
    UnauthorizedOwner,
    #[msg("Stake amount should be greater than 0")]
    InvalidAmount,
    #[msg("Arithemetic Overflow")]
    Overflow,
    #[msg("Invalid Timestamp")]
    InvalidTimeStamp,
}