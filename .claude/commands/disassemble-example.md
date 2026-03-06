# Disassemble example

Quick compile and disassemble cycle for a specific example program.

Run the following commands sequentially from the example directory at
`examples/$ARGUMENTS`:

1. Remove stale binary:

```sh
rm -f ../target/deploy/${ARGUMENTS//-/_}.so
```

1. Build for the disassembly architecture:

```sh
cargo build-sbf --arch v2 --tools-version 1.52
```

1. Disassemble and write output:

```sh
sbpf disassemble ../target/deploy/${ARGUMENTS//-/_}.so \
  > artifacts/rs-disassembly.s
```

If `$ARGUMENTS` is empty, infer the example name from the current
conversation context (e.g. recent file paths or discussion). If you still
cannot determine it, ask which example to disassemble.

Report the result. Show the disassembly content or the relevant section
if the user is focused on specific functions.
