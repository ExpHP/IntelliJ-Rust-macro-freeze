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

pub unsafe trait Idx: Copy + 'static + Eq + Debug + Ord + Hash + Send + Sync {
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

/// A Vec or slice that uses newtype indices.
///
/// `V` is only ever `[T]` or `Vec<T>`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Indexed<I: Idx, V: ?Sized> {
    _marker: PhantomData<fn(&I)>,
    pub raw: V,
}

pub type IndexVec<I, T> = Indexed<I, Vec<T>>;

// Whether `Indexed` is `Send` depends only on the data,
// not the phantom data.
unsafe impl<I: Idx, V> Send for Indexed<I, V> where V: Send {}


//--------------------------------------------------------

impl<I: Idx, V: Deref<Target=[T]>, T> Deref for Indexed<I, V> {
    type Target = Indexed<I, [T]>;

    #[inline]
    fn deref(&self) -> &Self::Target { loop {}    }
}

impl<I: Idx, V: DerefMut<Target=[T]>, T> DerefMut for Indexed<I, V> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { loop {}    }
}
