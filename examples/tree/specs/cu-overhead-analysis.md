<!-- cspell:word macrodup -->

# CU overhead analysis

Methodology for attributing CU overhead in Rust implementations
relative to hand-written assembly. Produces actionable
optimization targets.

## Inputs

Each analysis requires:

- ASM snippet for the section under review (from
  `artifacts/tests/*/snippets/`).
- RS disassembly snippet for the same section (from
  `artifacts/rs-disassembly.s`).
- CU table from `artifacts/tests/*/result.txt`.
- Relevant algorithm spec for context (from `specs/`).

## Workflow

### Step 1 — Section isolation

Pick one ANCHOR section. Work on one section at a time.
Identify the corresponding ASM and RS code blocks.

### Step 2 — Instruction count

Count executed instructions per test case path for both ASM
and RS. Record the delta (RS - ASM) for each test case.

### Step 3 — Structural alignment

Walk both snippets in parallel, marking each instruction as:

- **Matched.** Same operation, same position.
- **Reordered.** Same operation, different position.
- **Extra (RS).** Present in RS but not ASM.
- **Extra (ASM).** Present in ASM but not RS.
- **Divergent.** Different operation at corresponding position.

### Step 4 — Overhead classification

Classify every extra RS instruction into a category:

| Category         | Code | Description                                     |
| ---------------- | ---- | ----------------------------------------------- |
| Redundant branch | RB   | Null check or conditional that ASM avoids.      |
| Macrodup.        | MD   | Instruction duplicated across macro exp. sites. |
| Register reload  | RR   | Value loaded from memory that ASM keeps in reg. |
| Register spill   | RS   | Stack spill/reload from register pressure.      |
| Jump overhead    | JO   | Extra branch to reach shared code.              |
| Compiler art.    | CA   | No logical purpose (nop, identity move).        |

### Step 5 — Per-test CU attribution

For each test case, trace the executed path and attribute the
CU delta to specific extra instructions with their category
codes.

### Step 6 — Pattern aggregation

Aggregate across all test cases:

- Total extra CUs per category.
- Percentage of total overhead per category.
- Reducibility assessment (can this overhead be eliminated
  through Rust source changes?).

### Step 7 — Optimization target ranking

Rank by reducible CUs (highest first). Each target becomes a
work item for Engineering.

## Deliverable format

Each analysis produces a document at
`examples/<name>/specs/cu-analysis-<section>.md` containing:

- Summary table (section, test count, mean/min/max overhead).
- Per-test attribution tables.
- Pattern aggregation table.
- Optimization targets (prioritized).
- Engineering recommendations.

## Overhead baselines

- **Target:** \<=10% for hot paths.
- **Acceptable:** 10-20% for cold paths.
- **Needs work:** >20%.

## Relationship to parity workflow

Analysis runs first to identify WHERE overhead comes from and
WHY. The parity optimization loop (see `parity-workflow.md`)
then drives improvements on the identified targets. After
Engineering implements changes, Analysis re-measures to confirm
overhead reduction.

```text
Analysis identifies overhead → Spec documents targets →
Engineering optimizes Rust → Build + test → Analysis
re-measures → QA verifies → repeat until baseline met
```
