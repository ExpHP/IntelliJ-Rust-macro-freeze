
// (this line is necessary for the lockup for some reason)
use std::hash::Hash;

#[macro_export]
macro_rules! newtype_index {
    // 0. "Expand macros" must be enabled.
    //    (Inspections and "Use cargo check" have no impact.)
    //
    // 1. wait until syntax highlighting makes a second pass over the module
    //    (the pass that colorizes `Hash` and `macro_rules!`)
    //    (it won't lock up any earlier)
    //
    // 2. place the cursor before the '$' and type $()*
    //    so that the line reads:
    //
    //     ($()* $type:ident) => (
    //
    // 3. wait another second or two.
    //
    // 4. try typing more.  It will be frozen.
    ($type:ident) => (
        pub struct $type(usize);

    );
}
