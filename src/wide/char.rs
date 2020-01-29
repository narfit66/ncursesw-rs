/*
    src/wide/char.rs

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

use std::convert::{TryFrom, From, TryInto, Into};
use std::char::{CharTryFromError, EscapeUnicode, EscapeDebug, EscapeDefault, ToLowercase, ToUppercase};

use ncurseswerror::NCurseswError;
use shims::bindings::WEOF;
use shims::ncurses::{wint_t, wchar_t};

/// Wide character (UTF-8 character).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WideChar {
    raw: wint_t
}

impl WideChar {
    pub fn new(ch: char) -> Self {
        Self { raw: wint_t::from(ch) }
    }

    pub fn is_digit(self, radix: u32) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_digit(radix)
        }
    }

    pub fn to_digit(self, radix: u32) -> Option<u32> {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.to_digit(radix)
        }
    }

    pub fn escape_unicode(self) -> EscapeUnicode {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.escape_unicode()
        }
    }

    pub fn escape_debug(self) -> EscapeDebug {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.escape_debug()
        }
    }

    pub fn escape_default(self) -> EscapeDefault {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.escape_default()
        }
    }

    pub fn len_utf8(self) -> usize {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.len_utf8()
        }
    }

    pub fn len_utf16(self) -> usize {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.len_utf16()
        }
    }

    pub fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.encode_utf8(dst)
        }
    }

    pub fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.encode_utf16(dst)
        }
    }

    pub fn is_alphabetic(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_alphabetic()
        }
    }

    pub fn is_lowercase(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_lowercase()
        }
    }

    pub fn is_uppercase(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_uppercase()
        }
    }

    pub fn is_whitespace(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_whitespace()
        }
    }

    pub fn is_alphanumeric(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_alphanumeric()
        }
    }

    pub fn is_control(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_control()
        }
    }

    pub fn is_numeric(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_numeric()
        }
    }

    pub fn to_lowercase(self) -> ToLowercase {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.to_lowercase()
        }
    }

    pub fn to_uppercase(self) -> ToUppercase {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.to_uppercase()
        }
    }

    pub fn is_ascii(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii()
        }
    }

    pub fn to_ascii_uppercase(self) -> char {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.to_ascii_uppercase()
        }
    }

    pub fn to_ascii_lowercase(self) -> char {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.to_ascii_lowercase()
        }
    }

    pub fn eq_ignore_ascii_case(self, other: char) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.eq_ignore_ascii_case(&other)
        }
    }

    pub fn make_ascii_uppercase(&mut self) {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => {
                let mut ascii = c;
                ascii.make_ascii_uppercase();

                self.raw = wint_t::from(ascii)
            }
        }
    }

    pub fn make_ascii_lowercase(&mut self) {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => {
                let mut ascii = c;
                ascii.make_ascii_lowercase();

                self.raw = wint_t::from(ascii)
            }
        }
    }

    pub fn is_ascii_alphabetic(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_alphabetic()
        }
    }

    pub fn is_ascii_uppercase(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_uppercase()
        }
    }

    pub fn is_ascii_lowercase(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_lowercase()
        }
    }

    pub fn is_ascii_alphanumeric(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_alphanumeric()
        }
    }

    pub fn is_ascii_digit(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_digit()
        }
    }

    pub fn is_ascii_hexdigit(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_hexdigit()
        }
    }

    pub fn is_ascii_punctuation(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_punctuation()
        }
    }

    pub fn is_ascii_graphic(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_graphic()
        }
    }

    pub fn is_ascii_whitespace(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_whitespace()
        }
    }

    pub fn is_ascii_control(self) -> bool {
        match self.as_char() {
            Err(e) => panic!(e),
            Ok(c)  => c.is_ascii_control()
        }
    }

    pub fn is_weof(self) -> bool {
        self.raw == WEOF
    }

    pub fn as_char(self) -> result!(char) {
        let ch = char::try_from(self.raw)?;

        Ok(ch)
    }
}

impl From<wint_t> for WideChar {
    fn from(raw: wint_t) -> Self {
        Self { raw }
    }
}

impl Into<wint_t> for WideChar {
    fn into(self) -> wint_t {
        self.raw
    }
}

impl From<wchar_t> for WideChar {
    fn from(value: wchar_t) -> Self {
        Self { raw: value as wint_t }
    }
}

impl Into<wchar_t> for WideChar {
    fn into(self) -> wchar_t {
        self.raw.to_owned() as wchar_t
    }
}

impl TryInto<char> for WideChar {
    type Error = CharTryFromError;

    fn try_into(self) -> Result<char, Self::Error> {
        char::try_from(self.raw)
    }
}
