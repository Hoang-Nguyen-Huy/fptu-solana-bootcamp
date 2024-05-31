use solana_client::rpc_client::RpcClient;
use solana_sdk:: {
    pubkey:: Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    system_program,
    transaction::Transaction,
};

use anyhow::Result;
use std::fs::File;
use std::str::FromStr;

const TRANSFER_NEW_ACC: u64 = 5000;
const TRANSFER_SPEC_ACC: u64 = 7000;
const RECEIVER_SPEC_PUBKEY: &str = "63EEC9FfGyksm7PkVC6z8uAmqozbQcTzbkWJNsgqjkFs"; 

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

fn create_and_transfer (
    client: &RpcClient, 
    payer: &Keypair,
    new_receiver_account: &Keypair,
    receiver_pubkey: &Pubkey,
    space: u64,
    transfer_new_acc: u64,
    transfer_spec_acc: u64,
) -> Result<()>  {
    let rent = client.get_minimum_balance_for_rent_exemption(space.try_into()?)?;
    let create_acc_instr = system_instruction::create_account(
        &payer.pubkey(),
        &new_receiver_account.pubkey(),
        rent, 
        space as u64,
        &system_program::id(),
    );

    let transfer_new_acc_instr = system_instruction::transfer(
        &payer.pubkey(),
        &new_receiver_account.pubkey(),
        transfer_new_acc
    );

    let transfer_spec_acc_instr = system_instruction::transfer(
        &payer.pubkey(),
        receiver_pubkey,
        transfer_spec_acc
    );

    let create_blockhash = client.get_latest_blockhash()?;
    let create_transaction = Transaction::new_signed_with_payer(
        &[create_acc_instr, transfer_new_acc_instr, transfer_spec_acc_instr],
        Some(&payer.pubkey()),
        &[&payer, &new_receiver_account],
        create_blockhash,
    );

    let create_sig = client.send_and_confirm_transaction(&create_transaction)?;

    let explorer_url = util::get_signature_explorer_url(&create_sig.to_string());

    println!("explorer url: {}", explorer_url);
//  https://explorer.solana.com/tx/5mTMS7b4VwvkTvwrnM8C6EPog1koUto76bn8CcWMese93kiovP7tnbtH6F53H8bd8owDRj5pchEzpqKY4Bef6xgg?cluster=devnet
    Ok(())
}

fn main() -> Result<()> {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let payer = read_keypair_from_file("../payer-keypair.json");
    let new_receiver_account = Keypair::new();
    let receiver_spec_pubkey = Pubkey::from_str(RECEIVER_SPEC_PUBKEY)?;
    const ACCOUNT_SPACE: u64 = 0;

    create_and_transfer(&client, &payer, &new_receiver_account, &receiver_spec_pubkey, ACCOUNT_SPACE, TRANSFER_NEW_ACC, TRANSFER_SPEC_ACC)?;

    Ok(())
}