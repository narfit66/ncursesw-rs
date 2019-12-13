/*
    src/menu/menuoptions.rs

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

use menu::MenuOption;
use shims::constants;

/// Menu options.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MenuOptions {
    raw: i32
}

impl MenuOptions {
    option_getter!(is_one_value, O_ONEVALUE);
    option_setter!(set_one_value, O_ONEVALUE);

    option_getter!(is_show_description, O_SHOWDESC);
    option_setter!(set_show_description, O_SHOWDESC);

    option_getter!(is_row_major, O_ROWMAJOR);
    option_setter!(set_row_major, O_ROWMAJOR);

    option_getter!(is_ignore_case, O_IGNORECASE);
    option_setter!(set_ignore_case, O_IGNORECASE);

    option_getter!(is_show_match, O_SHOWMATCH);
    option_setter!(set_show_match, O_SHOWMATCH);

    option_getter!(is_non_cyclic, O_NONCYCLIC);
    option_setter!(set_non_cyclic, O_NONCYCLIC);

    option_getter!(is_mouse_menu, O_MOUSE_MENU);
    option_setter!(set_mouse_menu, O_MOUSE_MENU);
}

impl Default for MenuOptions {
    fn default() -> Self {
        Self { raw: 0 }
    }
}

/// Implement the | operator for adding ItemOptions to ItemOptions
impl BitOr for MenuOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw | rhs.raw }
    }
}

/// Implement the ^ operator for removing ItemOptions from ItemOptions
impl BitXor for MenuOptions {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw ^ rhs.raw }
    }
}

/// Implement the | operator for adding an ItemOption to ItemOptions
impl BitOr<MenuOption> for MenuOptions {
    type Output = Self;

    fn bitor(mut self, rhs: MenuOption) -> Self::Output {
        match rhs {
            MenuOption::OneValue        => self.set_one_value(true),
            MenuOption::ShowDescription => self.set_show_description(true),
            MenuOption::RowMajor        => self.set_row_major(true),
            MenuOption::IgnoreCase      => self.set_ignore_case(true),
            MenuOption::ShowMatch       => self.set_show_match(true),
            MenuOption::NonCyclic       => self.set_non_cyclic(true),
            MenuOption::MouseMenu       => self.set_mouse_menu(true)
        }

        self
    }
}

/// Implement the ^ operator for disabling an ItemOption from ItemOptions
impl BitXor<MenuOption> for MenuOptions {
    type Output = Self;

    fn bitxor(mut self, rhs: MenuOption) -> Self::Output {
        match rhs {
            MenuOption::OneValue        => self.set_one_value(false),
            MenuOption::ShowDescription => self.set_show_description(false),
            MenuOption::RowMajor        => self.set_row_major(false),
            MenuOption::IgnoreCase      => self.set_ignore_case(false),
            MenuOption::ShowMatch       => self.set_show_match(false),
            MenuOption::NonCyclic       => self.set_non_cyclic(false),
            MenuOption::MouseMenu       => self.set_mouse_menu(false)
        }

        self
    }
}

impl From<MenuOption> for MenuOptions {
    fn from(menu_option: MenuOption) -> Self {
        Self::default() | menu_option
    }
}

impl From<i32> for MenuOptions {
    fn from(raw: i32) -> Self {
        Self { raw }
    }
}

impl Into<i32> for MenuOptions {
    fn into(self) -> i32 {
        self.raw
    }
}
