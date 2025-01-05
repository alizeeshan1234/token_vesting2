use anchor_lang::prelude::*;
use crate::constants::ANCHOR_DISCREMENATOR;
use anchor_spl::token::*;

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct TokenVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,  //Employer Account

    #[account(
        init,
        space = ANCHOR_DISCREMENATOR + TokenVestingData::INIT_SPACE,
        payer = signer,
        seeds = [b"token_vesting_account", signer.key().as_ref()],
        bump,
    )]
    pub token_vesting_account: Account<'info, TokenVestingData>,

    #[account(
        init,
        payer = signer,
        seeds = [b"token_vault_account", company_name.as_bytes().as_ref()],
        bump,
        token::mint = mint,
        token::authority = signer,
    )]
    pub token_vault_account: Account<'info, TokenAccount>, 

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

}

#[derive(Accounts)]
pub struct EmployeeAccount<'info> {
    #[account(mut)]
    pub employee: Signer<'info>,

    #[account(
        init,
        payer = employee,
        seeds = [b"employee_token_account", employee.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = employee,
    )]
    pub employee_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>, //Employee

    #[account(
        seeds = [b"token_vault_account", company_name.as_bytes().as_ref()],
        bump,
    )]
    pub token_vault_account: Account<'info, TokenAccount>, //Vested tokens are holded in the vault account

    #[account(
        seeds = [b"employee_token_account", signer.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = signer,
    )]
    pub employee_token_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"token_vesting_account", signer.key().as_ref()],
        bump,
    )]
    pub token_vesting_account: Account<'info, TokenVestingData>,

    pub mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct TokenVestingData {
    #[max_len(50)]
    pub company_name: String,
    pub total_vesting_amount: u64,
    pub total_claimed_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_period: i64,
}
