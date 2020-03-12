/*
    src/form/fieldoptions.rs

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

use std::ops::{BitOr, BitXor};
use crate::{form::FieldOption, shims::constants};

/// Field options.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FieldOptions {
    raw: i32
}

impl FieldOptions {
    option_getter!(is_visible, O_VISIBLE);
    option_setter!(set_visible, O_VISIBLE);

    option_getter!(is_active, O_ACTIVE);
    option_setter!(set_active, O_ACTIVE);

    option_getter!(is_public, O_PUBLIC);
    option_setter!(set_public, O_PUBLIC);

    option_getter!(is_edit, O_EDIT);
    option_setter!(set_edit, O_EDIT);

    option_getter!(is_wrap, O_WRAP);
    option_setter!(set_wrap, O_WRAP);

    option_getter!(is_blank, O_BLANK);
    option_setter!(set_blank, O_BLANK);

    option_getter!(is_auto_skip, O_AUTOSKIP);
    option_setter!(set_auto_skip, O_AUTOSKIP);

    option_getter!(is_null_ok, O_NULLOK);
    option_setter!(set_null_ok, O_NULLOK);

    option_getter!(is_pass_ok, O_PASSOK);
    option_setter!(set_pass_ok, O_PASSOK);

    option_getter!(is_static, O_STATIC);
    option_setter!(set_static, O_STATIC);

    option_getter!(is_dynamic_justify, O_DYNAMIC_JUSTIFY);
    option_setter!(set_dynamic_justify, O_DYNAMIC_JUSTIFY);

    option_getter!(is_no_left_strip, O_NO_LEFT_STRIP);
    option_setter!(set_no_left_strip, O_NO_LEFT_STRIP);
}

/// Implement the | operator for adding FieldOption to FieldOptions
impl BitOr for FieldOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw | rhs.raw }
    }
}

/// Implement the ^ operator for removing FieldOption from FieldOptions
impl BitXor for FieldOptions {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw ^ rhs.raw }
    }
}

/// Implement the | operator for adding an FieldOption to FieldOptions
impl BitOr<FieldOption> for FieldOptions {
    type Output = Self;

    fn bitor(self, rhs: FieldOption) -> Self::Output {
        match rhs {
            FieldOption::Visible        => self.set_visible(true),
            FieldOption::Active         => self.set_active(true),
            FieldOption::Public         => self.set_public(true),
            FieldOption::Edit           => self.set_edit(true),
            FieldOption::Wrap           => self.set_wrap(true),
            FieldOption::Blank          => self.set_blank(true),
            FieldOption::AutoSkip       => self.set_auto_skip(true),
            FieldOption::NullOk         => self.set_null_ok(true),
            FieldOption::PassOk         => self.set_pass_ok(true),
            FieldOption::Static         => self.set_static(true),
            FieldOption::DynamicJustify => self.set_dynamic_justify(true),
            FieldOption::NoLeftStrip    => self.set_no_left_strip(true)
        }
    }
}

/// Implement the ^ operator for disabling an FieldOption from FieldOptions
impl BitXor<FieldOption> for FieldOptions {
    type Output = Self;

    fn bitxor(self, rhs: FieldOption) -> Self::Output {
        match rhs {
            FieldOption::Visible        => self.set_visible(false),
            FieldOption::Active         => self.set_active(false),
            FieldOption::Public         => self.set_public(false),
            FieldOption::Edit           => self.set_edit(false),
            FieldOption::Wrap           => self.set_wrap(false),
            FieldOption::Blank          => self.set_blank(false),
            FieldOption::AutoSkip       => self.set_auto_skip(false),
            FieldOption::NullOk         => self.set_null_ok(false),
            FieldOption::PassOk         => self.set_pass_ok(false),
            FieldOption::Static         => self.set_static(false),
            FieldOption::DynamicJustify => self.set_dynamic_justify(false),
            FieldOption::NoLeftStrip    => self.set_no_left_strip(false)
        }
    }
}

impl From<FieldOption> for FieldOptions {
    fn from(field_option: FieldOption) -> Self {
        Self::default() | field_option
    }
}

impl From<i32> for FieldOptions {
    fn from(raw: i32) -> Self {
        Self { raw }
    }
}

impl Into<i32> for FieldOptions {
    fn into(self) -> i32 {
        self.raw
    }
}
