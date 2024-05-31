use solana_client::rpc_client::RpcClient;
use solana_sdk:: {
    signature::{Keypair, Signer},
    system_instruction,
    system_program,
    transaction::Transaction,
};

use anyhow::Result;
use std::fs::File;

const TRANSFER_AMOUNT: u64 = 5000;

mod util {
    pub fn get_signature_explorer_url(signature: &str) -> String {
        format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature)
    }
}

fn read_keypair_from_file(filepath: &str) -> Keypair {
    let file = File::open(filepath).expect("Unable to open keypair file");
    let keypair: Vec<u8> = serde_json::from_reader(file).expect("Unable to parse keypair file");
    return Keypair::from_bytes(&keypair).expect("Unable to create keypair from bytes");
}

fn create_acc_transfer (
    client: &RpcClient,
    payer: &Keypair,
    new_receiver_account: &Keypair, 
    space: u64,
    transfer_amount: u64,
) -> Result<()> {
    let rent = client.get_minimum_balance_for_rent_exemption(space.try_into()?)?;
    let create_acc_instr = system_instruction::create_account(
        &payer.pubkey(),
        &new_receiver_account.pubkey(),
        rent, 
        space as u64,
        &system_program::id(),
    );

    let transfer_instr = system_instruction::transfer(
        &payer.pubkey(),
        &new_receiver_account.pubkey(),
        transfer_amount
    );

    let create_blockhash = client.get_latest_blockhash()?;
    let create_transaction = Transaction::new_signed_with_payer(
        &[create_acc_instr, transfer_instr],
        Some(&payer.pubkey()),
        &[&payer, &new_receiver_account],
        create_blockhash,
    );

    let create_sig = client.send_and_confirm_transaction(&create_transaction)?;

    let explorer_url = util::get_signature_explorer_url(&create_sig.to_string());
    
    println!("explorer url: {}", explorer_url);
//  https://explorer.solana.com/tx/5Yvi5RwrPi4w1H4NJycSXhfPVgpN4Mvc2sKonMUT4Gx7grqzcpbGqE2PqoQp2daw7SAypy534rNK9a1Ae7NthsZt?cluster=devnet
    Ok(())
}

fn main () -> Result<()> {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let payer = read_keypair_from_file("../payer-keypair.json");
    let new_receiver_account = Keypair::new();
    const ACCOUNT_SPACE: u64 = 0;

    create_acc_transfer(&client, &payer, &new_receiver_account, ACCOUNT_SPACE, TRANSFER_AMOUNT)?;

    Ok(())
}