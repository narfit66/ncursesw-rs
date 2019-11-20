/*
    src/ncurseswerror.rs

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

use std::{num, char};
use crate::{COLORS, COLOR_PAIRS};
use menu::NCurseswMenuError;

custom_error::custom_error! {
/// NCursesw Errors/Events.
pub NCurseswError
    NCursesFunction { func: String, rc: i32 } = "ncurses::{func}(), rc={rc}",
    PanelsFunction { func: String, rc: i32 } = "npanels::{func}(), rc={rc}",
    MouseFunction { func: String, rc: i32 } = "nmouse::{func}(), rc={rc}",
    MenuFunction { func: String, error: NCurseswMenuError } = "nmenu::{func}(), error={error}",
    InterruptedCall = "interrupted system call (EINTR)",
    KeyResize = "KEY_RESIZE",
    KeyEvent = "KEY_EVENT",
    IntError { source: num::TryFromIntError } = "{source}",
    CharError { source: char::CharTryFromError } = "{source}",
    ColorParseError { color: String } = "'{color}' is not a known color",
    ColorLimit = @{ format!("Terminal only supports a maximum of {} colors", COLORS()) },
    ColorPairLimit = @{ format!("Terminal only supports a maximum of {} color pairs", COLOR_PAIRS()) },

    FOpen { fname: String, mode: String } = "bindings::fopen({fname}, {mode})",

    NulError { source: std::ffi::NulError } = "{source}"
}
