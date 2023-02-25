use anchor_lang::prelude::*;
use anchor_lang::prelude::borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    msg,
};
use spl_token_swap::instruction::{Swap, SwapInstruction};

struct OrcaAccounts<'info> {
    pub orca_swap_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub address: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
    pub user_transfer_authority: AccountInfo<'info>,
    pub user_source: AccountInfo<'info>,
    pub pool_source: AccountInfo<'info>,
    pub pool_destination: AccountInfo<'info>,
    pub user_destination: AccountInfo<'info>,
    pub pool_token_mint: AccountInfo<'info>,
    pub fee_account: AccountInfo<'info>,
}

pub fn orca_swap(
    get_next_account: fn() -> Result<&'static AccountInfo<'static>>,
    payload: &[u8],
) -> Result<()> {
    #[derive(BorshDeserialize)]
    pub struct OrcaPayload {
        amount_in: u64,
        minimum_amount_out: u64,
    }

    let args = OrcaPayload::try_from_slice(payload)?;

    // construct instruction data
    let data = SwapInstruction::Swap(Swap {
        amount_in: args.amount_in,
        minimum_amount_out: args.minimum_amount_out,
    });

    let ctx = OrcaAccounts {
        orca_swap_program: get_next_account()?.to_account_info(),
        token_program: get_next_account()?.to_account_info(),
        address: get_next_account()?.to_account_info(),
        authority: get_next_account()?.to_account_info(),
        user_transfer_authority: get_next_account()?.to_account_info(),
        user_source: get_next_account()?.to_account_info(),
        pool_source: get_next_account()?.to_account_info(),
        pool_destination: get_next_account()?.to_account_info(),
        user_destination: get_next_account()?.to_account_info(),
        pool_token_mint: get_next_account()?.to_account_info(),
        fee_account: get_next_account()?.to_account_info(),
    };

    // construct swap instruction
    let swap_ix = Instruction {
        program_id: ctx.orca_swap_program.key(),
        accounts: vec![
            AccountMeta::new_readonly(ctx.address.key(), false),
            AccountMeta::new_readonly(ctx.authority.key(), false),
            AccountMeta::new_readonly(ctx.user_transfer_authority.key(), true),
            AccountMeta::new(ctx.user_source.key(), false),
            AccountMeta::new(ctx.pool_source.key(), false),
            AccountMeta::new(ctx.pool_destination.key(), false),
            AccountMeta::new(ctx.user_destination.key(), false),
            AccountMeta::new(ctx.pool_token_mint.key(), false),
            AccountMeta::new(ctx.fee_account.key(), false),
            AccountMeta::new_readonly(ctx.token_program.key(), false),
        ],
        data: data.pack(),
    };

    // execute CPI
    solana_program::program::invoke(
        &swap_ix,
        &[
            ctx.orca_swap_program.to_account_info(),
            ctx.address.to_account_info(),
            ctx.authority.to_account_info(),
            ctx.user_transfer_authority.to_account_info(),
            ctx.user_source.to_account_info(),
            ctx.pool_source.to_account_info(),
            ctx.pool_destination.to_account_info(),
            ctx.user_destination.to_account_info(),
            ctx.pool_token_mint.to_account_info(),
            ctx.fee_account.to_account_info(),
            ctx.token_program.to_account_info(),
        ],
    )?;

    Ok(())
}

pub fn whirlpool_swap(
    get_next_account: fn() -> Result<&'static AccountInfo<'static>>,
    payload: &[u8],
) -> Result<()> {
    #[derive(BorshDeserialize)]
    pub struct WhirlpoolPayload {
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
    }

    let args = WhirlpoolPayload::try_from_slice(payload)?;

    let cpi_program = get_next_account()?.to_account_info();

    let cpi_accounts = whirlpools::cpi::accounts::Swap {
        whirlpool: get_next_account()?.to_account_info(),
        token_program: get_next_account()?.to_account_info(),
        token_authority: get_next_account()?.to_account_info(),
        token_owner_account_a: get_next_account()?.to_account_info(),
        token_vault_a: get_next_account()?.to_account_info(),
        token_owner_account_b: get_next_account()?.to_account_info(),
        token_vault_b: get_next_account()?.to_account_info(),
        tick_array0: get_next_account()?.to_account_info(),
        tick_array1: get_next_account()?.to_account_info(),
        tick_array2: get_next_account()?.to_account_info(),
        oracle: get_next_account()?.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    // execute CPI
    msg!("CPI: whirlpool swap instruction");
    whirlpools::cpi::swap(
        cpi_ctx,
        args.amount,
        args.other_amount_threshold,
        args.sqrt_price_limit,
        args.amount_specified_is_input,
        args.a_to_b,
    )?;

    Ok(())
}
