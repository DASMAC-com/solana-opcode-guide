# Quickstart

This quickstart guide will help you set up your environment to build and test
a ["Hello, World!" program] implemented in both SBPF assembly and Rust, allowing
you to compare the two implementations side-by-side.

## :wrench: Set up your environment {#env-setup}

1. Install [`rustup`] if you don't have it.
1. Install the latest version of [`solana`].

1. Update your [`PATH`] to include key [SBPF] tools packaged with the `solana`
   install, in particular the [`dump.sh`] script [called internally] by
   [`cargo build-sbf`] `--dump`, and the [patched LLVM binaries] it requires
   (covered below). This will look something like:

   ```sh
   # Solana tools.
   SOLANA_RELEASE="$HOME/.local/share/solana/install/active_release/bin"
   SOLANA_SBPF_TOOLS="$SOLANA_RELEASE/platform-tools-sdk/sbf"
   export PATH="$SOLANA_RELEASE:$PATH"
   export PATH="$SOLANA_SBPF_TOOLS/scripts:$PATH"
   export PATH="$SOLANA_SBPF_TOOLS/dependencies/platform-tools/llvm/bin:$PATH"
   ```

   > [!tip]
   > This example is from `~/.zshrc` on a Mac with [Oh My Zsh].

1. Install [`cargo build-sbf`] with [`tools-version`] `v1.51`, which is required
   (as of the time of this writing) to compile to [SBPF v3 and v4][sbpf v4]
   since [`platform-tools` v1.52 removed] the `sbpfv3-solana-solana` and `sbpfv4-solana-solana` targets and there is no newer supporting version than
   `v1.51`.

   ```sh:no-line-numbers
   cargo-build-sbf --install-only --tools-version v1.51
   ```

1. Install [`rustfilt`], which is also required by [`dump.sh`]:

   ```sh:no-line-numbers
   cargo install rustfilt
   ```

1. Install [`sbpf`].

   ```sh:no-line-numbers
   cargo install --git https://github.com/blueshift-gg/sbpf.git
   ```

1. (Optional) Install the [VS Code SBPF Assembly extension].

   > [!tip]
   > Pending the acceptance of [#10] you can even install the additional
   > highlighting features contained therein using a development version of the
   > extension.

## :zap: Run the `hello-dasmac` example

1. Clone the [Solana Opcode Guide] repository.

   ```sh:no-line-numbers
   git clone https://github.com/dasmac-com/solana-opcode-guide.git
   ```

1. Navigate to the `examples/hello-dasmac` directory.

   ```sh:no-line-numbers
   cd solana-opcode-guide/examples/hello-dasmac
   ```

1. Compare the [assembly] and [Rust] program implementations:

   | Implementation | Location                          |
   | -------------- | --------------------------------- |
   | Assembly       | `src/hello-dasmac/hello-dasmac.s` |
   | Rust           | `src/program.rs`                  |

   > [!tip]
   > Other examples in the `examples` directory use a similar layout, since the
   > [`sbpf`] `build` command expects `src/<program-name>/<program-name>.s`.

   ::: code-group

   <!-- markdownlint-disable MD013 -->

   <<< ../../examples/hello-dasmac/src/hello-dasmac/hello-dasmac.s{asm}

   <!-- markdownlint-enable MD013 -->

   <<< ../../examples/hello-dasmac/src/program.rs

   :::

1. Build the assembly implementation.

   ```sh:no-line-numbers
   sbpf build
   ```

1. Run [`dump.sh`](#env-setup) on the assembly build [ELF][sbpf] output at
   `deploy/hello-dasmac.so`:

   ```sh:no-line-numbers
   dump.sh deploy/hello-dasmac.so deploy/asm-dump.txt
   ```

1. Build the Rust implementation with [SBPF v4] and dump the build. This
   operation should create the following files in `../target/deploy`
   (`solana-opcode-guide/examples/target/deploy`):

   | File                    | Description                   |
   | ----------------------- | ----------------------------- |
   | `hello_dasmac.so`       | Rust build [ELF][sbpf] output |
   | `hello_dasmac-dump.txt` | Dump of the output            |

   ```sh:no-line-numbers
   cargo build-sbf --arch v4 --tools-version v1.51 --dump
   ```

1. Compare the two dumps, in particular the below highlighted sections. Note the
   considerable overhead introduced by the Rust implementation:

   | Implementation | Dump                                     |
   | -------------- | ---------------------------------------- |
   | Assembly       | `deploy/asm-dump.txt`                    |
   | Rust           | `../target/deploy/hello_dasmac-dump.txt` |

   ::: details Output

   ::: code-group

   <!-- markdownlint-disable MD013 -->

   <<< ../../examples/hello-dasmac/dumps/asm.txt{9,13,16,19-20,27,83-89} [Assembly]

   <<< ../../examples/hello-dasmac/dumps/rs.txt{9,13,16,19-20,27,107-374} [Rust]

   <!-- markdownlint-enable MD013 -->

   :::

   > [!tip]
   >
   > You can generate a similar output using the [`sbpf`] `disassemble` command:
   >
   > ```sh:no-line-numbers
   > sbpf disassemble deploy/hello-dasmac.so > deploy/asm-disassembly.json
   > ```
   >
   > ::: details Output
   >
   > <!-- markdownlint-disable MD013 -->
   >
   > <<< ../../examples/hello-dasmac/dumps/asm-disassembly.json
   >
   > <!-- markdownlint-enable MD013 -->
   >
   > :::

1. Run the assembly implementation test.

   ```sh:no-line-numbers
   cargo test -- --test test_asm
   ```

   > [!tip]
   > If you get errors, you might need to clear your `solana` installation
   > cache and re-install `solana` (this is a [known issue] with the Solana
   > toolchain).
   >
   > ```sh:no-line-numbers
   > rm -rf ~/.cache/solana
   > ```

1. Rebuild the Rust implementation and run its test.

   <!-- markdownlint-disable MD013 -->

   ```sh:no-line-numbers
   cargo build-sbf --arch v3 --tools-version v1.51 && cargo test -- --test test_rs
   ```

   <!-- markdownlint-enable MD013 -->

   > [!note]
   > As of the time of this writing, although [SBPF v4] compilation is
   > supported, the runtime only supports [loading up to SBPF v3].

1. Compare the two outputs, noting in particular the [compute unit] overhead
   introduced by the Rust implementation (despite its use of the [`pinocchio`]
   optimization framework):

   ::: code-group

   <!-- markdownlint-disable MD013 -->

   <<< ../../examples/hello-dasmac/test-runs/asm.txt{3 sh:line-numbers} [Assembly]

   <<< ../../examples/hello-dasmac/test-runs/rs.txt{3 sh:line-numbers} [Rust]

   <!-- markdownlint-enable MD013 -->

   :::

## :tada: Congratulations :tada:

You have successfully assembled, disassembled, and tested your first SBPF
program!

> [!note]
> The assembly file and testing framework in this example were adapted from the
> [`sbpf`] `init` command.

["hello, world!" program]: https://en.wikipedia.org/wiki/%22Hello,_World!%22_program
[assembly]: https://en.wikipedia.org/wiki/Assembly_language
[called internally]: https://github.com/anza-xyz/agave/blob/v3.1.2/platform-tools-sdk/cargo-build-sbf/src/post_processing.rs#L93
[compute unit]: https://solana.com/docs/references/terminology#compute-units
[known issue]: https://stackoverflow.com/a/78398587
[patched llvm binaries]: https://github.com/anza-xyz/platform-tools
[#10]: https://github.com/deanmlittle/vscode-sbpf-asm/pull/10
[loading up to sbpf v3]: https://github.com/anza-xyz/agave/blob/v3.1.2/feature-set/src/lib.rs#L140-L141
[oh my zsh]: https://ohmyz.sh/
[`platform-tools` v1.52 removed]: https://github.com/anza-xyz/platform-tools/commit/9dcb73be29b1140467243867f38a388520c85251#diff-4d2a8eefdf2a9783512a35da4dc7676a66404b6f3826a8af9aad038722da6823L114-L115
[rust]: https://solana.com/docs/programs/rust
[sbpf]: https://solana.com/docs/core/programs
[sbpf v4]: https://github.com/anza-xyz/sbpf
[solana opcode guide]: https://github.com/dasmac-com/solana-opcode-guide
[vs code sbpf assembly extension]: https://marketplace.visualstudio.com/items?itemName=deanmlittle.vscode-sbpf-asm
[`cargo build-sbf`]: https://github.com/anza-xyz/agave/blob/v3.1.2/platform-tools-sdk/cargo-build-sbf
[`dump.sh`]: https://github.com/anza-xyz/agave/blob/v3.1.2/platform-tools-sdk/sbf/scripts/dump.sh
[`path`]: https://en.wikipedia.org/wiki/PATH_(variable)
[`pinocchio`]: https://github.com/anza-xyz/pinocchio
[`platform-tools`]: https://github.com/anza-xyz/platform-tools
[`rustfilt`]: https://github.com/luser/rustfilt
[`sbpf`]: https://github.com/blueshift-gg/sbpf
[`solana`]: https://docs.anza.xyz/cli/install
[`tools-version`]: https://github.com/anza-xyz/agave/blob/v3.1.2/platform-tools-sdk/cargo-build-sbf/src/toolchain.rs#L487
[`rustup`]: https://rustup.rs/