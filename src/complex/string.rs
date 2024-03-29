/*
    src/complex/string.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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

#![allow(clippy::from_over_into)]

use std::convert::TryInto;
use crate::{
    gen::{AttributesType, ColorPairType, ColorAttributeTypes, RawWithNul},
    complex::ComplexChar,
    ncurseswerror::NCurseswError,
    shims::ncurses::{wchar_t, cchar_t},
    wide::WideString
};

/// Complex character string (wide characters and renditions).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComplexString {
    inner: Vec<cchar_t>
}

impl ComplexString {
    pub fn new() -> Self {
        Self { inner: vec!() }
    }

    pub fn from_wide_string<A, P, T>(str: &WideString, attrs: &A, color_pair: &P) -> result!(Self)
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        let wch: Vec<wchar_t> = WideString::into(str.clone());
        let mut raw = vec!();

        for ch in wch {
            match crate::setcchar(u32::try_into(ch as u32)?, attrs, color_pair) {
                Err(e)    => return Err(e),
                Ok(cchar) => raw.push(ComplexChar::into(cchar))
            }
        }

        Ok(Self { inner: raw })
    }

    pub fn from_str<S, A, P, T>(str: S, attrs: &A, color_pair: &P) -> result!(Self)
        where S: Into<String>,
              A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        let mut raw = vec!();

        for ch in str.into().to_string().chars() {
            match crate::setcchar(ch, attrs, color_pair) {
                Err(e)    => return Err(e),
                Ok(cchar) => raw.push(ComplexChar::into(cchar))
            }
        }

        Ok(Self { inner: raw })
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { inner: Vec::with_capacity(capacity) }
    }

    //pub unsafe fn from_raw_parts(buf: *mut ChtypeChar, length: usize, capacity: usize) -> Self { }
    //pub unsafe fn from_chtype_unchecked<B>(bytes: B) -> Self where B: Into<Vec<chtype>> { }
    //pub fn from_ascii<B>(bytes: B) -> Result<ChtypeString, FromAsciiError<B>> where B: Into<Vec<u8>> + AsRef<[u8]> { }

    pub fn push_str(&mut self, rhs: &Self) {
        self.inner.append(&mut Self::into(rhs.to_owned()));
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }

    pub fn push(&mut self, rhs: &ComplexChar) {
        self.inner.push(ComplexChar::into(rhs.to_owned()));
    }

    pub fn truncate(&mut self, new_len: usize) {
        self.inner.truncate(new_len)
    }

    pub fn pop(&mut self) -> Option<ComplexChar> {
        self.inner.pop().map(ComplexChar::from)
    }

    pub fn remove(&mut self, idx: usize) -> ComplexChar {
        ComplexChar::from(self.inner.remove(idx))
    }

    pub fn insert(&mut self, idx: usize, ch: ComplexChar) {
        self.inner.insert(idx, ComplexChar::into(ch))
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }
}

impl Default for ComplexString {
    fn default() -> Self {
        Self::new()
    }
}

impl <'a>From<&'a Vec<ComplexChar>> for ComplexString {
    fn from(vwch: &'a Vec<ComplexChar>) -> Self {
        Self { inner: vwch.iter().map(|wch| ComplexChar::into(*wch)).collect() }
    }
}

impl Into<Vec<ComplexChar>> for ComplexString {
    fn into(self) -> Vec<ComplexChar> {
        self.inner.iter().map(|cch| ComplexChar::from(*cch)).collect()
    }
}

impl <'a>From<&'a [cchar_t]> for ComplexString {
    fn from(slice: &'a [cchar_t]) -> Self {
        Self { inner : slice.to_vec() }
    }
}

impl Into<Vec<cchar_t>> for ComplexString {
    fn into(self) -> Vec<cchar_t> {
        self.inner
    }
}

impl RawWithNul<Vec<cchar_t>> for ComplexString {
    fn raw_with_nul(self) -> Vec<cchar_t> {
        let mut raw = self.inner;

        raw.push(unsafe { std::mem::zeroed() });

        raw.to_owned()
    }
}

impl AsRef<ComplexString> for ComplexString {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<ComplexString> for ComplexString {
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}
