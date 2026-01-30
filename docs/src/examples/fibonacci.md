<!-- cspell:word 7etsuo -->

# Fibonacci

<!-- @include: ./disclaimer.md -->

## :1234: Fibonacci sequence background

The [Fibonacci sequence] is a classic mathematical sequence where each number is
the sum of the two preceding ones, typically starting with $F(0) = 0$ and
$F(1) = 1$. This example demonstrates how to compute Fibonacci numbers
efficiently in SBPF assembly, using the program's return code to communicate the
result.

The Fibonacci program takes a single byte of instruction data representing the
sequence index $n$, and returns $F(n)$ as a [custom program error code], except
for $F(0) = 0$, since a [return value of zero indicates success]. This
approach leverages the fact that [the program's return value] can be used to
communicate arbitrary `u32` values back to the caller.

## :shield: Input validation

Like the [memo example](./memo), the program first validates that no accounts
are passed by checking the [number of accounts in the input buffer]:

<<< ../../../examples/fibonacci/artifacts/snippets/asm/accounts.txt{1-4 asm}

If accounts are detected, the program immediately exits with error code
`E_ACCOUNTS` (`0xffffffff`):

<<< ../../../examples/fibonacci/artifacts/snippets/asm/abort-accounts.txt{asm}

::: details Full program

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{asm}

:::

Next, the program validates that the requested Fibonacci index $n$ doesn't
exceed `MAX_N` (47), which is the largest index whose Fibonacci number fits in a
`u32` while leaving room for two error codes:

<<< ../../../examples/fibonacci/artifacts/snippets/asm/constants.txt{5-6 asm}

<<< ../../../examples/fibonacci/artifacts/snippets/asm/max-n.txt{9-13 asm}

::: details `test_max_fib_u32`

<<< ../../../examples/fibonacci/artifacts/tests/max_fib_u32/test.txt{rust}

:::

If the index is too large, the program exits with error code `E_MAX_N`
(`0xfffffffe`).

<<< ../../../examples/fibonacci/artifacts/snippets/asm/abort-n.txt{asm}

::: details Full program

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{asm}

:::

## :repeat: Fibonacci computation loop

The algorithm uses three registers to compute Fibonacci numbers iteratively:

- `r6`: $F(i-1)$: initialized to 0
- `r7`: $F(i)$: initialized to 1
- `r8`: Loop counter, decremented from $n$ to 1

These registers (`r6`, `r7`, `r8`) are
[call-preserved (callee-saved/non-volatile) registers] in the
[SBPF calling convention], meaning their values are preserved across function
calls. This makes them ideal for storing loop state that needs to persist across
iterations.

> [!tip]
> This example does not implement `call` so the call-preserved property isn't
> strictly needed. However, using call-preserved registers is a good practice
> for maintaining state across function calls in more complex programs.

The program handles the special cases $F(0) = 0$ and $F(1) = 1$ by checking if
$n \leq 1$ and returning early if so:

<<< ../../../examples/fibonacci/artifacts/snippets/asm/constants.txt{7 asm}

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/fibonacci/artifacts/snippets/asm/special-return.txt{7-15 asm}

<!-- markdownlint-enable MD013 -->

For $n > 1$, the program enters a loop that computes successive Fibonacci
numbers using $F(n) = F(n-1) + F(n-2)$:

<<< ../../../examples/fibonacci/artifacts/snippets/asm/loop.txt{7-22 asm}

Each iteration performs the following:

1. Save $F(i-1)$ from `r6` into scratch register `r9`.
1. Move $F(i)$ from `r7` to `r6`, making it the new $F(i-1)$.
1. Add the old $F(i-1)$ from `r9` to `r7`, computing $F(i+1)$.
1. Decrement the loop counter in `r8`.
1. Return if the counter is still greater than 1.

::: details Full program

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{asm:line-numbers}

:::

## :chart_with_upwards_trend: Compute unit consumption

The assembly implementation demonstrates $O(n)$ linear compute unit growth with
the Fibonacci index, consuming 5 compute units per iteration:

::: details `test_asm`

<<< ../../../examples/fibonacci/artifacts/tests/asm/test.txt{rs:line-numbers}

:::

::: details Test results

<<< ../../../examples/fibonacci/artifacts/tests/asm/result.txt{1-49 sh}

:::

## :crab: Rust implementation

The Rust implementation mirrors the assembly logic but uses a function to
encapsulate the Fibonacci computation, which is written specifically to produce
a comparable assembly loop output:

<<< ../../../examples/fibonacci/src/program.rs{7,25-40 rs:line-numbers}

Note that unlike the assembly version, the Rust implementation doesn't check
for the number of passed accounts because [Pinocchio's `entrypoint!` macro]
by default [handles account parsing automatically], including specifying a
maximum number of accounts, [past which any extra accounts are ignored].

::: details `rs-disassembly.s` (core Fibonacci logic highlighted)

<!-- markdownlint-disable MD013 -->

<<< ../../../examples/fibonacci/artifacts/rs-disassembly.s{43-56 asm:line-numbers}

<!-- markdownlint-enable MD013 -->

:::

The Rust implementation introduces some compute unit overhead compared to
assembly:

::: details `test_rs`

<<< ../../../examples/fibonacci/artifacts/tests/rs/test.txt{rs:line-numbers}

:::

::: details Test results

<<< ../../../examples/fibonacci/artifacts/tests/rs/result.txt{1-49 sh}

:::

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/fibonacci/src/tests.rs{rs:line-numbers}

:::

> [!note]
> The assembly file in this example was adapted from
> [an implementation by `7etsuo`].

[an implementation by `7etsuo`]: https://github.com/7etsuo/Learn-Solana-BPF-Assembly/blob/17eb280a9ca02132f40b3fb61be5e469670dd2d1/examples/solana-fibonacci-asm/src/fib/fib.s
[call-preserved (callee-saved/non-volatile) registers]: https://en.wikipedia.org/wiki/X86_calling_conventions#Callee-saved_(non-volatile)_registers
[custom program error code]: https://docs.rs/solana-program/latest/solana_program/program_error/enum.ProgramError.html#variant.Custom
[fibonacci sequence]: https://en.wikipedia.org/wiki/Fibonacci_sequence
[handles account parsing automatically]: https://github.com/anza-xyz/pinocchio/blob/pinocchio@v0.9.2/sdk/pinocchio/src/entrypoint/mod.rs#L172-L191
[number of accounts in the input buffer]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L531
[past which any extra accounts are ignored]: https://github.com/anza-xyz/pinocchio/blob/pinocchio@v0.9.2/sdk/pinocchio/src/entrypoint/mod.rs#L162-L163
[pinocchio's `entrypoint!` macro]: https://github.com/anza-xyz/pinocchio/blob/pinocchio@v0.9.2/sdk/pinocchio/src/entrypoint.rs#L18-L23
[return value of zero indicates success]: https://docs.rs/solana-program-entrypoint/3.1.1/solana_program_entrypoint/constant.SUCCESS.html
[sbpf calling convention]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/doc/bytecode.md#registers
[the program's return value]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/interpreter.rs#L574
