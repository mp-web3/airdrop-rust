# SEND Rust Registration Q1 2025

## Rust Registration Q2 2025
Program ID: `Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv`

---

## Lesson One: Enrollment dApp

### You will:
- Learn how to use `solana-sdk` to create a new keypair
- Airdrop yourself some Solana devnet tokens
- Make Solana transfers on devnet
- Empty your devnet wallet into your Turbin3 wallet
- Use your Turbin3 Private Key to enroll in the Turbin3 enrollment dApp

### Prerequisites:
- Rust and Cargo installed
- A fresh folder created to follow this and future tutorials

---

## 1. Create a new Keypair

### 1.1 Setting Up
```bash
cargo init --lib
```
Edit `Cargo.toml`:
```toml
[dependencies]
solana-sdk = "1.15.2"
```
Create `src/lib.rs` and annotate with:
```rust
#[cfg(test)]
mod tests {
    use solana_sdk;

    #[test]
    fn keygen() {}
    #[test]
    fn airdrop() {}
    #[test]
    fn transfer_sol() {}
}
```

### 1.2 Generating a Keypair
```rust
use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};

#[test]
fn keygen() {
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
    println!("\nTo save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}
```

Run:
```bash
cargo test keygen
```
Then create:
```bash
touch dev-wallet.json
```
Paste in your private key array.

### 1.3 Import/Export to Phantom

Edit `Cargo.toml`:
```toml
bs58 = "..."
```
Add:
```rust
use bs58;
use std::io::{self, BufRead};

#[test]
fn base58_to_wallet() {
    println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
    println!("Input your private key as a wallet file byte array:");
    let stdin = io::stdin();
    let wallet = stdin.lock().lines().next().unwrap().unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("Your private key is:");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}
```

---

## 2. Claim Token Airdrop

Edit `Cargo.toml`:
```toml
solana-client = "1.15.2"
```
```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};

const RPC_URL: &str = "https://api.devnet.solana.com";

#[test]
fn airdrop() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => println!("Success! TX: https://explorer.solana.com/tx/{}?cluster=devnet", s),
        Err(e) => println!("Oops, something went wrong: {}", e)
    }
}
```

---

## 3. Transfer Tokens to Your Turbin3 Address

Add to `Cargo.toml`:
```toml
solana-program = "1.15.2"
```

```rust
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction};
use std::str::FromStr;

fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let pubkey = keypair.pubkey();
    let to_pubkey = Pubkey::from_str("<your Turbin3 public key>").unwrap();

    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&pubkey, &to_pubkey, 1_000_000)],
        Some(&pubkey),
        &[&keypair],
        recent_blockhash
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
    println!("Success! TX: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
}
```

---

## 4. Empty devnet Wallet into Turbin3

Add `Message`:
```rust
use solana_sdk::message::Message;
```

```rust
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
```

---

## 5. Submit Completion of Prerequisites Program

Concepts:
- **PDA**: Deterministic program-generated address
- **IDL**: Defines public interface of Solana program

### 5.1 Consuming IDL in Rust

Edit `Cargo.toml`:
```toml
borsh = "0.10.3"
solana-idlgen = { git = "https://github.com/deanmlittle/solana-idlgen.git" }
```

```bash
mkdir src/programs
touch src/programs/mod.rs src/programs/Turbin3_prereq.rs
```

In `mod.rs`:
```rust
pub mod Turbin3_prereq;
```

In `Turbin3_prereq.rs`:
```rust
use solana_idlgen::idlgen;

idlgen!({
  "version": "0.1.0",
  "name": "Turbin3_prereq",
  "instructions": [{ "name": "complete", ... }],
  "metadata": { "address": "Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv" }
});
```

In `lib.rs`:
```rust
mod programs;
use crate::programs::Turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs};
```

### 5.2 Create a PDA
```rust
let prereq = Turbin3PrereqProgram::derive_program_address(&[b"preQ225", signer.pubkey().to_bytes().as_ref()]);
```

### 5.3 Complete Submission
```rust
let args = CompleteArgs { github: b"testaccount".to_vec() };
let blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");

let transaction = Turbin3PrereqProgram::complete(
    &[&signer.pubkey(), &prereq, &system_program::id()],
    &args,
    Some(&signer.pubkey()),
    &[&signer],
    blockhash
);

let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
println!("Success! TX: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
```

---

## 🎉 Congratulations!
You have completed the Rust registration process for Q2 2025. Time to build!

