# Quickstart

## Set up your environment {#env-setup}

1. Install the latest version of [`solana`].
1. Update your [`PATH`] to include key [SBPF] tools packaged with the `solana`
   install, in particular the [`dump.sh`] script [called internally] by
   [`cargo build-sbf`] `--dump`, and the [LLVM] binaries it requires. This will
   look something like:

   ```sh
   # Solana tools.
   export SOLANA_RELEASE="$HOME/.local/share/solana/install/active_release/bin"
   export SOLANA_SBPF_TOOLS="$SOLANA_RELEASE/platform-tools-sdk/sbf"
   export PATH="$SOLANA_RELEASE:$PATH"
   export PATH="$SOLANA_SBPF_TOOLS/scripts:$PATH"
   export PATH="$SOLANA_SBPF_TOOLS/dependencies/platform-tools/llvm/bin:$PATH"
   ```

   > [!tip]
   > This example is from `~/.zshrc` on a Mac with [Oh My Zsh].

1. Note the pinned [`tools-version`] in `examples/Cargo.toml`, which is required
   (as of the time of this writing) for `cargo build-sbf --arch v4` to access
   the `sbpfv4-solana-solana` target that was [removed in v1.52] of the
   [`platform-tools`].

   ::: details Cargo.toml

   <<< ../../examples/Cargo.toml

   :::

1. Install [`rustfilt`], which is also required by [`dump.sh`]:

   ```sh
   cargo install rustfilt
   ```

1. Install [`sbpf`].

   ```sh
   cargo install --git https://github.com/blueshift-gg/sbpf.git
   ```

1. (Optional) Install the [VS Code SBPF Assembly extension].

## Run the `hello-dasmac` example

1. Clone the [Solana Opcode Guide] repository.

   ```sh
   git clone https://github.com/dasmac-com/solana-opcode-guide.git
   ```
1. Navigate to the `examples/hello-dasmac` directory.

   ```sh
   cd solana-opcode-guide/examples/hello-dasmac
   ```

1. Compare the [assembly] and [Rust] program implementations:

   | Implementation | Location |
   | -------------- | -------- |
   | Assembly       | `src/hello-dasmac/hello-dasmac.s` |
   | Rust           | `src/program.rs` |

   > [!tip]
   > Other examples in the `examples` directory use a similar layout, since the
   > [`sbpf`] `build` command expects `src/<program-name>/<program-name>.s`.

   ::: code-group

   <<< ../../examples/hello-dasmac/src/hello-dasmac/hello-dasmac.s{asm:line-numbers}

   <<< ../../examples/hello-dasmac/src/program.rs{rs:line-numbers}

   :::

1. Build the assembly implementation.

   ```sh
   sbpf build
   ```

1. Run [`dump.sh`](#env-setup) on the assembly build [ELF][SBPF] output at
   `deploy/hello-dasmac.so`:

   ```sh
   dump.sh deploy/hello-dasmac.so deploy/asm-dump.txt
   ```

1. Build the Rust implementation with [SBPF v4] and dump the build. This
   operation should create the following files in `../target/deploy`
   (`solana-opcode-guide/examples/target/deploy`):

   | File | Description |
   | ---- | ----------- |
   | `hello_dasmac.so` | Rust build [ELF][SBPF] output |
   | `hello_dasmac-dump.txt` | Dump of the output |

   ```sh
   cargo build-sbf --arch v4 --dump
   ```

1. Compare the two dumps, in particular the below highlighted sections. Note the
   considerable overhead introduced by the Rust implementation:

   | Implementation | Dump |
   | -------------- | -------- |
   | Assembly       | `deploy/asm-dump.txt` |
   | Rust           | `../target/deploy/hello_dasmac-dump.txt` |

   ::: details Output

   ::: code-group

   <<< ../../examples/hello-dasmac/dump-examples/asm.txt{10,14,18,20-21,28,86-90 text:line-numbers} [Assembly]

   <<< ../../examples/hello-dasmac/dump-examples/rs.txt{10,14,18,20-21,28,117-365,367-377 text:line-numbers} [Rust]

   :::

   > [!tip]
   > You can generate a similar output using the [`sbpf`] `disassemble` command:
   > ```sh
   > sbpf disassemble deploy/hello-dasmac.so > deploy/asm-disassembly.txt
   > ```
   > ::: details Output
   > <<< ../../examples/hello-dasmac/dump-examples/asm-disassembly.txt{json:line-numbers} [asm-disassembly.txt]
   > :::

1. Run the assembly implementation test.

   ```sh
   cargo test -- --test asm
   ```

   > [!tip]
   > If you get errors, you might need to clear your `solana` installation
   > cache and re-install `solana` (this is a [known issue] with the Solana
   > toolchain).
   >
   > ```sh
   > rm -rf ~/.cache/solana
   > ```

1. Rebuild the Rust implementation and run its test.

   ```sh
   cargo build-sbf --arch v3 && cargo test -- --test rs
   ```

   > [!note]
   > As of the time of this writing, although [SBPF v4] compilation is
   > supported, the runtime only supports [loading up to SBPF v3].

1. Compare the two outputs, noting in particular the [compute unit] overhead
   introduced by the Rust implementation (despite its use of the [`pinocchio`]
   optimization framework):

   ::: code-group

   ```sh{4} [Assembly]
   running 1 test
   [... DEBUG ...] Program DASMAC... invoke [1]
   [... DEBUG ...] Program log: Hello, DASMAC!
   [... DEBUG ...] Program DASMAC... consumed 104 of 1400000 compute units
   [... DEBUG ...] Program DASMAC... success
   test tests::asm ... ok
   ```

   ```sh{4} [Rust]
   running 1 test
   [... DEBUG ...] Program DASMAC... invoke [1]
   [... DEBUG ...] Program log: Hello, DASMAC!
   [... DEBUG ...] Program DASMAC... consumed 108 of 1400000 compute units
   [... DEBUG ...] Program DASMAC... success
   test tests::rs ... ok
   ```
   :::

## :tada: Congratulations!

You have successfully assembled, disassembled, and tested your first SBPF
program!

> [!note]
> This example was adapted from the [`sbpf`] `init` command.

[VS Code sBPF Assembly extension]: https://marketplace.visualstudio.com/items?itemName=deanmlittle.vscode-sbpf-asm
[known issue]: https://stackoverflow.com/a/78398587
[Solana Opcode Guide]: https://github.com/dasmac-com/solana-opcode-guide
[`sbpf`]: https://github.com/blueshift-gg/sbpf
[`solana`]: https://docs.anza.xyz/cli/install
[`PATH`]: https://en.wikipedia.org/wiki/PATH_(variable)
[Oh My Zsh]: https://ohmyz.sh/
[`dump.sh`]: https://github.com/anza-xyz/agave/blob/master/platform-tools-sdk/sbf/scripts/dump.sh
[LLVM]: https://llvm.org/
[SBPF]: https://solana.com/docs/core/programs
[`rustfilt`]: https://github.com/luser/rustfilt
[called internally]: https://github.com/anza-xyz/agave/blob/v3.1.2/platform-tools-sdk/cargo-build-sbf/src/post_processing.rs#L93
[compute unit]: https://solana.com/docs/references/terminology#compute-units
[`pinocchio`]: https://github.com/anza-xyz/pinocchio
[assembly]: https://en.wikipedia.org/wiki/Assembly_language
[Rust]: https://solana.com/docs/programs/rust
[`tools-version`]: https://github.com/anza-xyz/agave/blob/v3.1.2/platform-tools-sdk/cargo-build-sbf/src/toolchain.rs#L487
[`cargo build-sbf`]:https://github.com/anza-xyz/agave/blob/v3.1.2/platform-tools-sdk/cargo-build-sbf
[removed in v1.52]: https://github.com/anza-xyz/platform-tools/commit/9dcb73be29b1140467243867f38a388520c85251#diff-4d2a8eefdf2a9783512a35da4dc7676a66404b6f3826a8af9aad038722da6823L100
[`platform-tools`]: https://github.com/anza-xyz/platform-tools
[SBPF v4]: https://github.com/anza-xyz/sbpf
[loading up to SBPF v3]: https://github.com/anza-xyz/agave/blob/v3.1.2/feature-set/src/lib.rs#L140-L141