/*
    src/include/color.rs

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

use std::str::FromStr;

impl FromStr for Color {
    type Err = NCurseswError;

    fn from_str(color: &str) -> Result<Self, Self::Err> {
        match color {
            "default"       => Ok(Color::TerminalDefault),
            "black"         => Ok(Color::Dark(BaseColor::Black)),
            "red"           => Ok(Color::Dark(BaseColor::Red)),
            "green"         => Ok(Color::Dark(BaseColor::Green)),
            "yellow"        => Ok(Color::Dark(BaseColor::Yellow)),
            "blue"          => Ok(Color::Dark(BaseColor::Blue)),
            "magenta"       => Ok(Color::Dark(BaseColor::Magenta)),
            "cyan"          => Ok(Color::Dark(BaseColor::Cyan)),
            "white"         => Ok(Color::Dark(BaseColor::White)),
            "light black"   => Ok(Color::Light(BaseColor::Black)),
            "light red"     => Ok(Color::Light(BaseColor::Red)),
            "light green"   => Ok(Color::Light(BaseColor::Green)),
            "light yellow"  => Ok(Color::Light(BaseColor::Yellow)),
            "light blue"    => Ok(Color::Light(BaseColor::Blue)),
            "light magenta" => Ok(Color::Light(BaseColor::Magenta)),
            "light cyan"    => Ok(Color::Light(BaseColor::Cyan)),
            "light white"   => Ok(Color::Light(BaseColor::White)),
            _               => Err(NCurseswError::ColorParseError { color: color.to_string() })
        }
    }
}
