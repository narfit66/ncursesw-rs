/*
    src/extend/color.rs

    Copyright (c) 2019-2021 Stephen Whittle  All rights reserved.

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

#![allow(deprecated)]

use crate::{
    NCurseswError,
    gen::ColorType,
    ncursescolortype::{set_ncurses_colortype, NCursesColorType},
    shims::ncurses::SCREEN,
    extend::{ColorPalette, RGB},
    ncurses::{
        init_extended_color, extended_color_content,
        init_extended_color_sp, extended_color_content_sp
    }
};

/// A terminal color.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    screen:        Option<SCREEN>,
    color_palette: ColorPalette
}

impl Color {
    pub(in crate) fn _from(screen: Option<SCREEN>, color_palette: ColorPalette) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "Color::_from() : screen.is_null()");

        set_ncurses_colortype(NCursesColorType::Extend);

        Self { screen, color_palette }
    }
}

impl Color {
    pub fn new(color_palette: ColorPalette) -> Self {
        Self::_from(None, color_palette)
    }

    pub fn new_sp(screen: SCREEN, color_palette: ColorPalette) -> Self {
        Self::_from(Some(screen), color_palette)
    }

    /// Set the screen of the `Color`.
    ///
    /// Use with caution!!! This function only need's to be used if using the screen type
    /// functions and is provided to allow the alignment of the screen pointer with the
    /// screen that the `ColorPair` are for as this crate will apply a screen of `None`
    /// by default when retriving `Attributes` from functions such as `attr_get()` and
    /// `wattr_get()`.
    pub unsafe fn set_screen(&mut self, screen: Option<SCREEN>) {
        self.screen = screen
    }

    pub fn screen(&self) -> Option<SCREEN> {
        self.screen
    }

    pub fn color_palette(&self) -> ColorPalette {
        self.color_palette
    }

    pub fn set_rgb(&self, rgb: RGB) -> result!(()) {
        match self.screen {
            None         => init_extended_color(self.color_palette.number(), rgb),
            Some(screen) => init_extended_color_sp(screen, self.color_palette.number(), rgb)
        }
    }

    pub fn rgb(&self) -> result!(RGB) {
        match self.screen {
            None         => extended_color_content(self.color_palette.number()),
            Some(screen) => extended_color_content_sp(screen, self.color_palette.number())
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::_from(None, ColorPalette::default())
    }
}

impl ColorType<i32> for Color {
    fn number(&self) -> i32 {
        self.color_palette.number()
    }
}
