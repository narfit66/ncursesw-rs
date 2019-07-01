/*
    src/wide/string.rs

    Copyright (c) 2019 Stephen Whittle  All rights reserved.

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

#![allow(clippy::should_implement_trait)]

use std::convert::{From, Into};

use gen::*;
use shims::ncurses::wchar_t;
use wide::WideChar;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WideString {
    raw: Vec<wchar_t>
}

impl WideString {
    pub fn new() -> Self {
        Self { raw: vec!() }
    }

    pub fn from_str(str: &str) -> Self {
        Self { raw: str.chars().map(|c| u32::from(c) as wchar_t).collect() }
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

    pub fn push(&mut self, rhs: WideChar) {
        self.raw.push(WideChar::into(rhs.to_owned()));
    }

    pub fn truncate(&mut self, new_len: usize) {
        self.raw.truncate(new_len)
    }

    pub fn pop(&mut self) -> Option<WideChar> {
        match self.raw.pop() {
            None    => None,
            Some(c) => Some(WideChar::from(c))
        }
    }

    pub fn remove(&mut self, idx: usize) -> WideChar {
        WideChar::from(self.raw.remove(idx))
    }

    pub fn insert(&mut self, idx: usize, ch: WideChar) {
        self.raw.insert(idx, WideChar::into(ch))
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

impl Default for WideString {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> From<&'a Vec<WideChar>> for WideString {
    fn from(vwch: &'a Vec<WideChar>) -> Self {
        Self { raw: vwch.iter().map(|wch| WideChar::into(*wch)).collect() }
    }
}

impl<'a> From<&'a [wchar_t]> for WideString {
    fn from(slice: &'a [wchar_t]) -> Self {
        Self { raw : slice.to_vec() }
    }
}

impl Into<Vec<wchar_t>> for WideString {
    fn into(self) -> Vec<wchar_t> {
        self.raw.clone()
    }
}

impl RawWithNul<Vec<wchar_t>> for WideString {
    fn raw_with_nul(self) -> Vec<wchar_t> {
        let mut raw: Vec<wchar_t> = Self::into(self);
        raw.push(0x00);

        raw
    }
}
