Quick build and test cycle for a specific example program.

Run the following commands sequentially from the example directory at
`examples/$ARGUMENTS`:

```sh
cargo build-sbf --arch v3 --tools-version 1.51
sbpf build
cargo test -- --test-threads 1
```

If `$ARGUMENTS` is empty, infer the example name from the current
conversation context (e.g. recent file paths or discussion). If you still
cannot determine it, ask which example to test.

Report the result of each step. If any step fails, stop and show the error.
