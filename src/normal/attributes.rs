/*
    src/normal/attributes.rs

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

#![allow(deprecated)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::from_over_into)]

use crate::{
    normal::{Attribute, ColorPair},
    gen::ColorPairType,
    ncurses::{COLOR_PAIR, PAIR_NUMBER},
    shims::ncurses::short_t
};

include!("../include/attributes.rs");

impl_attributes_type!(short_t);

impl Attributes {
    /// Return the `ColorPair` associated with the `Attributes`.
    pub fn color_pair(&self) -> ColorPair {
        ColorPair::_from(self.screen, PAIR_NUMBER(self.raw))
    }
}

/// Implement the | operator for setting a `ColorPair` on a `Attributes`.
///
/// Note: as only one color pair can be applied to attributes at any one time any previously Or'd
/// color_pair will be Xor'd out of the attributes before Or'ing the new color pair.
impl BitOr<ColorPair> for Attributes {
    type Output = Attributes;

    fn bitor(mut self, rhs: ColorPair) -> Self::Output {
        assert!(self.screen == rhs.screen());

        self.raw ^= COLOR_PAIR(i32::from(self.color_pair().number()));
        self.raw |= COLOR_PAIR(i32::from(rhs.number()));

        self
    }
}

/// Implement the ^ operator for removing a `ColorPair` on a `Attributes`.
impl BitXor<ColorPair> for Attributes {
    type Output = Self;

    fn bitxor(mut self, rhs: ColorPair) -> Self::Output {
        assert!(self.screen == rhs.screen());

        self.raw ^= COLOR_PAIR(i32::from(rhs.number()));

        self
    }
}

impl Into<i32> for Attributes {
    fn into(self) -> i32 {
        self.raw as i32
    }
}
