/*
    src/cursortype.rs

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

use std::convert::{TryFrom, Into};

use ncurseswerror::NCurseswError;

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

impl Default for CursorType {
    /// The default cursor type
    ///
    /// ## Example
    /// ```rust
    /// extern crate ncursesw;
    ///
    /// use ncursesw::*;
    ///
    /// assert_eq!(CursorType::default(), CursorType::Visible);
    /// ```
    fn default() -> Self {
        CursorType::Visible
    }
}

impl TryFrom<i32> for CursorType {
    type Error = NCurseswError;

    fn try_from(cursor: i32) -> Result<Self, Self::Error> {
        match cursor {
            0 => Ok(CursorType::Invisible),
            1 => Ok(CursorType::Visible),
            2 => Ok(CursorType::VeryVisible),
            _ => Err(NCurseswError::InternalError)
        }
    }
}

impl Into<i32> for CursorType {
    fn into(self) -> i32 {
        match self {
            CursorType::Invisible   => 0,
            CursorType::Visible     => 1,
            CursorType::VeryVisible => 2
        }
    }
}
