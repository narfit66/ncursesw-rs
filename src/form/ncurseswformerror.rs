/*
    src/form/ncurseswformerror.rs

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

/// NCursesw form errors.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum NCurseswFormError {
    /// Routine detected an incorrect or out-of-range argument.
    #[error("nform::{func}() : bad argument")]
    BadArgument { func: String },
    #[error("nform::{func}() : bad state")]
    BadState { func: String },
    /// Field is connected to a form.
    #[error("nform::{func}() : connected")]
    Connected { func: String },
    #[error("nform::{func}() : current")]
    Current { func: String },
    #[error("nform::{func}() : invalid field")]
    InvalidField { func: String },
    #[error("nform::{func}() : not connected")]
    NotConnected { func: String },
    #[error("nform::{func}() : not posted")]
    NotPosted { func: String },
    #[error("nform::{func}() : not selectable")]
    NotSelectable { func: String },
    #[error("nform::{func}() : no match")]
    NoMatch { func: String },
    #[error("nform::{func}() : no room")]
    NoRoom { func: String },
    /// The routine succeeded.
    #[error("nform::{func}() : ok",)]
    Ok { func: String },
    #[error("nform::{func}() : posted")]
    Posted { func: String },
    #[error("nform::{func}() : request denied")]
    RequestDenied { func: String },
    /// System error occurred, (see errno)
    #[error("nform::{}() : {} (#{})", func, errno, errno.0)]
    SystemError { func: String, errno: Errno },
    #[error("nform::{func}() : unknown command")]
    UnknownCommand { func: String },
    #[error("nform::{func} : error number {errno}")]
    UnknownError { func: String, errno: i32 },

    #[error("{source}")]
    IntError { #[from] source: num::TryFromIntError },
    #[error("{source}")]
    NulError { #[from] source: ffi::NulError },
    #[error("{source}")]
    Infallible { #[from] source: convert::Infallible }
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
