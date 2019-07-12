/*
    src/normal/colorpair.rs

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

#![allow(clippy::trivially_copy_pass_by_ref)]

use std::convert::{From, Into};
use std::ops::BitOr;

use gen::{ColorPairType, ColorPairGeneric, ColorPairColors};
use normal::{Attribute, Attributes, Colors, Color};
use ncurseswerror::NCurseswError;
use shims::ncurses::{attr_t, short_t};
use crate::{COLOR_PAIR, PAIR_NUMBER, init_pair, pair_content};

include!("../include/colorpair.rs");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColorPair {
    raw: short_t
}

impl ColorPair {
    pub fn new(pair: short_t, colors: Colors) -> result!(Self) {
        init_pair(pair, colors)
    }

    pub(crate) fn as_attr_t(&self) -> attr_t {
        COLOR_PAIR(*self)
    }
}

impl ColorPairColors<Colors, Color, short_t> for ColorPair {
    fn colors(&self) -> result!(Colors) {
        pair_content(*self)
    }
}

impl ColorPairType<short_t> for ColorPair {
    fn number(&self) -> short_t {
        self.raw
    }
}

impl ColorPairGeneric<short_t> for ColorPair {
    fn as_short_t(&self) -> short_t {
        self.number()
    }
}

/// Implement the | operator for combining a `ColorPair` and a `Attribute` to produce `Attributes`
impl BitOr<Attribute> for ColorPair {
    type Output = Attributes;

    fn bitor(self, rhs: Attribute) -> Self::Output {
        Attributes::default() | self | rhs
    }
}

impl From<short_t> for ColorPair {
    fn from(raw: short_t) -> Self {
        Self { raw }
    }
}

impl Into<short_t> for ColorPair {
    fn into(self) -> short_t {
        self.raw
    }
}

impl From<Attributes> for ColorPair {
    fn from(attrs: Attributes) -> Self {
        PAIR_NUMBER(attrs)
    }
}

impl From<i32> for ColorPair {
    fn from(raw: i32) -> Self {
        Self { raw: raw as short_t }
    }
}

impl Into<i32> for ColorPair {
    fn into(self) -> i32 {
        i32::from(self.raw)
    }
}
