/*
    src/normal/color.rs

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

#![allow(deprecated)]

use crate::{
    NCurseswError,
    gen::ColorType,
    ncursescolortype::*,
    normal::{ColorPalette, RGB},
    shims::ncurses::{short_t, SCREEN},
    ncurses::{
        init_color, color_content, init_color_sp, color_content_sp
    }
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    screen:        Option<SCREEN>,
    color_palette: ColorPalette
}

impl Color {
    pub(in crate) fn _from(screen: Option<SCREEN>, color_palette: ColorPalette) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "Color::_from() : screen.is_null()");

        set_ncurses_colortype(NCursesColorType::Normal);

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

    pub fn screen(&self) -> Option<SCREEN> {
        self.screen
    }

    pub fn color_palette(&self) -> ColorPalette {
        self.color_palette
    }

    pub fn set_rgb(&self, rgb: RGB) -> result!(()) {
        self.screen.map_or_else(|| init_color(self.color_palette.number(), rgb), |screen| init_color_sp(screen, self.color_palette.number(), rgb))
    }

    pub fn rgb(&self) -> result!(RGB) {
        self.screen.map_or_else(|| color_content(self.color_palette.number()), |screen| color_content_sp(screen, self.color_palette.number()))
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::_from(None, ColorPalette::default())
    }
}

impl ColorType<short_t> for Color {
    fn number(&self) -> i32 {
        i32::from(self.color_palette.number())
    }
}
