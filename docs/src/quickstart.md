# Quickstart

## Environment setup

1. Install the latest version of [`solana`].
1. Install [`sbpf`].

   ```sh
   cargo install --git https://github.com/blueshift-gg/sbpf.git
   ```
1. (Optional) Install the [VS Code SBPF Assembly extension].

## Run the `hello_dasmac` example

1. Clone the [Solana Opcode Guide] repository.

   ```sh
   git clone https://github.com/dasmac-com/solana-opcode-guide.git
   ```
1. Navigate to the examples directory.

   ```sh
   cd solana-opcode-guide/examples
   ```

1. Build the examples.

   ```sh
   sbpf build
   ```

1. Run the `hello_dasmac` test.

   ```sh
   cargo test -- --test hello_dasmac
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

   ```sh
   ...
   [... DEBUG ... stable_log] Program log: Hello, DASMAC!
   ...
   ```

## Review the assembly file

1. Open the `hello_dasmac.s` file:

   <<< ../../examples/src/hello_dasmac/hello_dasmac.s{asm:line-numbers}

1. Disassemble the program to view the bytecode:

   ```sh
   sbpf disassemble deploy/hello_dasmac.so
   ```

[VS Code sBPF Assembly extension]: https://marketplace.visualstudio.com/items?itemName=deanmlittle.vscode-sbpf-asm
[known issue]: https://stackoverflow.com/a/78398587
[Solana Opcode Guide]: https://github.com/dasmac-com/solana-opcode-guide
[`sbpf`]: https://github.com/blueshift-gg/sbpf
[`solana`]: https://docs.anza.xyz/cli/install
