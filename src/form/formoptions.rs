/*
    src/form/formoptions.rs

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

use std::{convert::{From, Into}, ops::{BitOr, BitXor}};

use form::FormOption;
use shims::constants;

/// Form options.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FormOptions {
    raw: i32
}

impl FormOptions {
    option_getter!(is_newline_overload, O_NL_OVERLOAD);
    option_setter!(set_newline_overload, O_NL_OVERLOAD);

    option_getter!(is_backspace_overload, O_BS_OVERLOAD);
    option_setter!(set_backspace_overload, O_BS_OVERLOAD);
}

impl Default for FormOptions {
    fn default() -> Self {
        Self { raw: 0 }
    }
}

/// Implement the | operator for adding FormOption to FormOptions
impl BitOr for FormOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw | rhs.raw }
    }
}

/// Implement the ^ operator for removing FormOption from FormOptions
impl BitXor for FormOptions {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw ^ rhs.raw }
    }
}

/// Implement the | operator for adding an FormOption to FormOptions
impl BitOr<FormOption> for FormOptions {
    type Output = Self;

    fn bitor(mut self, rhs: FormOption) -> Self::Output {
        match rhs {
            FormOption::NewlineOverload => self.set_newline_overload(true),
            FormOption::BackspaceOverload => self.set_backspace_overload(true)
        }

        self
    }
}

/// Implement the ^ operator for disabling an FormOption from FormOptions
impl BitXor<FormOption> for FormOptions {
    type Output = Self;

    fn bitxor(mut self, rhs: FormOption) -> Self::Output {
        match rhs {
            FormOption::NewlineOverload => self.set_newline_overload(false),
            FormOption::BackspaceOverload => self.set_backspace_overload(false)
        }

        self
    }
}

impl From<FormOption> for FormOptions {
    fn from(form_option: FormOption) -> Self {
        Self::default() | form_option
    }
}

impl From<i32> for FormOptions {
    fn from(raw: i32) -> Self {
        Self { raw }
    }
}

impl Into<i32> for FormOptions {
    fn into(self) -> i32 {
        self.raw
    }
}
