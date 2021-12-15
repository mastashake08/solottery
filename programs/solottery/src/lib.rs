use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solottery {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.ticket_count = 0;

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<Increment>, data: u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.accounts[base_account.ticket_count] = data;
        base_account.ticket_count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer=user, space= 8 + 8)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub ticket_count: u64,
    pub accounts: [u64; 100]
}
