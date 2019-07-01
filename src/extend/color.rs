/*
    src/extend/color.rs

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

use gen::ColorType;
use ncurseswerror::NCurseswError;
use extend::rgb::RGB;
use shims::constants::{EXT_COLOR_BLACK, EXT_COLOR_RED, EXT_COLOR_GREEN, EXT_COLOR_YELLOW, EXT_COLOR_BLUE, EXT_COLOR_MAGENTA, EXT_COLOR_CYAN, EXT_COLOR_WHITE};
use crate::{init_extended_color, extended_color_content};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Custom(i32)
}

impl Color {
    pub fn new(number: i32, rgb: RGB) -> result!(Self) {
        init_extended_color(number, rgb)
    }

    pub fn rgb(&self) -> result!(RGB) {
        extended_color_content(*self)
    }
}

impl ColorType<i32> for Color {
    fn number(&self) -> i32 {
        Self::into(*self)
    }
}

impl From<i32> for Color {
    fn from(color: i32) -> Self {
        match color {
            -1                => Color::Default,
            EXT_COLOR_BLACK   => Color::Black,
            EXT_COLOR_RED     => Color::Red,
            EXT_COLOR_GREEN   => Color::Green,
            EXT_COLOR_YELLOW  => Color::Yellow,
            EXT_COLOR_BLUE    => Color::Blue,
            EXT_COLOR_MAGENTA => Color::Magenta,
            EXT_COLOR_CYAN    => Color::Cyan,
            EXT_COLOR_WHITE   => Color::White,
            n                 => Color::Custom(n)
        }
    }
}

impl Into<i32> for Color {
    fn into(self) -> i32 {
        match self {
            Color::Default   => -1,
            Color::Black     => EXT_COLOR_BLACK,
            Color::Red       => EXT_COLOR_RED,
            Color::Green     => EXT_COLOR_GREEN,
            Color::Yellow    => EXT_COLOR_YELLOW,
            Color::Blue      => EXT_COLOR_BLUE,
            Color::Magenta   => EXT_COLOR_MAGENTA,
            Color::Cyan      => EXT_COLOR_CYAN,
            Color::White     => EXT_COLOR_WHITE,
            Color::Custom(n) => n
        }
    }
}
