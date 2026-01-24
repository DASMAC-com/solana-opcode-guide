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
.equ SIZE_OF_U64_2X, 16 # Size of u64 times 2.

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
# Offset from user account data to PDA lamports.
.equ USER_DATA_TO_PDA_LAMPORTS_OFF, 10320
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

# CreateAccount instruction data.
# -------------------------------
.equ INIT_CPI_N_ACCOUNTS, 2 # Number of accounts for CPI.
.equ INIT_CPI_INSN_DATA_LEN, 52 # Length of instruction data.
.equ INIT_CPI_DISCRIMINATOR, 0 # Discriminator.
.equ INIT_CPI_N_SIGNERS_SEEDS, 1 # Number of signers seeds.
.equ INIT_CPI_ACCT_SIZE, 9 # Account size.

# Stack frame layout for initialize operation.
# --------------------------------------------
# System Program pubkey for CreateAccount CPI.
.equ STK_INIT_SYSTEM_PROGRAM_PUBKEY_OFF, 392
.equ STK_INIT_INSN_OFF, 360 # SolInstruction for CreateAccount CPI.
# Accounts address in SolInstruction.
.equ STK_INIT_INSN_ACCOUNTS_ADDR_OFF, 352
# Accounts length in SolInstruction.
.equ STK_INIT_INSN_ACCOUNTS_LEN_OFF, 344
.equ STK_INIT_INSN_DATA_ADDR_OFF, 336 # Data address in SolInstruction.
.equ STK_INIT_INSN_DATA_LEN_OFF, 328 # Data length in SolInstruction.
# Offset from System Program pubkey to account metas.
.equ STK_INIT_SYSTEM_PROGRAM_PUBKEY_TO_ACCOUNT_METAS_OFF, 72
# Offset from account metas to instruction data.
.equ STK_INIT_ACCOUNT_METAS_TO_INSN_DATA_OFF, 32
.equ STK_INIT_INSN_DATA_OFF, 288 # CreateAccount instruction data.
# Offset of lamports field inside CreateAccount instruction data.
.equ STK_INIT_INSN_DATA_LAMPORTS_OFF, 284
# Offset of space field inside CreateAccount instruction data.
.equ STK_INIT_INSN_DATA_SPACE_OFF, 276
# Offset of owner field inside CreateAccount instruction data.
.equ STK_INIT_INSN_DATA_OWNER_OFF, 268
.equ STK_INIT_ACCT_INFOS_OFF, 232 # User account infos.
# User account meta pubkey address.
.equ STK_INIT_ACCT_META_USER_PUBKEY_ADDR_OFF, 320
# User account meta is_writable.
.equ STK_INIT_ACCT_META_USER_IS_WRITABLE_OFF, 312
# User account meta is_signer.
.equ STK_INIT_ACCT_META_USER_IS_SIGNER_OFF, 311
# PDA account meta pubkey address.
.equ STK_INIT_ACCT_META_PDA_PUBKEY_ADDR_OFF, 304
# PDA account meta is_writable.
.equ STK_INIT_ACCT_META_PDA_IS_WRITABLE_OFF, 296
# PDA account meta is_signer.
.equ STK_INIT_ACCT_META_PDA_IS_SIGNER_OFF, 295
# User account info key address.
.equ STK_INIT_ACCT_INFO_USER_KEY_ADDR_OFF, 232
# PDA account info key address.
.equ STK_INIT_ACCT_INFO_PDA_KEY_ADDR_OFF, 176
# User account info Lamports pointer.
.equ STK_INIT_ACCT_INFO_USER_LAMPORTS_ADDR_OFF, 224
# PDA account info Lamports pointer.
.equ STK_INIT_ACCT_INFO_PDA_LAMPORTS_ADDR_OFF, 168
# User account info owner pubkey pointer.
.equ STK_INIT_ACCT_INFO_USER_OWNER_ADDR_OFF, 200
# PDA account info owner pubkey pointer.
.equ STK_INIT_ACCT_INFO_PDA_OWNER_ADDR_OFF, 144
# User account info data pointer.
.equ STK_INIT_ACCT_INFO_USER_DATA_ADDR_OFF, 208
# PDA account info data pointer.
.equ STK_INIT_ACCT_INFO_PDA_DATA_ADDR_OFF, 152
# User account info is_signer.
.equ STK_INIT_ACCT_INFO_USER_IS_SIGNER_OFF, 184
# User account info is_writable.
.equ STK_INIT_ACCT_INFO_USER_IS_WRITABLE_OFF, 183
# PDA account info is_signer.
.equ STK_INIT_ACCT_INFO_PDA_IS_SIGNER_OFF, 128
# PDA account info is_writable.
.equ STK_INIT_ACCT_INFO_PDA_IS_WRITABLE_OFF, 127
.equ STK_INIT_SEED_0_ADDR_OFF, 120 # Pointer to user pubkey.
.equ STK_INIT_SEED_0_LEN_OFF, 112 # Length of user pubkey.
.equ STK_INIT_SEED_1_ADDR_OFF, 104 # Pointer to bump seed.
.equ STK_INIT_SEED_1_LEN_OFF, 96 # Length of bump seed.
.equ STK_INIT_SIGNERS_SEEDS_OFF, 88 # Pointer to signer seeds array.
.equ STK_INIT_PDA_OFF, 72 # PDA.
.equ STK_INIT_RENT_OFF, 40 # Rent struct return.
.equ STK_INIT_MEMCMP_RESULT_OFF, 16 # Compare result of sol_memcmp.
.equ STK_INIT_BUMP_SEED_OFF, 8 # Bump seed.

# Assorted constants.
# -------------------
.equ NO_OFFSET, 0 # Offset of zero.
.equ SUCCESS, 0 # Indicates successful operation.
.equ BOOL_TRUE, 1 # Boolean true.
# Double wide boolean true for two consecutive fields.
.equ BOOL_TRUE_2X, 0xffff
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
    # As an optimization, store this pointer on the stack in the account
    # info for the PDA, rather than deriving the pointer again later.
    stxdw [r10 - STK_INIT_ACCT_INFO_PDA_KEY_ADDR_OFF], r2
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
    # Store value directly in instruction data on stack.
    stxdw [r10 - STK_INIT_INSN_DATA_LAMPORTS_OFF], r2
    mov64 r1, r9 # Restore input buffer pointer.

    # Populate SolInstruction on stack.
    # ---------------------------------
    mov64 r3, r10 # Get stack frame pointer for stepping through stack.
    # Update to point to zero-initialized System Program pubkey on stack.
    sub64 r3, STK_INIT_SYSTEM_PROGRAM_PUBKEY_OFF
    stxdw [r10 - STK_INIT_INSN_OFF], r3 # Store as CPI program ID.
    # Advance to point to account metas.
    add64 r3, STK_INIT_SYSTEM_PROGRAM_PUBKEY_TO_ACCOUNT_METAS_OFF
    # Store pointer to account metas as CPI account metas address.
    stxdw [r10 - STK_INIT_INSN_ACCOUNTS_ADDR_OFF], r3
    # Store number of CPI accounts (fits in 32-bit immediate).
    stxw [r10 - STK_INIT_INSN_ACCOUNTS_LEN_OFF], INIT_CPI_N_ACCOUNTS
    # Advance to point to instruction data.
    add64 r3, STK_INIT_ACCOUNT_METAS_TO_INSN_DATA_OFF
    stxdw [r10 - STK_INIT_INSN_DATA_ADDR_OFF], r3 # Store CPI data address.
    # Store instruction data length (fits in 32-bit immediate).
    stxw [r10 - STK_INIT_INSN_DATA_LEN_OFF], INIT_CPI_INSN_DATA_LEN

    # Populate CreateAccount instruction data on stack.
    # ---------------------------------------------------------------------
    # - Discriminator is already set to 0 since stack is zero initialized.
    # - Lamports field was already set in the minimum balance calculation.
    # ---------------------------------------------------------------------
    # Store the data length of the account to create (fits in 32 bits).
    stxw [r10 - STK_INIT_INSN_DATA_SPACE_OFF], INIT_CPI_ACCT_SIZE
    add64 r1, PROGRAM_ID_INIT_OFF # Get pointer to program ID.
    # As an optimization, store this pointer on the stack in the account
    # info for the PDA, rather than deriving the pointer again later.
    stxdw [r10 - STK_INIT_ACCT_INFO_PDA_OWNER_ADDR_OFF], r1
    mov64 r2, r10 # Get pointer to stack frame.
    sub64 r2, STK_INIT_INSN_DATA_OWNER_OFF # Point to owner field.
    mov64 r3, SIZE_OF_PUBKEY # Set length of bytes to copy.
    call sol_memcpy_ # Copy program ID into CreateAccount owner field.

    # Flag user and PDA accounts as CPI writable signers.
    # ---------------------------------------------------
    stxh [r10 - STK_INIT_ACCT_META_USER_IS_WRITABLE_OFF], BOOL_TRUE_2X
    stxh [r10 - STK_INIT_ACCT_META_PDA_IS_WRITABLE_OFF], BOOL_TRUE_2X
    stxh [r10 - STK_INIT_ACCT_INFO_USER_IS_SIGNER_OFF], BOOL_TRUE_2X
    stxh [r10 - STK_INIT_ACCT_INFO_PDA_IS_SIGNER_OFF], BOOL_TRUE_2X
    # Optimize out 4 CUs by omitting the following assignments, which are
    # covered by the double wide boolean true assign since is_signer
    # follows is_writable in SolAccountMeta, and is_writable follows
    # is_signer in SolAccountInfo.
    # ```
    # stxb [r10 - STK_INIT_ACCT_META_USER_IS_SIGNER_OFF], BOOL_TRUE
    # stxb [r10 - STK_INIT_ACCT_META_PDA_IS_SIGNER_OFF], BOOL_TRUE
    # stxb [r10 - STK_INIT_ACCT_INFO_USER_IS_WRITABLE_OFF], BOOL_TRUE
    # stxb [r10 - STK_INIT_ACCT_INFO_PDA_IS_WRITABLE_OFF], BOOL_TRUE
    # ```

    # Walk through remaining pointer fields for account metas and infos.
    # ---------------------------------------------------------------------
    # - Rent epoch is ignored since is not needed.
    # - Data length and executable status are ignored since both values are
    #   zero and the stack is zero-initialized.
    # - PDA owner is ignored since it was set above as an optimization
    #   during the CreateAccount instruction population.
    # - PDA pubkey is ignored since it was set above as an optimization
    #   during the PDA computation operation.
    # ---------------------------
    mov64 r2, r9 # Get input buffer pointer.
    add64 r2, USER_PUBKEY_OFF # Update to point at user pubkey.
    # Store in account meta and account info.
    stxdw [r10 - STK_INIT_ACCT_META_USER_PUBKEY_ADDR_OFF], r2
    stxdw [r10 - STK_INIT_ACCT_INFO_USER_KEY_ADDR_OFF], r2
    add64 r2, SIZE_OF_PUBKEY # Advance to point at user owner.
    # Store in account info.
    stxdw [r10 - STK_INIT_ACCT_INFO_USER_OWNER_ADDR_OFF], r2
    add64 r2, SIZE_OF_PUBKEY # Advance to point at user Lamports.
    # Store in account info.
    stxdw [r10 - STK_INIT_ACCT_INFO_USER_LAMPORTS_ADDR_OFF], r2
    add64 r2, SIZE_OF_U64_2X # Advance to point to user account data.
    # Store in account info.
    stxdw [r10 - STK_INIT_ACCT_INFO_USER_DATA_ADDR_OFF], r2

    # Advance to point to PDA Lamports.
    add64 r2, USER_DATA_TO_PDA_LAMPORTS_OFF
    # Store in account info.
    stxdw [r10 - STK_INIT_ACCT_INFO_PDA_LAMPORTS_ADDR_OFF], r2
    add64 r2, SIZE_OF_U64_2X # Advance to point to PDA account data.
    # Store in account info.
    stxdw [r10 - STK_INIT_ACCT_INFO_PDA_DATA_ADDR_OFF], r2

    # Invoke CreateAccount CPI.
    # -------------------------
    mov64 r1, r10 # Get stack frame pointer.
    sub64 r1, STK_INIT_INSN_OFF # Point to instruction.
    mov64 r2, r10 # Get stack frame pointer.
    sub64 r2, STK_INIT_ACCT_INFOS_OFF # Point to account infos.
    mov64 r3, INIT_CPI_N_ACCOUNTS # Indicate number of account infos.
    mov64 r4, INIT_CPI_N_SIGNERS_SEEDS # Indicate a single signer.
    mov64 r5, r10 # Get stack frame pointer.
    sub64 r5, STK_INIT_SIGNERS_SEEDS_OFF # Point to single SignerSeeds.
    call sol_invoke_signed_c

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
