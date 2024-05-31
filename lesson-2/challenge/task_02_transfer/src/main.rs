use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};

use anyhow::Result;
use std::fs::File;
use std::str::FromStr;

const RECEIVER_PUBKEY: &str = "63EEC9FfGyksm7PkVC6z8uAmqozbQcTzbkWJNsgqjkFs";
const TRANSFER_AMOUNT: u64 = 5000;

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

fn transfer_lamport (
    client: &RpcClient,
    payer: &Keypair, 
    receiver_account: &Pubkey, 
    tranfser_amount: u64,
) -> Result<()> {
    let transfer_instr = system_instruction::transfer(
        &payer.pubkey(),
        receiver_account,
        tranfser_amount,
    );

    let create_blockhash = client.get_latest_blockhash()?;
    let create_transaction = Transaction::new_signed_with_payer(
        &[transfer_instr],
        Some(&payer.pubkey()),
        &[payer],
        create_blockhash,
    );

    let create_sig = client.send_and_confirm_transaction(&create_transaction)?;
    
    let explorer_url = util::get_signature_explorer_url(&create_sig.to_string());

    println!("explorer url: {}", explorer_url);
//    https://explorer.solana.com/tx/4s3J68TSQWqpE6Y4QJRxTUWtpUzKgrb66eKf3PtUKkV4sERsfzRAZH6xR61oymZzu4h6DDRiu3zq3WQFXmM5vLs2?cluster=devnet
    Ok(())
}

fn main() -> Result<()>  {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let payer = read_keypair_from_file("../payer-keypair.json");
    let receiver_pubkey = Pubkey::from_str(RECEIVER_PUBKEY)?;

    transfer_lamport(&client, &payer, &receiver_pubkey, TRANSFER_AMOUNT)?;

    Ok(())
}