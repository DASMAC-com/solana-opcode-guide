# Assembly offset safety

## Rule: no arithmetic on offset constants in assembly

SBF load/store instructions encode memory offsets as i16 immediates. When two or
more offset constants are added together in an assembly instruction, no
compile-time check validates that the sum fits in i16. The assembler would
eventually reject an overflow, but the error is late and unclear.

### Prohibited

Adding multiple offset constants in a single instruction:

```asm
ldxdw r9, [r1 + TREE_DATA_OFF + TREE_TOP_OFF]
```

### Required

Define a single constant for the combined offset so that the full
value is validated at definition time. Then use that constant
alone in the instruction:

```asm
ldxdw r9, [r1 + IB_TREE_DATA_TOP_OFF]
```

### When separate offsets are fine

Using an offset constant in an `add64` followed by a different
offset in a subsequent load/store is safe, since each immediate
is validated independently:

```asm
add64 r6, TREE_DATA_OFF
stxdw [r6 + TREE_TOP_OFF], r4
```
