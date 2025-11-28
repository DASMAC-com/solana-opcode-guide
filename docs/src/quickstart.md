# Quickstart

## Set up your environment

1. Install the latest version of [`solana`].
1. Update your [`PATH`] to include key [SBPF] tools packaged with the `solana`
   install, in particular the [`dump.sh`] script and the [LLVM] binaries it
   requires. This will look something like:


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

1. Install [`rustfilt`], which is required by [`dump.sh`]:

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
1. Navigate to the `hello-dasmac` directory.

   ```sh
   cd solana-opcode-guide/examples/hello-dasmac
   ```

1. Compare the assembly and Rust program implementations:

   | Implementation | Location |
   | -------------- | -------- |
   | Assembly       | `src/hello-dasmac/hello-dasmac.s` |
   | Rust           | `src/program.rs` |

   > [!tip]
   > All future examples are in the `examples` directory and use a similar
   > layout, since the [`sbpf`] `build` command expects
   > `src/<program-name>/<program-name>.s`.

   ::: code-group

   <<< ../../examples/hello-dasmac/src/hello-dasmac/hello-dasmac.s{asm:line-numbers}

   <<< ../../examples/hello-dasmac/src/program.rs{rs:line-numbers}

   :::

1. Build the Rust implementation, and dump the [ELF][SBPF] output. By default
   this will create the following files in `../target/deploy`
   (`solana-opcode-guide/examples/target/deploy`):

    1. `hello_dasmac.so` - The compiled SBPF program in ELF format.
    1. `hello_dasmac.so.dmp` - A text dump of the compiled SBPF program.

   ```sh
   cargo build-sbf --dump
   ```

1. Build the assembly implementation.

   ```sh
   sbpf build
   ```

1. Run [`dump.sh`] on the assembly build.

   ```sh
   dump.sh deploy/hello-dasmac.so deploy/asm-dump.txt
   ```

1. Compare the outputs of the two builds.

   | Implementation | Dump |
   | -------------- | -------- |
   | Assembly       | `deploy/hello-dasmac.so` |
   | Rust           | `../target/deploy/hello_dasmac.so` |

1. Run the `asm` test.

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

1. Inspect the output:

   ```sh{3}
   running 1 test
   [... DEBUG ...] Program DASMAC... invoke [1]
   [... DEBUG ...] Program log: Hello, DASMAC!
   [... DEBUG ...] Program DASMAC... consumed 104 of 1400000 compute units
   [... DEBUG ...] Program DASMAC... success
   test tests::hello_dasmac ... ok
   ```

## Review the assembly file

1. Open the `hello-dasmac.s` assembly file:



1. Disassemble the program:

   ```sh
   sbpf disassemble deploy/hello-dasmac.so
   ```

## :tada: Congratulations!

You have successfully assembled and disassembled your first SBPF program!

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