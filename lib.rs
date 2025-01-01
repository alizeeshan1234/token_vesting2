pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::accounts::*;

declare_id!("GtSHNM7qETizmneeqouK5BAfcDAn428SMovfutmqCSZ9");

#[program]
pub mod token_vesting {
    use super::*;

    pub fn initialize_account(ctx: Context<TokenVestingAccount>, company_name: String, vesting_amount: u64, vesting_start_time: i64, vesting_end_time: i64, vesting_cliff_period: i64) -> Result<()> {
        initialize_vesting(ctx, company_name, vesting_amount, vesting_start_time, vesting_end_time, vesting_cliff_period)?;
        Ok(())
    }

    pub fn initialize_employee_account(ctx: Context<EmployeeAccount>) -> Result<()> {
        initialize_employee(ctx)?;
        Ok(())
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>, company_name: String) -> Result<()> {
        claim_tokens(ctx, company_name)?;
        Ok(())
    }
}
