// SPDX-License-Identifier: Apache-2.0

//! Helpful primitives for developing the crate.

mod impl_const_id;

use std::io::{Read, Result, Write};
use std::mem::{size_of, MaybeUninit};
use std::slice::{from_raw_parts, from_raw_parts_mut};

pub trait TypeLoad: Read {
    fn load<T: Sized + Copy>(&mut self) -> Result<T> {
        let mut t = MaybeUninit::<T>::uninit();
        let s = unsafe { from_raw_parts_mut(t.as_mut_ptr() as _, size_of::<T>()) };
        self.read_exact(s)?;
        let t = unsafe { t.assume_init() };
        Ok(t)
    }
}

pub trait TypeSave: Write {
    fn save<T: Sized + Copy>(&mut self, value: &T) -> Result<()> {
        let p = value as *const T as *const u8;
        let s = unsafe { from_raw_parts(p, size_of::<T>()) };
        self.write_all(s)
    }
}

impl<T: Read> TypeLoad for T {}
impl<T: Write> TypeSave for T {}
