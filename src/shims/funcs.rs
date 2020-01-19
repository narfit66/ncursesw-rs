/*
    src/shims/funcs.rs

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

#![allow(clippy::missing_safety_doc)]

use std::os::unix::io::AsRawFd;

use shims::{ncurses::FILE, bindings};
use cstring::*;

pub(in crate) unsafe fn fdopen<FD: AsRawFd>(file: FD, mode: &[i8]) -> Option<FILE> {
    bindings::fdopen(file.as_raw_fd(), mode.as_ptr()).as_mut().map(|ptr| ptr as FILE)
}

pub fn setlocale(lc: i32, locale: &[i8]) -> Option<String> {
    unsafe { (bindings::setlocale(lc, locale.as_ptr())).as_mut().map(|ptr| FromCStr::from_c_str(ptr)) }
}
