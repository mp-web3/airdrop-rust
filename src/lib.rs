#[cfg(test)]
mod tests {
    use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("\nTo save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }
}
