use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};

const E_MAX_N: u32 = 0xfffffffe;
const MAX_N: u8 = 47;
const MAX_N_SPECIAL_CASE: u8 = 1;

entrypoint!(process_instruction);

#[cfg_attr(not(target_os = "solana"), allow(unused_variables))]
fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let n = instruction_data[0];

    match n {
        0 => Ok(()),
        MAX_N_SPECIAL_CASE => Err(ProgramError::Custom(MAX_N_SPECIAL_CASE as u32)),
        n if n > MAX_N => Err(ProgramError::Custom(E_MAX_N)),
        _ => Err(ProgramError::Custom(fib(n as u64))),
    }
}

// If r8 is a u8 the compiler generates an extra opcode to cast it to u8.
fn fib(mut r8: u64) -> u32 {
    let mut r6: u32 = 0;
    let mut r7: u32 = 1;
    loop {
        let r9 = r6;
        r6 = r7;
        unsafe {
            r7 = r7.unchecked_add(r9);
            r8 = r8.unchecked_sub(1);
        };
        if r8 == 1 {
            return r7;
        };
    }
}
