#![cfg(feature = "test-bpf")]

use std::mem::size_of;

use anchor_client::solana_sdk::{client::SyncClient, system_program, sysvar};
use arrayref::array_refs;
use proxy_token_emission::context::ProgramSettings;
use solana_sdk::{program_option::COption, program_pack::Pack, rent::Rent};
use spl_associated_token_account::get_associated_token_address;
use spl_token::state::Mint;

use {
    anchor_client::{
        anchor_lang::Discriminator,
        solana_sdk::{
            account::Account,
            commitment_config::CommitmentConfig,
            pubkey::Pubkey,
            signature::{Keypair, Signer},
            transaction::Transaction,
        },
        Client, Cluster,
    },
    anyhow::Result,
    arrayref::mut_array_refs,
    proxy_token_emission::{
        accounts::{
            ChangeAuthority as ChangeAuthorityAccounts, InitializeMint as InitializeAccount,
            MakeMint as MakeMintAccounts,
        },
        context, entry,
        instruction::{ChangeAuthority, InitializeMint, MakeMint},
        ID,
    },
    solana_program_test::{processor, tokio, ProgramTest},
    std::rc::Rc,
};

#[tokio::test]
async fn update_foo() -> Result<()> {
    let mut pt = ProgramTest::new(
        "proxy_token_emission",
        proxy_token_emission::ID,
        processor!(entry),
    );

    let rent = Rent::default();

    let authority = Keypair::new();
    let authority_account = {
        Account {
            lamports: 10_0000_00000,
            owner: system_program::ID,
            ..Account::default()
        }
    };

    let settings_tup = Pubkey::find_program_address(&[b"mint".as_ref()], &proxy_token_emission::ID);
    let mint_keypair = Keypair::new();
    let mint = mint_keypair.pubkey();
    let mint_acc = {
        let mut acc_data = vec![0; Mint::LEN];
        let mint_data = Mint {
            mint_authority: COption::Some(settings_tup.0),
            decimals: 9,
            is_initialized: true,
            freeze_authority: COption::Some(settings_tup.0),
            ..Mint::default()
        };

        Mint::pack(mint_data, &mut acc_data[..])?;

        Account {
            lamports: rent.minimum_balance(acc_data.len()),
            data: acc_data,
            owner: spl_token::ID,
            executable: false,
            ..Account::default()
        }
    };

    let settings_tup = Pubkey::find_program_address(&[b"mint".as_ref()], &proxy_token_emission::ID);
    let settings_acc = {
        let mut acc_data = vec![0; size_of::<ProgramSettings>() + 8];
        let (discr, data) = mut_array_refs![&mut acc_data[..], 8;..;];

        *discr = context::ProgramSettings::discriminator();

        data.copy_from_slice(bytemuck::bytes_of(&context::ProgramSettings {
            authority: authority.pubkey(),
            mint: mint,
        }));

        Account {
            lamports: rent.minimum_balance(acc_data.len()),
            data: acc_data,
            owner: proxy_token_emission::ID,
            executable: false,
            ..Account::default()
        }
    };

    pt.add_account(authority.pubkey().clone(), authority_account);
    pt.add_account(mint, mint_acc);
    pt.add_account(settings_tup.0, settings_acc);

    let (mut banks_client, payer, recent_blockhash) = pt.start().await;
    println!("{:?}", payer.pubkey());
    println!("{:?}", get_associated_token_address(&payer.pubkey(), &mint));
    let client = Client::new_with_options(
        Cluster::Debug,
        Rc::new(Keypair::new()),
        CommitmentConfig::processed(),
    );

    let program = client.program(ID);

    let change_auth_ix = program
        .request()
        .accounts(ChangeAuthorityAccounts {
            authority: authority.pubkey(),
            settings: settings_tup.0,
            new_authority: payer.pubkey(),
        })
        .args(ChangeAuthority)
        .instructions()
        .unwrap()
        .pop()
        .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[change_auth_ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
    println!("{:?}", transaction);
    banks_client.process_transaction(transaction).await.unwrap();

    let change_auth_ix = program
        .request()
        .accounts(ChangeAuthorityAccounts {
            authority: payer.pubkey(),
            settings: settings_tup.0,
            new_authority: authority.pubkey(),
        })
        .args(ChangeAuthority)
        .instructions()
        .unwrap()
        .pop()
        .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[change_auth_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(transaction).await.unwrap();

    let make_mint_ix = program
        .request()
        .accounts(MakeMintAccounts {
            user: authority.pubkey(),
            settings: settings_tup.0,
            recipient: payer.pubkey(),
            mint_wallet: get_associated_token_address(&payer.pubkey(), &mint),
            mint,
            system_program: system_program::ID,
            spl_token_program: spl_token::ID,
            spl_ata_program: spl_associated_token_account::ID,
            rent: sysvar::rent::ID,
        })
        .args(MakeMint {
            amount: 10_000_3000,
            bump: settings_tup.1,
        })
        .instructions()
        .unwrap()
        .pop()
        .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[make_mint_ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
    println!("{:?}", transaction);
    banks_client.process_transaction(transaction).await.unwrap();

    Ok(())
}
