/*
    src/menu/ncurseswmenuerror.rs

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

use std::{num, ffi, convert};

use errno::{Errno, errno};
use thiserror::Error;

use crate::shims::constants;

/// NCursesw menu errors.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum NCurseswMenuError {
    /// Routine detected an incorrect or out-of-range argument.
    #[error("nmenu::{func}() : bad argument")]
    BadArgument { func: String },
    #[error("nmenu::{func}() : bad state")]
    BadState { func: String },
    /// Item is connected to a menu.
    #[error("nmenu::{func}() : connected")]
    Connected { func: String },
    #[error("nmenu::{func}() : current")]
    Current { func: String },
    #[error("nmenu::{func}() : invalid field")]
    InvalidField { func: String },
    #[error("nmenu::{func}() : not connected")]
    NotConnected { func: String },
    #[error("nmenu::{func}() : not connected")]
    NotPosted { func: String },
    #[error("nmenu::{func}() : not selectable")]
    NotSelectable { func: String },
    #[error("nmenu::{func}() : no match")]
    NoMatch { func: String },
    #[error("nmenu::{func}() : no room")]
    NoRoom { func: String },
    /// The routine succeeded.
    #[error("nmenu::{func}() : ok")]
    Ok { func: String },
    #[error("nmenu::{func}() : posted")]
    Posted { func: String },
    #[error("nmenu::{func}() : request denied")]
    RequestDenied { func: String },
    /// System error occurred, (see errno)
    #[error("nmenu::{}() : {} (#{})", func, errno, errno.0)]
    SystemError { func: String, errno: Errno },
    #[error("nmenu::{func}() : unknown command")]
    UnknownCommand { func: String },
    #[error("nmenu::{func} : error number {errno}")]
    UnknownError { func: String, errno: i32 },

    #[error("{source}")]
    IntError { #[from] source: num::TryFromIntError },
    #[error("{source}")]
    NulError { #[from] source: ffi::NulError },
    #[error("{source}")]
    Infallible { #[from] source: convert::Infallible }
}

pub fn ncursesw_menu_error_from_rc(func: &str, err: i32) -> NCurseswMenuError {
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
