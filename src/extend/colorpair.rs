/*
    src/extend/colorpair.rs

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

use std::convert::{From, Into};

use extend::{Colors, Color};
use gen::{ColorPairType, ColorPairGeneric, ColorPairColors};
use ncursescolortype::NCursesColorType;
use ncurseswerror::NCurseswError;
use crate::{init_extended_pair, extended_pair_content};

include!("../include/colorpair.rs");

extend_colorpair!(NCursesColorType::Extended);

/// A extended color pair.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ColorPair {
    raw: i32
}

impl ColorPair {
    /// Create a new extended color pair.
    ///
    /// ## Example
    /// ```rust
    /// extern crate ncursesw;
    ///
    /// # use std::error::Error;
    /// use ncursesw::*;
    /// use ncursesw::normal::*;
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// #     let h = initscr()?;
    /// #     if has_colors() {
    /// start_color()?;
    ///
    /// let blue = Color::Dark(BaseColor::Blue);
    /// let yellow = Color::Dark(BaseColor::Yellow);
    ///
    /// let color_pair1 = ColorPair::new(1, Colors::new(blue, yellow))?;
    ///
    /// let colors = color_pair1.colors()?;
    ///
    /// assert!(colors.foreground() == blue && colors.background() == yellow);
    /// #     }
    /// #
    /// #     delwin(h)?;
    /// #     // endwin()?;
    /// #     Ok(())
    /// # }
    /// ```
    pub fn new(pair: i32, colors: Colors) -> result!(Self) {
        init_extended_pair(pair, colors)
    }
}

/// Return the colors (foreground and background) of the color pair.
impl ColorPairColors<Colors, Color, i32> for ColorPair {
    /// ## Example
    /// ```rust
    /// extern crate ncursesw;
    ///
    /// # use std::error::Error;
    /// use ncursesw::*;
    /// use ncursesw::extend::*;
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// #     let h = initscr()?;
    /// #     if has_colors() {
    /// start_color()?;
    ///
    /// let blue = Color::Dark(BaseColor::Blue);
    /// let yellow = Color::Dark(BaseColor::Yellow);
    ///
    /// let color_pair1 = ColorPair::new(1, Colors::new(blue, yellow))?;
    ///
    /// let colors = color_pair1.colors()?;
    ///
    /// assert!(colors.foreground() == blue && colors.background() == yellow);
    /// #     }
    /// #
    /// #     delwin(h)?;
    /// #     // endwin()?;
    /// #     Ok(())
    /// # }
    /// ```
    fn colors(&self) -> result!(Colors) {
        extended_pair_content(*self)
    }
}

/// Return the number of the color pair.
impl ColorPairType<i32> for ColorPair {
    /// ## Example
    /// ```rust
    /// extern crate ncursesw;
    ///
    /// # use std::error::Error;
    /// use ncursesw::*;
    /// use ncursesw::extend::*;
    ///
    /// # fn main() -> Result<(), Box<Error>> {
    /// #     let h = initscr()?;
    /// #     if has_colors() {
    /// start_color()?;
    ///
    /// let blue = Color::Dark(BaseColor::Blue);
    /// let yellow = Color::Dark(BaseColor::Yellow);
    ///
    /// let color_pair1 = ColorPair::new(1, Colors::new(blue, yellow))?;
    ///
    /// let colors = color_pair1.colors()?;
    ///
    /// assert!(color_pair1.number() == 1);
    /// #     }
    /// #
    /// #     delwin(h)?;
    /// #     // endwin()?;
    /// #     Ok(())
    /// # }
    /// ```
    fn number(&self) -> i32 {
        self.raw
    }
}

impl ColorPairGeneric<i32> for ColorPair {
    fn as_const_ptr(&self) -> *const libc::c_void {
        let color_pair = self.number();
        let ptr: *const i32 = &color_pair;

        ptr as *const libc::c_void
    }

    fn as_mut_ptr(&self) -> *mut libc::c_void {
        let mut color_pair = self.number();
        let ptr: *mut i32 = &mut color_pair;

        ptr as *mut libc::c_void
    }
}

impl From<i32> for ColorPair {
    fn from(raw: i32) -> Self {
        Self { raw }
    }
}

impl Into<i32> for ColorPair {
    fn into(self) -> i32 {
        self.raw
    }
}
