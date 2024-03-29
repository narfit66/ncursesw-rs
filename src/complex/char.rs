/*
    src/complex/char.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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

#![allow(clippy::from_over_into)]

use std::convert::TryInto;
use crate::{
    gen::{AttributesType, ColorPairType, ColorAttributeTypes},
    ncurseswerror::NCurseswError,
    shims::bindings::cchar_t,
    wide::WideChar,
    ncurses::{setcchar, wunctrl}
};

/// Complex character (wide character and rendition).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComplexChar {
    inner: cchar_t
}

impl ComplexChar {
    pub fn from_wide_char<A, P, T>(ch: WideChar, attrs: &A, color_pair: &P) -> result!(Self)
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        setcchar(WideChar::try_into(ch)?, attrs, color_pair)
    }

    pub fn from_char<A, P, T>(ch: char, attrs: &A, color_pair: &P) -> result!(Self)
        where A: AttributesType<T>,
              P: ColorPairType<T>,
              T: ColorAttributeTypes
    {
        setcchar(ch, attrs, color_pair)
    }

    pub fn as_wide_char(self) -> result!(WideChar) {
        wunctrl(self)
    }
}

impl From<cchar_t> for ComplexChar {
    fn from(raw: cchar_t) -> Self {
        Self { inner: raw }
    }
}

impl Into<cchar_t> for ComplexChar {
    fn into(self) -> cchar_t {
        self.inner
    }
}
