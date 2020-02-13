/*
    src/normal/attribute.rs

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

use std::{convert::Into, ops::BitOr};

use crate::{ColorPairType, normal::{Attributes, ColorPair}, shims::ncurses::attr_t};

include!("../include/attribute.rs");

/// Implement the | operator for combining two 'Attribute's into Attributes
impl BitOr for Attribute {
    type Output = Attributes;

    fn bitor(self, rhs: Attribute) -> Self::Output {
        Attributes::default() | self | rhs
    }
}

/// Implement the | operator for combining a `ColorPair` and an `Attribute` to produce `Attributes`
impl BitOr<ColorPair> for Attribute {
    type Output = Attributes;

    fn bitor(self, rhs: ColorPair) -> Self::Output {
        let attributes: attr_t = Attributes::into(Attributes::default());
        let attribute: attr_t = Self::into(self);

        Attributes::_from(rhs.screen(), attributes | attribute | rhs.as_attr_t())
    }
}
