# Solana Opcode Guide

<!-- markdownlint-disable MD036 -->

*Courtesy of Distributed Atomic State Machine Algorithms Corporation (DASMAC)*

<!-- markdownlint-enable MD036 -->

The Solana Opcode Guide is built with [VitePress] and can be viewed at
[opcodes.dasmac.com].

## Local Deployment

1. Navigate to the docs directory:

   ```sh
   cd docs
   ```

1. Ensure you have [`npm`].

1. Install dependencies and run dev server:

   ```sh
   npm install
   npm run dev
   ```

1. Open [localhost:5173].

## Linting

1. Ensure you have [`pre-commit`].

1. Run `quick-lint`:

   ```sh
   pre-commit run --config cfg/pre-commit/quick-lint.yml --all-files
   ```

[localhost:5173]: http://localhost:5173
[opcodes.dasmac.com]: https://opcodes.dasmac.com
[vitepress]: https://vitepress.dev/
[`npm`]: https://nodejs.org/en/download
[`pre-commit`]: https://pre-commit.com/
