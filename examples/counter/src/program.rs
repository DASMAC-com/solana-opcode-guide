use core::panic;

use pinocchio::{
    entrypoint::{InstructionContext, MaybeAccount},
    lazy_program_entrypoint, no_allocator, nostd_panic_handler, ProgramResult,
};

lazy_program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

const E_N_ACCOUNTS: u32 = 1;
const E_USER_DATA_LEN: u32 = 2;
const E_PDA_DATA_LEN: u32 = 3;
const E_SYSTEM_PROGRAM_DATA_LEN: u32 = 4;
const E_PDA_DUPLICATE: u32 = 5;
const E_SYSTEM_PROGRAM_DUPLICATE: u32 = 6;

const N_ACCOUNTS_INCREMENT: u64 = 2;
const N_ACCOUNTS_INITIALIZE: u64 = 3;
const SYSTEM_PROGRAM_DATA_LEN: usize = b"system_program".len();

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    match context.remaining() {
        N_ACCOUNTS_INCREMENT => {}
        N_ACCOUNTS_INITIALIZE => {
            // Safe because number of accounts has been checked.
            let user = unsafe { context.next_account_unchecked().assume_account() };
            if !user.is_data_empty() {
                return Err(pinocchio::error::ProgramError::Custom(E_USER_DATA_LEN));
            }

            // Safe because number of accounts has been checked.
            let pda = match unsafe { context.next_account_unchecked() } {
                MaybeAccount::Account(account) => account,
                MaybeAccount::Duplicated(_) => {
                    return Err(pinocchio::error::ProgramError::Custom(E_PDA_DUPLICATE))
                }
            };
            if !pda.is_data_empty() {
                return Err(pinocchio::error::ProgramError::Custom(E_PDA_DATA_LEN));
            }

            // Safe because number of accounts has been checked.
            let system_program = match unsafe { context.next_account_unchecked() } {
                MaybeAccount::Account(account) => account,
                MaybeAccount::Duplicated(_) => {
                    return Err(pinocchio::error::ProgramError::Custom(
                        E_SYSTEM_PROGRAM_DUPLICATE,
                    ))
                }
            };
            if system_program.data_len() != SYSTEM_PROGRAM_DATA_LEN {
                return Err(pinocchio::error::ProgramError::Custom(
                    E_SYSTEM_PROGRAM_DATA_LEN,
                ));
            }
        }
        _ => return Err(pinocchio::error::ProgramError::Custom(E_N_ACCOUNTS)),
    }
    Ok(())
}
