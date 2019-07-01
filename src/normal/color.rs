/*
    src/normal/color.rs

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
use normal::rgb::RGB;
use shims::ncurses::short_t;
use shims::constants::{COLOR_BLACK, COLOR_RED, COLOR_GREEN, COLOR_YELLOW, COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE};
use crate::{init_color, color_content};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i16)]
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
    Custom(short_t)
}

impl Color {
    pub fn new(number: short_t, rgb: RGB) -> result!(Self) {
        init_color(number, rgb)
    }

    pub fn rgb(&self) -> result!(RGB) {
        color_content(*self)
    }
}

impl ColorType<short_t> for Color {
    fn number(&self) -> i32 {
        let number: short_t = Self::into(*self);

        i32::from(number)
    }
}

impl From<short_t> for Color {
    fn from(color: short_t) -> Self {
        match color {
            -1            => Color::Default,
            COLOR_BLACK   => Color::Black,
            COLOR_RED     => Color::Red,
            COLOR_GREEN   => Color::Green,
            COLOR_YELLOW  => Color::Yellow,
            COLOR_BLUE    => Color::Blue,
            COLOR_MAGENTA => Color::Magenta,
            COLOR_CYAN    => Color::Cyan,
            COLOR_WHITE   => Color::White,
            n             => Color::Custom(n)
        }
    }
}

impl Into<short_t> for Color {
    fn into(self) -> short_t {
        match self {
            Color::Default   => -1,
            Color::Black     => COLOR_BLACK,
            Color::Red       => COLOR_RED,
            Color::Green     => COLOR_GREEN,
            Color::Yellow    => COLOR_YELLOW,
            Color::Blue      => COLOR_BLUE,
            Color::Magenta   => COLOR_MAGENTA,
            Color::Cyan      => COLOR_CYAN,
            Color::White     => COLOR_WHITE,
            Color::Custom(n) => n
        }
    }
}
