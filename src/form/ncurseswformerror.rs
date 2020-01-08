/*
    src/form/ncurseswformerror.rs

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

use std::{num, ffi, convert};

use errno::{Errno, errno};
use shims::constants;

custom_error::custom_error! {
/// NCursesw menu errors.
#[derive(PartialEq, Eq)]
pub NCurseswFormError
    /// Routine detected an incorrect or out-of-range argument.
    BadArgument { func: String } = "nform::{func}() : bad argument",
    BadState { func: String } = "nform::{func}() : bad state",
    /// Item is connected to a menu.
    Connected { func: String } = "nform::{func}() : connected",
    Current { func: String } = "nform::{func}() : current",
    InvalidField { func: String } = "nform::{func}() : invalid field",
    NotConnected { func: String } = "nform::{func}() : not connected",
    NotPosted { func: String } = "nform::{func}() : not posted",
    NotSelectable { func: String } = "nform::{func}() : not selectable",
    NoMatch { func: String } = "nform::{func}() : no match",
    NoRoom { func: String } = "nform::{func}() : no room",
    /// The routine succeeded.
    Ok { func: String } = "nform::{func}() : ok",
    Posted { func: String } = "nform::{func}() : posted",
    RequestDenied { func: String } = "nform::{func}() : request denied",
    /// System error occurred, (see errno)
    SystemError { func: String, errno: Errno } = @{ format!("nform::{}() : {} (#{})", func, errno, errno.0) },
    UnknownCommand { func: String } = "nform::{func}() : unknown command",
    UnknownError { func: String, errno: i32 } = "nform::{func} : error number {errno}",

    IntError { source: num::TryFromIntError } = "{source}",
    NulError { source: ffi::NulError } = "{source}",
    Infallible { source: convert::Infallible } = "{source}"
}

pub fn ncursesw_form_error_from_rc(func: &str, err: i32) -> NCurseswFormError {
    let func = func.to_string();

    match err {
        constants::E_BAD_ARGUMENT    => NCurseswFormError::BadArgument { func },
        constants::E_BAD_STATE       => NCurseswFormError::BadState { func },
        constants::E_CONNECTED       => NCurseswFormError::Connected { func },
        constants::E_CURRENT         => NCurseswFormError::Current { func },
        constants::E_INVALID_FIELD   => NCurseswFormError::InvalidField { func },
        constants::E_NOT_CONNECTED   => NCurseswFormError::NotConnected { func },
        constants::E_NOT_POSTED      => NCurseswFormError::NotPosted { func },
        constants::E_NOT_SELECTABLE  => NCurseswFormError::NotSelectable { func },
        constants::E_NO_MATCH        => NCurseswFormError::NoMatch { func },
        constants::E_NO_ROOM         => NCurseswFormError::NoRoom { func },
        constants::E_OK              => NCurseswFormError::Ok { func },
        constants::E_POSTED          => NCurseswFormError::Posted { func },
        constants::E_REQUEST_DENIED  => NCurseswFormError::RequestDenied { func },
        constants::E_SYSTEM_ERROR    => NCurseswFormError::SystemError { func, errno: errno() },
        constants::E_UNKNOWN_COMMAND => NCurseswFormError::UnknownCommand { func },
        _                            => NCurseswFormError::UnknownError { func, errno: err }
    }
}

pub(in crate::form) fn ncursesw_form_error_system_error(func: &str) -> NCurseswFormError {
    NCurseswFormError::SystemError { func: func.to_string(), errno: errno() }
}
