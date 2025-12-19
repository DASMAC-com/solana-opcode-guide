use fib_rs::Fib;
use pinocchio::{entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult};

const E_MAX_N: u32 = 0xfffffffe;
const MAX_N: u8 = 47;

entrypoint!(process_instruction);

#[cfg_attr(not(target_os = "solana"), allow(unused_variables))]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[pinocchio::account_info::AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let n = instruction_data[0];

    match n {
        0 => Ok(()),
        n if n <= MAX_N => Err(ProgramError::Custom(
            Fib::single(n.into()).try_into().unwrap(),
        )),
        _ => Err(ProgramError::Custom(E_MAX_N)),
    }
}
