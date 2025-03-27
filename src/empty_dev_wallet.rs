use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction, message::Message};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;
use std::env;
use dotenv::dotenv;

pub fn empty_dev_wallet() {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("RPC_URL environment variable not set");
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let pubkey = keypair.pubkey();
    let to_pubkey = Pubkey::from_str(&env::var("TURBIN3_WALLET_ADDRESS").expect("TURBIN3_WALLET_ADDRESS environment variable not set")).unwrap();

    let rpc_client = RpcClient::new(&rpc_url);
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");

    let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
    let message = Message::new_with_blockhash(
        &[transfer(&pubkey, &to_pubkey, balance)],
        Some(&pubkey),
        &recent_blockhash
    );
    let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&pubkey, &to_pubkey, balance - fee)],
        Some(&pubkey),
        &[&keypair],
        recent_blockhash
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
    println!("Success! TX: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}