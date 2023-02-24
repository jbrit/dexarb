mod helpers;

struct Exchange {
    swap_function: fn(FnMut, &[u8]) -> Result<()>,
}

pub fn get_exchange<NextAccount>(id: u8) -> Exchange {
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
