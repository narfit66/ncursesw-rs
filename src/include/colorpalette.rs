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

use crate::NCurseswError;

impl FromStr for ColorPalette {
    type Err = NCurseswError;

    /// Parse a string to instance a color palette.
    ///
    /// Valid values are:
    ///     'default', 'black', 'red', 'green', 'yellow', 'blue', 'magenta', 'cyan', 'white',
    ///     'light black', 'light red', 'light green', 'light yellow', 'light blue',
    ///     'light magenta', 'light cyan', 'light white'
    ///
    /// ## Example
    /// ```rust
    /// extern crate ncursesw;
    ///
    /// # use std::error::Error;
    /// use std::str::FromStr;
    /// use ncursesw::*;
    ///
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let default = ColorPalette::from_str("default")?;
    /// let black = ColorPalette::from_str("black")?;
    /// let red = ColorPalette::from_str("red")?;
    /// let green = ColorPalette::from_str("green")?;
    /// let yellow = ColorPalette::from_str("yellow")?;
    /// let blue = ColorPalette::from_str("blue")?;
    /// let magenta = ColorPalette::from_str("magenta")?;
    /// let cyan = ColorPalette::from_str("cyan")?;
    /// let white = ColorPalette::from_str("white")?;
    /// let light_black = ColorPalette::from_str("light black")?;
    /// let light_red = ColorPalette::from_str("light red")?;
    /// let light_green = ColorPalette::from_str("light green")?;
    /// let light_yellow = ColorPalette::from_str("light yellow")?;
    /// let light_blue = ColorPalette::from_str("light blue")?;
    /// let light_magenta = ColorPalette::from_str("light magenta")?;
    /// let light_cyan = ColorPalette::from_str("light cyan")?;
    /// let light_white = ColorPalette::from_str("light white")?;
    ///
    /// assert!(default == ColorPalette::TerminalDefault);
    /// assert!(black == ColorPalette::Black);
    /// assert!(red == ColorPalette::Red);
    /// assert!(green == ColorPalette::Green);
    /// assert!(yellow == ColorPalette::Yellow);
    /// assert!(blue == ColorPalette::Blue);
    /// assert!(magenta == ColorPalette::Magenta);
    /// assert!(cyan == ColorPalette::Cyan);
    /// assert!(white == ColorPalette::White);
    /// assert!(light_black == ColorPalette::LightBlack);
    /// assert!(light_red == ColorPalette::LightRed);
    /// assert!(light_green == ColorPalette::LightGreen);
    /// assert!(light_yellow == ColorPalette::LightYellow);
    /// assert!(light_blue == ColorPalette::LightBlue);
    /// assert!(light_magenta == ColorPalette::LightMagenta);
    /// assert!(light_cyan == ColorPalette::LightCyan);
    /// assert!(light_white == ColorPalette::LightWhite);
    /// #
    /// #     Ok(())
    /// # }
    /// ```
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
