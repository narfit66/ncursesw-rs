/*
    src/utils.rs

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

use semver::Version;

use lccategory::LcCategory;
use shims;
use shims::bindings;
use cstring::*;
use crate::ncurseswerror::NCurseswError;

pub fn ncurses_version() -> Version {
    Version {
        major: u64::from(bindings::NCURSES_VERSION_MAJOR),
        minor: u64::from(bindings::NCURSES_VERSION_MINOR),
        patch: u64::from(bindings::NCURSES_VERSION_PATCH),
        pre:   vec!(),
        build: vec!()
    }
}

pub fn setlocale(lc: LcCategory, locale: &str) -> result!(String) {
    Ok(shims::utils::setlocale(match lc {
            LcCategory::All      => bindings::LC_ALL,
            LcCategory::Collate  => bindings::LC_COLLATE,
            LcCategory::CType    => bindings::LC_CTYPE,
            LcCategory::Monetary => bindings::LC_MONETARY,
            LcCategory::Numeric  => bindings::LC_NUMERIC,
            LcCategory::Time     => bindings::LC_TIME,
            LcCategory::Messages => bindings::LC_MESSAGES
        } as i32,
        unsafe { c_str_with_nul!(locale) }
    ))
}
