/*
    src/normal/colorpair.rs

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

use std::{convert::{TryFrom, From, Into}, ops::BitOr};

use crate::{
    gen::{ColorType, ColorsType, ColorPairType, ColorPairGeneric, ColorPairColors},
    normal::{Attribute, Attributes, Colors, Color},
    ncursescolortype::*,
    ncurseswerror::NCurseswError,
    shims::{ncurses, ncurses::{SCREEN, attr_t, short_t}},
    ncurses::{
        COLOR_PAIR, PAIR_NUMBER,
        init_pair, pair_content,
        init_pair_sp, pair_content_sp
    }
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColorPair {
    screen: Option<SCREEN>,
    number: short_t
}

impl ColorPair {
    pub(in crate) fn _from(screen: Option<SCREEN>, number: short_t) -> Self {
        set_ncurses_colortype(NCursesColorType::Normal);

        Self { screen, number }
    }
}

impl ColorPair {
    pub fn new(pair: short_t, colors: Colors) -> result!(Self) {
        init_pair(pair, colors)
    }

    pub fn new_sp(screen: SCREEN, pair: short_t, colors: Colors) -> result!(Self) {
        init_pair_sp(screen, pair, colors)
    }

    pub fn screen(&self) -> Option<SCREEN> {
        self.screen
    }

    pub fn set_screen(&mut self, screen: Option<SCREEN>) {
        self.screen = screen
    }

    pub(in crate) fn as_attr_t(&self) -> attr_t {
        COLOR_PAIR(*self)
    }

    pub fn default_sp(screen: SCREEN) -> Self {
        Self::_from(Some(screen), 0)
    }
}

impl ColorPairColors<Colors, Color, short_t> for ColorPair {
    fn colors(&self) -> result!(Colors) {
        if let Some(sp) = self.screen {
            pair_content_sp(sp, *self)
        } else {
            pair_content(*self)
        }
    }
}

impl ColorPairType<short_t> for ColorPair {
    fn number(&self) -> short_t {
        self.number
    }
}

impl ColorPairGeneric<short_t> for ColorPair {
    fn as_short_t(&self) -> short_t {
        self.number()
    }
}

impl BitOr<Attribute> for ColorPair {
    type Output = Attributes;

    fn bitor(self, rhs: Attribute) -> Self::Output {
        Attributes::default() | self | rhs
    }
}

impl Into<short_t> for ColorPair {
    fn into(self) -> short_t {
        self.number
    }
}

impl From<Attributes> for ColorPair {
    fn from(attrs: Attributes) -> Self {
        PAIR_NUMBER(attrs)
    }
}

impl Into<i32> for ColorPair {
    fn into(self) -> i32 {
        i32::from(self.number)
    }
}

pub fn alloc_pair(colors: Colors) -> result!(ColorPair) {
    let number = ncurses::alloc_pair(colors.foreground().number(), colors.background().number());

    if number.is_negative() {
        Err(ncurses_function_error_with_rc!("alloc_pair", number))
    } else {
        Ok(ColorPair::_from(None, short_t::try_from(number)?))
    }
}

pub fn find_pair(colors: Colors) -> result!(Option<ColorPair>) {
    let number = ncurses::find_pair(colors.foreground().number(), colors.background().number());

    Ok(if number.is_negative() {
        None
    } else {
        Some(ColorPair::_from(None, short_t::try_from(number)?))
    })
}

pub fn alloc_pair_sp(screen: ncurses::SCREEN, colors: Colors) -> result!(ColorPair) {
    let number = unsafe { ncurses::alloc_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

    if number.is_negative() {
        Err(ncurses_function_error_with_rc!("alloc_pair_sp", number))
    } else {
        Ok(ColorPair::_from(Some(screen), short_t::try_from(number)?))
    }
}

pub fn find_pair_sp(screen: ncurses::SCREEN, colors: Colors) -> result!(Option<ColorPair>) {
    let number = unsafe { ncurses::find_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

    Ok(if number.is_negative() {
        None
    } else {
        Some(ColorPair::_from(Some(screen), short_t::try_from(number)?))
    })
}
