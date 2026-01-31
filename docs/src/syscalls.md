# Syscalls

This table links key [Solana syscalls] with select examples that use them.

| Syscall                          | Select example |
| -------------------------------- | -------------- |
| [`sol_log`]                      | [Quickstart]   |
| [`sol_invoke_signed_c`]          | [Transfer]     |
| [`sol_try_find_program_address`] | [Counter]      |
| [`sol_get_rent_sysvar`]          | [Counter]      |
| [`sol_create_program_address`]   | [Counter]      |

[counter]: examples/counter
[quickstart]: quickstart
[solana syscalls]: https://github.com/anza-xyz/solana-sdk/blob/frozen-abi-macro@v3.2.0/define-syscall/src/definitions.rs
[transfer]: examples/transfer
[`sol_create_program_address`]: https://github.com/anza-xyz/agave/blob/v3.1.7/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L64-L72
[`sol_get_rent_sysvar`]: https://github.com/anza-xyz/agave/blob/v3.1.7/syscalls/src/lib.rs#L409
[`sol_invoke_signed_c`]: https://github.com/anza-xyz/agave/blob/v3.1.7/platform-tools-sdk/sbf/c/inc/sol/inc/cpi.inc#L56-L65
[`sol_log`]: https://github.com/anza-xyz/agave/blob/v3.1.7/platform-tools-sdk/sbf/c/inc/sol/inc/log.inc#L14-L18
[`sol_try_find_program_address`]: https://github.com/anza-xyz/agave/blob/v3.1.7/platform-tools-sdk/sbf/c/inc/sol/inc/pubkey.inc#L74-L83