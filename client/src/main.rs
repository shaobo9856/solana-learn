use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    commitment_config::CommitmentConfig,
    instruction::{Instruction, AccountMeta},
};
use std::fs::File;
use std::io::Read;
use anyhow::{Result, Context,anyhow};
use bs58;

const NAME_LEN: usize = 32;

fn load_keypair(path: &str) -> Result<Keypair> {
    let mut file = File::open(path).context("Cant open keypair file")?;
    let mut data = String::new();
    file.read_to_string(&mut data).context("Cant read keypair file")?;
    let bytes: Vec<u8> = serde_json::from_str(&data).context("invalid keypair file")?;
    Keypair::from_bytes(&bytes).context("invalid keypair file")
}

fn main() -> Result<()> {
    let rpc_url = "https://api.testnet.solana.com";
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let keypair_path = "/Users/shao9856/.config/solana/id.json";
    
    let payer = load_keypair(keypair_path)?;
    let program_id = "5NMc41R2sqrSq43Swde51o159B7aCEeMaLzZfJvnVTu9"
        .parse::<Pubkey>()
        .context("Invalid program ID")?;

    let account_to_write_to = Keypair::new();

    let first_name = "Bob";
    let last_name = "Smith";

    let mut name_data = vec![0u8; NAME_LEN * 2];
    name_data[..first_name.len()].copy_from_slice(first_name.as_bytes());
    name_data[NAME_LEN..NAME_LEN + last_name.len()].copy_from_slice(last_name.as_bytes());

    // Create account to store name data
    let rent = client.get_minimum_balance_for_rent_exemption(NAME_LEN * 2)?;
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &account_to_write_to.pubkey(),
        rent,
        (NAME_LEN * 2) as u64,
        &program_id,
    );

    let instruction = Instruction::new_with_bytes(
        program_id,
        &name_data, 
        vec![
            AccountMeta::new(account_to_write_to.pubkey(), false),
            AccountMeta::new_readonly(payer.pubkey(), true),
        ],
    );

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_ix, instruction],
        Some(&payer.pubkey()),
        &[&payer, &account_to_write_to],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction signature: {}", signature);

    // Read account data to verify name storage
    let account_data = client.get_account_data(&account_to_write_to.pubkey())?;
    let first_name = String::from_utf8_lossy(&account_data[..NAME_LEN]).trim_end_matches('\0').to_string();
    let last_name = String::from_utf8_lossy(&account_data[NAME_LEN..]).trim_end_matches('\0').to_string();
    
    println!("Name: {} {}", first_name, last_name);

    Ok(())
}