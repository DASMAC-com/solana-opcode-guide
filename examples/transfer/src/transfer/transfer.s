.equ N_ACCOUNTS_OFFSET, 0
.equ N_ACCOUNTS_EXPECTED, 3
.equ E_N_ACCOUNTS, 0

.global entrypoint

entrypoint:
    # Check number of accounts.
    ldxdw r2, [r1 + N_ACCOUNTS_OFFSET]
    jne r2, N_ACCOUNTS_EXPECTED, e_n_accounts

    exit

e_n_accounts:
    mov32 r0, E_N_ACCOUNTS
    exit