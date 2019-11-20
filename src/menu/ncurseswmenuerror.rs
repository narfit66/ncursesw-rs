/*
    src/menu/ncurseswmenuerror.rs

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


use errno::{Errno, errno};
use shims::constants;

custom_error::custom_error! {
/// NCursesw menu errors.
pub NCurseswMenuError
    /// Routine detected an incorrect or out-of-range argument.
    BadArgument { func: String } = "nmenu::{func}() : bad argument",
    BadState { func: String } = "nmenu::{func}() : bad state",
    /// Item is connected to a menu.
    Connected { func: String } = "nmenu::{func}() : connected",
    Current { func: String } = "nmenu::{func}() : current",
    InvalidField { func: String } = "nmenu::{func}() : invalid field",
    NotConnected { func: String } = "nmenu::{func}() : not connected",
    NotPosted { func: String } = "nmenu::{func}() : not posted",
    NotSelectable { func: String } = "nmenu::{func}() : not selectable",
    NoMatch { func: String } = "nmenu::{func}() : no match",
    NoRoom { func: String } = "nmenu::{func}() : no room",
    /// The routine succeeded.
    Ok { func: String } = "nmenu::{func}() : ok",
    Posted { func: String } = "nmenu::{func}() : posted",
    RequestDenied { func: String } = "nmenu::{func}() : request denied",
    /// System error occurred, (see errno)
    SystemError { func: String, errno: Errno } = @{ format!("nmenu::{}() : {} (#{})", func, errno, errno.0) },
    UnknownCommand { func: String } = "nmenu::{func}() : unknown command",
    UnknownError { func: String, errno: i32 } = "nmenu::{func} : error number {errno}",

    NulError { source: std::ffi::NulError } = "{source}"
}

pub(in crate::menu) fn ncursesw_menu_error_from_rc(func: &str, err: i32) -> NCurseswMenuError {
    let func = func.to_string();

    match err {
        constants::E_BAD_ARGUMENT    => NCurseswMenuError::BadArgument { func },
        constants::E_BAD_STATE       => NCurseswMenuError::BadState { func },
        constants::E_CONNECTED       => NCurseswMenuError::Connected { func },
        constants::E_CURRENT         => NCurseswMenuError::Current { func },
        constants::E_INVALID_FIELD   => NCurseswMenuError::InvalidField { func },
        constants::E_NOT_CONNECTED   => NCurseswMenuError::NotConnected { func },
        constants::E_NOT_POSTED      => NCurseswMenuError::NotPosted { func },
        constants::E_NOT_SELECTABLE  => NCurseswMenuError::NotSelectable { func },
        constants::E_NO_MATCH        => NCurseswMenuError::NoMatch { func },
        constants::E_NO_ROOM         => NCurseswMenuError::NoRoom { func },
        constants::E_OK              => NCurseswMenuError::Ok { func },
        constants::E_POSTED          => NCurseswMenuError::Posted { func },
        constants::E_REQUEST_DENIED  => NCurseswMenuError::RequestDenied { func },
        constants::E_SYSTEM_ERROR    => NCurseswMenuError::SystemError { func, errno: errno() },
        constants::E_UNKNOWN_COMMAND => NCurseswMenuError::UnknownCommand { func },
        _                            => NCurseswMenuError::UnknownError { func, errno: err }
    }
}

pub(in crate::menu) fn ncursesw_menu_error_system_error(func: &str) -> NCurseswMenuError {
    NCurseswMenuError::SystemError { func: func.to_string(), errno: errno() }
}
