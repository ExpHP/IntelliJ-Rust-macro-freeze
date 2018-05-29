//
// File adapted for use by rsp2. Originally from rust-lang/rust.
//
// -----------------------------------------------------------------
// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::fmt::{self, Debug};
use std::hash::Hash;

pub unsafe trait Idx {
}

unsafe impl Idx for usize {

}

#[macro_export]
macro_rules! newtype_index {
    ($type:ident) => (
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type(usize);

    );
}
