/*
    src/attributescolorpairset.rs

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

use crate::{normal, extend};

/// Normal and extended attributes and color pair returned by ncurses functions.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AttributesColorPairSet {
    /// `normal` attributes and color pair.
    Normal(normal::AttributesColorPair),
    /// extended attributes and color pair.
    Extend(extend::AttributesColorPair)
}

impl AttributesColorPairSet {
    /// Unwrap `self` as a `normal::AttributesColorPair` or `panic`.
    pub fn unwrap_as_normal(&self) -> normal::AttributesColorPair {
        if let AttributesColorPairSet::Normal(attributes_colorpair) = *self {
            attributes_colorpair
        } else {
            panic!("failed to unwrap AttributesColorPairSet::Normal()!!!")
        }
    }

    /// Unwrap `self` as a `extend::AttributesColorPair` or `panic`.
    pub fn unwrap_as_extend(&self) -> extend::AttributesColorPair {
        if let AttributesColorPairSet::Extend(attributes_colorpair) = *self {
            attributes_colorpair
        } else {
            panic!("failed to unwrap AttributesColorPairSet::Extended()!!!")
        }
    }
}
