/*
    src/lccategory.rs

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

use crate::shims::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LcCategory {
    All,
    Collate,
    CType,
    Monetary,
    Numeric,
    Time,
    Messages
}

impl Into<i32> for LcCategory {
    fn into(self) -> i32 {
        match self {
            LcCategory::All      => bindings::LC_ALL as i32,
            LcCategory::Collate  => bindings::LC_COLLATE as i32,
            LcCategory::CType    => bindings::LC_CTYPE as i32,
            LcCategory::Monetary => bindings::LC_MONETARY as i32,
            LcCategory::Numeric  => bindings::LC_NUMERIC as i32,
            LcCategory::Time     => bindings::LC_TIME as i32,
            LcCategory::Messages => bindings::LC_MESSAGES as i32
        }
    }
}
