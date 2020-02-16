/*
    src/include/attributes.rs

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

use std::{fmt, ops::{BitOr, BitXor}};

use crate::{
    gen::AttributesType,
    shims::{ncurses::{SCREEN, attr_t}, constants}
};

macro_rules! attributes_getter {
    ($func: ident, $attr: ident) => {
        pub fn $func(&self) -> bool {
            (self.raw & constants::$attr) > 0
        }
    };
}

macro_rules! attributes_setter {
    ($func: ident, $attr: ident) => {
        pub fn $func(&self, enabled: bool) -> Self {
            Self::_from(self.screen, if enabled {
                self.raw | constants::$attr
            } else {
                self.raw ^ constants::$attr
            })
        }
    };
}

macro_rules! impl_attributes_type {
    ($type: ty) => {
        impl AttributesType<$type> for Attributes {
            fn screen(&self) -> Option<SCREEN> {
                self.screen
            }

            fn as_attr_t(&self) -> attr_t {
                self.raw
            }
        }
    };
}

/// Attributes.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Attributes {
    screen: Option<SCREEN>,
    raw:    attr_t
}

impl Attributes {
    pub(in crate) fn _from(screen: Option<SCREEN>, raw: attr_t) -> Self {
        assert!(screen.map_or_else(|| true, |screen| !screen.is_null()), "Attributes::_from() : screen.is_null()");

        Self { screen, raw }
    }
}

impl Attributes {
    /// Set the screen pointer of the `Attributes`.
    ///
    /// Use with caution!!! This function only need's to be used if using the screen type
    /// functions and is provided to allow the alignment of the screen pointer with the
    /// screen that the `Attributes` are for as this crate will apply a screen of `None`
    /// by default when retriving `Attributes` from functions such as `attr_get()` and
    /// `wattr_get()`.
    pub fn set_screen(&mut self, screen: Option<SCREEN>) {
        self.screen = screen
    }

    pub fn is_normal(&self) -> bool {
        self.raw == constants::A_NORMAL
    }

    pub fn set_normal(&self) -> Self {
        Self::_from(self.screen, constants::A_NORMAL)
    }

    attributes_getter!(is_char_text, A_CHARTEXT);
    attributes_setter!(set_char_text, A_CHARTEXT);

    attributes_getter!(is_standout, A_STANDOUT);
    attributes_setter!(set_standout, A_STANDOUT);

    attributes_getter!(is_underline, A_UNDERLINE);
    attributes_setter!(set_underline, A_UNDERLINE);

    attributes_getter!(is_reverse, A_REVERSE);
    attributes_setter!(set_reverse, A_REVERSE);

    attributes_getter!(is_blink, A_BLINK);
    attributes_setter!(set_blink, A_BLINK);

    attributes_getter!(is_dim, A_DIM);
    attributes_setter!(set_dim, A_DIM);

    attributes_getter!(is_bold, A_BOLD);
    attributes_setter!(set_bold, A_BOLD);

    attributes_getter!(is_alternate_char_set, A_ALTCHARSET);
    attributes_setter!(set_alternative_char_set, A_ALTCHARSET);

    attributes_getter!(is_invisible, A_INVIS);
    attributes_setter!(set_invisible, A_INVIS);

    attributes_getter!(is_protected, A_PROTECT);
    attributes_setter!(set_protected, A_PROTECT);

    attributes_getter!(is_horizontal, A_HORIZONTAL);
    attributes_setter!(set_horizontal, A_HORIZONTAL);

    attributes_getter!(is_left, A_LEFT);
    attributes_setter!(set_left, A_LEFT);

    attributes_getter!(is_low, A_LOW);
    attributes_setter!(set_low, A_LOW);

    attributes_getter!(is_right, A_RIGHT);
    attributes_setter!(set_right, A_RIGHT);

    attributes_getter!(is_top, A_TOP);
    attributes_setter!(set_top, A_TOP);

    attributes_getter!(is_vertical, A_VERTICAL);
    attributes_setter!(set_vertical, A_VERTICAL);

    attributes_getter!(is_italic, A_ITALIC);
    attributes_setter!(set_italic, A_ITALIC);
}

/// Implement the | operator for adding Attributes to Attributes
impl BitOr for Attributes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert!(self.screen == rhs.screen);

        Self::_from(self.screen, self.raw | rhs.raw)
    }
}

/// Implement the ^ operator for removing Attributes from Attributes
impl BitXor for Attributes {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert!(self.screen == rhs.screen);

        Self::_from(self.screen, self.raw ^ rhs.raw)
    }
}

/// Implement the | operator for adding an Attribute to Attributes
impl BitOr<Attribute> for Attributes {
    type Output = Self;

    fn bitor(self, rhs: Attribute) -> Self::Output {
        match rhs {
            Attribute::Normal             => self.set_normal(),
            Attribute::CharText           => self.set_char_text(true),
            Attribute::Standout           => self.set_standout(true),
            Attribute::Underline          => self.set_underline(true),
            Attribute::Reverse            => self.set_reverse(true),
            Attribute::Blink              => self.set_blink(true),
            Attribute::Dim                => self.set_dim(true),
            Attribute::Bold               => self.set_bold(true),
            Attribute::AlternativeCharSet => self.set_alternative_char_set(true),
            Attribute::Invisible          => self.set_invisible(true),
            Attribute::Protected          => self.set_protected(true),
            Attribute::Horizontal         => self.set_horizontal(true),
            Attribute::Left               => self.set_left(true),
            Attribute::Low                => self.set_low(true),
            Attribute::Right              => self.set_right(true),
            Attribute::Top                => self.set_top(true),
            Attribute::Vertical           => self.set_vertical(true),
            Attribute::Italic             => self.set_italic(true)
        }
    }
}

/// Implement the ^ operator for disabling an Attribute from Attributes
impl BitXor<Attribute> for Attributes {
    type Output = Self;

    fn bitxor(self, rhs: Attribute) -> Self::Output {
        match rhs {
            Attribute::Normal             => self.set_normal(),
            Attribute::CharText           => self.set_char_text(false),
            Attribute::Standout           => self.set_standout(false),
            Attribute::Underline          => self.set_underline(false),
            Attribute::Reverse            => self.set_reverse(false),
            Attribute::Blink              => self.set_blink(false),
            Attribute::Dim                => self.set_dim(false),
            Attribute::Bold               => self.set_bold(false),
            Attribute::AlternativeCharSet => self.set_alternative_char_set(false),
            Attribute::Invisible          => self.set_invisible(false),
            Attribute::Protected          => self.set_protected(false),
            Attribute::Horizontal         => self.set_horizontal(false),
            Attribute::Left               => self.set_left(false),
            Attribute::Low                => self.set_low(false),
            Attribute::Right              => self.set_right(false),
            Attribute::Top                => self.set_top(false),
            Attribute::Vertical           => self.set_vertical(false),
            Attribute::Italic             => self.set_italic(false)
        }
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self::_from(None, constants::A_NORMAL)
    }
}

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        Self::default() | attribute
    }
}

impl Into<attr_t> for Attributes {
    fn into(self) -> attr_t {
        self.raw
    }
}

impl fmt::Debug for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attributes {{ screen: {:?}, raw: 0b{:032b} }}", self.screen, self.raw)
    }
}
