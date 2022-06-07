#![cfg(feature = "test-bpf")]

use std::rc::Rc;

use solana_program::pubkey::Pubkey;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    system_instruction, system_program,
};
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};
use solana_sdk::nonce_keyed_account::NonceKeyedAccount;
use solana_sdk::program_pack::Pack;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use solana_sdk::{client, entrypoint_native};

use anchor_client::{Client, Cluster};

use borsh::BorshDeserialize;
use proxy_token_emission::{
    accounts::{
        InitializeMint as InitializeAccount,
        MakeMint as MakeMintAccounts,
        
    },
    entry,
    instruction::{InitializeMint, MakeMint, ChangeAuthority},
    ID,
};
use spl_token::state::Mint;

use spl_associated_token_account::get_associated_token_address;

struct EnvProxyMint {
    ctx: ProgramTestContext,
    payer: Keypair,
    user : Keypair,
    anchor_client: Client,
}

impl EnvProxyMint {
    async fn new() -> Self {
        let mut program_test = ProgramTest::new("proxy_token_emission", ID, processor!(entry));
        // program_test.add_program(
        //     "spl_associated_token_account",
        //     spl_associated_token_account::ID,
        //     processor!(spl_associated_token_account::processor::process_instruction)
        // );
        let mut ctx = program_test.start_with_context().await;

        let payer = Keypair::new();
        let user = Keypair::new();
        let raw_key = payer.to_bytes();

        // credit admin and user accounts
        ctx.banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &ctx.payer.pubkey(),
                    &payer.pubkey(),
                    5_000_000_000,
                )],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer],
                ctx.last_blockhash,
            ))
            .await
            .unwrap();
            
        ctx.banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[system_instruction::transfer(
                    &ctx.payer.pubkey(),
                    &user.pubkey(),
                    5_000_000_000,
                )],
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer],
                ctx.last_blockhash,
            ))
            .await
            .unwrap();

       

        let anchor_client = Client::new_with_options(
            Cluster::Debug,
            Rc::new(payer),
            solana_sdk::commitment_config::CommitmentConfig::processed(),
        );
       
        EnvProxyMint {
            ctx,
            payer: Keypair::from_bytes(&raw_key).unwrap(),
            user,
            anchor_client,
        }
      
    }
}

#[tokio::test]
async fn test_proxy() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = EnvProxyMint::new().await;

    let pda_key = Pubkey::find_program_address(&[b"mint".as_ref()], &proxy_token_emission::ID);

    let rent = env.ctx.banks_client.get_rent().await?;

    let mint = Keypair::new();

    let program = env.anchor_client.program(proxy_token_emission::ID);
    let mut tx = program
        .request()
        .signer(&env.payer)
        .signer(&mint)
        .accounts(InitializeAccount {
            data: pda_key.0,
            mint: mint.pubkey(),
            user: env.payer.pubkey(),
            system_program: system_program::ID,
            spl_token_program: spl_token::ID,
            rent: solana_program::sysvar::rent::ID,
        })
        .args(InitializeMint)
        .build_tx(env.ctx.last_blockhash)?;

    env.ctx.banks_client.process_transaction(tx).await.unwrap();

    let mut account = env.ctx.banks_client.get_account(pda_key.0).await?.unwrap();

    let info = proxy_token_emission::context::ProgramSettings::deserialize(&mut &account.data[8..])?;
    let token_wallet = get_associated_token_address(&env.payer.pubkey(), &info.mint);
    let mut mint_acc = env.ctx.banks_client.get_account(info.mint).await?.unwrap();
    let mint_data = spl_token::state::Mint::unpack(&mint_acc.data[..])?;

    tx = program
        .request()
        .signer(&env.payer)
        .accounts(MakeMintAccounts {
            user: env.payer.pubkey(),
            settings: pda_key.0,
            mint_wallet: token_wallet,
            mint: info.mint,
            recipient: env.payer.pubkey(),
            system_program: system_program::ID,
            spl_token_program: spl_token::ID,
            spl_ata_program: spl_associated_token_account::ID,
            rent: solana_program::sysvar::rent::ID,
        })
        .args(MakeMint { amount: 10_00000_999, bump: pda_key.1 })
        .build_tx(env.ctx.last_blockhash)?;
    env.ctx.banks_client.process_transaction(tx).await.unwrap();

    let mut recipinet_token_account = env
        .ctx
        .banks_client
        .get_account(token_wallet)
        .await?
        .unwrap();

    let token_acc = spl_token::state::Account::unpack(&recipinet_token_account.data[..])?;

    println!(
        "\n*****************************************************
        \ntoken created by porgram: {}",
        mint.pubkey()
    );
    println!("recipient/payer: {}", env.payer.pubkey());
    println!(
        "token wallet for recipient/payer: {}",
        get_associated_token_address(&env.payer.pubkey(), &mint.pubkey())
    );

    println!(
        "\n*****************************************************
        \nbalance {} for {} with token wallet ({}) is {}
    \n*****************************************************\n",
        info.mint,
        env.payer.pubkey(),
        token_wallet,
        token_acc.amount
    );
    
    tx = program
        .request()
        .signer(&env.payer)
        .accounts(proxy_token_emission::accounts::ChangeAuthority {
            authority: env.payer.pubkey(),
            new_authority: env.user.pubkey(),
            settings: pda_key.0
        })
        .args(proxy_token_emission::instruction::ChangeAuthority)
        .build_tx(env.ctx.last_blockhash)?;
    
    env.ctx.banks_client.process_transaction(tx).await.unwrap();
    
    println!("\nswitching back\n");
    
    tx = program
        .request()
        .signer(&env.user)
        .accounts(proxy_token_emission::accounts::ChangeAuthority {
            authority: env.user.pubkey(),
            new_authority: env.payer.pubkey(),
            settings: pda_key.0
        })
        .args(proxy_token_emission::instruction::ChangeAuthority)
        .build_tx(env.ctx.last_blockhash)?;
    
    env.ctx.banks_client.process_transaction(tx).await.unwrap();
    
   
    
    Ok(())
}
