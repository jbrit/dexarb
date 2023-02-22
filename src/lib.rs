use solana_program::{
    account_info::AccountInfo,
    account_info::next_account_info,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let get_next_data = || instruction_data.iter().next().unwrap();
    let steps = get_next_data();
    match steps {
        // not starting from zero, so programmer doesn't think of steps as an index
        1 => {
            single_step_swap(program_id, accounts, get_next_data())
        }
        2 => {
            two_step_swap(program_id, accounts, get_next_data(), get_next_data())
        }
        3 => {
            three_step_swap(program_id, accounts, get_next_data(), get_next_data(), get_next_data())
        }
        _ => {
            panic!("Invalid number of steps")
        }
    }
}

fn single_step_swap(program_id: &Pubkey, accounts: &[AccountInfo], exchange_id: &u8) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mut get_next_account = || next_account_info(accounts_iter);
    let account = get_next_account()?;
    // make checks on all the accounts based on exchange_id and initiates swap
    Ok(())
}
fn two_step_swap(program_id: &Pubkey, accounts: &[AccountInfo], exchange_a_id: &u8, exchange_b_id: &u8) -> ProgramResult {
    Ok(())
}
fn three_step_swap(program_id: &Pubkey, accounts: &[AccountInfo], exchange_a_id: &u8, exchange_b_id: &u8, exchange_c_id: &u8) -> ProgramResult {
    Ok(())
}