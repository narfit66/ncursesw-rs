/*
    src/menu/itemoptions.rs

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

use std::ops::{BitOr, BitXor};
use crate::{menu::ItemOption, shims::constants};

/// Menu item options.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ItemOptions {
    raw: i32
}

impl ItemOptions {
    option_getter!(is_selectable, O_SELECTABLE);
    option_setter!(set_selectable, O_SELECTABLE);
}

/// Implement the | operator for adding ItemOptions to ItemOptions
impl BitOr for ItemOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw | rhs.raw }
    }
}

/// Implement the ^ operator for removing ItemOptions from ItemOptions
impl BitXor for ItemOptions {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw ^ rhs.raw }
    }
}

/// Implement the | operator for adding an ItemOption to ItemOptions
impl BitOr<ItemOption> for ItemOptions {
    type Output = Self;

    fn bitor(self, rhs: ItemOption) -> Self::Output {
        match rhs {
            ItemOption::Selectable => self.set_selectable(true)
        }
    }
}

/// Implement the ^ operator for disabling an ItemOption from ItemOptions
impl BitXor<ItemOption> for ItemOptions {
    type Output = Self;

    fn bitxor(self, rhs: ItemOption) -> Self::Output {
        match rhs {
            ItemOption::Selectable => self.set_selectable(false)
        }
    }
}

impl From<ItemOption> for ItemOptions {
    fn from(item_option: ItemOption) -> Self {
        Self::default() | item_option
    }
}

impl From<i32> for ItemOptions {
    fn from(raw: i32) -> Self {
        Self { raw }
    }
}

impl Into<i32> for ItemOptions {
    fn into(self) -> i32 {
        self.raw
    }
}
