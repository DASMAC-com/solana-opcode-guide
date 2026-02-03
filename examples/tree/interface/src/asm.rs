extern crate alloc;

use macros::extend_constant_group;

extend_constant_group!(input_buffer {
    prefix = "IB",
    /// Number of accounts passed in input.
    N_ACCOUNTS_OFF = 0,
});
