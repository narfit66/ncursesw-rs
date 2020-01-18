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

use std::{num, char, ffi};
use errno::Errno;
use crate::{
    COLORS, COLOR_PAIRS, panels::NCurseswPanelsError, mouse::NCurseswMouseError,
    menu::NCurseswMenuError, form::NCurseswFormError
};

custom_error::custom_error! {
/// NCursesw Errors/Events.
#[derive(PartialEq, Eq)]
pub NCurseswError
    LibraryError { func: String, rc: i32 } = "ncurses::{func}(), rc={rc}",
    InterruptedCall = "interrupted system call (EINTR)",
    KeyResize = "KEY_RESIZE",
    KeyEvent = "KEY_EVENT",
    ColorParseError { color: String } = "'{color}' is not a known color",
    ColorLimit = @{ format!("Terminal only supports a maximum of {} colors", COLORS()) },
    ColorPairLimit = @{ format!("Terminal only supports a maximum of {} color pairs", COLOR_PAIRS()) },

    IntError { source: num::TryFromIntError } = "{source}",
    CharError { source: char::CharTryFromError } = "{source}",
    NulError { source: ffi::NulError } = "{source}",

    // Error types for internal module errors.

    PanelsError { source: NCurseswPanelsError } = "{source}",
    MouseError { source: NCurseswMouseError } = "{source}",
    MenuError { source: NCurseswMenuError } = "{source}",
    FormError { source: NCurseswFormError } = "{source}",

    OSError { func: String, errno: Errno} = @{ format!("{}() : {} (#{})", func, errno, errno.0) },
    FOpen { fname: String, mode: String } = "bindings::fopen({fname}, {mode})"
}
