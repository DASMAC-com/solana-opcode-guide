---
title: Quickstart
nav_order: 2
---

# Quickstart

1. Install [`solana`].
1. Install [`sbpf`].
1. Clone the [Solana Opcode Guide] repository.

   ```sh
   git clone https://github.com/dasmac-com/solana-opcode-guide.git
   ```
1. Navigate to the examples directory.

   ```sh
   cd solana-opcode-guide/examples
   ```

1. Build and run the examples using `sbpf`.

   ```sh
   sbpf build
   sbpf test
   ```

   1. If you get errors, you might need to clear your `solana` installation
      cache and re-install `solana` (this is a [known issue] with the Solana
      toolchain).

      ```sh
      rm -rf ~/.cache/solana
      ```

1. (Optional) Install the [VS Code SBPF Assembly extension].

[VS Code sBPF Assembly extension]: https://marketplace.visualstudio.com/items?itemName=deanmlittle.vscode-sbpf-asm
[known issue]: https://stackoverflow.com/a/78398587
[Solana Opcode Guide]: https://github.com/dasmac-com/solana-opcode-guide
[`sbpf`]: https://github.com/blueshift-gg/sbpf
[`solana`]: https://docs.anza.xyz/cli/install
