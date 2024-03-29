/*
    src/cursortype.rs

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

/// Cursor type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CursorType {
    /// Makes the cursor invisible. Supported on most terminals.
    Invisible,
    /// Makes the cursor visible.
    Visible,
    /// Makes the cursor "highly" visible in some way. Not supported on all terminals.
    VeryVisible
}

impl CursorType {
    pub(in crate) fn new(cursor: i32) -> Option<Self> {
        match cursor {
            0 => Some(CursorType::Invisible),
            1 => Some(CursorType::Visible),
            2 => Some(CursorType::VeryVisible),
            _ => None
        }
    }

    pub(in crate) fn value(self) -> i32 {
        match self {
            CursorType::Invisible   => 0,
            CursorType::Visible     => 1,
            CursorType::VeryVisible => 2
        }
    }
}

impl Default for CursorType {
    /// The default cursor type
    fn default() -> Self {
        CursorType::Visible
    }
}
