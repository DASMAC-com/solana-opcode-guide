use fib_rs::Fib;
use test_utils::{setup_test, ProgramLanguage};

const MAX_FIB_INDEX_U32: u8 = 47;

#[test]
fn test_asm() {
    setup_test(ProgramLanguage::Assembly);
}

/// Verify the index of the maximum Fibonacci number that fits in a u32.
#[test]
fn verify_max_fib_u32() {
    assert!(Fib::single(MAX_FIB_INDEX_U32.into()) <= u32::MAX.into());
    assert!(Fib::single((MAX_FIB_INDEX_U32 + 1).into()) > u32::MAX.into());
}
