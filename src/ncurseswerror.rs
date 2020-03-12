/*
    src/ncurseswerror.rs

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

use std::{num, char, ffi, convert};
use errno::{errno, Errno};
use thiserror::Error;
use crate::{
    COLORS, COLOR_PAIRS, panels::NCurseswPanelsError, mouse::NCurseswMouseError,
    menu::NCurseswMenuError, form::NCurseswFormError
};

/// NCursesw Errors/Events.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum NCurseswError {
    #[error("ncurses::{func}(){}{}", rc_error(*rc), os_level_error())]
    LibraryError { func: String, rc: Option<i32> },
    #[error("interrupted system call (EINTR)")]
    InterruptedCall,
    #[error("KEY_RESIZE")]
    KeyResize,
    #[error("KEY_EVENT")]
    KeyEvent,
    #[error("'{color}' is not a known color")]
    ColorParseError { color: String },
    #[error("Terminal only supports a maximum of {} colors", COLORS())]
    ColorLimit,
    #[error("Terminal only supports a maximum of {} color pairs", COLOR_PAIRS())]
    ColorPairLimit,
    #[error("Invalid capability")]
    InvalidCapability,

    #[error("{source}")]
    IntError { #[from] source: num::TryFromIntError },
    #[error("{source}")]
    CharError { #[from] source: char::CharTryFromError },
    #[error("{source}")]
    NulError { #[from] source: ffi::NulError },
    #[error("{source}")]
    Infallible { #[from] source: convert::Infallible },

    // Error types for internal module errors.

    #[error("{source}")]
    PanelsError { #[from] source: NCurseswPanelsError },
    #[error("{source}")]
    MouseError { #[from] source: NCurseswMouseError },
    #[error("{source}")]
    MenuError { #[from] source: NCurseswMenuError },
    #[error("{source}")]
    FormError { #[from] source: NCurseswFormError },

    #[error("{}() : {} (#{})", func, errno, errno.0)]
    OSError { func: String, errno: Errno}
}

pub(in crate) fn rc_error(rc: Option<i32>) -> String {
    rc.map_or_else(|| String::new(), |rc| format!(", rc={}", rc))
}

pub(in crate) fn os_level_error() -> String {
    if errno().0 == 0 {
        String::new()
    } else {
        format!(", os_level={} ({})", errno().0, errno())
    }
}
