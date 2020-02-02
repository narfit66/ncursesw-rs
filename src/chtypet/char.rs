/*
    src/chtypet/char.rs

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

use std::{convert::{From, Into}, ops::{BitOr, BitXor}};
use ascii::AsciiChar;

use crate::{
    gen::*,
    normal::{Attribute, Attributes},
    shims::{
        constants::A_ATTRIBUTES,
        ncurses::{chtype, attr_t},
    }
};

/// Ascii character and rendition.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChtypeChar {
    raw: chtype
}

impl ChtypeChar {
    pub fn new(ch: AsciiChar) -> Self {
        Self { raw: chtype::from(ch.as_byte()) }
    }

    pub fn from_chtype(raw: chtype) -> Self {
        Self { raw }
    }

    /// Converts a Chtype character into a `u8`.
    pub fn as_byte(self) -> u8 {
        self.raw.to_be_bytes()[3]
    }

    /// Converts a Chtype character into a `char`.
    pub fn as_char(self) -> char {
        char::from(self.as_byte())
    }

    /// Converts a Chtype character into a `AsciiChar`.
    pub fn as_ascii_char(self) -> AsciiChar {
        AsciiChar::new(self.as_char())
    }

    /// Check if the character is a letter (a-z, A-Z).
    pub fn is_alphabetic(self) -> bool {
        self.as_ascii_char().is_alphabetic()
    }

    /// Check if the character is a number (0-9).
    pub fn is_digit(self) -> bool {
        self.as_ascii_char().is_ascii_digit()
    }

    /// Check if the character is a letter or number.
    pub fn is_alphanumeric(self) -> bool {
        self.as_ascii_char().is_alphanumeric()
    }

    /// Check if the character is a space or horizontal tab.
    pub fn is_blank(self) -> bool {
        self.as_ascii_char().is_ascii_blank()
    }

    /// Check if the character is a ' ', '\t', '\n' or '\r'.
    pub fn is_whitespace(self) -> bool {
        self.as_ascii_char().is_whitespace()
    }

    /// Check if the character is a control character.
    pub fn is_control(self) -> bool {
        self.as_ascii_char().is_ascii_control()
    }

    /// Get the attributes of the Chtype character.
    pub fn get_attributes(self) -> Attributes {
        Attributes::from(self.raw & A_ATTRIBUTES)
    }
}

impl BitOr<Attributes> for ChtypeChar {
    type Output = Self;

    fn bitor(self, rhs: Attributes) -> Self::Output {
        Self { raw: self.raw | rhs.as_attr_t() }
    }
}

impl BitXor<Attributes> for ChtypeChar {
    type Output = Self;

    fn bitxor(self, rhs: Attributes) -> Self::Output {
        Self { raw: self.raw ^ rhs.as_attr_t() }
    }
}

impl BitOr<Attribute> for ChtypeChar {
    type Output = Self;

    fn bitor(self, rhs: Attribute) -> Self::Output {
        let attr: attr_t = rhs.into();

        Self { raw: self.raw | attr }
    }
}

impl BitXor<Attribute> for ChtypeChar {
    type Output = Self;

    fn bitxor(self, rhs: Attribute) -> Self::Output {
        let attr: attr_t = rhs.into();

        Self { raw: self.raw ^ attr }
    }
}

impl From<chtype> for ChtypeChar {
    fn from(raw: chtype) -> Self {
        Self { raw }
    }
}

impl Into<chtype> for ChtypeChar {
    fn into(self) -> chtype {
        self.raw.to_owned()
    }
}

#[test]
fn chtype_char_test() {
    assert_eq!(ChtypeChar::new(AsciiChar::new('s')).as_ascii_char(), AsciiChar::new('s'));
}

#[test]
fn chtype_char_get_attributes() {
    let ch = ChtypeChar::new(AsciiChar::new('s'));
    let mut attrs = Attributes::default();

    attrs.set_bold(true);
    attrs.set_dim(true);

    let c = ch | attrs;

    assert_eq!(c.get_attributes(), attrs);
    assert_eq!(c.get_attributes().is_bold(), true);
    assert_eq!(attrs.is_bold(), true);
}
