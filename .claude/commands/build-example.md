Run the full build-examples pipeline that builds, dumps, disassembles,
tests, and verifies artifacts.

From the `examples/` directory, run:

```sh
cargo run --bin build-examples -- --example $ARGUMENTS
```

If `$ARGUMENTS` is empty, run without the `--example` flag to build all
examples:

```sh
cargo run --bin build-examples
```

This executes `examples/utils/build-examples/src/main.rs` which handles
building ELF files, generating dumps/disassembly, running tests, saving
test snippets, and verifying code snippets.

Report the outcome. If the command fails, show the relevant error output.
