/*
    src/extend/color.rs

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

#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(deprecated)]

use std::convert::{From, Into};

use crate::{
    basecolor::BaseColor,
    gen::ColorType,
    ncurseswerror::NCurseswError,
    ncursescolortype::*,
    extend::rgb::RGB,
    shims::{ncurses::SCREEN, constants::COLOR_WHITE},
    ncurses::{
        init_extended_color, extended_color_content,
        init_extended_color_sp, extended_color_content_sp
    }
};

const EXT_COLOR_WHITE: i32 = COLOR_WHITE as i32;
const LIGHT_COLOR_OFFSET: i32 = EXT_COLOR_WHITE + 1;

include!("../include/color.rs");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Color {
    TerminalDefault,
    Dark(BaseColor),
    Light(BaseColor),
    Custom(i32)
}

impl Color {
    pub fn new(number: i32, rgb: RGB) -> result!(Self) {
        let color = init_extended_color(number, rgb)?;

        set_ncurses_colortype(NCursesColorType::Extended);

        Ok(color)
    }

    pub fn new_sp(screen: SCREEN, number: i32, rgb: RGB) -> result!(Self) {
        let color = init_extended_color_sp(screen, number, rgb)?;

        set_ncurses_colortype(NCursesColorType::Extended);

        Ok(color)
    }

    pub fn rgb(&self) -> result!(RGB) {
        extended_color_content(*self)
    }

    pub fn rgb_sp(&self, screen: SCREEN) -> result!(RGB) {
        extended_color_content_sp(screen, *self)
    }
}

impl ColorType<i32> for Color {
    fn number(&self) -> i32 {
        Self::into(*self)
    }
}

impl From<i32> for Color {
    fn from(color: i32) -> Self {
        if color == -1 {
            Color::default()
        } else if color <= EXT_COLOR_WHITE {
            Color::Dark(BaseColor::from_i32(color))
        } else if color <= EXT_COLOR_WHITE + LIGHT_COLOR_OFFSET {
            Color::Light(BaseColor::from_i32(color - LIGHT_COLOR_OFFSET))
        } else {
            Color::Custom(color)
        }
    }
}

impl Into<i32> for Color {
    fn into(self) -> i32 {
        match self {
            Color::TerminalDefault => -1,
            Color::Dark(color)     => i32::from(color.dark()),
            Color::Light(color)    => i32::from(color.light()),
            Color::Custom(n)       => n
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        set_ncurses_colortype(NCursesColorType::Extended);

        Color::TerminalDefault
    }
}
