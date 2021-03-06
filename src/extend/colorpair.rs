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

use crate::{
    shims::{ncurses, ncurses::SCREEN},
    extend::{Colors, Color},
    gen::{ColorPairType, ColorPairGeneric, ColorPairColors, ColorType, ColorsType},
    ncursescolortype::*,
    ncurseswerror::NCurseswError,
    ncurses::{
        init_extended_pair, extended_pair_content,
        init_extended_pair_sp, extended_pair_content_sp
    }
};

/// A color pair comprising of a foreground and background color.
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
        self.screen.map_or_else(|| extended_pair_content(*self), |screen| extended_pair_content_sp(screen, *self))
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

/// Accepts a parameter for foreground and background color, and checks if
/// that color combination is already associated with a color pair, returning
/// an existing color pair or a new color pair.
///
/// - If the combination already exists, returns the existing pair.
/// - If the combination does not exist, allocates a new color pair and
///   returns that.
/// - If the table fills up, discards the least-recently allocated entry
///   and allocates a new color pair.
///
/// All of the color pairs are allocated from a table of possible color pairs.
/// The size of the table is determined by the terminfo pairs capability.
pub fn alloc_pair(colors: Colors) -> result!(ColorPair) {
    let number = ncurses::alloc_pair(colors.foreground().number(), colors.background().number());

    if number.is_negative() {
        Err(ncurses_function_error_with_rc!("alloc_pair", number))
    } else {
        Ok(ColorPair::_from(None, number))
    }
}

/// Returns a color pair if the given color combination has been associated
/// with a color pair, or `None` if not.
pub fn find_pair(colors: Colors) -> Option<ColorPair> {
    let number = ncurses::find_pair(colors.foreground().number(), colors.background().number());

    if number.is_negative() {
        None
    } else {
        Some(ColorPair::_from(None, number))
    }
}

/// Screen function of `alloc_pair()`.
pub fn alloc_pair_sp(screen: SCREEN, colors: Colors) -> result!(ColorPair) {
    let number = unsafe { ncurses::alloc_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

    if number.is_negative() {
        Err(ncurses_function_error_with_rc!("alloc_pair_sp", number))
    } else {
        Ok(ColorPair::_from(Some(screen), number))
    }
}

/// Screen function of `find_pair()`.
pub fn find_pair_sp(screen: SCREEN, colors: Colors) -> Option<ColorPair> {
    let number = unsafe { ncurses::find_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

    if number.is_negative() {
        None
    } else {
        Some(ColorPair::_from(Some(screen), number))
    }
}
