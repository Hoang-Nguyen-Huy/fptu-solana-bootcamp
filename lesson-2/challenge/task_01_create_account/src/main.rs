use solana_client::rpc_client::RpcClient; // Thư viện cung cấp các phương thức để tương tác vs mạng lưới Solana qua RPC (Remote Procedure Call)
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    system_program,
    transaction::Transaction,
}; // Thư viện cung cấp các công cụ và cấu trúc cần thiết để tương tác với hệ thống Solana, bao gồm các chữ ký số, hướng dẫn hệ thống và giao dịch

use anyhow::Result; // Thư viện này cung cấp cách dễ dàng để xử lý lỗi
use std::fs::File;  // Thư viện này được sử dụng để thao tác với các tệp tin

mod util {
    pub fn get_signature_explorer_url(signature: &str) -> String {
        format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature)
    } // Tạo ra một URL để truy cập vào trang explorer của Solana --> để xem chi tiết giao dịch bằng cách cung cấp chữ ký của giao dịch
}

fn read_keypair_from_file(filepath: &str) -> Keypair {
    let file = File::open(filepath).expect("Unable to open keypair file"); // Mở tệp tin từ đường dẫn cung cấp
    let keypair: Vec<u8> = serde_json::from_reader(file).expect("Unable to parse keypair file"); // Chuyển đổi nội dung tệp JSON thành một vector các byte
    Keypair::from_bytes(&keypair).expect("Unable to create keypair from bytes") // Tạo một 'Keypair' từ các byte đã đọc
} // Đọc 1 tệp JSON chứa thông tin về keypair và chuyển đổi nó thành một 'Keypair' của Solana

fn create_account(
    client: &RpcClient,
    payer: &Keypair,
    new_account: &Keypair,
    space: u64,
) -> Result<()> {
    let rent = client.get_minimum_balance_for_rent_exemption(space.try_into()?)?; // Lấy số SOL tối thiểu cần thiết để tài khoản không bị xóa do không đủ tiền thuê
    let create_instr = system_instruction::create_account(
        &payer.pubkey(),
        &new_account.pubkey(),
        rent,
        space as u64,
        &system_program::id(),
    ); // create instruction

    let create_blockhash = client.get_latest_blockhash()?; // Lấy blockhash mới nhất để sử dụng trong giao dịch
    let create_tx = Transaction::new_signed_with_payer(
        &[create_instr],
        Some(&payer.pubkey()),
        &[&payer, &new_account],
        create_blockhash,
    ); // create transaction and sign with payer

    let create_sig = client.send_and_confirm_transaction(&create_tx)?; // Gửi và xác nhận giao dịch trên mạng lưới Solana

    let explorer_url = util::get_signature_explorer_url(&create_sig.to_string()); // Lấy URL của giao dịch trên Solana Explorer
    println!("explorer url: {}", explorer_url);
//  https://explorer.solana.com/tx/5iiuLMQ1DNq9menSQKtFRSrGpRU5GJYLrd4QpnsgASCDb1xyyMsTF3YdRxyqWh4f51x1JiCJyw7WjFAPpbd3dnz?cluster=devnet

    Ok(())
} // Tạo 1 tài khoản mới trên mạng lưới Solana

fn main() -> Result<()> {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string()); // Tạo một kết nối RPC tới mạng lưới Solana Devnet

    let payer = read_keypair_from_file("payer-keypair.json"); // Đọc keypair của người thanh toán từ tệp JSON
    let new_account = Keypair::new(); // Tạo một keypair mới cho tài khoản
    const ACCOUNT_SPACE: u64 = 0;

    create_account(&client, &payer, &new_account, ACCOUNT_SPACE)?; // Tạo tài khoản mới vs người thanh toán là payer và tài khoản mới là new_account

    println!("New account created: {}", new_account.pubkey());

    Ok(())
}