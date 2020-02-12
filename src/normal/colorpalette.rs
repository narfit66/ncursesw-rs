/*
    src/normal/colorpalette.rs

    Copyright (c) 2020 Stephen Whittle  All rights reserved.

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

use crate::{
    COLORS,
    shims::{
        constants::{
            COLOR_BLACK, COLOR_RED, COLOR_GREEN, COLOR_YELLOW,
            COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE
        },
        ncurses::short_t
    }
};

const COLOR_DEFAULT: i32       = COLOR_BLACK - 1;
const COLOR_LIGHT_BLACK: i32   = COLOR_BLACK + 8;
const COLOR_LIGHT_RED: i32     = COLOR_RED + 8;
const COLOR_LIGHT_GREEN: i32   = COLOR_GREEN + 8;
const COLOR_LIGHT_YELLOW: i32  = COLOR_YELLOW + 8;
const COLOR_LIGHT_BLUE: i32    = COLOR_BLUE + 8;
const COLOR_LIGHT_MAGENTA: i32 = COLOR_MAGENTA + 8;
const COLOR_LIGHT_CYAN: i32    = COLOR_CYAN + 8;
const COLOR_LIGHT_WHITE: i32   = COLOR_WHITE + 8;

include!("../include/colorpalette.rs");

/// The color palette.
///
/// The first 8 color's are considered dark colors, the next 8
/// (if available) are considered light colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorPalette {
    /// Color #-1
    TerminalDefault,
    /// Black color
    ///
    /// Color #0
    Black,
    /// Red color
    ///
    /// Color #1
    Red,
    /// Green color
    ///
    /// Color #2
    Green,
    /// Yellow/Brown color (Red + Green)
    ///
    /// Color #3
    Yellow,
    /// Blue color
    ///
    /// Color #4
    Blue,
    /// Magenta color (Red + Blue)
    ///
    /// Color #5
    Magenta,
    /// Cyan color (Green + Blue)
    ///
    /// Color #6
    Cyan,
    /// White color (Red + Green + Blue)
    ///
    /// Color #7
    White,
    /// Light black color
    ///
    /// Color #8
    LightBlack,
    /// Light red color
    ///
    /// Color #9
    LightRed,
    /// Light green color
    ///
    /// Color #10
    LightGreen,
    /// Light yellow color (LightRed + LightGreen)
    ///
    /// Color #11
    LightYellow,
    /// Light blue color
    ///
    /// Color #12
    LightBlue,
    /// Light magenta color (LightRed + LightBlue)
    ///
    /// Color #13
    LightMagenta,
    /// Light cyan color (LightGreen + LightBlue)
    ///
    /// Color #14
    LightCyan,
    /// Light white color (LightRed + LightGreen + LightBlue)
    ///
    /// Color #15
    LightWhite,
    /// Custom color
    Custom(short_t)
}

impl ColorPalette {
    pub(in crate) fn _from(number: short_t) -> Self {
        assert!(i32::from(number) >= COLOR_DEFAULT && i32::from(number) < COLORS());

        match i32::from(number) {
            COLOR_DEFAULT       => ColorPalette::TerminalDefault,
            COLOR_BLACK         => ColorPalette::Black,
            COLOR_RED           => ColorPalette::Red,
            COLOR_GREEN         => ColorPalette::Green,
            COLOR_YELLOW        => ColorPalette::Yellow,
            COLOR_BLUE          => ColorPalette::Blue,
            COLOR_MAGENTA       => ColorPalette::Magenta,
            COLOR_CYAN          => ColorPalette::Cyan,
            COLOR_WHITE         => ColorPalette::White,
            COLOR_LIGHT_BLACK   => ColorPalette::LightBlack,
            COLOR_LIGHT_RED     => ColorPalette::LightRed,
            COLOR_LIGHT_GREEN   => ColorPalette::LightGreen,
            COLOR_LIGHT_YELLOW  => ColorPalette::LightYellow,
            COLOR_LIGHT_BLUE    => ColorPalette::LightBlue,
            COLOR_LIGHT_MAGENTA => ColorPalette::LightMagenta,
            COLOR_LIGHT_CYAN    => ColorPalette::LightCyan,
            COLOR_LIGHT_WHITE   => ColorPalette::LightWhite,
            _                   => ColorPalette::Custom(number)
        }
    }

    pub(in crate) fn number(self) -> short_t {
        match self {
            ColorPalette::TerminalDefault => COLOR_DEFAULT as short_t,
            ColorPalette::Black           => COLOR_BLACK as short_t,
            ColorPalette::Red             => COLOR_RED as short_t,
            ColorPalette::Green           => COLOR_GREEN as short_t,
            ColorPalette::Yellow          => COLOR_YELLOW as short_t,
            ColorPalette::Blue            => COLOR_BLUE as short_t,
            ColorPalette::Magenta         => COLOR_MAGENTA as short_t,
            ColorPalette::Cyan            => COLOR_CYAN as short_t,
            ColorPalette::White           => COLOR_WHITE as short_t,
            ColorPalette::LightBlack      => COLOR_LIGHT_BLACK as short_t,
            ColorPalette::LightRed        => COLOR_LIGHT_RED as short_t,
            ColorPalette::LightGreen      => COLOR_LIGHT_GREEN as short_t,
            ColorPalette::LightYellow     => COLOR_LIGHT_YELLOW as short_t,
            ColorPalette::LightBlue       => COLOR_LIGHT_BLUE as short_t,
            ColorPalette::LightMagenta    => COLOR_LIGHT_MAGENTA as short_t,
            ColorPalette::LightCyan       => COLOR_LIGHT_CYAN as short_t,
            ColorPalette::LightWhite      => COLOR_LIGHT_WHITE as short_t,
            ColorPalette::Custom(number)  => {
                assert!(i32::from(number) >= COLOR_LIGHT_BLACK && i32::from(number) < COLORS());

                number
            }
        }
    }
}
