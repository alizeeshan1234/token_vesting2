use anchor_lang::{accounts::program, prelude::*};
use crate::state::accounts::*;
use anchor_spl::token::{MintTo, Token, mint_to, TransferChecked, transfer_checked};

pub fn initialize_vesting(ctx: Context<TokenVestingAccount>, vesting_company_name: String, vesting_amount: u64, vesting_start_time: i64, vesting_end_time: i64, vesting_cliff_period: i64) -> Result<()> {
    let vesting_account = &mut ctx.accounts.token_vesting_account;

    require!(ctx.accounts.signer.key().is_on_curve(), Error::InvalidPublicKey);
    require!(vesting_start_time < vesting_end_time, Error::InvalidVestingTime);
    require!(vesting_amount > 0, Error::InvalidVestingAmount);
    require!(
        vesting_cliff_period >= vesting_start_time && vesting_cliff_period <= vesting_end_time,
        Error::InvalidCliffPeriod
    );

    msg!("Creating a new token vesting account...");        

    **vesting_account = TokenVestingData {
        company_name: vesting_company_name,
        total_vesting_amount: vesting_amount,
        total_claimed_amount: 0,
        start_time: vesting_start_time,
        end_time: vesting_end_time,
        cliff_period: vesting_cliff_period,
    };

    msg!("Token Vesting Account created successfully!");

    msg!("Minting tokens to the vault account...");

    require!(ctx.accounts.mint.supply >= vesting_amount, Error::InsufficientTokenSupply);

    let mint_tokens = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_vault_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_tokens);

    mint_to(cpi_context, vesting_amount)?;

    msg!("Tokens minted to the token vault account successfully!");
        
    Ok(())
}

pub fn initialize_employee(ctx: Context<EmployeeAccount>) -> Result<()> {
    require!(ctx.accounts.employee.key().is_on_curve(), Error::InvalidPublicKey);
    msg!("Employee Account Initialized Successfully!");
    Ok(())
}

pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
    let token_vesting_account_data = &mut ctx.accounts.token_vesting_account;
    let token_vault_account = &ctx.accounts.token_vault_account;
    let employee_token_account = &ctx.accounts.employee_token_account;
    let mint_account = &ctx.accounts.mint;

    let total_allocated_tokens = token_vesting_account_data.total_vesting_amount;
    let total_claimed_tokens = token_vesting_account_data.total_claimed_amount;

    let current_time = Clock::get()?.unix_timestamp;
    let vesting_start_time = token_vesting_account_data.start_time;
    let vesting_end_time = token_vesting_account_data.end_time;
    let cliff_period = token_vesting_account_data.cliff_period;

    require!(current_time >= cliff_period, Error::CliffPeriodNotEnded);

    let elapsed_time = current_time.saturating_sub(vesting_start_time);
    let total_vesting_duration = vesting_end_time.saturating_sub(vesting_start_time);

    let vested_tokens = if current_time >= vesting_end_time {
        total_allocated_tokens
    } else {
        (elapsed_time as u128)
            .saturating_mul(total_allocated_tokens as u128)
            .checked_div(total_vesting_duration as u128)
            .unwrap_or(0) as u64
    };

    let claimable_tokens = vested_tokens.saturating_sub(total_claimed_tokens);
    require!(claimable_tokens > 0, Error::InvalidVestingAmount);

    msg!("Claiming {} tokens for the employee...", claimable_tokens);

    let cpi_accounts = TransferChecked {
        from: token_vault_account.to_account_info(),
        to: employee_token_account.to_account_info(),
        mint: mint_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    transfer_checked(cpi_context, claimable_tokens, mint_account.decimals)?;

    token_vesting_account_data.total_claimed_amount = total_claimed_tokens
        .saturating_add(claimable_tokens);

    msg!("Successfully claimed {} tokens!", claimable_tokens);

    Ok(())
}


#[error_code]
pub enum Error {
    #[msg("Invalid Public Key")]
    InvalidPublicKey,

    #[msg("Invalid Vesting Period Provided!")]
    InvalidVestingTime,

    #[msg("Invalid Vesting Amount Provided!")]
    InvalidVestingAmount,

    #[msg("Invalid Cliff Period Provided!")]
    InvalidCliffPeriod,

    #[msg("Insufficient Token Suppely")]
    InsufficientTokenSupply,

    #[msg("Cliff Period Not Ended!")]
    CliffPeriodNotEnded

}