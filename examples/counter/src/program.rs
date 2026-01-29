use core::mem::transmute;
use pinocchio::{
    address::address_eq,
    cpi::{invoke_signed_unchecked, Seed, Signer},
    entrypoint::{InstructionContext, MaybeAccount},
    instruction::{InstructionAccount, InstructionView},
    lazy_program_entrypoint, no_allocator, nostd_panic_handler,
    sysvars::{
        rent::{Rent, ACCOUNT_STORAGE_OVERHEAD},
        Sysvar,
    },
    Address, ProgramResult,
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
const E_PDA_MISMATCH: u32 = 8;

const N_ACCOUNTS_INCREMENT: u64 = 2;
const N_ACCOUNTS_INITIALIZE: u64 = 3;
const SYSTEM_PROGRAM_DATA_LEN: usize = b"system_program".len();

#[repr(C, packed)]
struct PdaAccountData {
    counter: u64,
    bump: u8,
}

#[repr(C, packed)]
struct CreateAccountInstructionData {
    instruction_tag: u32,
    lamports: u64,
    space: u64,
    owner: Address,
}

pub fn process_instruction(mut context: InstructionContext) -> ProgramResult {
    match context.remaining() {
        N_ACCOUNTS_INCREMENT => {}
        N_ACCOUNTS_INITIALIZE => {
            // SAFETY: number of accounts has been checked.
            let user = unsafe { context.next_account_unchecked().assume_account() };
            if !user.is_data_empty() {
                return Err(pinocchio::error::ProgramError::Custom(E_USER_DATA_LEN));
            }

            // SAFETY: number of accounts has been checked.
            let pda = match unsafe { context.next_account_unchecked() } {
                MaybeAccount::Account(account) => account,
                MaybeAccount::Duplicated(_) => {
                    return Err(pinocchio::error::ProgramError::Custom(E_PDA_DUPLICATE))
                }
            };
            if !pda.is_data_empty() {
                return Err(pinocchio::error::ProgramError::Custom(E_PDA_DATA_LEN));
            }

            // SAFETY: number of accounts has been checked.
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

            // Prepare PDA seeds, check address.
            // SAFETY: known number of accounts have been read.
            let program_id = unsafe { context.program_id_unchecked() };
            let user_ref = user.address().as_array();
            let user_pubkey_seed = Seed::from(user_ref);
            let (expected_pda, bump) =
                Address::find_program_address(&[&user_pubkey_seed], program_id);
            if !address_eq(pda.address(), &expected_pda) {
                return Err(pinocchio::error::ProgramError::Custom(E_PDA_MISMATCH));
            }

            // Calculate lamports from rent sysvar (matches assembly behavior).
            // SAFETY: Rent sysvar has no return code.
            let rent = Rent::get().unwrap();
            // SAFETY: Rent is #[repr(C)] with lamports_per_byte as first field (u64).
            let lamports_per_byte = unsafe { *(&rent as *const Rent as *const u64) };
            let lamports =
                (size_of::<PdaAccountData>() as u64 + ACCOUNT_STORAGE_OVERHEAD) * lamports_per_byte;

            let instruction_data = CreateAccountInstructionData {
                instruction_tag: 0,
                lamports,
                space: size_of::<PdaAccountData>() as u64,
                owner: program_id.clone(),
            };
            // SAFETY: Sizes have been validated a priori.
            unsafe {
                invoke_signed_unchecked(
                    &InstructionView {
                        program_id: &pinocchio_system::ID,
                        accounts: &[
                            InstructionAccount::writable_signer(user.address()),
                            InstructionAccount::writable_signer(pda.address()),
                        ],
                        data: transmute::<_, &[u8; size_of::<CreateAccountInstructionData>()]>(
                            &instruction_data,
                        ),
                    },
                    &[(&user).into(), (&pda).into()],
                    &[Signer::from(&[user_pubkey_seed, Seed::from(&[bump])])],
                );
            }
            // Write bump seed to PDA data.
            // SAFETY: PDA account was just created with sufficient space.
            let pda_data_ptr = pda.data_ptr() as *mut PdaAccountData;
            unsafe {
                (*pda_data_ptr).bump = bump;
            }
        }
        _ => return Err(pinocchio::error::ProgramError::Custom(E_N_ACCOUNTS)),
    }
    Ok(())
}
