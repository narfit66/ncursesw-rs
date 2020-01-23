/*
    src/extend/colorpair.rs

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

use std::convert::Into;

use shims::ncurses;
use extend::{Colors, Color};
use gen::{ColorPairType, ColorPairGeneric, ColorPairColors};
use gen::{ColorType, ColorsType};
use ncursescolortype::*;
use ncurseswerror::NCurseswError;
use crate::{
    SCREEN,
    init_extended_pair, extended_pair_content,
    init_extended_pair_sp, extended_pair_content_sp
};

/// A extended color pair.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ColorPair {
    screen: Option<SCREEN>,
    number: i32
}

impl ColorPair {
    pub(in crate) fn _from(screen: Option<SCREEN>, number: i32) -> Self {
        set_ncurses_colortype(NCursesColorType::Extended);

        Self { screen, number }
    }
}

impl ColorPair {
    pub fn new(pair: i32, colors: Colors) -> result!(Self) {
        init_extended_pair(pair, colors)
    }

    pub fn new_sp(screen: SCREEN, pair: i32, colors: Colors) -> result!(Self) {
        init_extended_pair_sp(screen, pair, colors)
    }

    pub fn screen(&self) -> Option<SCREEN> {
        self.screen
    }

    pub fn default_sp(screen: SCREEN) -> Self {
        Self::_from(Some(screen), 0)
    }
}

impl ColorPairColors<Colors, Color, i32> for ColorPair {
    fn colors(&self) -> result!(Colors) {
        if let Some(sp) = self.screen {
            extended_pair_content_sp(sp, *self)
        } else {
            extended_pair_content(*self)
        }
    }
}

impl ColorPairType<i32> for ColorPair {
    fn number(&self) -> i32 {
        self.number
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

impl Into<i32> for ColorPair {
    fn into(self) -> i32 {
        self.number
    }
}

pub fn alloc_pair(colors: Colors) -> result!(ColorPair) {
    let number = ncurses::alloc_pair(colors.foreground().number(), colors.background().number());

    if number.is_negative() {
        Err(ncurses_function_error_with_rc!("alloc_pair", number))
    } else {
        Ok(ColorPair::_from(None, number))
    }
}

pub fn find_pair(colors: Colors) -> Option<ColorPair> {
    let number = ncurses::find_pair(colors.foreground().number(), colors.background().number());

    if number.is_negative() {
        None
    } else {
        Some(ColorPair::_from(None, number))
    }
}

pub fn alloc_pair_sp(screen: ncurses::SCREEN, colors: Colors) -> result!(ColorPair) {
    let number = unsafe { ncurses::alloc_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

    if number.is_negative() {
        Err(ncurses_function_error_with_rc!("alloc_pair_sp", number))
    } else {
        Ok(ColorPair::_from(Some(screen), number))
    }
}

pub fn find_pair_sp(screen: ncurses::SCREEN, colors: Colors) -> Option<ColorPair> {
    let number = unsafe { ncurses::find_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

    if number.is_negative() {
        None
    } else {
        Some(ColorPair::_from(Some(screen), number))
    }
}
