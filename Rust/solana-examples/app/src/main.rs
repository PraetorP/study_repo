use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::solana_sdk::signature::{Keypair, Signer};
use anchor_client::solana_sdk::system_instruction;
use anchor_client::{Client, Cluster, EventContext};
use anchor_study::accounts as program_acc;
use anchor_study::instruction as program_instruction;
use anchor_study::Counter;

use hello_world::accounts as hello_acc;
use hello_world::entry;
use hello_world::instruction as hello_instruction;

use anchor_client::anchor_lang::Accounts;
use anyhow::Result;
use clap::Clap;
use rand::rngs::OsRng;
use solana_sdk::system_program;
use std::error::Error;
use std::rc::Rc;
use std::time::Duration;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting test...");

    #[cfg(target_os = "linux")]
    // Wallet and cluster params.
    let mut payer = read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Example requires a keypair file");

    #[cfg(target_os = "windows")]
    let mut payer = read_keypair_file("../keys/id.json")?;
    println!("payer pubkey: {}", payer.pubkey());

    let url = Cluster::Custom(
        "http://localhost:8899".to_string(),
        "ws://127.0.0.1:8900".to_string(),
    );

    let client = Client::new_with_options(url, Rc::new(payer), CommitmentConfig::processed());

    let anchor_study_key = read_keypair_file("../target/deploy/anchor_study-keypair.json")?;
    let hello_world_key = read_keypair_file("../target/deploy/hello_world-keypair.json")?;

    println!("anchor study deploy key: {}", anchor_study_key.pubkey());
    println!(
        "hello world deploy key: {} || {} ",
        hello_world_key.pubkey(),
        hello_world::id()
    );

    let _ = anchor_study(&client, anchor_study::id())?;

    payer = read_keypair_file("../keys/id.json")?;

    let _ = hello_world(&client, &payer, hello_world::id())?;

    
    Ok(())
}

pub fn anchor_study(client: &Client, pid: Pubkey) -> Result<()> {
    let program = client.program(pid);

    // `Create` parameters.
    let counter = Keypair::generate(&mut OsRng);
    let authority = program.payer();

    program
        .request()
        .signer(&counter)
        .accounts(program_acc::Create {
            counter: counter.pubkey(),
            user: authority,
            system_program: system_program::ID,
        })
        .args(program_instruction::Create { authority })
        .send()?;

    

    let counter_account: Counter = program.account(counter.pubkey())?;

    let acc_info = program.rpc().get_account(&counter.pubkey())?;

    println!(
        "created account adress: {}  || owner {}",
        counter.pubkey(),
        acc_info.owner
    );

    assert_eq!(counter_account.authority, authority);
    assert_eq!(counter_account.count, 0);

    println!("Basic 2 success!");

    Ok(())
}

pub fn hello_world(client: &Client, signer: &Keypair, pid: Pubkey) -> Result<()> {
    println!("enter to hello world");
    let program = client.program(pid);
    println!("used pid: {}", pid);
    let authority = program.payer();
    println!("prepare to make pda key");
    println!("program id: {}", hello_world::ID);
    let pda_key =
        Pubkey::find_program_address(&[b"hello".as_ref(), authority.as_ref()], &hello_world::ID);

    println!("pda_acc: {:?}", pda_key.0);

    program
        .request()
        .signer(signer)
        .accounts(hello_acc::Create {
            greeted_account: pda_key.0,
            user: signer.pubkey(),
            system_program: system_program::ID,
        })
        .args(hello_instruction::CreateAcc)
        .send()?;

    println!("Hello World succes ! ");

    Ok(())
}
