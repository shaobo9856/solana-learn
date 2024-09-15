use solana_program::{
    account_info::{next_account_info, AccountInfo},
    pubkey::Pubkey,
    entrypoint,
    msg,
    program_error::ProgramError,
    entrypoint::ProgramResult,
};

// name length
const NAME_LEN: usize = 32;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.data_len() < NAME_LEN * 2 {
        return Err(ProgramError::AccountDataTooSmall);
    }

    let (first_name_data, last_name_data) = instruction_data.split_at(NAME_LEN);
    let first_name = std::str::from_utf8(first_name_data).map_err(|_| ProgramError::InvalidInstructionData)?;
    let last_name = std::str::from_utf8(last_name_data).map_err(|_| ProgramError::InvalidInstructionData)?;

    let full_name = format!("{} {}", first_name.trim_end_matches('\0'), last_name.trim_end_matches('\0'));
    msg!("user data: {}", full_name);

    let mut data = account.try_borrow_mut_data()?;
    data[..full_name.len()].copy_from_slice(full_name.as_bytes());

    Ok(())
}
