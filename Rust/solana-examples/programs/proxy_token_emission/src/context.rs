use {
    anchor_lang::prelude::*,
    anchor_lang::solana_program::{system_instruction, system_program, sysvar},
    anchor_spl::token::{Mint, MintTo},
    spl_associated_token_account
};

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = user,
        space = 72,
        seeds = [b"mint".as_ref()],
        bump
        )]
    pub data: AccountLoader<'info, ProgramSettings>,
    #[account(mut)]
    pub mint: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    #[account(address = sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

impl<'info> From<&InitializeMint<'info>>
    for CpiContext<'_, '_, '_, 'info, anchor_spl::token::InitializeMint<'info>>
{
    fn from(accounts: &InitializeMint<'info>) -> Self {
        let cpi_accounts = anchor_spl::token::InitializeMint {
            mint: accounts.mint.to_account_info(),
            rent: accounts.rent.clone()
        };
        let cpi_program = accounts.spl_token_program.clone();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
#[derive(Accounts)]
pub struct MakeMint<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"mint".as_ref()],
        bump 
    )]
    pub settings: AccountLoader<'info, ProgramSettings>,
    pub recipient: AccountInfo<'info>,
    #[account(mut)]
    pub mint_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    #[account(address = spl_associated_token_account::ID)]
    pub spl_ata_program: AccountInfo<'info>,
    #[account(address = sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
    
}

impl<'info> From<&MakeMint<'info>> 
    for CpiContext<'_, '_, '_, 'info, anchor_spl::associated_token::Create<'info>> {
    fn from(accounts: &MakeMint<'info>) -> Self {
        let cpi_accounts = anchor_spl::associated_token::Create{
            payer: accounts.user.to_account_info(),
            associated_token: accounts.mint_wallet.clone(),
            authority: accounts.recipient.to_account_info(),
            mint: accounts.mint.clone(),
            system_program: accounts.system_program.clone(),
            token_program: accounts.spl_token_program.clone(),
            rent: accounts.rent.clone(),
        };
        CpiContext::new(accounts.spl_ata_program.clone(), cpi_accounts)
    }
}

impl<'a, 'b, 'c,'info> MakeMint<'info> {
    pub fn cpi_mint_to(&self, signer_seeds: &'a [&'b [&'c [u8]]]) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo{
            mint: self.mint.clone(),
            to: self.mint_wallet.clone(),
            authority: self.settings.to_account_info(),
        };
        
        CpiContext::new_with_signer(self.spl_token_program.clone(), cpi_accounts, signer_seeds)
    }
}

#[derive(Accounts)]
pub struct ChangeAuthority<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
    mut,
    has_one = authority,
    seeds = [b"mint".as_ref()],
    bump
    )]
    pub settings: AccountLoader<'info, ProgramSettings>,
    
    pub new_authority: AccountInfo<'info>,

}

#[account(zero_copy)]
#[derive(AnchorDeserialize, AnchorSerialize, Default, Debug)]
pub struct ProgramSettings {
    pub authority: Pubkey,
    pub mint: Pubkey,
}

#[test]
fn anchor_beh() {
  
  #[derive(AnchorSerialize)]
  struct Thing {
    x: u8,
    y: u32,
    z: String // dynamically sized!
  }
  let things = vec![
    Thing { x: 0, y: 1, z: "foo".to_string() },
    Thing { x: 1, y: 2, z: "way bigger string".to_string() },
  ];
  

   eprintln!("{:?}", things.try_to_vec());
}