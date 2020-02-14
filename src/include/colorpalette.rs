/*
    src/include/colorpalette.rs

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

use std::str::FromStr;

use crate::{
    NCurseswError, COLORS,
    shims::constants::{
        COLOR_BLACK, COLOR_RED, COLOR_GREEN, COLOR_YELLOW,
        COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE
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

macro_rules! color_palette_enum {
    ($type: ty) => {
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
            Custom($type)
        }
    }
}

impl FromStr for ColorPalette {
    type Err = NCurseswError;

    /// Parse a string to instance a color palette.
    ///
    /// Valid values are:
    ///     'default', 'black', 'red', 'green', 'yellow', 'blue', 'magenta', 'cyan', 'white',
    ///     'light black', 'light red', 'light green', 'light yellow', 'light blue',
    ///     'light magenta', 'light cyan', 'light white'
    fn from_str(color: &str) -> Result<Self, Self::Err> {
        match color {
            "default"       => Ok(ColorPalette::TerminalDefault),
            "black"         => Ok(ColorPalette::Black),
            "red"           => Ok(ColorPalette::Red),
            "green"         => Ok(ColorPalette::Green),
            "yellow"        => Ok(ColorPalette::Yellow),
            "blue"          => Ok(ColorPalette::Blue),
            "magenta"       => Ok(ColorPalette::Magenta),
            "cyan"          => Ok(ColorPalette::Cyan),
            "white"         => Ok(ColorPalette::White),
            "light black"   => Ok(ColorPalette::LightBlack),
            "light red"     => Ok(ColorPalette::LightRed),
            "light green"   => Ok(ColorPalette::LightGreen),
            "light yellow"  => Ok(ColorPalette::LightYellow),
            "light blue"    => Ok(ColorPalette::LightBlue),
            "light magenta" => Ok(ColorPalette::LightMagenta),
            "light cyan"    => Ok(ColorPalette::LightCyan),
            "light white"   => Ok(ColorPalette::LightWhite),
            _               => Err(NCurseswError::ColorParseError { color: color.to_string() })
        }
    }
}
