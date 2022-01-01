/*
    src/extend/colorpair.rs

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

use std::ptr;
use crate::{
    NCurseswError,
    gen::{ColorPairType, ColorPairGeneric, ColorPairColors, ColorsType},
    ncursescolortype::{set_ncurses_colortype, NCursesColorType},
    shims::ncurses::SCREEN,
    extend::{Colors, Color},
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
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "ColorPair::_from() : screen.is_null()");

        set_ncurses_colortype(NCursesColorType::Extend);

        Self { screen, number }
    }
}

impl ColorPair {
    pub fn new(pair: i32, colors: Colors) -> result!(Self) {
        assert!(colors.screen().is_none(), "ColorPair::new() : colors.screen().is_some()");

        init_extended_pair(pair, colors)
    }

    pub fn new_sp(screen: SCREEN, pair: i32, colors: Colors) -> result!(Self) {
        assert!(colors.screen().map_or_else(|| false, |colors_screen| ptr::eq(screen, colors_screen)), "ColorPair::new_sp() : screen.is_null() || screen != colors.screen()");

        init_extended_pair_sp(screen, pair, colors)
    }

    /// # Safety
    ///
    /// Set the screen of the `ColorPair`.
    ///
    /// Use with caution!!! This function only need's to be used if using the screen type
    /// functions and is provided to allow the alignment of the screen pointer with the
    /// screen that the `ColorPair` are for as this crate will apply a screen of `None`
    /// by default when retriving `Attributes` from functions such as `attr_get()` and
    /// `wattr_get()`.
    pub unsafe fn set_screen(&mut self, screen: Option<SCREEN>) {
        self.screen = screen
    }
}

impl ColorPairColors<Colors, Color, i32> for ColorPair {
    fn colors(&self) -> result!(Colors) {
        match self.screen {
            None         => extended_pair_content(self.number),
            Some(screen) => extended_pair_content_sp(screen, self.number)
        }
    }
}

impl ColorPairType<i32> for ColorPair {
    fn screen(&self) -> Option<SCREEN> {
        self.screen
    }

    fn number(&self) -> i32 {
        self.number
    }
}

impl ColorPairGeneric<i32> for ColorPair {
    fn as_const_ptr(&self) -> *const libc::c_void {
        let color_pair = self.number;
        let ptr: *const i32 = &color_pair;

        ptr as *const libc::c_void
    }

    fn as_mut_ptr(&self) -> *mut libc::c_void {
        let mut color_pair = self.number;
        let ptr: *mut i32 = &mut color_pair;

        ptr as *mut libc::c_void
    }
}
