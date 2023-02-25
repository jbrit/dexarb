use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint,
    entrypoint::ProgramResult, pubkey::Pubkey,
};

mod exchanges;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let steps = instruction_data[0];
    let ex_a = instruction_data[1];
    let ex_b = instruction_data[2];
    let ex_c = instruction_data[3];
    match steps {
        // not starting from zero, so programmer doesn't think of steps as an index
        1 => single_step_swap(program_id, accounts, instruction_data, ex_a),
        2 => two_step_swap(program_id, accounts, instruction_data, ex_a, ex_b),
        3 => three_step_swap(program_id, accounts, instruction_data, ex_a, ex_b, ex_c),
        _ => {
            panic!("Invalid number of steps")
        }
    }
}

fn single_step_swap(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
    exchange_id: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let mut get_next_account = || next_account_info(accounts_iter);
    let exchange = exchanges::get_exchange(exchange_id);
    (exchange.swap_function)(get_next_account, &instruction_data[4..]);
    Ok(())
}
fn two_step_swap(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
    _exchange_a_id: u8,
    _exchange_b_id: u8,
) -> ProgramResult {
    Ok(())
}
fn three_step_swap(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
    _exchange_a_id: u8,
    _exchange_b_id: u8,
    _exchange_c_id: u8,
) -> ProgramResult {
    Ok(())
}
