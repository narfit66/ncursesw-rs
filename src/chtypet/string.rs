/*
    src/chtypet/string.rs

    Copyright (c) 2019, 2020 Stephen Whittle  All rights reserved.

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom
    the Software is furnished to do so, subject to the following conditions:
    The above copyright notice and this permission notice shall be included
    in all copies or substantial portions of the Software.
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
    THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
    IN THE SOFTWARE.
*/

use std::ops::{BitOr, BitXor};
use ascii::AsciiString;
use crate::{
    gen::*,
    chtypet::ChtypeChar,
    shims::ncurses::{chtype, attr_t},
    normal::{Attribute, Attributes}
};

/// Ascii string and rendition.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChtypeString {
    raw: Vec<chtype>
}

impl ChtypeString {
    pub fn new() -> Self {
        Self { raw: vec!() }
    }

    pub fn from_ascii_string(str: &AsciiString) -> Self {
        Self { raw: str.as_bytes().iter().map(|b| chtype::from(*b)).collect() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { raw: Vec::with_capacity(capacity) }
    }

    //pub unsafe fn from_raw_parts(buf: *mut ChtypeChar, length: usize, capacity: usize) -> Self { }
    //pub unsafe fn from_chtype_unchecked<B>(bytes: B) -> Self where B: Into<Vec<chtype>> { }
    //pub fn from_ascii<B>(bytes: B) -> Result<ChtypeString, FromAsciiError<B>> where B: Into<Vec<u8>> + AsRef<[u8]> { }

    pub fn push_str(&mut self, rhs: &Self) {
        self.raw.append(&mut Self::into(rhs.to_owned()));
    }

    pub fn capacity(&self) -> usize {
        self.raw.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.raw.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.raw.reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.raw.shrink_to_fit()
    }

    pub fn push(&mut self, rhs: ChtypeChar) {
        self.raw.push(ChtypeChar::into(rhs.to_owned()));
    }

    pub fn truncate(&mut self, new_len: usize) {
        self.raw.truncate(new_len)
    }

    pub fn pop(&mut self) -> Option<ChtypeChar> {
        self.raw.pop().map(|c| ChtypeChar::from(c))
    }

    pub fn remove(&mut self, idx: usize) -> ChtypeChar {
        ChtypeChar::from(self.raw.remove(idx))
    }

    pub fn insert(&mut self, idx: usize, ch: ChtypeChar) {
        self.raw.insert(idx, ChtypeChar::into(ch))
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.raw.clear()
    }
}

impl Default for ChtypeString {
    fn default() -> Self {
        Self::new()
    }
}

impl BitOr<Attributes> for ChtypeString {
    type Output = Self;

    fn bitor(self, rhs: Attributes) -> Self::Output {
        Self { raw: self.raw.iter().map(|c| c | rhs.as_attr_t()).collect() }
    }
}

impl BitXor<Attributes> for ChtypeString {
    type Output = Self;

    fn bitxor(self, rhs: Attributes) -> Self::Output {
        Self { raw: self.raw.iter().map(|c| c ^ rhs.as_attr_t()).collect() }
    }
}

impl BitOr<Attribute> for ChtypeString {
    type Output = Self;

    fn bitor(self, rhs: Attribute) -> Self::Output {
        let attr: attr_t = rhs.into();

        Self { raw: self.raw.iter().map(|c| c | attr).collect() }
    }
}

impl BitXor<Attribute> for ChtypeString {
    type Output = Self;

    fn bitxor(self, rhs: Attribute) -> Self::Output {
        let attr: attr_t = rhs.into();

        Self { raw: self.raw.iter().map(|c| c ^ attr).collect() }
    }
}

impl<'a> From<&'a [chtype]> for ChtypeString {
    fn from(slice: &'a [chtype]) -> Self {
        Self { raw: slice.to_vec() }
    }
}

impl Into<Vec<chtype>> for ChtypeString {
    fn into(self) -> Vec<chtype> {
        self.raw.clone()
    }
}

impl RawWithNul<Vec<chtype>> for ChtypeString {
    fn raw_with_nul(self) -> Vec<chtype> {
        let mut vec_of_chtype: Vec<chtype> = Self::into(self);

        vec_of_chtype.push(0x00);

        vec_of_chtype
    }
}
