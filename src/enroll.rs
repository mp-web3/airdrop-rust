use crate::programs::Turbin3_prereq::{derive_prereq_address, create_submit_instruction, PROGRAM_ID};
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use std::env;
use dotenv::dotenv;

pub fn enroll() {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let keypair = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");
    
    let rpc_client = RpcClient::new(&rpc_url);
    
    // Print current balance
    match rpc_client.get_balance(&keypair.pubkey()) {
        Ok(balance) => println!("Wallet balance: {} SOL", balance as f64 / 1_000_000_000.0),
        Err(e) => println!("Error getting balance: {}", e),
    }
    
    println!("Using program: {}", PROGRAM_ID);
    
    // Derive the PDA for the enrollment
    let (prereq, _) = derive_prereq_address(&keypair.pubkey());
    println!("Using PDA: {}", prereq);
    
    // Check if the PDA already exists
    match rpc_client.get_account_data(&prereq) {
        Ok(_) => {
            println!("You've already enrolled! The PDA account already exists.");
            println!("Your enrollment is confirmed!");
            println!("Success! You can view your registration on devnet: https://explorer.solana.com/address/{}?cluster=devnet", prereq);
            return;
        },
        Err(_) => println!("PDA account not found. Proceeding with enrollment..."),
    }
    
    // Create the instruction
    let instruction = create_submit_instruction(
        &keypair.pubkey(),
        &prereq,
        b"mp-web3".to_vec()
    );
    
    // Create and send the transaction
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash
    );
    
    // First send the transaction without confirming to see if there are any errors
    match rpc_client.send_transaction(&transaction) {
        Ok(signature) => {
            println!("Transaction sent! Signature: {}", signature);
            
            // Now confirm the transaction
            match rpc_client.confirm_transaction(&signature) {
                Ok(_) => println!("Success! TX: https://explorer.solana.com/tx/{}/?cluster=devnet", signature),
                Err(e) => println!("Error confirming transaction: {:?}", e),
            }
        },
        Err(e) => println!("Error sending transaction: {:?}", e),
    }
}