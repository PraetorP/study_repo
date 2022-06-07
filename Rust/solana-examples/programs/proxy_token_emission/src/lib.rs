use { 
    anchor_lang::prelude::*,
    anchor_lang::solana_program::{system_instruction, system_program, sysvar},
    anchor_spl::token,
    anchor_spl::token::Mint,
    context::*,
    error::ErrorCode,
    spl_associated_token_account::{
        create_associated_token_account,
        get_associated_token_address
    }
};

pub mod context;
pub mod error;

declare_id!("CXEaM4xCce2Kpcvi7xxxVmn5rkKPdB7odCwztTphNcA9");
#[program]
pub mod proxy_nft_emission {
    
    use anchor_lang::accounts;

    use super::*;
    
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> ProgramResult {
        let mut data = ctx.accounts.data.load_init()?;
        msg!("initizization for {}", ctx.accounts.user.key);
        msg!("user is signer: {}", ctx.accounts.user.is_signer);
        msg!("user signer key: {:?}", ctx.accounts.user.signer_key());

        msg!("mint key: {}", ctx.accounts.mint.key);
        msg!("creating account for token...");

        let lamports = sysvar::rent::Rent::get()
            .unwrap()
            .minimum_balance(token::Mint::LEN);

        let create_acc_ix = system_instruction::create_account(
            ctx.accounts.user.key,
            ctx.accounts.mint.key,
            lamports,
            Mint::LEN as u64,
            &spl_token::id(),
        );
        
        msg!("instruction created");
        
        anchor_lang::solana_program::program::invoke(
            &create_acc_ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.system_program.clone(),
            ],
        )?;

        let authority_key = ctx.accounts.data.to_account_info().key;
        
        token::initialize_mint(
            (&*ctx.accounts).into(),
            9,
            authority_key,
            Some(authority_key),
        )?;
        data.authority = *ctx.accounts.user.key;
        data.mint = *ctx.accounts.mint.key;

        msg!("ctx.accounts.mint_data.authority: {}", data.authority);
        msg!("ctx.accounts.mint_data.mint: {}", data.mint);
        Ok(())
    }

    pub fn make_mint(ctx: Context<MakeMint>, amount: u64, bump: u8) -> ProgramResult {
        let mut settings = ctx.accounts.settings.load()?;
        
        if *ctx.accounts.user.key != settings.authority {
            return Err(ErrorCode::InvalidAuthorityKey.into());
        }
        
        let token_wallet = get_associated_token_address(ctx.accounts.recipient.key, &settings.mint);

        if token_wallet != *ctx.accounts.mint_wallet.key {
            return Err(ErrorCode::InvalidMintWalletAdress.into());
        }

        if ctx.accounts.mint_wallet.data_is_empty() {
            anchor_spl::associated_token::create((&*ctx.accounts).into())?;
        }

        token::mint_to(
            ctx.accounts.cpi_mint_to(&[&[&b"mint"[..], &[bump]]]),
            amount,
        )?;
        
        Ok(())
    }

    pub fn change_authority(ctx: Context<ChangeAuthority>) -> ProgramResult {
        if *ctx.accounts.new_authority.owner != system_program::ID {
            return Err(ErrorCode::InvalidAuthorityKey.into());
        }
        
        let mut settings = ctx.accounts.settings.load_mut()?;
        msg!("changing authority ...");
        msg!("current authority: {}", settings.authority);
        
        settings.authority = ctx.accounts.new_authority.key();
        
        msg!("Done. New authority: {}", settings.authority);

        Ok(())
    }
}
