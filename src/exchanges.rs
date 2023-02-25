use anchor_lang::prelude::*;

mod helpers;

struct Exchange {
    pub swap_function: fn(fn() -> Result<&'static AccountInfo<'static>>, &[u8]) -> Result<()>,
}

pub fn get_exchange(id: u8) -> Exchange {
    match id {
        0 => Exchange {
            swap_function: helpers::orca_swap,
        },
        1 => Exchange {
            swap_function: helpers::whirlpool_swap,
        },
        _ => panic!(""),
    }
}