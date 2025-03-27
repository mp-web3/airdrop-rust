use solana_sdk::{
    pubkey::Pubkey,
    instruction::{AccountMeta, Instruction},
    signer::Signer,
};
use std::str::FromStr;

// Program ID from the URL
pub const PROGRAM_ID: &str = "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa";

// 'complete' instruction discriminator from the new IDL
const COMPLETE_DISCRIMINATOR: [u8; 8] = [0, 77, 224, 147, 136, 25, 88, 76];

pub fn get_program_id() -> Pubkey {
    Pubkey::from_str(PROGRAM_ID).unwrap()
}

pub fn derive_prereq_address(signer: &Pubkey) -> (Pubkey, u8) {
    // Updated seed to "prereq" based on the new IDL
    Pubkey::find_program_address(&[b"prereq", signer.as_ref()], &get_program_id())
}

pub fn create_submit_instruction(
    signer: &Pubkey,
    prereq: &Pubkey,
    github_username: Vec<u8>,
) -> Instruction {
    // Create the instruction data
    let mut instruction_data = Vec::with_capacity(4 + github_username.len() + 8);
    
    // Add discriminator for 'complete' instruction
    instruction_data.extend_from_slice(&COMPLETE_DISCRIMINATOR);
    
    // Add github username length and data
    let len = github_username.len() as u32;
    instruction_data.extend_from_slice(&len.to_le_bytes());
    instruction_data.extend_from_slice(&github_username);
    
    // Create the instruction with accounts
    Instruction {
        program_id: get_program_id(),
        accounts: vec![
            AccountMeta::new(*signer, true),  // signer (writable, signer)
            AccountMeta::new(*prereq, false), // prereq (writable, not signer)
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false), // system_program
        ],
        data: instruction_data,
    }
}