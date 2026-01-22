# Error codes.
# ------------
.equ E_N_ACCOUNTS, 1 # Invalid number of accounts.
.equ E_USER_DATA_LEN, 2 # User data length is nonzero.
.equ E_PDA_DATA_LEN, 3 # PDA data length is nonzero.
.equ E_SYSTEM_PROGRAM_DATA_LEN, 4 # System Program data length is nonzero.
.equ E_PDA_DUPLICATE, 5 # PDA is a duplicate account.
.equ E_SYSTEM_PROGRAM_DUPLICATE, 6 # System Program is a duplicate account.
.equ E_UNABLE_TO_DERIVE_PDA, 7 # Unable to derive PDA.
.equ E_PDA_MISMATCH, 8 # Passed PDA does not match computed PDA.

# Size of assorted types.
# -----------------------
.equ SIZE_OF_PUBKEY, 32 # Size of Pubkey.
.equ SIZE_OF_U8, 1 # Size of u8.

# Memory map layout.
# ------------------
.equ NON_DUP_MARKER, 0xff # Flag that an account is not a duplicate.
.equ DATA_LEN_ZERO, 0 # Data length of zero.
.equ DATA_LEN_SYSTEM_PROGRAM, 14 # Data length of System Program.
.equ N_ACCOUNTS_INCREMENT, 2 # Number of accounts for increment operation.
.equ N_ACCOUNTS_INIT, 3 # Number of accounts for initialize operation.
.equ N_ACCOUNTS_OFF, 0 # Number of accounts in virtual memory map.
.equ USER_DATA_LEN_OFF, 88 # User data length.
.equ USER_PUBKEY_OFF, 16 # User pubkey.
.equ PDA_NON_DUP_MARKER_OFF, 10344 # PDA non-duplicate marker.
.equ PDA_PUBKEY_OFF, 10352 # PDA pubkey.
.equ PDA_DATA_LEN_OFF, 10424 # PDA data length.
# PDA account data length plus account overhead.
.equ PDA_DATA_WITH_ACCOUNT_OVERHEAD, 137
.equ PDA_BUMP_SEED_OFF, 10440 # PDA bump seed.
# System Program non-duplicate marker.
.equ SYSTEM_PROGRAM_NON_DUP_MARKER_OFF, 20680
.equ SYSTEM_PROGRAM_DATA_LEN_OFF, 20760 # System program data length.
.equ PROGRAM_ID_INIT_OFF, 31040 # Program ID during initialize operation.

# Stack frame layout for initialize operation.
# --------------------------------------------
.equ STK_INIT_INSN_OFF, 360 # SolInstruction for CreateAccount CPI.
# Offset of lamports field inside CreateAccount instruction data.
.equ STK_INIT_INSN_DATA_LAMPORTS_OFF, 316
.equ STK_INIT_SEED_0_ADDR_OFF, 120 # Pointer to user pubkey.
.equ STK_INIT_SEED_0_LEN_OFF, 112 # Length of user pubkey.
.equ STK_INIT_SEED_1_ADDR_OFF, 104 # Pointer to bump seed.
.equ STK_INIT_SEED_1_LEN_OFF, 96 # Length of bump seed.
.equ STK_INIT_PDA_OFF, 72 # PDA.
.equ STK_INIT_RENT_OFF, 40 # Rent struct return.
.equ STK_INIT_MEMCMP_RESULT_OFF, 16 # Compare result of sol_memcmp.
.equ STK_INIT_BUMP_SEED_OFF, 8 # Bump seed.

# Assorted constants.
# -------------------
.equ NO_OFFSET, 0 # Offset of zero.
.equ SUCCESS, 0 # Indicates successful operation.
.equ COMPARE_EQUAL, 0 # Compare result indicating equality.

.global entrypoint

entrypoint:
    ldxdw r2, [r1 + N_ACCOUNTS_OFF] # Get n accounts from input buffer.
    jeq r2, N_ACCOUNTS_INCREMENT, increment # Fast path to cheap operation.
    jeq r2, N_ACCOUNTS_INIT, initialize # Low priority, expensive anyways.
    mov64 r0, E_N_ACCOUNTS # Else fail.
    exit

initialize:

    # Check input memory map.
    # -----------------------
    ldxdw r2, [r1 + USER_DATA_LEN_OFF] # Get user data length.
    jne r2, DATA_LEN_ZERO, e_user_data_len # Exit if user account has data.
    ldxb r2, [r1 + PDA_NON_DUP_MARKER_OFF] # Check if PDA is a duplicate.
    jne r2, NON_DUP_MARKER, e_pda_duplicate # Exit if PDA is a duplicate.
    ldxdw r2, [r1 + PDA_DATA_LEN_OFF] # Get PDA data length.
    jne r2, DATA_LEN_ZERO, e_pda_data_len # Exit if PDA account has data.
    # Exit early if System Program is a duplicate.
    ldxb r2, [r1 + SYSTEM_PROGRAM_NON_DUP_MARKER_OFF]
    jne r2, NON_DUP_MARKER, e_system_program_duplicate
    # Exit early if System Program data length is invalid.
    ldxdw r2, [r1 + SYSTEM_PROGRAM_DATA_LEN_OFF]
    jne r2, DATA_LEN_SYSTEM_PROGRAM, e_system_program_data_len

    # Initialize signer seed for user pubkey.
    # ---------------------------------------
    mov64 r2, r1 # Get input buffer pointer.
    add64 r2, USER_PUBKEY_OFF # Update pointer to point at user pubkey.
    # Store pointer in seed 0 pointer field.
    stxdw [r10 - STK_INIT_SEED_0_ADDR_OFF], r2
    mov64 r2, SIZE_OF_PUBKEY # Store length of pubkey.
    # Store length in seed 0 length field.
    stxdw [r10 - STK_INIT_SEED_0_LEN_OFF], r2

    # Initialize signer seed for PDA bump key.
    # ----------------------------------------
    mov64 r2, r10 # Get stack frame pointer.
    sub64 r2, STK_INIT_BUMP_SEED_OFF # Update to point at PDA bump seed.
    # Store pointer in seed 1 pointer field.
    stxdw [r10 - STK_INIT_SEED_1_ADDR_OFF], r2
    mov64 r2, SIZE_OF_U8 # Store length of bump seed.
    # Store length in seed 1 length field.
    stxdw [r10 - STK_INIT_SEED_1_LEN_OFF], r2

    # Compute PDA.
    # ------------
    mov64 r9, r1 # Store input buffer pointer for later.
    mov64 r1, r10 # Get stack frame pointer.
    # Update to point at user pubkey signer seed.
    sub64 r1, STK_INIT_SEED_0_ADDR_OFF
    mov64 r2, 1 # Indicate single signer seed (user pubkey).
    mov64 r3, r9 # Get input buffer pointer.
    add64 r3, PROGRAM_ID_INIT_OFF # Update to point at program ID.
    mov64 r4, r10 # Get stack frame pointer.
    sub64 r4, STK_INIT_PDA_OFF # Update to point to PDA region on stack.
    mov64 r5, r10 # Get stack frame pointer.
    sub64 r5, STK_INIT_BUMP_SEED_OFF # Update to point to bump seed region.
    call sol_try_find_program_address # Find PDA.
    # Skip check to error out if unable to derive a PDA (failure to derive
    # is practically impossible to test since odds of not finding bump seed
    # are astronomically low):
    # ```
    # jne r0, SUCCESS, e_unable_to_derive_pda
    # ```
    mov64 r1, r9 # Restore input buffer pointer.

    # Compare computed PDA against passed account.
    # --------------------------------------------
    # Update input buffer pointer to point to passed PDA.
    add64 r1, PDA_PUBKEY_OFF
    mov64 r2, r10 # Get stack frame pointer.
    sub64 r2, STK_INIT_PDA_OFF # Update to point to computed PDA.
    mov64 r3, SIZE_OF_PUBKEY # Flag size of bytes to compare.
    mov64 r4, r10 # Get stack frame pointer.
    sub64 r4, STK_INIT_MEMCMP_RESULT_OFF # Update to point to result.
    call sol_memcmp_
    ldxw r2, [r4 + NO_OFFSET] # Get compare result.
    jne r2, COMPARE_EQUAL, e_pda_mismatch # Error out if PDA mismatch.
    # Skip input buffer restoration since next block overwrites r1:
    # ```
    # mov64 r1, r9 # Restore input buffer pointer.
    # ```

    # Calculate Lamports required for new account.
    # --------------------------------------------
    mov64 r1, r10 # Get stack frame pointer.
    sub64 r1, STK_INIT_RENT_OFF # Update to point to Rent struct.
    call sol_get_rent_sysvar # Get Rent struct.
    ldxdw r2, [r1 + NO_OFFSET] # Get Lamports per byte field.
    # Multiply by sum of PDA account data length, account storage overhead.
    mul64 r2, PDA_DATA_WITH_ACCOUNT_OVERHEAD
    mov64 r0, r2

    exit

increment:
    exit

e_user_data_len:
    mov32 r0, E_USER_DATA_LEN
    exit

e_pda_data_len:
    mov32 r0, E_PDA_DATA_LEN
    exit

e_system_program_data_len:
    mov32 r0, E_SYSTEM_PROGRAM_DATA_LEN
    exit

e_pda_duplicate:
    mov32 r0, E_PDA_DUPLICATE
    exit

e_system_program_duplicate:
    mov32 r0, E_SYSTEM_PROGRAM_DUPLICATE
    exit

e_pda_mismatch:
    mov32 r0, E_PDA_MISMATCH
    exit
