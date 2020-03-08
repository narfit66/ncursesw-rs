/*
    src/include/attribute.rs

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

use crate::shims::constants::{
    A_NORMAL, A_CHARTEXT, A_STANDOUT, A_UNDERLINE, A_REVERSE, A_BLINK, A_DIM,
    A_BOLD, A_ALTCHARSET, A_INVIS, A_PROTECT, A_HORIZONTAL, A_LEFT, A_LOW,
    A_RIGHT, A_TOP, A_VERTICAL, A_ITALIC
};

/// Terminal Attribute.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Attribute {
    /// Normal display (no highlight).
    Normal,
    /// Bit-mask to extract a character.
    CharText,
    /// Best highlighting mode of the terminal.
    Standout,
    /// Underlining.
    Underline,
    /// Reverse video.
    Reverse,
    /// Blinking.
    Blink,
    /// Half bright.
    Dim,
    /// Extra bright or bold.
    Bold,
    /// Alternate character set.
    AlternativeCharSet,
    /// Invisible or blank mode.
    Invisible,
    /// Protected mode.
    Protected,
    Horizontal,
    Left,
    Low,
    Right,
    Top,
    Vertical,
    /// Italics.
    Italic
}

impl Into<attr_t> for Attribute {
    fn into(self) -> attr_t {
        match self {
            Attribute::Normal             => A_NORMAL,
            Attribute::CharText           => A_CHARTEXT,
            Attribute::Standout           => A_STANDOUT,
            Attribute::Underline          => A_UNDERLINE,
            Attribute::Reverse            => A_REVERSE,
            Attribute::Blink              => A_BLINK,
            Attribute::Dim                => A_DIM,
            Attribute::Bold               => A_BOLD,
            Attribute::AlternativeCharSet => A_ALTCHARSET,
            Attribute::Invisible          => A_INVIS,
            Attribute::Protected          => A_PROTECT,
            Attribute::Horizontal         => A_HORIZONTAL,
            Attribute::Left               => A_LEFT,
            Attribute::Low                => A_LOW,
            Attribute::Right              => A_RIGHT,
            Attribute::Top                => A_TOP,
            Attribute::Vertical           => A_VERTICAL,
            Attribute::Italic             => A_ITALIC
        }
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute::Normal
    }
}
