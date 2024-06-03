use mpl_token_metadata::instructions as metadata_instruction;
use mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs;
use mpl_token_metadata::types::DataV2;
use mpl_token_metadata::ID as metadata_program_id; // mpl_token_metadat: dùng làm việc vs metada của token
use solana_client::rpc_client::RpcClient; // solana_client, solana_sdk: cung cấp các chức năng để tương tác với mạng Solana
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use solana_sdk::{program_pack::Pack, signature::Keypair, signer::Signer, system_instruction};
use spl_associated_token_account::get_associated_token_address; 
use spl_associated_token_account::instruction; // spl_associated_token_account, spl_token: thư viện SPL để làm việc với token trên Solana
use spl_token::instruction as token_instruction;
use spl_token::state::Mint;
use spl_token::ID as token_program_id;

use anyhow::Result;
use std::fs::File;

const MINT_DECIMALS: u8 = 6;
const MINT_AMOUNT: u64 = 100;

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

fn mint_my_first_token (
    client: &RpcClient,
    payer: &Keypair,
    mint_account_key: &Keypair,
) -> Result<()> {
    let (metadata_account_address, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            &metadata_program_id.to_bytes(),
            &mint_account_key.pubkey().to_bytes(),
        ],
        &metadata_program_id
    ); // để tìm địa chỉ chương trình

    let associated_token_account_address = get_associated_token_address(&payer.pubkey(), &mint_account_key.pubkey()); // lấy địa chỉ tài khoản token liên kết

    let rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;
    let create_mint_account = system_instruction::create_account(
        &payer.pubkey(),
        &mint_account_key.pubkey(),
        rent,
        Mint::LEN as u64,
        &token_program_id,
    );

    let init_mint_account = token_instruction::initialize_mint(
        &token_program_id,
        &mint_account_key.pubkey(),
        &payer.pubkey(),
        Some(&mint_account_key.pubkey()),
        MINT_DECIMALS
    );

    let create_metadata_account = metadata_instruction::CreateMetadataAccountV3 {
        metadata: metadata_account_address,
        mint: mint_account_key.pubkey(),
        mint_authority: payer.pubkey(),
        payer: payer.pubkey(),
        update_authority: (payer.pubkey(), true),
        system_program: system_program::id(),
        rent: None
    }
    .instruction(CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
            name: "Test Token".to_string(),
            symbol: "TT".to_string(),
            uri: "https://raw.githubusercontent.com/hoang-nguyen-huy/fptu-solana-bootcamp/main/assets/tt-token.json".to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None, 
            uses: None,
        },
        is_mutable: true, 
        collection_details: None, 
    });

    let create_associated_token_account = instruction::create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint_account_key.pubkey(),
        &token_program_id,
    );

    let mint_to_account = token_instruction::mint_to(
        &token_program_id,
        &mint_account_key.pubkey(),
        &associated_token_account_address,
        &payer.pubkey(),
        &[&payer.pubkey()],
        MINT_AMOUNT,
    );

    let mut instructions = Vec::with_capacity(5);
    instructions.push(create_mint_account);
    instructions.push(init_mint_account?);
    instructions.push(create_metadata_account);

    if client.get_account(&associated_token_account_address).is_err() {
        instructions.push(create_associated_token_account);
    }
    instructions.push(mint_to_account?);

    let create_blockhash = client.get_latest_blockhash()?;
    let create_transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[&payer, &mint_account_key],
        create_blockhash,
    );

    let create_sig = client.send_and_confirm_transaction(&create_transaction)?;

    let explorer_url = util::get_signature_explorer_url(&create_sig.to_string());

    println!("explorer url: {}", explorer_url);

//  https://explorer.solana.com/tx/4VhS75u2tDpxpXbT78kHWRQXjKNEMfHJzunH3LUKy1e5x48hXHnJq8zMkiU48cBmjLo4c1tURwZq7ze2kJqXZdiD?cluster=devnet

    Ok(())
}

fn main() -> Result<()> {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let payer = read_keypair_from_file("../payer-keypair.json");
    let mint_account_key = Keypair::new();

    mint_my_first_token(&client, &payer, &mint_account_key)?;

    Ok(())
}
