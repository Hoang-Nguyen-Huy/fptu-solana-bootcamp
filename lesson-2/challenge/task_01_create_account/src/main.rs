use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    system_program,
    transaction::Transaction,
};

use anyhow::Result;
use std::fs::File;

mod util {
    pub fn get_signature_explorer_url(signature: &str) -> String {
        format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature)
    }
}

fn read_keypair_from_file(filepath: &str) -> Keypair {
    let file = File::open(filepath).expect("Unable to open keypair file");
    let keypair: Vec<u8> = serde_json::from_reader(file).expect("Unable to parse keypair file");
    Keypair::from_bytes(&keypair).expect("Unable to create keypair from bytes")
}

fn create_account(
    client: &RpcClient,
    payer: &Keypair,
    new_account: &Keypair,
    space: u64,
) -> Result<()> {
    let rent = client.get_minimum_balance_for_rent_exemption(space.try_into()?)?;
    let create_instr = system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        rent,
        space as u64,
        &system_program::id(),
    ); // create instruction

    let create_blockhash = client.get_latest_blockhash()?;
    let create_tx = Transaction::new_signed_with_payer(
        &[create_instr],
        Some(&payer.pubkey()),
        &[&payer, &new_account],
        create_blockhash,
    ); // create transaction

    let create_sig = client.send_and_confirm_transaction(&create_tx)?;

    let explorer_url = util::get_signature_explorer_url(&create_sig.to_string());
    println!("explorer url: {}", explorer_url);
//  https://explorer.solana.com/tx/5iiuLMQ1DNq9menSQKtFRSrGpRU5GJYLrd4QpnsgASCDb1xyyMsTF3YdRxyqWh4f51x1JiCJyw7WjFAPpbd3dnz?cluster=devnet

    Ok(())
}

fn main() -> Result<()> {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let payer = read_keypair_from_file("payer-keypair.json");
    let new_account = Keypair::new();
    const ACCOUNT_SPACE: u64 = 0;

    create_account(&client, &payer, &new_account, ACCOUNT_SPACE)?;

    println!("New account created: {}", new_account.pubkey());

    Ok(())
}