/*
    src/normal/funcs.rs

    Copyright (c) 2021 Stephen Whittle  All rights reserved.

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

use std::{ptr, convert::TryFrom};
use crate::{
    SCREEN, NCurseswError,
    gen::{ColorType, ColorsType},
    shims::{ncurses, ncurses::short_t},
    normal::{ColorPair, Colors}
};

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
    assert!(colors.screen().is_none(), "alloc_pair() : colors.screen().is_some()");

    let number = ncurses::alloc_pair(colors.foreground().number(), colors.background().number());

    if number.is_negative() {
        Err(ncurses_function_error_with_rc!("alloc_pair", number))
    } else {
        Ok(ColorPair::_from(None, short_t::try_from(number)?))
    }
}

/// Returns a color pair if the given color combination has been associated
/// with a color pair, or `None` if not.
pub fn find_pair(colors: Colors) -> result!(Option<ColorPair>) {
    assert!(colors.screen().is_none(), "find_pair() : colors.screen().is_some()");

    let number = ncurses::find_pair(colors.foreground().number(), colors.background().number());

    Ok(if number.is_negative() {
        None
    } else {
        Some(ColorPair::_from(None, short_t::try_from(number)?))
    })
}

/// Screen function of `alloc_pair()`.
pub fn alloc_pair_sp(screen: SCREEN, colors: Colors) -> result!(ColorPair) {
    assert!(colors.screen().map_or_else(|| false, |colors_scr| ptr::eq(screen, colors_scr)), "alloc_pair_sp() : screen.is_null() || screen != colors.screen()");

    let number = unsafe { ncurses::alloc_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

    if number.is_negative() {
        Err(ncurses_function_error_with_rc!("alloc_pair_sp", number))
    } else {
        Ok(ColorPair::_from(Some(screen), short_t::try_from(number)?))
    }
}

/// Screen function of `find_pair()`.
pub fn find_pair_sp(screen: SCREEN, colors: Colors) -> result!(Option<ColorPair>) {
    assert!(colors.screen().map_or_else(|| false, |colors_scr| ptr::eq(screen, colors_scr)), "find_pair_sp() : screen.is_null() || screen != colors.screen()");

    let number = unsafe { ncurses::find_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

    Ok(if number.is_negative() {
        None
    } else {
        Some(ColorPair::_from(Some(screen), short_t::try_from(number)?))
    })
}
