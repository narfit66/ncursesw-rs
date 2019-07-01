/*
    src/legacy.rs

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

use std::convert::{Into, TryFrom};
use ncurseswerror::NCurseswError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Legacy {
    Level0,
    Level1,
    Level2
}

impl TryFrom<i32> for Legacy {
    type Error = NCurseswError;

    fn try_from(cursor: i32) -> Result<Self, Self::Error> {
        match cursor {
            0 => Ok(Legacy::Level0),
            1 => Ok(Legacy::Level1),
            2 => Ok(Legacy::Level2),
            _ => Err(NCurseswError::NCursesFunction { func: "use_legacy_level".to_string() })
        }
    }
}

impl Into<i32> for Legacy {
    fn into(self) -> i32 {
        match self {
            Legacy::Level0 => 0,
            Legacy::Level1 => 1,
            Legacy::Level2 => 2
        }
    }
}
