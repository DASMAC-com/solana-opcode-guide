# Parity workflow

Reusable workflow for verifying that Rust and assembly
implementations produce equivalent behavior and comparable
instruction-level output. Applicable to any example in
`examples/`.

## Generating and comparing disassembly

Use `/disassemble-example <name>` to generate Rust disassembly
(e.g., `/disassemble-example tree`). The output lives at
`examples/<name>/artifacts/rs-disassembly.s`. Compare it
side-by-side with the corresponding assembly source:

- Run `/disassemble-example <name>` to regenerate the Rust
  disassembly artifact.
- Open `examples/<name>/artifacts/rs-disassembly.s` and locate
  the function or block under review.
- Open the corresponding section of the assembly source and
  align the two side by side.
- Walk through instruction by instruction, noting structural
  differences: extra jumps, reordered operations, missing
  inlining, different register usage.

The goal is structural equivalence, not byte-identical output.
The Rust compiler may reorder instructions or use different
registers while producing equivalent behavior.

## Identifying optimization opportunities

Disassembly comparison reveals concrete optimization targets.
Look for:

- **Duplicated instruction sequences.** Repeated blocks that
  could be factored into a shared label or inlined at each site.
- **Unnecessary jumps.** Branches to code that immediately
  follows — often introduced by nested `if/else` or macro
  expansion.
- **Register pressure differences.** Cases where the Rust
  compiler spills to the stack while the assembly keeps values
  in registers, or vice versa.
- **Missed inlining opportunities.** Function calls that the
  compiler did not inline despite `#[inline(always)]`, or
  macros that expand into more instructions than the equivalent
  hand-written code.

Macro expansion can produce duplicated code at each call site.
Before restructuring to eliminate duplication, evaluate whether
it actually hurts CU count — the compiler often optimizes each
expansion independently, producing tighter code than a unified
block. Only restructure when measurable improvement is
confirmed.

## Branch coverage verification checklist

Trace each test through both the Rust and assembly
implementations to confirm that both paths execute the same
logical branches.

- Start with a branch table that maps each conditional jump to
  the test numbers that exercise it. For example, the "Assembly
  branches" table in `remove-tests.md`:

  ```text
  | ID  | Branch                    | Tests             |
  | --- | ------------------------- | ----------------- |
  | B1  | found : L==null           | 10,12,14,16,17,18 |
  ...
  ```

- For each branch ID, pick a representative test and manually
  trace it through the assembly path, confirming which branch
  is taken or not taken.

- Trace the same test through the Rust path, confirming the
  corresponding conditional takes the same direction.

- Record any discrepancies. A branch exercised in assembly but
  unreachable in Rust (or vice versa) indicates a structural
  divergence that must be resolved.

Use the branch-to-test mapping format as a template for new
instructions. Each new instruction's spec should include an
equivalent table covering every conditional jump in both
implementations.

## Iterative optimization loop

Parity verification is iterative. The cycle:

- **Edit code.** Make a targeted change to the Rust
  implementation (restructure control flow, adjust macro
  boundaries, inline a helper).
- **Regenerate disassembly.** Run `/disassemble-example <name>`
  to produce updated Rust disassembly.
- **Compare with assembly.** Diff the new disassembly against
  the assembly source to confirm the change had the intended
  effect.
- **Run tests.** Run `/build-example <name>` and wait for the
  full pipeline to finish (build, dump, disassemble, test,
  snippets, verification). Then check the result file at
  `examples/<name>/artifacts/tests/*/result.txt` to confirm
  all tests pass and review the CU comparison table for
  overhead changes.
- **Verify parity.** Confirm that the Rust disassembly now
  matches the assembly structure more closely (or produces
  equivalent/better code). Use the CU table in the result
  file as the ground truth for overhead — do not manually
  calculate CU from logs.
- **Repeat.** Continue until the target section is at parity.

Stop when one of these conditions holds:

- The Rust disassembly matches the assembly structure for the
  section under review (same labels, same branch topology, same
  instruction count within a small margin).
- The Rust disassembly produces demonstrably better code (fewer
  instructions, fewer branches) while maintaining identical
  test results.
- The Rust overhead is within acceptable bounds for the use
  case, or further optimization would require restructuring
  that degrades code clarity without meaningful CU improvement.

## CU overhead analysis

Before optimizing, run the CU overhead analysis workflow to
identify WHERE overhead comes from and prioritize targets. See
`cu-overhead-analysis.md` for the full methodology.

The analysis pipeline feeds into the optimization loop:

- **Analysis** identifies overhead sources and classifies them
  (redundant branches, macro duplication, register spills,
  etc.).
- **Optimization targets** are ranked by reducible CUs and
  handed to Engineering.
- **This parity workflow** drives the iterative optimization
  cycle on each target.
- **Analysis re-measures** after each optimization to confirm
  overhead reduction.
