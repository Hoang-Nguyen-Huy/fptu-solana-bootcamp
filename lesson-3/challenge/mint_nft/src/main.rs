use mpl_token_metadata::instructions as metadata_instruction;
use mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs;
use mpl_token_metadata::types::DataV2;
use mpl_token_metadata::ID as metadata_program_id;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;
use solana_sdk::transaction::Transaction;
use solana_sdk::{program_pack::Pack, signature::Keypair, signer::Signer, system_instruction};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::{self as token_instruction, AuthorityType};
use spl_associated_token_account::instruction;
use spl_token::state::Mint;
use spl_token::ID as token_program_id;

use anyhow::Result;
use std::fs::File;

const NFT_ROYALTY: u16 = 1000;

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
    nft_mint_account_key: &Keypair,
) -> Result<()> {
    let (metadata_account_address, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            &metadata_program_id.to_bytes(),
            &nft_mint_account_key.pubkey().to_bytes(),
        ],
        &metadata_program_id
    );

    let associated_token_account_address = get_associated_token_address(&payer.pubkey(), &nft_mint_account_key.pubkey());

    let rent = client.get_minimum_balance_for_rent_exemption(Mint::LEN)?;
    let create_nft_mint_account = system_instruction::create_account(
        &payer.pubkey(),
        &nft_mint_account_key.pubkey(),
        rent,
        Mint::LEN as u64,
        &token_program_id,
    );

    let init_nft_mint_account = token_instruction::initialize_mint(
        &token_program_id,
        &nft_mint_account_key.pubkey(),
        &payer.pubkey(),
        Some(&nft_mint_account_key.pubkey()),
        0,
    );

    let create_metadata_account = metadata_instruction::CreateMetadataAccountV3 {
        metadata: metadata_account_address,
        mint: nft_mint_account_key.pubkey(),
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
            seller_fee_basis_points: NFT_ROYALTY,
            creators: None,
            collection: None, 
            uses: None,
        },
        is_mutable: true, 
        collection_details: None, 
    });

    let create_nft = instruction::create_associated_token_account(
        &payer.pubkey(),
        &payer.pubkey(),
        &nft_mint_account_key.pubkey(),
        &token_program_id,
    );

    let init_nft = token_instruction::mint_to(
        &token_program_id,
        &nft_mint_account_key.pubkey(),
        &associated_token_account_address,
        &payer.pubkey(),
        &[&payer.pubkey()],
        1,
    );

    let remove_mint_authority = token_instruction::set_authority(
        &token_program_id,
        &nft_mint_account_key.pubkey(),
        None,
        AuthorityType::MintTokens,
        &payer.pubkey(),
        &[&payer.pubkey(), &nft_mint_account_key.pubkey()],
    );

    let mut instructions = Vec::with_capacity(5);
    instructions.push(create_nft_mint_account);
    instructions.push(init_nft_mint_account?);
    instructions.push(create_metadata_account);

    if client.get_account(&associated_token_account_address).is_err() {
        instructions.push(create_nft);
    }
    instructions.push(init_nft?);
    instructions.push(remove_mint_authority?);

    let create_blockhash = client.get_latest_blockhash()?;
    let create_transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[&payer, &nft_mint_account_key],
        create_blockhash,
    );

    let create_sig = client.send_and_confirm_transaction(&create_transaction)?;

    let explorer_url = util::get_signature_explorer_url(&create_sig.to_string());

    println!("explorer url: {}", explorer_url);

//  https://explorer.solana.com/tx/5zjxnYRbE8bV3zZ59Xfdf6jwnKRfpgk68pgyseNw3jbPz7UwXvC4En3zznjRaVREPs4SYo8XUtU4ZUVuLLiQENfR?cluster=devnet

    Ok(())
}

fn main() -> Result<()> {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let payer = read_keypair_from_file("../payer-keypair.json");
    let nft_mint_account_key = Keypair::new();

    mint_my_first_token(&client, &payer, &nft_mint_account_key)?;

    Ok(())
}
