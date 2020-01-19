/*
    src/legacy.rs

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Legacy {
    Level0,
    Level1,
    Level2
}

impl Legacy {
    pub(in crate) fn new(level: i32) -> Option<Self> {
        match level {
            0 => Some(Legacy::Level0),
            1 => Some(Legacy::Level1),
            2 => Some(Legacy::Level2),
            _ => None
        }
    }

    pub(in crate) fn value(self) -> i32 {
        match self {
            Legacy::Level0 => 0,
            Legacy::Level1 => 1,
            Legacy::Level2 => 2
        }
    }
}
