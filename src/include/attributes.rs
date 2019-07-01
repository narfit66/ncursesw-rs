/*
    src/include/attributes.rs

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

use std::convert::{From, Into};
use std::ops::{BitOr, BitXor};

use gen::{AttributesType, AttributesGeneric};

use shims::ncurses::attr_t;
use shims::constants::{A_NORMAL, A_CHARTEXT, A_STANDOUT, A_UNDERLINE, A_REVERSE, A_BLINK, A_DIM, A_BOLD, A_ALTCHARSET, A_INVIS, A_PROTECT, A_HORIZONTAL, A_LEFT, A_LOW, A_RIGHT, A_TOP, A_VERTICAL, A_ITALIC};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Attributes {
    raw: attr_t
}

macro_rules! getter {
    ($name: ident, $attr: ident) => {
        fn $name(&self) -> bool {
            (self.raw & $attr) > 0
        }
    };
}

macro_rules! setter {
    ($name: ident, $attr: ident) => {
        fn $name(&mut self, enabled: bool) {
            if enabled {
                self.raw |= $attr;
            } else {
                self.raw ^= $attr;
            }
        }
    };
}
macro_rules! impl_attributes_type {
    ($type: ty) => {
        impl AttributesType<$type> for Attributes {
            fn is_normal(&self) -> bool {
                self.raw == A_NORMAL
            }

            fn set_normal(&mut self) {
                self.raw = A_NORMAL
            }

            getter!(is_char_text, A_CHARTEXT);
            setter!(set_char_text, A_CHARTEXT);

            getter!(is_standout, A_STANDOUT);
            setter!(set_standout, A_STANDOUT);

            getter!(is_underline, A_UNDERLINE);
            setter!(set_underline, A_UNDERLINE);

            getter!(is_reverse, A_REVERSE);
            setter!(set_reverse, A_REVERSE);

            getter!(is_blink, A_BLINK);
            setter!(set_blink, A_BLINK);

            getter!(is_dim, A_DIM);
            setter!(set_dim, A_DIM);

            getter!(is_bold, A_BOLD);
            setter!(set_bold, A_BOLD);

            getter!(is_alternate_char_set, A_ALTCHARSET);
            setter!(set_alternative_char_set, A_ALTCHARSET);

            getter!(is_invisible, A_INVIS);
            setter!(set_invisible, A_INVIS);

            getter!(is_protected, A_PROTECT);
            setter!(set_protected, A_PROTECT);

            getter!(is_horizontal, A_HORIZONTAL);
            setter!(set_horizontal, A_HORIZONTAL);

            getter!(is_left, A_LEFT);
            setter!(set_left, A_LEFT);

            getter!(is_low, A_LOW);
            setter!(set_low, A_LOW);

            getter!(is_right, A_RIGHT);
            setter!(set_right, A_RIGHT);

            getter!(is_top, A_TOP);
            setter!(set_top, A_TOP);

            getter!(is_vertical, A_VERTICAL);
            setter!(set_vertical, A_VERTICAL);

            getter!(is_italic, A_ITALIC);
            setter!(set_italic, A_ITALIC);
        }
    };
}

impl AttributesGeneric for Attributes {
    fn as_attr_t(&self) -> attr_t {
        self.raw
    }
}

/// Implement the | operator for adding Attributes to Attributes
///
/// # Example
///
/// ```
/// let mut attributes = Attributes::default() | Attribute::Bold;
/// let other = Attributes::default() | Attribute::Reverse;
/// attributes = attributes | other;
/// assert!(attributes.is_bold());
/// assert!(attributes.is_reverse());
/// assert!(!attributes.is_dim());
/// ```
impl BitOr for Attributes {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw | rhs.raw }
    }
}

/// Implement the ^ operator for removing Attributes from Attributes
///
/// # Example
///
/// ```
/// let mut attributes = Attributes::default() | Attribute::Blink | Attribute::Bold;
/// let other = Attributes::default() | Attribute::Reverse | Attribute::Bold;
/// attributes = attributes ^ other;
/// assert!(!attributes.is_bold());
/// assert!(attributes.is_reverse());
/// assert!(attributes.is_blink());
/// ```
impl BitXor for Attributes {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { raw: self.raw ^ rhs.raw }
    }
}

/// Implement the | operator for adding an Attribute to Attributes
///
/// # Example
///
/// ```
/// let mut attributes = Attributes::default();
/// assert!(!attributes.is_bold());
/// attributes = attributes | Attribute::Bold;
/// assert!(attributes.is_bold());
/// ```
impl BitOr<Attribute> for Attributes {
    type Output = Self;

    fn bitor(mut self, rhs: Attribute) -> Self::Output {
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

        self
    }
}

/// Implement the ^ operator for disabling an Attribute from Attributes
///
/// # Example
///
/// ```
/// let mut attributes = Attributes::from(Attribute::Bold);
/// assert!(attributes.is_bold());
/// attributes = attributes ^ Attribute::Bold;
/// assert!(!attributes.is_bold());
/// ```
impl BitXor<Attribute> for Attributes {
    type Output = Self;

    fn bitxor(mut self, rhs: Attribute) -> Self::Output {
        match rhs {
            Attribute::Normal             => (),
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

        self
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self { raw: A_NORMAL }
    }
}

impl From<Attribute> for Attributes {
    fn from(attribute: Attribute) -> Self {
        Self::default() | attribute
    }
}

impl From<attr_t> for Attributes {
    fn from(raw: attr_t) -> Self {
        Self { raw }
    }
}

impl Into<attr_t> for Attributes {
    fn into(self) -> attr_t {
        self.raw
    }
}
