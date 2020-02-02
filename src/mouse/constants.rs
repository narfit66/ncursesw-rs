/*
    src/mouse/constants.rs

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

use crate::shims::bindings;

wrap_const!(NCURSES_MOUSE_VERSION: i32);

const MASK_SHIFT: i32     = 7 - NCURSES_MOUSE_VERSION;
const MODIFIER_SHIFT: i32 = 4 + NCURSES_MOUSE_VERSION;

// Mouse Support
macro_rules! ncurses_mouse_mask( ($b: expr, $m: expr) => ($m << (($b - 1) * MASK_SHIFT)); );

wrap_const!(NCURSES_BUTTON_RELEASED: i32);
wrap_const!(NCURSES_BUTTON_PRESSED: i32);
wrap_const!(NCURSES_BUTTON_CLICKED: i32);
wrap_const!(NCURSES_DOUBLE_CLICKED: i32);
wrap_const!(NCURSES_TRIPLE_CLICKED: i32);
wrap_const!(NCURSES_RESERVED_EVENT: i32);

// Event masks
pub const BUTTON1_RELEASED: i32       = ncurses_mouse_mask!(1, NCURSES_BUTTON_RELEASED);
pub const BUTTON1_PRESSED: i32        = ncurses_mouse_mask!(1, NCURSES_BUTTON_PRESSED);
pub const BUTTON1_CLICKED: i32        = ncurses_mouse_mask!(1, NCURSES_BUTTON_CLICKED);
pub const BUTTON1_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(1, NCURSES_DOUBLE_CLICKED);
pub const BUTTON1_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(1, NCURSES_TRIPLE_CLICKED);

pub const BUTTON2_RELEASED: i32       = ncurses_mouse_mask!(2, NCURSES_BUTTON_RELEASED);
pub const BUTTON2_PRESSED: i32        = ncurses_mouse_mask!(2, NCURSES_BUTTON_PRESSED);
pub const BUTTON2_CLICKED: i32        = ncurses_mouse_mask!(2, NCURSES_BUTTON_CLICKED);
pub const BUTTON2_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(2, NCURSES_DOUBLE_CLICKED);
pub const BUTTON2_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(2, NCURSES_TRIPLE_CLICKED);

pub const BUTTON3_RELEASED: i32       = ncurses_mouse_mask!(3, NCURSES_BUTTON_RELEASED);
pub const BUTTON3_PRESSED: i32        = ncurses_mouse_mask!(3, NCURSES_BUTTON_PRESSED);
pub const BUTTON3_CLICKED: i32        = ncurses_mouse_mask!(3, NCURSES_BUTTON_CLICKED);
pub const BUTTON3_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(3, NCURSES_DOUBLE_CLICKED);
pub const BUTTON3_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(3, NCURSES_TRIPLE_CLICKED);

pub const BUTTON4_RELEASED: i32       = ncurses_mouse_mask!(4, NCURSES_BUTTON_RELEASED);
pub const BUTTON4_PRESSED: i32        = ncurses_mouse_mask!(4, NCURSES_BUTTON_PRESSED);
pub const BUTTON4_CLICKED: i32        = ncurses_mouse_mask!(4, NCURSES_BUTTON_CLICKED);
pub const BUTTON4_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(4, NCURSES_DOUBLE_CLICKED);
pub const BUTTON4_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(4, NCURSES_TRIPLE_CLICKED);

pub const BUTTON5_RELEASED: i32       = ncurses_mouse_mask!(5, NCURSES_BUTTON_RELEASED);
pub const BUTTON5_PRESSED: i32        = ncurses_mouse_mask!(5, NCURSES_BUTTON_PRESSED);
pub const BUTTON5_CLICKED: i32        = ncurses_mouse_mask!(5, NCURSES_BUTTON_CLICKED);
pub const BUTTON5_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(5, NCURSES_DOUBLE_CLICKED);
pub const BUTTON5_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(5, NCURSES_TRIPLE_CLICKED);

pub const BUTTON_CTRL: i32            = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x001);
pub const BUTTON_SHIFT: i32           = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x002);
pub const BUTTON_ALT: i32             = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x004);

pub const REPORT_MOUSE_POSITION: i32  = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x008);
pub const ALL_MOUSE_EVENTS: i32       = REPORT_MOUSE_POSITION - 1;
