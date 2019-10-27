/*
    src/basecolor.rs

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

use std::str::FromStr;

use ncurseswerror::NCurseswError;
use shims::constants::{
    COLOR_BLACK, COLOR_RED, COLOR_GREEN, COLOR_YELLOW, COLOR_BLUE,
    COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE
};
use shims::ncurses::short_t;

/// One of the 8 base colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
pub enum BaseColor {
    /// Black color
    ///
    /// Color #0
    Black,
    /// Red color
    ///
    /// Color #1
    Red,
    /// Green color
    ///
    /// Color #2
    Green,
    /// Yellow color (Red + Green)
    ///
    /// Color #3
    Yellow,
    /// Blue color
    ///
    /// Color #4
    Blue,
    /// Magenta color (Red + Blue)
    ///
    /// Color #5
    Magenta,
    /// Cyan color (Green + Blue)
    ///
    /// Color #6
    Cyan,
    /// White color (Red + Green + Blue)
    ///
    /// Color #7
    White
}

impl BaseColor {
    pub(in crate) fn from_short_t(color: short_t) -> Self {
        match color {
            COLOR_BLACK   => BaseColor::Black,
            COLOR_RED     => BaseColor::Red,
            COLOR_GREEN   => BaseColor::Green,
            COLOR_YELLOW  => BaseColor::Yellow,
            COLOR_BLUE    => BaseColor::Blue,
            COLOR_MAGENTA => BaseColor::Magenta,
            COLOR_CYAN    => BaseColor::Cyan,
            COLOR_WHITE   => BaseColor::White,
            _             => unreachable!()
        }
    }

    pub(in crate) fn from_i32(color: i32) -> Self {
        Self::from_short_t(color as short_t)
    }

    /// Return the `dark` value of the base color (the first 8 color's are considered dark colors).
    pub(in crate) fn dark(&self) -> i16 {
        match self {
            BaseColor::Black   => COLOR_BLACK,
            BaseColor::Red     => COLOR_RED,
            BaseColor::Green   => COLOR_GREEN,
            BaseColor::Yellow  => COLOR_YELLOW,
            BaseColor::Blue    => COLOR_BLUE,
            BaseColor::Magenta => COLOR_MAGENTA,
            BaseColor::Cyan    => COLOR_CYAN,
            BaseColor::White   => COLOR_WHITE
        }
    }

    /// Return the `light` value of the base color (the first 8 color's are considered dark colors, the next 8 are the lighter/brighter equivilants).
    pub(in crate) fn light(&self) -> i16 {
        self.dark() + COLOR_WHITE + 1
    }
}

impl FromStr for BaseColor {
    type Err = NCurseswError;

    /// Parse a string to instance a base color.
    ///
    /// Valid values are 'black', 'red', 'green', 'yellow', 'blue', 'magenta', 'cyan', 'white'
    ///
    /// ## Example
    /// ```rust
    /// # extern crate ncursesw;
    /// #
    /// # use std::error::Error;
    /// # use ncursesw::*;
    /// use std::str::FromStr;
    ///
    /// #
    /// # fn main() -> Result<(), Box<Error>> {
    /// let red = BaseColor::from_str("red")?;
    /// let green = BaseColor::from_str("green")?;
    /// let blue = BaseColor::from_str("blue")?;
    ///
    /// assert!(red == BaseColor::Red);
    /// assert!(green == BaseColor::Green);
    /// assert!(blue == BaseColor::Blue);
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    fn from_str(color: &str) -> Result<Self, Self::Err> {
        match color {
            "black"   => Ok(BaseColor::Black),
            "red"     => Ok(BaseColor::Red),
            "green"   => Ok(BaseColor::Green),
            "yellow"  => Ok(BaseColor::Yellow),
            "blue"    => Ok(BaseColor::Blue),
            "magenta" => Ok(BaseColor::Magenta),
            "cyan"    => Ok(BaseColor::Cyan),
            "white"   => Ok(BaseColor::White),
            _         => Err(NCurseswError::ColorParseError { color: color.to_string() })
        }
    }
}
