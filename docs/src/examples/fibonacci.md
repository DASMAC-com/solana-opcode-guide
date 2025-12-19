# Fibonacci

<!--@include: ./disclaimer.md-->

## :1234: Fibonacci sequence background

The [Fibonacci sequence] is a classic mathematical sequence where each number is the sum of the two preceding ones, typically starting with F(0) = 0 and F(1) = 1. This example demonstrates how to compute Fibonacci numbers efficiently in SBPF assembly, using the program's return code to communicate the result.

The Fibonacci program takes a single byte of instruction data representing the sequence index `n`, and returns F(n) as a [custom program error code]. This approach leverages the fact that [the program's return value] can be used to communicate arbitrary u32 values back to the caller.

## :shield: Input validation

Like the [memo example](./memo), the program first validates that no accounts are passed by checking the [number of accounts in the input buffer]:

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{11-17 asm:line-numbers}

If accounts are detected, the program immediately exits with error code `E_ACCOUNTS` (0xffffffff).

Next, the program validates that the requested Fibonacci index `n` doesn't exceed `MAX_N` (47), which is the largest index whose Fibonacci number fits in a u32 while leaving room for two error codes:

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{19-23 asm:line-numbers}

If the index is too large, the program exits with error code `E_MAX_N` (0xfffffffe).

::: details Full program

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{asm:line-numbers}

:::

## :repeat: Fibonacci computation loop

The algorithm uses three registers to compute Fibonacci numbers iteratively:
- `r6`: F(i-1) - initialized to 0
- `r7`: F(i) - initialized to 1
- `r8`: Loop counter, decremented from n to 1

The program handles the special cases F(0) = 0 and F(1) = 1 by checking if n ≤ 1 and returning early:

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{25-33 asm:line-numbers}

For n > 1, the program enters a loop that computes successive Fibonacci numbers using the recurrence relation F(n) = F(n-1) + F(n-2):

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{35-50 asm:line-numbers}

Each iteration performs these steps:
1. Save F(i-1) from `r6` into scratch register `r9`
2. Move F(i) from `r7` to `r6`, making it the new F(i-1)
3. Add the old F(i-1) from `r9` to `r7`, computing F(i+1)
4. Decrement the loop counter in `r8`
5. Continue looping until the counter reaches 1

::: details Full program

<<< ../../../examples/fibonacci/src/fibonacci/fibonacci.s{asm:line-numbers}

:::

## :chart_with_upwards_trend: Compute unit consumption

The assembly implementation demonstrates linear compute unit growth with the Fibonacci index. Note that F(0) returns success (not an error), while F(1) and above return the Fibonacci value as a custom error code:

<<< ../../../examples/fibonacci/artifacts/tests/asm/result.txt{1-10 sh:line-numbers}

Each loop iteration adds approximately 5 compute units, demonstrating the tight efficiency of the assembly implementation.

::: details `test_asm`

<<< ../../../examples/fibonacci/artifacts/tests/asm/test.txt{rs:line-numbers}

:::

## :crab: Rust implementation

The Rust implementation mirrors the assembly logic but uses a function to encapsulate the Fibonacci computation:

<<< ../../../examples/fibonacci/src/program.rs{9-23 rs:line-numbers}

The main entrypoint validates inputs and calls the `fib` function:

<<< ../../../examples/fibonacci/src/program.rs{26-39 rs:line-numbers}

Note the use of `unchecked_add` and `unchecked_sub` to avoid overflow checks, since we've already validated that n ≤ 47.

The Rust implementation introduces some compute unit overhead compared to assembly:

<<< ../../../examples/fibonacci/artifacts/tests/rs/result.txt{1-10 sh:line-numbers}

::: details `test_rs`

<<< ../../../examples/fibonacci/artifacts/tests/rs/test.txt{rs:line-numbers}

:::

## :white_check_mark: All tests

::: details `tests.rs`

<<< ../../../examples/fibonacci/src/tests.rs{rs:line-numbers}

:::

[custom program error code]: https://docs.rs/solana-program/latest/solana_program/program_error/enum.ProgramError.html#variant.Custom
[fibonacci sequence]: https://en.wikipedia.org/wiki/Fibonacci_sequence
[memo example]: ./memo
[number of accounts in the input buffer]: https://github.com/anza-xyz/agave/blob/v3.1.3/program-runtime/src/serialization.rs#L531
[the program's return value]: https://github.com/anza-xyz/sbpf/blob/v0.13.0/src/interpreter.rs#L574
