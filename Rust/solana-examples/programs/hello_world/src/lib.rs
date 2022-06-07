use anchor_lang::prelude::*;


declare_id!("6txgmfccGSptbTvpa6CboLXvQFpYiXN4DZAtmm9t432o");
#[program]
pub mod dispatcher {

    use super::*;

    pub fn create_acc(ctx: Context<Create>) -> ProgramResult {
        let new_acc = &mut ctx.accounts.greeted_account;
        new_acc.authority = *ctx.accounts.user.key;
        new_acc.counter = 0;
        msg!("created pda acc for {}", ctx.accounts.user.key);
        

        Ok(())
    }

    pub fn make_greet(ctx: Context<Greet>) -> ProgramResult {
        let new_acc = &mut ctx.accounts.greeted_account;
        new_acc.counter += 1;
        msg!(
            "account {} was greeted {} time(s)",
            ctx.accounts.user.key,
            new_acc.counter
        );

        Ok(())
    }
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
    init,
    payer = user,
    // constraint = greeted_account.authority == *user.key,
    space = 48,
    seeds = [b"hello".as_ref(), user.key.as_ref()],
    bump
    )]
    pub greeted_account: Account<'info, Info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]

pub struct Greet<'info> {
    #[account(
    mut,
    // constraint = greeted_account.authority == *user.key,
    seeds = [b"hello".as_ref(), user.key.as_ref()],
    bump
    )]
    pub greeted_account: Account<'info, Info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Info {
    pub authority: Pubkey,
    pub counter: u64,
}
