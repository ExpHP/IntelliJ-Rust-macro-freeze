
use std::fmt::{self, Debug};
use std::hash::Hash;

pub unsafe trait Idx {
}

unsafe impl Idx for usize {

}

#[macro_export]
macro_rules! newtype_index {
    ($type:ident) => (
        pub struct $type(usize);

    );
}
