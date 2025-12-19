use pinocchio::{entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

const E_MAX_N: u32 = 0xfffffffe;
const MAX_N: u8 = 47;

entrypoint!(process_instruction);

fn fib(n: u8) -> u32 {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 0..n {
        let tmp = a;
        a = b;
        b += tmp;
    }
    a
}

#[cfg_attr(not(target_os = "solana"), allow(unused_variables))]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[pinocchio::account_info::AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let n = instruction_data[0];

    match n {
        0 => Ok(()),
        n if n <= MAX_N => Err(ProgramError::Custom(fib(n))),
        _ => Err(ProgramError::Custom(E_MAX_N)),
    }
}
