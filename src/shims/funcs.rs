/*
    src/shims/funcs.rs

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

#![allow(clippy::missing_safety_doc)]

use bindings;
use crate::cstring::*;

type FILE = *mut bindings::FILE;

pub unsafe fn fopen(path: &[i8], mode: &[i8]) -> Option<FILE> {
    bindings::fopen(path.as_ptr(), mode.as_ptr()).as_mut().map(|ptr| ptr as FILE)
}

pub unsafe fn setlocale(lc: i32, locale: &[i8]) -> Option<String> {
    (bindings::setlocale(lc, locale.as_ptr()) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
}
