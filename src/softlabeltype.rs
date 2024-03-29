/*
    src/softlabeltype.rs

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

#![allow(clippy::trivially_copy_pass_by_ref)]

/// The soft-label layout type
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SoftLabelType {
    /// a three-two-three layout
    ThreeTwoThree,
    /// A four-four layout
    FourFour,
    /// A four-four layout with an index
    FourFourFour,
    /// A four-four layout with an index
    FourFourFourIndex
}

impl SoftLabelType {
    pub(in crate) fn value(&self) -> i32 {
        match self {
            SoftLabelType::ThreeTwoThree     => 0,
            SoftLabelType::FourFour          => 1,
            SoftLabelType::FourFourFour      => 2,
            SoftLabelType::FourFourFourIndex => 3
        }
    }

    /// Returns the minimum label number.
    pub fn min_label(&self) -> i32 {
        1
    }

    /// Returns the maximum label number.
    pub fn max_label(&self) -> i32 {
        if *self == SoftLabelType::ThreeTwoThree || *self == SoftLabelType::FourFour {
            8
        } else {
            12
        }
    }

    /// Returns the maximum label length.
    pub fn max_label_len(&self) -> i32 {
        if *self == SoftLabelType::ThreeTwoThree || *self == SoftLabelType::FourFour {
            8
        } else {
            5
        }
    }
}
