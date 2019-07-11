/*
    src/normal/attributes.rs

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

use normal::{Attribute, ColorPair};
use crate::{COLOR_PAIR, PAIR_NUMBER};

use shims::ncurses::short_t;

include!("../include/attributes.rs");

impl_attributes_type!(short_t);

impl Attributes {
    pub fn color_pair(&self) -> ColorPair {
        PAIR_NUMBER(*self)
    }
}

/// Implement the | operator for setting a color pair on an `Attributes` object
///
/// Note: as only one color pair can be applied to attributes at any one time any previously Or'd
/// color_pair will be Xor'd out of the attributes before Or'ing the new color pair..
impl BitOr<ColorPair> for Attributes {
    type Output = Attributes;

    fn bitor(mut self, rhs: ColorPair) -> Self::Output {
        self.raw ^= COLOR_PAIR(self.color_pair());
        self.raw |= COLOR_PAIR(rhs);

        self
    }
}

impl BitXor<ColorPair> for Attributes {
    type Output = Self;

    fn bitxor(mut self, rhs: ColorPair) -> Self::Output {
        self.raw ^= COLOR_PAIR(rhs);

        self
    }
}

impl Into<i32> for Attributes {
    fn into(self) -> i32 {
        self.raw as i32
    }
}
