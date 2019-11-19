/*
    src/ncurses.rs

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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use std::{convert::{From, TryFrom}, path, char, ptr, slice, time, mem};

use libc::{c_void, EINTR};
use constants::{
    ERR, OK, KEY_MIN, KEY_MAX, KEY_CODE_YES, KEY_RESIZE,
    KEY_EVENT, TRUE, FALSE
};

use {normal, extend};
use attributescolorpairset::*;
use changed::*;
use characterresult::*;
use chtypet::*;
use complex::*;
use cursortype::*;
use cstring::*;
use gen::*;
use keybinding::*;
use legacy::*;
use origin::*;
use orientation::*;
use justification::*;
use wide::*;
use ncursescolortype::*;
use ncurseswerror::*;
use region::*;
use size::*;
use softlabeltype::*;

use shims::ncurses;
use shims::bindings::CCHARW_MAX;

// The maximum buffer size used in a variety of functions.
const LINE_MAX: usize = 4096;

/// NCurses window raw pointer.
type WINDOW = ncurses::WINDOW;
/// NCurses screen raw pointer.
type SCREEN = ncurses::SCREEN;
/// Ripoff line callback function signature.
type RipoffInit = crate::shims::bindings::RipoffInit;

/// Raw attribute type value.
type attr_t = ncurses::attr_t;
type cchar_t = ncurses::cchar_t;
type chtype = ncurses::chtype;
type short_t = ncurses::short_t;
type wchar_t = ncurses::wchar_t;
type wint_t = ncurses::wint_t;

/// Return the raw pointer to the current screen.
pub fn curscr() -> WINDOW {
    unsafe {
        ncurses::curscr()
    }
}

/// Return the raw pointer to the new screen.
pub fn newscr() -> WINDOW {
    unsafe {
        ncurses::newscr()
    }
}

/// Return the raw pointer to the standard screen.
pub fn stdscr() -> WINDOW {
    unsafe {
        ncurses::stdscr()
    }
}

/// Return the number of colors available.
///
/// This is initialized by `start_color` to the maximum number of colors the terminal can support.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let number_of_colors = COLORS();
///
/// assert!(number_of_colors > 0);
/// #     }
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn COLORS() -> i32 {
    ncurses::COLORS()
}

/// Return the attribute value of a given `normal` color pair.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::str::FromStr;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let yellow = Color::from_str("yellow")?;
/// let blue = Color::from_str("blue")?;
/// 
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
///
/// assert!(COLOR_PAIR(color_pair0) == 0b0000000000);
/// assert!(COLOR_PAIR(color_pair1) == 0b0100000000);
/// #     }
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn COLOR_PAIR(color_pair: normal::ColorPair) -> attr_t {
    ncurses::COLOR_PAIR(normal::ColorPair::into(color_pair)) as attr_t
}

/// Return the color pair from  given `normal` attributes value.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::str::FromStr;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let yellow = Color::from_str("yellow")?;
/// let blue = Color::from_str("blue")?;
/// 
/// let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
/// let attrs = Attribute::Bold | color_pair1;
///
/// assert!(PAIR_NUMBER(attrs) == color_pair1);
/// #     }
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn PAIR_NUMBER(attrs: normal::Attributes) -> normal::ColorPair {
    normal::ColorPair::from(ncurses::PAIR_NUMBER(normal::Attributes::into(attrs)))
}

/// Return the number of color pairs available.
///
/// This is initialized by `start_color` to the maximum number of color pairs the terminal can support.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let number_of_color_pairs = COLOR_PAIRS();
///
/// assert!(number_of_color_pairs > 0);
/// #     }
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn COLOR_PAIRS() -> i32 {
    ncurses::COLOR_PAIRS()
}

/// Return the number of columns (x-axis) available on the terminal.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let number_of_columns = COLS();
///
/// assert!(number_of_columns > 0);
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn COLS() -> i32 {
    ncurses::COLS()
}

/// Return the delay used to interpret termianl keyboard escape sequences.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// use std::time;
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let delay = ESCDELAY()?;
///
/// assert!(delay == time::Duration::from_millis(1000), "delay={:?}", delay);
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn ESCDELAY() -> result!(time::Duration) {
    let delay = time::Duration::from_millis(u64::try_from(ncurses::ESCDELAY())?);

    Ok(delay)
}

/// Return the number of lines (y-axis) available on the terminal.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let number_of_lines = LINES();
///
/// assert!(number_of_lines > 0);
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn LINES() -> i32 {
    ncurses::LINES()
}

/// Return the number of columns a tab represents on the terminal.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let tabsize = TABSIZE();
///
/// assert!(tabsize > 0);
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn TABSIZE() -> i32 {
    ncurses::TABSIZE()
}

/// Add/Output a complex character to the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;
///
/// add_wch(complex_char)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn add_wch(wch: ComplexChar) -> result!(()) {
    match ncurses::add_wch(&ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("add_wch", rc))
    }
}

/// Add/Output a complex character string of a given length to the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair0)?;
///
/// // this will output "Testing..Testing.."
/// add_wchnstr(&complex_str, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn add_wchnstr(wchstr: &ComplexString, number: i32) -> result!(()) {
    match ncurses::add_wchnstr(raw_with_nul_as_slice!(wchstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("add_wchnstr", rc))
    }
}

/// Add/Output a complex character string to the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair0)?;
///
/// // this will output "Testing..Testing..1..2..3.."
/// add_wchstr(&complex_str)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn add_wchstr(wchstr: &ComplexString) -> result!(()) {
    match ncurses::add_wchstr(raw_with_nul_as_slice!(wchstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("add_wchstr", rc))
    }
}

/// Add/Output a ascii character and `normal` attribute/color pair combination to the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char) | attrs;
///
/// addch(chtype_char)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn addch(ch: ChtypeChar) -> result!(()) {
    match ncurses::addch(ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addch", rc))
    }
}

/// Add/Output a ascii character string and `normal` attribute/color pair combination of a given length to the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
/// let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;
///
/// // this will output "Testing..Testing.."
/// addchnstr(&chtype_str, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn addchnstr(chstr: &ChtypeString, number: i32) -> result!(()) {
    match ncurses::addchnstr(raw_with_nul_as_slice!(chstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addchnstr", rc))
    }
}

/// Add/Output a ascii character string and `normal` attribute/color pair combination to the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
/// let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;
///
/// // this will output "Testing..Testing..1..2..3.."
/// addchstr(&chtype_str)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn addchstr(chstr: &ChtypeString) -> result!(()) {
    match ncurses::addchstr(raw_with_nul_as_slice!(chstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addchstr", rc))
    }
}

/// Add/Output a character string of a given length to the standard screen.
///
/// Note: Originally this function whould just output characters in the ascii character
///       set but as of ABI 6 (and maybe eariler) this function will output any unicode
///       UTF-8 character string.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let s = "Testing..Testing..1..2..3..";
///
/// // this will output "Testing..Testing.."
/// addnstr(&s, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn addnstr(str: &str, number: i32) -> result!(()) {
    match ncurses::addnstr(unsafe { c_str_with_nul!(str) }, number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addnstr", rc)),
    }
}

/// Add/Output a wide character unicode UTF-8 string of a given length to the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // this will output "Testing..Testing.."
/// addnwstr(&wide_str, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn addnwstr(wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::addnwstr(raw_with_nul_as_slice!(wstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addnwstr", rc))
    }
}

/// Add/Output a character string to the standard screen.
///
/// Note: Originally this function whould just output characters in the ascii character
///       set but as of ABI 6 (and maybe eariler) this function will output any unicode
///       UTF-8 character string.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let s = "Testing..Testing..1..2..3..";
///
/// // this will output "Testing..Testing..1..2..3.."
/// addstr(&s)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn addstr(str: &str) -> result!(()) {
    match ncurses::addstr(unsafe { c_str_with_nul!(str) }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addstr", rc))
    }
}

/// Add/Output a wide character unicode UTF-8 string to the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // this will output "Testing..Testing..1..2..3.."
/// addwstr(&wide_str)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn addwstr(wstr: &WideString) -> result!(()) {
    match ncurses::addwstr(raw_with_nul_as_slice!(wstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addwstr", rc))
    }
}

/// Assign the colors given as the default foreground and background colors of color pair 0
///
/// For both normal and extended color types the foreground or background color may be set
/// as `Color::TerminalDefault` this will default the color before `initscr()` was called.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let yellow = Color::Dark(BaseColor::Yellow);
/// let blue = Color::Dark(BaseColor::Blue);
///
/// let colors = Colors::new(yellow, blue);
///
/// assume_default_colors(colors)?;
///
/// let color_pair0 = ColorPair::default();
///
/// assert!(color_pair0.colors()?.foreground() == yellow);
/// assert!(color_pair0.colors()?.background() == blue);
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn assume_default_colors<S, C, T>(colors: S) -> result!(())
    where S: ColorsType<C, T>,
          C: ColorType<T>,
          T: ColorAttributeTypes
{
    match ncurses::assume_default_colors(colors.foreground().number(), colors.background().number()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("assume_default_colors", rc))
    }
}

/// Return the current attributes and color pair on the standard screen.
///
/// Notes: This does *NOT* return the attribute and color pair rendition when defined
///        by `chtype` and/or `cchar` type add/insert functions as these are cell based
///        but when set by functions such as `attr_set`.
///        When returning a `normal` attribute and color pair the attribute does *NOT*
///        contain the color pair so this must be OR'd to back for some functions.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// attr_set(attrs1, color_pair1)?;
/// addch(chtype_char | attrs0)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn attr_get() -> result!(AttributesColorPairSet) {
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let mut opts: [i32; 1] = [0];

    match unsafe { ncurses::attr_get(attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts.as_mut_ptr() as *mut c_void) } {
        OK => Ok(match ncurses_colortype() {
                     NCursesColorType::Normal => {
                         AttributesColorPairSet::Normal(
                             normal::AttributesColorPair::new(
                                 normal::Attributes::from(attrs[0]),
                                 normal::ColorPair::from(color_pair[0])
                             )
                         )
                     },
                     NCursesColorType::Extended => {
                         AttributesColorPairSet::Extended(
                             extend::AttributesColorPair::new(
                                 extend::Attributes::from(attrs[0]),
                                 extend::ColorPair::from(opts[0])
                             )
                         )
                     }
              }),
        rc => Err(ncurses_function_error_with_rc!("attr_get", rc))
    }
}

/// Switch off the given attributes on the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | Attribute::Dim | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// attr_set(attrs1, color_pair1)?;
/// addch(chtype_char | attrs0)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// attr_off(Attributes::default() | Attribute::Dim)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(!s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn attr_off<A, T>(attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::attr_off(attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attr_off", rc))
    }
}

/// Switch on the given attributes on the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// attr_set(attrs1, color_pair1)?;
/// addch(chtype_char | attrs0)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(!s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// attr_on(Attributes::default() | Attribute::Dim)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn attr_on<A, T>(attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::attr_on(attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attr_on", rc))
    }
}

/// Set the current attributes and color pair on the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// attr_set(attrs1, color_pair1)?;
/// addch(chtype_char | attrs0)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn attr_set<A, P, T>(attrs: A, color_pair: P) -> result!(())
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::attr_set(attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attr_set", rc))
    }
}

/// Switch off the given `normal` attributes on the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | Attribute::Dim | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// attrset(attrs1)?;
/// addch(chtype_char | attrs0)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// attroff(Attributes::default() | Attribute::Dim)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(!s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn attroff(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attroff(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attroff", rc))
    }
}

/// Switch on the given `normal` attributes on the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// attrset(attrs1)?;
/// addch(chtype_char | attrs0)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(!s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// attron(Attributes::default() | Attribute::Dim)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn attron(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attron(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attron", rc))
    }
}

/// Set the current `normal` attributes and color pair on the standard screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// attrset(attrs1)?;
/// addch(chtype_char | attrs0)?;
///
/// match attr_get()? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn attrset(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attrset(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attrset", rc))
    }
}

/// Return the output speed of the terminal. The number returned is in bits per second, for example 9600.
pub fn baudrate() -> i32 {
    ncurses::baudrate()
}

/// Sounds an audible alarm on the terminal, if possible; otherwise flashes the screen (visible bell).
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// beep()?;
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn beep() -> result!(()) {
    match ncurses::beep() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("beep", rc))
    }
}

/// Set the background property on the standard screen and then apply this setting to every character position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use shims::ncurses::ACS_CKBOARD;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// bkgd(ChtypeChar::from_chtype(ACS_CKBOARD()))?;
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn bkgd(ch: ChtypeChar) -> result!(()) {
    match ncurses::bkgd(ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("bkgd", rc))
    }
}

/// Manipulate the background of the standard screen.
///
/// The window background is a `chtype` consisting of any combination of attributes
/// (i.e., rendition) and a character. The attribute part of the background is
/// combined (OR'ed) with all non-blank characters that are written into the window
/// with waddch. Both the character and attribute parts of the background are combined
/// with the blank characters. The background becomes a property of the character and
/// moves with the character through any scrolling and insert/delete line/character operations.
///
/// To the extent possible on a particular terminal, the attribute part of the
/// background is displayed as the graphic rendition of the character put on the screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use shims::ncurses::ACS_CKBOARD;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// bkgdset(ChtypeChar::from_chtype(ACS_CKBOARD()));
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn bkgdset(ch: ChtypeChar) {
    ncurses::bkgdset(ChtypeChar::into(ch))
}

/// Set the background property on the standard screen and then apply this setting to every character position in that window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::extend::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let yellow = Color::Dark(BaseColor::Yellow);
/// let blue = Color::Dark(BaseColor::Blue);
///
/// let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
/// let mut attrs = Attributes::default();
/// attrs.set_dim(true);
///
/// match std::char::from_u32(0x20) {
///     Some(c) => {
///         let background_char = ComplexChar::from_char(c, &attrs, &color_pair1)?;
///         bkgrnd(background_char)?;
///     },
///     None    => panic!("unable to convert to character!")
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn bkgrnd(wch: ComplexChar) -> result!(()) {
    match ncurses::bkgrnd(&ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("bkgrnd", rc))
    }
}

/// Manipulate the background on the standard screen.
///
/// The window background is a `cchar_t` consisting of any combination of attributes
/// (i.e., rendition) and a complex character. The attribute part of the background
/// is combined (OR'ed) with all non-blank characters that are written into the window
/// with `waddch`. Both the character and attribute parts of the background are combined
/// with the blank characters. The background becomes a property of the character and moves
/// with the character through any scrolling and insert/delete line/character operations.

/// To the extent possible on a particular terminal, the attribute part of the background
/// is displayed as the graphic rendition of the character put on the screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::extend::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let yellow = Color::Dark(BaseColor::Yellow);
/// let blue = Color::Dark(BaseColor::Blue);
///
/// let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
/// let mut attrs = Attributes::default();
/// attrs.set_dim(true);
///
/// match std::char::from_u32(0x20) {
///     Some(c) => {
///         let background_char = ComplexChar::from_char(c, &attrs, &color_pair1)?;
///         bkgrndset(background_char);
///     },
///     None    => panic!("unable to convert to character!")
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn bkgrndset(wch: ComplexChar) {
    ncurses::bkgrndset(&ComplexChar::into(wch))
}

/// Draw a box around the edges of the standard screen.
///
/// ls - left side,
/// rs - right side,
/// ts - top side,
/// bs - bottom side,
/// tl - top left-hand corner,
/// tr - top right-hand corner,
/// bl - bottom left-hand corner, and
/// br - bottom right-hand corner.
///
/// If any of these arguments is zero, then the corresponding
/// default values are used instead:
///     ACS_VLINE,
///     ACS_VLINE,
///     ACS_HLINE,
///     ACS_HLINE,
///     ACS_ULCORNER,
///     ACS_URCORNER,
///     ACS_LLCORNER,
///     ACS_LRCORNER.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use shims::ncurses::{
///     ACS_VLINE, ACS_HLINE, ACS_ULCORNER,
///     ACS_URCORNER, ACS_LLCORNER, ACS_LRCORNER
/// };
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let ls = ChtypeChar::from_chtype(ACS_VLINE());
/// let rs = ChtypeChar::from_chtype(ACS_VLINE());
/// let ts = ChtypeChar::from_chtype(ACS_HLINE());
/// let bs = ChtypeChar::from_chtype(ACS_HLINE());
/// let tl = ChtypeChar::from_chtype(ACS_ULCORNER());
/// let tr = ChtypeChar::from_chtype(ACS_URCORNER());
/// let bl = ChtypeChar::from_chtype(ACS_LLCORNER());
/// let br = ChtypeChar::from_chtype(ACS_LRCORNER());
///
/// border(ls, rs, ts, bs, tl, tr, bl, br)?;
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn border(
    ls: ChtypeChar,
    rs: ChtypeChar,
    ts: ChtypeChar,
    bs: ChtypeChar,
    tl: ChtypeChar,
    tr: ChtypeChar,
    bl: ChtypeChar,
    br: ChtypeChar) -> result!(())
{
    match ncurses::border(
        ChtypeChar::into(ls),
        ChtypeChar::into(rs),
        ChtypeChar::into(ts),
        ChtypeChar::into(bs),
        ChtypeChar::into(tl),
        ChtypeChar::into(tr),
        ChtypeChar::into(bl),
        ChtypeChar::into(br)
    ) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("border", rc))
    }
}

pub fn border_set(
    ls: ComplexChar,
    rs: ComplexChar,
    ts: ComplexChar,
    bs: ComplexChar,
    tl: ComplexChar,
    tr: ComplexChar,
    bl: ComplexChar,
    br: ComplexChar) -> result!(())
{
    match ncurses::border_set(
        &ComplexChar::into(ls),
        &ComplexChar::into(rs),
        &ComplexChar::into(ts),
        &ComplexChar::into(bs),
        &ComplexChar::into(tl),
        &ComplexChar::into(tr),
        &ComplexChar::into(bl),
        &ComplexChar::into(br)
    ) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("border_set", rc))
    }
}

pub fn r#box(handle: WINDOW, verch: ChtypeChar, horch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::r#box(handle, ChtypeChar::into(verch), ChtypeChar::into(horch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("box", rc))
    }
}

pub fn box_set(handle: WINDOW, verch: ComplexChar, horch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::box_set(handle, &ComplexChar::into(verch), &ComplexChar::into(horch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("box_set", rc))
    }
}

pub fn can_change_color() -> bool {
    ncurses::can_change_color()
}

basic_ncurses_function!(cbreak, "cbreak");

pub fn chgat<A, P, T>(number: i32, attrs: A, color_pair: P) -> result!(())
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::chgat(number, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_const_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("chgat", rc))
    }
}

basic_ncurses_function!(clear, "clear");

pub fn clearok(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::clearok(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("clearok", rc))
    }
}

basic_ncurses_function!(clrtobot, "clrtobot");

basic_ncurses_function!(clrtoeol, "clrtoeol");

pub fn color_content(color: normal::Color) -> result!(normal::RGB) {
    let mut r: [short_t; 1] = [0];
    let mut g: [short_t; 1] = [0];
    let mut b: [short_t; 1] = [0];

    match unsafe { ncurses::color_content(normal::Color::into(color), r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()) } {
        OK => Ok(normal::RGB::new(r[0], g[0], b[0])),
        rc => Err(ncurses_function_error_with_rc!("color_content", rc))
    }
}

pub fn color_set<P, T>(color_pair: P) -> result!(())
    where P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::color_set(color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("color_set", rc))
    }
}

pub fn copywin(
    src_handle: WINDOW,
    dst_handle: WINDOW,
    smin: Origin,
    dmin: Origin,
    dmax: Origin,
    overlay: bool) -> result!(())
{
    match unsafe { ncurses::copywin(src_handle, dst_handle, smin.y, smin.x, dmin.y, dmin.x, dmax.y, dmax.x, if overlay {
        TRUE
    } else {
        FALSE
    }) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("copywin", rc))
    }
}

pub fn curs_set(cursor: CursorType) -> result!(CursorType) {
    match ncurses::curs_set(match cursor {
        CursorType::Invisible   => 0,
        CursorType::Visible     => 1,
        CursorType::VeryVisible => 2
    }) {
        0  => Ok(CursorType::Invisible),
        1  => Ok(CursorType::Visible),
        2  => Ok(CursorType::VeryVisible),
        rc => Err(ncurses_function_error_with_rc!("curs_set", rc))
    }
}

pub fn curses_version() -> String {
    ncurses::curses_version()
}

basic_ncurses_function!(def_prog_mode, "def_prog_mode");

basic_ncurses_function!(def_shell_mode, "def_shell_mode");

pub fn define_key(definition: Option<&str>, keycode: KeyBinding) -> result!(()) {
    match unsafe { ncurses::define_key(
        match definition {
            None    => ptr::null_mut(),
            Some(s) => s.to_c_str().as_ptr() as *mut i8
        },
        KeyBinding::into(keycode)
    )} {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("define_key", rc))
    }
}

pub fn delay_output(ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    match ncurses::delay_output(ms) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("delay_output", rc))
    }
}

basic_ncurses_function!(delch, "delch");

basic_ncurses_function!(deleteln, "deleteln");

pub fn delscreen(sp: SCREEN) {
    unsafe { ncurses::delscreen(sp) }
}

basic_ncurses_function_with_window!(delwin, "delwin");

pub fn derwin(orig: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    match unsafe { ncurses::derwin(orig, size.lines, size.columns, origin.y, origin.x) } {
        None      => Err(ncurses_function_error!("derwin")),
        Some(win) => Ok(win)
    }
}

basic_ncurses_function!(doupdate, "doupdate");

pub fn dupwin(handle: WINDOW) -> result!(WINDOW) {
    match unsafe { ncurses::dupwin(handle) } {
        None         => Err(ncurses_function_error!("dupwin")),
        Some(handle) => Ok(handle)
    }
}

basic_ncurses_function!(echo, "echo");

pub fn echo_wchar(wch: ComplexChar) -> result!(()) {
    match ncurses::echo_wchar(&ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("echo_wchar", rc))
    }
}

pub fn echochar(ch: ChtypeChar) -> result!(()) {
    match ncurses::echochar(ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("echochar", rc))
    }
}

basic_ncurses_function!(endwin, "endwin");

basic_ncurses_function!(erase, "erase");

pub fn erasechar() -> result!(char) {
    let rc = ncurses::erasechar();

    if rc < 0 {
        Err(ncurses_function_error_with_rc!("erasechar", i32::from(rc)))
    } else {
        Ok(char::from(rc as u8))
    }
}

pub fn erasewchar() -> result!(WideChar) {
    let mut wch: [wchar_t; 1] = [0];

    match unsafe { ncurses::erasewchar(wch.as_mut_ptr()) } {
        OK => Ok(WideChar::from(wch[0])),
        rc => Err(ncurses_function_error_with_rc!("erasewchar", rc))
    }
}

pub fn extended_color_content(color: extend::Color) -> result!(extend::RGB) {
    let mut r: [i32; 1] = [0];
    let mut g: [i32; 1] = [0];
    let mut b: [i32; 1] = [0];

    match unsafe { ncurses::extended_color_content(extend::Color::into(color), r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()) } {
        OK => Ok(extend::RGB::new(r[0], g[0], b[0])),
        rc => Err(ncurses_function_error_with_rc!("extended_color_content", rc))
    }
}

pub fn extended_pair_content(color_pair: extend::ColorPair) -> result!(extend::Colors) {
    let mut fg: [i32; 1] = [0];
    let mut bg: [i32; 1] = [0];

    match unsafe { ncurses::extended_pair_content(extend::ColorPair::into(color_pair), fg.as_mut_ptr(), bg.as_mut_ptr()) } {
        OK => Ok(extend::Colors::new(extend::Color::from(fg[0]), extend::Color::from(bg[0]))),
        rc => Err(ncurses_function_error_with_rc!("extended_pair_content", rc))
    }
}

pub fn extended_slk_color(color_pair: extend::ColorPair) -> result!(()) {
    match ncurses::extended_slk_color(color_pair.number()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("extended_slk_color", rc))
    }
}

simple_ncurses_function!(filter);

/// Flashes the screen (visible bell), and if that is not possible, audible alarm on the terminal.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// flash()?;
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn flash() -> result!(()) {
    match ncurses::flash() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("flash", rc))
    }
}

basic_ncurses_function!(flushinp, "flushinp");

#[deprecated(since = "0.1.3", note = "specified color_pair must go out of scope before reuse of it's color pair number otherwise unpredicable results may occur.")]
pub fn free_pair<P, T>(color_pair: P) -> result!(())
    where P: ColorPairType<T>,
          i32: From<P>,
          T: ColorAttributeTypes
{
    match ncurses::free_pair(color_pair.into()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("free_pair", rc))
    }
}

pub fn get_escdelay() -> result!(time::Duration) {
    let delay = time::Duration::from_millis(u64::try_from(ncurses::get_escdelay())?);

    Ok(delay)
}

pub fn get_wch() -> result!(CharacterResult<WideChar>) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::get_wch(wch.as_mut_ptr()) } {
        EINTR        => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE   => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT    => Err(NCurseswError::KeyEvent),
        KEY_CODE_YES => {
            match wch[0] as i32 {
                #[cfg(feature = "key_resize_as_error")]
                KEY_RESIZE => Err(NCurseswError::KeyResize),
                #[cfg(feature = "key_event_as_error")]
                KEY_EVENT  => Err(NCurseswError::KeyEvent),
                _          => Ok(CharacterResult::Key(KeyBinding::from(wch[0])))
            }
        },
        rc           => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("get_wch", rc))
            } else {
                Ok(CharacterResult::Character(WideChar::from(wch[0])))
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use getn_wstr() instead")]
pub fn get_wstr() -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::get_wstr(ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("get_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::get_wstr() : ptr.is_null()");

                let buf_as_bytes = unsafe { slice::from_raw_parts(ptr as *mut wchar_t, LINE_MAX) };

                for (idx, &byte) in buf_as_bytes.iter().enumerate() {
                    if byte == 0x00 {
                        return Ok(WideString::from(&buf_as_bytes[..idx]));
                    }
                }

                Ok(WideString::from(buf_as_bytes))
            }
        }
    }
}

pub fn getattrs(handle: WINDOW) -> normal::Attributes {
    normal::Attributes::from(unsafe { ncurses::getattrs(handle) as attr_t })
}

pub fn getbegx(handle: WINDOW) -> result!(i32) {
    let x = unsafe { ncurses::getbegx(handle) };

    if x < 0 {
        Err(ncurses_function_error_with_rc!("getbegx", x))
    } else {
        Ok(x)
    }
}

pub fn getbegy(handle: WINDOW) -> result!(i32) {
    let y = unsafe { ncurses::getbegy(handle) };

    if y < 0 {
        Err(ncurses_function_error_with_rc!("getbegy", y))
    } else {
        Ok(y)
    }
}

pub fn getbegyx(handle: WINDOW) -> result!(Origin) {
    let y = unsafe { ncurses::getbegy(handle) };
    let x = unsafe { ncurses::getbegx(handle) };

    if y < 0 {
        Err(ncurses_function_error_with_rc!("getbegyx (y)", y))
    } else if x < 0 {
        Err(ncurses_function_error_with_rc!("getbegyx (x)", x))
    } else {
        Ok(Origin { y, x })
    }
}

/// Returns the standard screen's current background character/attribute pair.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use shims::ncurses::ACS_CKBOARD;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let background_char = ChtypeChar::from_chtype(ACS_CKBOARD());
///
/// wbkgd(win, background_char)?;
///
/// assert!(getbkgd(win) == background_char);
///
/// delwin(win)?;
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn getbkgd(handle: WINDOW) -> ChtypeChar {
    ChtypeChar::from(unsafe { ncurses::getbkgd(handle) })
}

/// Returns the standard screen's current background character/attribute pair.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::extend::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let yellow = Color::Dark(BaseColor::Yellow);
/// let blue = Color::Dark(BaseColor::Blue);
///
/// let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
/// let mut attrs = Attributes::default();
/// attrs.set_dim(true);
///
/// match std::char::from_u32(0x20) {
///     Some(c) => {
///         let background_char = ComplexChar::from_char(c, &attrs, &color_pair1)?;
///         bkgrndset(background_char);
///
///         assert!(getbkgrnd()? == background_char);
///     },
///     None    => panic!("unable to convert to character!")
/// }
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn getbkgrnd() -> result!(ComplexChar) {
    let mut wch: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::getbkgrnd(wch.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wch[0])),
        rc => Err(ncurses_function_error_with_rc!("getbkgrnd", rc))
    }
}

pub fn getcchar(wcval: ComplexChar) -> result!(WideCharAndAttributes) {
    let mut wch: [wchar_t; CCHARW_MAX as usize] = [0; CCHARW_MAX as usize];
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let opts: *mut i32 = ptr::null_mut();

    let attribute_colorpair_set = |attrs: attr_t, color_pair: short_t, ext_color_pair: i32| -> AttributesColorPairSet {
        match ncurses_colortype() {
            NCursesColorType::Normal   => {
                AttributesColorPairSet::Normal(
                    normal::AttributesColorPair::new(
                        normal::Attributes::from(attrs),
                        normal::ColorPair::from(color_pair)
                    )
                )
            },
            NCursesColorType::Extended => {
                AttributesColorPairSet::Extended(
                    extend::AttributesColorPair::new(
                        extend::Attributes::from(attrs),
                        extend::ColorPair::from(ext_color_pair)
                    )
                )
            }
        }
    };

    match unsafe { ncurses::getcchar(&ComplexChar::into(wcval), wch.as_mut_ptr(), attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts) } {
        OK => {
            // TODO : get opts working correct so not to rely on bodge!
            //assert!(!opts.is_null(), "ncursesw::getcchar() : opts.is_null()");
            //
            //Ok(WideCharAndAttributes::new(WideChar::from(wch[0]), attribute_colorpair_set(attrs[0], color_pair[0], unsafe { ptr::read(opts) })))

            let c: cchar_t = ComplexChar::into(wcval); // bodge to get extended color pair.

            Ok(WideCharAndAttributes::new(WideChar::from(wch[0]), attribute_colorpair_set(attrs[0], color_pair[0], c.ext_color)))
        }
        rc => Err(ncurses_function_error_with_rc!("getcchar", rc))
    }
}

pub fn getch() -> result!(CharacterResult<char>) {
    match ncurses::getch() {
        EINTR      => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("getch", rc))
            } else if rc >= KEY_MIN && rc <= KEY_MAX {
                Ok(CharacterResult::Key(KeyBinding::from(rc)))
            } else {
                Ok(CharacterResult::Character(char::from(rc as i8 as u8)))
            }
        }
    }
}

pub fn getcurx(handle: WINDOW) -> result!(i32) {
    let x = unsafe { ncurses::getcurx(handle) };

    if x < 0 {
        Err(ncurses_function_error_with_rc!("getcurx", x))
    } else {
        Ok(x)
    }
}

pub fn getcury(handle: WINDOW) -> result!(i32) {
    let y = unsafe { ncurses::getcury(handle) };

    if y < 0 {
        Err(ncurses_function_error_with_rc!("getcury", y))
    } else {
        Ok(y)
    }
}

pub fn getcuryx(handle: WINDOW) -> result!(Origin) {
    let y = unsafe { ncurses::getcury(handle) };
    let x = unsafe { ncurses::getcurx(handle) };

    if y < 0 {
        Err(ncurses_function_error_with_rc!("getcuryx (y)", y))
    } else if x < 0 {
        Err(ncurses_function_error_with_rc!("getcuryx (x)", x))
    } else {
        Ok(Origin { y, x })
    }
}

pub fn getmaxx(handle: WINDOW) -> result!(i32) {
    let x = unsafe { ncurses::getmaxx(handle) };

    if x < 0 {
        Err(ncurses_function_error_with_rc!("getmaxx", x))
    } else {
        Ok(x)
    }
}

pub fn getmaxy(handle: WINDOW) -> result!(i32) {
    let y = unsafe { ncurses::getmaxy(handle) };

    if y < 0 {
        Err(ncurses_function_error_with_rc!("getmaxy", y))
    } else {
        Ok(y)
    }
}

pub fn getmaxyx(handle: WINDOW) -> result!(Size) {
    let lines = unsafe { ncurses::getmaxy(handle) };
    let columns = unsafe { ncurses::getmaxx(handle) };

    if lines < 0 {
        Err(ncurses_function_error_with_rc!("getmaxyx (y)", lines))
    } else if columns < 0 {
        Err(ncurses_function_error_with_rc!("getmaxyx (x)", columns))
    } else {
        Ok(Size { lines, columns })
    }
}

pub fn getn_wstr(number: i32) -> result!(WideString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::getn_wstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::getn_wstr(ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("getn_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::getn_wstr() : ptr.is_null()");

                let buf_as_bytes = unsafe { slice::from_raw_parts(ptr as *mut wchar_t, LINE_MAX) };

                for (idx, &byte) in buf_as_bytes.iter().enumerate() {
                    if byte == 0x00 {
                        return Ok(WideString::from(&buf_as_bytes[..idx]));
                    }
                }

                Ok(WideString::from(buf_as_bytes))
            }
        }
    }
}

pub fn getnstr(number: i32) -> result!(String) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::getnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::getnstr(ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("getnstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::getnstr() : ptr.is_null()");

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

pub fn getparx(handle: WINDOW) -> result!(i32) {
    let x = unsafe { ncurses::getparx(handle) };

    if x < 0 {
        Err(ncurses_function_error_with_rc!("getparx", x))
    } else {
        Ok(x)
    }
}

pub fn getpary(handle: WINDOW) -> result!(i32) {
    let y = unsafe { ncurses::getpary(handle) };

    if y < 0 {
        Err(ncurses_function_error_with_rc!("getpary", y))
    } else {
        Ok(y)
    }
}

pub fn getparyx(handle: WINDOW) -> result!(Origin) {
    let y = unsafe { ncurses::getpary(handle) };
    let x = unsafe { ncurses::getparx(handle) };

    if y < 0 {
        Err(ncurses_function_error_with_rc!("getparyx (y)", y))
    } else if x < 0 {
        Err(ncurses_function_error_with_rc!("getparyx (x)", x))
    } else {
        Ok(Origin { y, x })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use getnstr() instead")]
pub fn getstr() -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::getstr(ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("getstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::getstr() : ptr.is_null()");

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

pub fn getwin(path: &path::Path) -> result!(WINDOW) {
    let mode = "r";

    match crate::shims::utils::fopen(path, mode) {
        None     => Err(NCurseswError::FOpen { fname: path.display().to_string(), mode: mode.to_string() }),
        Some(fp) => match unsafe { ncurses::getwin(fp) } {
            None      => Err(ncurses_function_error!("getwin")),
            Some(win) => Ok(win)
        }
    }
}

pub fn halfdelay(tenths: time::Duration) -> result!(()) {
    let delay = i32::try_from(tenths.as_secs())? / 10;

    match ncurses::halfdelay(delay) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("delay_output", rc))
    }
}

pub fn has_colors() -> bool {
    ncurses::has_colors()
}

pub fn has_ic() -> bool {
    ncurses::has_ic()
}

pub fn has_il() -> bool {
    ncurses::has_il()
}

pub fn has_key(ch: KeyBinding) -> bool {
    ncurses::has_key(KeyBinding::into(ch)) == TRUE
}

pub fn hline(ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::hline(ChtypeChar::into(ch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("hline", rc))
    }
}

pub fn hline_set(wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::hline_set(&ComplexChar::into(wch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("hline_set", rc))
    }
}

pub fn idcok(handle: WINDOW, bf: bool) {
    unsafe {
        ncurses::idcok(handle, bf)
    }
}

pub fn idlok(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::idlok(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("idlok", rc))
    }
}

pub fn immedok(handle: WINDOW, bf: bool) {
    unsafe { ncurses::immedok(handle, bf) }
}

pub fn in_wch() -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::in_wch(wcval.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wcval[0])),
        rc => Err(ncurses_function_error_with_rc!("in_wch", rc))
    }
}

pub fn in_wchnstr(number: i32) -> result!(ComplexString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::in_wchnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::in_wchnstr(ptr, number) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::in_wchnstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, number as usize) }))
        },
        rc => Err(ncurses_function_error_with_rc!("in_wchnstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use in_wchnstr() instead")]
pub fn in_wchstr() -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::in_wchstr(ptr) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::in_wchstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("in_wchstr", rc))
    }
}

pub fn inch() -> ChtypeChar {
    ChtypeChar::from(ncurses::inch())
}

pub fn inchnstr(number: i32) -> result!(ChtypeString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::inchnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::inchnstr(ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("inchnstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::inchnstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::inchnstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use inchnstr() instead")]
pub fn inchstr() -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::inchstr(ptr) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("inchstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::inchstr() : ptr.is_null()");
        assert!(len> 0 && len <= LINE_MAX as i32, "ncursesw::inchstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

pub fn init_color(color_number: short_t, rgb: normal::RGB) -> result!(normal::Color) {
    if i32::from(color_number) >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match ncurses::init_color(color_number, rgb.red(), rgb.green(), rgb.blue()) {
            OK => {
                set_ncurses_colortype(NCursesColorType::Normal);

                Ok(normal::Color::from(color_number))
            },
            rc => Err(ncurses_function_error_with_rc!("init_color", rc))
        }
    }
}

pub fn init_extended_color(color_number: i32, rgb: extend::RGB) -> result!(extend::Color) {
    if color_number >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match ncurses::init_extended_color(color_number, rgb.red(), rgb.green(), rgb.blue()) {
            OK => {
                set_ncurses_colortype(NCursesColorType::Extended);

                Ok(extend::Color::from(color_number))
            },
            rc => Err(ncurses_function_error_with_rc!("init_extended_color", rc))
        }
    }
}

pub fn init_extended_pair(pair_number: i32, colors: extend::Colors) -> result!(extend::ColorPair) {
    if pair_number >= COLOR_PAIRS() {
        Err(NCurseswError::ColorPairLimit)
    } else if colors.foreground().number() >= COLORS() || colors.background().number() >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match ncurses::init_extended_pair(pair_number, extend::Color::into(colors.foreground()), extend::Color::into(colors.background())) {
            OK => {
                set_ncurses_colortype(NCursesColorType::Extended);

                Ok(extend::ColorPair::from(pair_number))
            },
            rc => Err(ncurses_function_error_with_rc!("init_extended_pair", rc))
        }
    }
}

pub fn init_pair(pair_number: short_t, colors: normal::Colors) -> result!(normal::ColorPair) {
    if i32::from(pair_number) >= COLOR_PAIRS() {
        Err(NCurseswError::ColorPairLimit)
    } else if colors.foreground().number() >= COLORS() || colors.background().number() >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match ncurses::init_pair(pair_number, normal::Color::into(colors.foreground()), normal::Color::into(colors.background())) {
            OK => {
                set_ncurses_colortype(NCursesColorType::Normal);

                Ok(normal::ColorPair::from(pair_number))
            },
            rc => Err(ncurses_function_error_with_rc!("init_pair", rc))
        }
    }
}

/// Initialize the NCurses data structures and return the standard screen.
///
/// `initscr` is normally the first curses routine to call when initializing a program. A few special
/// routines sometimes need to be called before it; these are `slk_init`, `filter`, `ripoffline`, `use_env`.
/// For multiple-terminal applications, newterm may be called before `initscr`.
///
/// The `initscr` code determines the terminal type and initializes all curses data structures. `initscr`
/// also causes the first call to `refresh` to clear the screen. If errors occur, `initscr` writes an
/// appropriate error message to standard error and exits; otherwise, a pointer is returned to `stdscr`.
pub fn initscr() -> result!(WINDOW) {
    match unsafe { ncurses::initscr() } {
        None      => Err(ncurses_function_error!("initscr")),
        Some(win) => Ok(win)
    }
}

pub fn innstr(number: i32) -> result!(String) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::innstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::innstr(ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("innstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::innstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::innstr() : len={}, LINEMAX={}", len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

pub fn innwstr(number: i32) -> result!(WideString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::innwstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    let len = unsafe { ncurses::innwstr(ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("innwstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::innwstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::innwstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

/// Insert a wide character string (unicode UTF-8) of a given length on the standard screen.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // insert "Testing..Testing.."
/// ins_nwstr(&wide_str, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn ins_nwstr(wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::ins_nwstr(raw_with_nul_as_slice!(wstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ins_nwstr", rc))
    }
}

/// Insert a complex character on the standard screen.
///
/// Insert the complex character with rendition before the character under the cursor.
/// All characters to the right of the cursor are moved one space to the right, with
/// the possibility of the rightmost character on the line being lost. The insertion
/// operation does not change the cursor position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;
///
/// ins_wch(complex_char)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn ins_wch(wch: ComplexChar) -> result!(()) {
    match ncurses::ins_wch(&ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ins_wch", rc))
    }
}

/// Insert a wide character string (unicode UTF-8) on the standard screen.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // insert "Testing..Testing..1..2..3.."
/// ins_wstr(&wide_str)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn ins_wstr(wstr: &WideString) -> result!(()) {
    match ncurses::ins_wstr(raw_with_nul_as_slice!(wstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ins_wstr", rc))
    }
}

/// Insert a ascii character and `normal` attribute/color pair combination to the standard screen.
///
/// Insert the character with rendition before the character under the cursor.
/// All characters to the right of the cursor are moved one space to the right, with
/// the possibility of the rightmost character on the line being lost. The insertion
/// operation does not change the cursor position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char) | attrs;
///
/// insch(chtype_char)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn insch(ch: ChtypeChar) -> result!(()) {
    match ncurses::insch(ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insch", rc))
    }
}

pub fn insdelln(n: i32) -> result!(()) {
    match ncurses::insdelln(n) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insdelln", rc))
    }
}

basic_ncurses_function!(insertln, "insertln");

/// Insert a string of a given length on the standard screen.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let s = "Testing..Testing..1..2..3..";
///
/// // insert "Testing..Testing.."
/// insnstr(&s, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn insnstr(str: &str, number: i32) -> result!(()) {
    match ncurses::insnstr(unsafe { c_str_with_nul!(str) }, number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insnstr", rc))
    }
}

/// Insert a string on the standard screen.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let s = "Testing..Testing..1..2..3..";
///
/// // insert "Testing..Testing..1..2..3.."
/// insstr(&s)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn insstr(str: &str) -> result!(()) {
    match ncurses::insstr(unsafe { c_str_with_nul!(str) }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use innstr() instead")]
pub fn instr() -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::instr(ptr) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("instr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::instr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::instr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

pub fn intrflush(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::intrflush(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("intrflush", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use innwstr() instead")]
pub fn inwstr() -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::inwstr(ptr) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::inwstr() : ptr.is_null()");

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("inwstr", rc))
    }
}

simple_ncurses_function_with_window_returns_bool!(is_cleared);

simple_ncurses_function_with_window_returns_bool!(is_idcok);

simple_ncurses_function_with_window_returns_bool!(is_idlok);

simple_ncurses_function_with_window_returns_bool!(is_immedok);

simple_ncurses_function_with_window_returns_bool!(is_keypad);

simple_ncurses_function_with_window_returns_bool!(is_leaveok);

pub fn is_linetouched(handle: WINDOW, line: i32) -> bool {
    unsafe {
        ncurses::is_linetouched(handle, line)
    }
}

simple_ncurses_function_with_window_returns_bool!(is_nodelay);

simple_ncurses_function_with_window_returns_bool!(is_notimeout);

simple_ncurses_function_with_window_returns_bool!(is_pad);

simple_ncurses_function_with_window_returns_bool!(is_scrollok);

simple_ncurses_function_with_window_returns_bool!(is_syncok);

pub fn is_term_resized(size: Size) -> bool {
    ncurses::is_term_resized(size.lines, size.columns)
}

simple_ncurses_function_with_window_returns_bool!(is_wintouched);

pub fn isendwin() -> bool {
    ncurses::isendwin()
}

pub fn key_defined(definition: &str) -> result!(KeyBinding) {
    let c = ncurses::key_defined(unsafe { c_str_with_nul!(definition) });

    if c < 0 {
        Err(ncurses_function_error_with_rc!("key_defined", c))
    } else {
        Ok(KeyBinding::from(c))
    }
}

pub fn key_name(w: KeyBinding) -> result!(String) {
    match ncurses::key_name(KeyBinding::into(w)) {
        None    => Err(ncurses_function_error!("key_name")),
        Some(s) => Ok(s)
    }
}

pub fn keybound(keycode: KeyBinding, count: i32) -> result!(String) {
    match ncurses::keybound(KeyBinding::into(keycode), count) {
        None    => Err(ncurses_function_error!("keybound")),
        Some(s) => Ok(s)
    }
}

pub fn keyname(c: KeyBinding) -> result!(String) {
    match ncurses::keyname(KeyBinding::into(c)) {
        None    => Err(ncurses_function_error!("keyname")),
        Some(s) => Ok(s)
    }
}

pub fn keyok(keycode: KeyBinding, enable: bool) -> result!(()) {
    match ncurses::keyok(KeyBinding::into(keycode), enable) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("keyok", rc))
    }
}

pub fn keypad(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::keypad(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("keypad", rc))
    }
}

pub fn killchar() -> result!(char) {
    let rc = ncurses::killchar();

    if rc < 0 {
        Err(ncurses_function_error_with_rc!("killchar", i32::from(rc)))
    } else {
        Ok(char::from(rc as u8))
    }
}

pub fn killwchar() -> result!(WideChar) {
    let mut wch: [wchar_t; 1] = [0];

    match unsafe { ncurses::killwchar(wch.as_mut_ptr()) } {
        OK => Ok(WideChar::from(wch[0])),
        rc => Err(ncurses_function_error_with_rc!("killwchar", rc))
    }
}

pub fn leaveok(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::leaveok(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("leaveok", rc))
    }
}

pub fn longname() -> result!(String) {
    match ncurses::longname() {
        None    => Err(ncurses_function_error!("longname")),
        Some(s) => Ok(s)
    }
}

pub fn mcprint(_data: *mut i8, _len: i32) -> i32 {
    unimplemented!();
}

pub fn meta(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::meta(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("meta", rc))
    }
}

pub fn r#move(origin: Origin) -> result!(()) {
    match ncurses::r#move(origin.y, origin.x) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("move", rc))
    }
}

/// Add/Output a complex character to the standard screen at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;
///
/// mvadd_wch(origin, complex_char)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvadd_wch(origin: Origin, wch: ComplexChar) -> result!(()) {
    match ncurses::mvadd_wch(origin.y, origin.x, &ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvadd_wch", rc))
    }
}

/// Add/Output a complex character string of a given length to the standard screen at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair0)?;
///
/// // this will output "Testing..Testing.." at line 5, column 10
/// mvadd_wchnstr(origin, &complex_str, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvadd_wchnstr(origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
    match ncurses::mvadd_wchnstr(origin.y, origin.x, raw_with_nul_as_slice!(wchstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvadd_wchnstr", rc))
    }
}

/// Add/Output a complex character string to the standard screen at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair0)?;
///
/// // this will output "Testing..Testing..1..2..3.." at line 5, column 10
/// mvadd_wchstr(origin, &complex_str)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvadd_wchstr(origin: Origin, wchstr: &ComplexString) -> result!(()) {
    match ncurses::mvadd_wchstr(origin.y, origin.x, raw_with_nul_as_slice!(wchstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvadd_wchstr", rc))
    }
}

/// Add/Output a ascii character and `normal` attribute/color pair combination to the standard screen at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char) | attrs;
///
/// mvaddch(origin, chtype_char)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvaddch(origin: Origin, ch: ChtypeChar) -> result!(()) {
    match ncurses::mvaddch(origin.y, origin.x, ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddch", rc))
    }
}

/// Add/Output a ascii character string and `normal` attribute/color pair combination of a given length to the standard screen at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
/// let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;
///
/// // this will output "Testing..Testing.." at line 5, column 10
/// mvaddchnstr(origin, &chtype_str, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvaddchnstr(origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
    match ncurses::mvaddchnstr(origin.y, origin.x, raw_with_nul_as_slice!(chstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddchnstr", rc))
    }
}

/// Add/Output a ascii character string and `normal` attribute/color pair combination to the standard screen at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
/// let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;
///
/// // this will output "Testing..Testing..1..2..3.." at line 5, column 10
/// mvaddchstr(origin, &chtype_str)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvaddchstr(origin: Origin, chstr: &ChtypeString) -> result!(()) {
    match ncurses::mvaddchstr(origin.y, origin.x, raw_with_nul_as_slice!(chstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddchstr", rc))
    }
}

/// Add/Output a character string of a given length to the standard screen at a given origin.
///
/// Note: Originally this function whould just output characters in the ascii character
///       set but as of ABI 6 (and maybe eariler) this function will output any unicode
///       UTF-8 character string.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // this will output "Testing..Testing.." at line 5, column 10
/// mvaddnstr(origin, &s, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvaddnstr(origin: Origin, str: &str, number: i32) -> result!(()) {
    match ncurses::mvaddnstr(origin.y, origin.x, unsafe { c_str_with_nul!(str) }, number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddnstr", rc))
    }
}

/// Add/Output a wide character unicode UTF-8 string of a given length to the standard screen at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // this will output "Testing..Testing.." at line 5, column 10
/// mvaddnwstr(origin, &wide_str, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvaddnwstr(origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::mvaddnwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddnwstr", rc))
    }
}

/// Add/Output a character string to the standard screen at a given origin.
///
/// Note: Originally this function whould just output characters in the ascii character
///       set but as of ABI 6 (and maybe eariler) this function will output any unicode
///       UTF-8 character string.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // this will output "Testing..Testing..1..2..3.."
/// mvaddstr(origin, &s)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvaddstr(origin: Origin, str: &str) -> result!(()) {
    match ncurses::mvaddstr(origin.y, origin.x, unsafe { c_str_with_nul!(str) }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddstr", rc))
    }
}

/// Add/Output a wide character unicode UTF-8 string to the standard screen at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // this will output "Testing..Testing..1..2..3.." at line 5, column 10
/// mvaddwstr(origin, &wide_str)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvaddwstr(origin: Origin, wstr: &WideString) -> result!(()) {
    match ncurses::mvaddwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddwstr", rc))
    }
}

pub fn mvchgat<A, P, T>(origin: Origin, number: i32, attrs: A, color_pair: P) -> result!(())
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::mvchgat(origin.y, origin.x, number, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_const_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvchgat", rc))
    }
}

pub fn mvcur(old: Origin, new: Origin) -> result!(()) {
    match ncurses::mvcur(old.y, old.x, new.y, new.x) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvcur", rc))
    }
}

pub fn mvdelch(origin: Origin) -> result!(()) {
    match ncurses::mvdelch(origin.y, origin.x) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvdelch", rc))
    }
}

pub fn mvderwin(handle: WINDOW, origin: Origin) -> result!(()) {
    match unsafe { ncurses::mvderwin(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvderwin", rc))
    }
}

pub fn mvget_wch(origin: Origin) -> result!(CharacterResult<WideChar>) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::mvget_wch(origin.y, origin.x, wch.as_mut_ptr()) } {
        EINTR        => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE   => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT    => Err(NCurseswError::KeyEvent),
        KEY_CODE_YES => {
            match wch[0] as i32 {
                #[cfg(feature = "key_resize_as_error")]
                KEY_RESIZE => Err(NCurseswError::KeyResize),
                #[cfg(feature = "key_event_as_error")]
                KEY_EVENT  => Err(NCurseswError::KeyEvent),
                _          => Ok(CharacterResult::Key(KeyBinding::from(wch[0])))
            }
        },
        rc           => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvget_wch", rc))
            } else {
                Ok(CharacterResult::Character(WideChar::from(wch[0])))
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvgetn_wstr() instead")]
pub fn mvget_wstr(origin: Origin) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvget_wstr(origin.y, origin.x, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvget_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::mvget_wstr() : ptr.is_null()");

                let buf_as_bytes = unsafe { slice::from_raw_parts(ptr as *mut wchar_t, LINE_MAX) };

                for (idx, &byte) in buf_as_bytes.iter().enumerate() {
                    if byte == 0x00 {
                        return Ok(WideString::from(&buf_as_bytes[..idx]));
                    }
                }

                Ok(WideString::from(buf_as_bytes))
            }
        }
    }
}

pub fn mvgetch(origin: Origin) -> result!(CharacterResult<char>) {
    match ncurses::mvgetch(origin.y, origin.x) {
        EINTR      => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvgetch", rc))
            } else if rc >= KEY_MIN && rc <= KEY_MAX {
                Ok(CharacterResult::Key(KeyBinding::from(rc)))
            } else {
                Ok(CharacterResult::Character(char::from(rc as i8 as u8)))
            }
        }
    }
}

pub fn mvgetn_wstr(origin: Origin, number: i32) -> result!(WideString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvgetn_wstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetn_wstr(origin.y, origin.x, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvgetn_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::mvgetn_wstr() : ptr.is_null()");

                let buf_as_bytes = unsafe { slice::from_raw_parts(ptr as *mut wchar_t, LINE_MAX) };

                for (idx, &byte) in buf_as_bytes.iter().enumerate() {
                    if byte == 0x00 {
                        return Ok(WideString::from(&buf_as_bytes[..idx]));
                    }
                }

                Ok(WideString::from(buf_as_bytes))
            }
        }
    }
}

pub fn mvgetnstr(origin: Origin, number: i32) -> result!(String) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvgetnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetnstr(origin.y, origin.x, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvgetnstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::mvgetnstr() : ptr.is_null()");

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvgetnstr() instead")]
pub fn mvgetstr(origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetstr(origin.y, origin.x, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvgetstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::mvgetstr() : ptr.is_null()");

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

pub fn mvhline(origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::mvhline(origin.y, origin.x, ChtypeChar::into(ch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvhline", rc))
    }
}

pub fn mvhline_set(origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::mvhline_set(origin.y, origin.x, &ComplexChar::into(wch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvhline_set", rc))
    }
}

pub fn mvin_wch(origin: Origin) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::mvin_wch(origin.y, origin.x, wcval.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wcval[0])),
        rc => Err(ncurses_function_error_with_rc!("mvin_wch", rc))
    }
}

pub fn mvin_wchnstr(origin: Origin, number: i32) -> result!(ComplexString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvin_wchnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvin_wchnstr(origin.y, origin.x, ptr, number) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::mvin_wchnstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, number as usize) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvin_wchnstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvin_wchnstr() instead")]
pub fn mvin_wchstr(origin: Origin) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvin_wchstr(origin.y, origin.x, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::mvin_wchstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvin_wchstr", rc))
    }
}

pub fn mvinch(origin: Origin) -> ChtypeChar {
    ChtypeChar::from(ncurses::mvinch(origin.y, origin.x))
}

pub fn mvinchnstr(origin: Origin, number: i32) -> result!(ChtypeString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvinchnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinchnstr(origin.y, origin.x, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvinchnstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvinchnstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvinchnstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvinchnstr() instead")]
pub fn mvinchstr(origin: Origin) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinchstr(origin.y, origin.x, ptr) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvinchstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvinchstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvinchstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

pub fn mvinnstr(origin: Origin, number: i32) -> result!(String) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvinnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinnstr(origin.y, origin.x, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvinnstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvinnstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvinnstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

pub fn mvinnwstr(origin: Origin, number: i32) -> result!(WideString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvinnwstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinnwstr(origin.y, origin.x, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvinnwstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvinnwstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvinnwstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

/// Insert a wide character string (unicode UTF-8) of a given length on the standard screen at a given origin.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // insert "Testing..Testing.." at line 5, column 10
/// mvins_nwstr(origin, &wide_str, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvins_nwstr(origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::mvins_nwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvins_nwstr", rc))
    }
}

/// Insert a complex character on the standard screen at the given origin.
///
/// Insert the complex character with rendition before the character under the cursor.
/// All characters to the right of the cursor are moved one space to the right, with
/// the possibility of the rightmost character on the line being lost. The insertion
/// operation does not change the cursor position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;
///
/// mvins_wch(origin, complex_char)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvins_wch(origin: Origin, wch: ComplexChar) -> result!(()) {
    match ncurses::mvins_wch(origin.y, origin.x, &ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvins_wch", rc))
    }
}

/// Insert a wide character string (unicode UTF-8) on the standard screen at a given origin.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // insert "Testing..Testing..1..2..3.." at line 5, column 10
/// mvins_wstr(origin, &wide_str)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvins_wstr(origin: Origin, wstr: &WideString) -> result!(()) {
    match ncurses::mvins_wstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvins_wstr", rc))
    }
}

/// Insert a ascii character and `normal` attribute/color pair combination to the standard screen at a given origin.
///
/// Insert the character with rendition before the character under the cursor.
/// All characters to the right of the cursor are moved one space to the right, with
/// the possibility of the rightmost character on the line being lost. The insertion
/// operation does not change the cursor position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char) | attrs;
///
/// mvinsch(Origin { y: 5, x: 10 }, chtype_char)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvinsch(origin: Origin, ch: ChtypeChar) -> result!(()) {
    match ncurses::mvinsch(origin.y, origin.x, ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvinsch", rc))
    }
}

/// Insert a string of a given length on the standard screen at the given origin.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let s = "Testing..Testing..1..2..3..";
///
/// // insert "Testing..Testing.." at line 5, column 10
/// mvinsnstr(Origin { y: 5, x: 10 }, &s, 18)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvinsnstr(origin: Origin, str: &str, number: i32) -> result!(()) {
    match ncurses::mvinsnstr(origin.y, origin.x, unsafe { c_str_with_nul!(str) }, number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvinsnstr", rc))
    }
}

/// Insert a string on the standard screen at the given origin.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let s = "Testing..Testing..1..2..3..";
///
/// // insert "Testing..Testing..1..2..3.." at line 5, column 10
/// mvinsstr(Origin { y: 5, x: 10 }, &s)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvinsstr(origin: Origin, str: &str) -> result!(()) {
    match ncurses::mvinsstr(origin.y, origin.x, unsafe { c_str_with_nul!(str) }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvinsstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvinnstr() instead")]
pub fn mvinstr(origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinstr(origin.y, origin.x, ptr) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvinstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvinstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvinstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvinnwstr() instead")]
pub fn mvinwstr(origin: Origin) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvinwstr(origin.y, origin.x, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::mvinwstr() : ptr.is_null()");

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvinwstr", rc))
    }
}

pub fn mvvline(origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::mvvline(origin.y, origin.x, ChtypeChar::into(ch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvvline", rc))
    }
}

pub fn mvvline_set(origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::mvvline_set(origin.y, origin.x, &ComplexChar::into(wch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvvline_set", rc))
    }
}

/// Add/Output a complex character on a given window at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;
///
/// mvwadd_wch(win, origin, complex_char)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwadd_wch(handle: WINDOW, origin: Origin, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::mvwadd_wch(handle, origin.y, origin.x, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwadd_wch", rc))
    }
}

/// Add/Output a complex character string of a given length on a given window at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair0)?;
///
/// // this will output "Testing..Testing.." at line 5, column 10 on the window `win`.
/// mvwadd_wchnstr(win, origin, &complex_str, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwadd_wchnstr(handle: WINDOW, origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwadd_wchnstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wchstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwadd_wchnstr", rc))
    }
}

/// Add/Output a complex character string on a given window at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair0)?;
///
/// // this will output "Testing..Testing..1..2..3.." at line 5, column 10 on the window `win`.
/// mvwadd_wchstr(win, origin, &complex_str)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwadd_wchstr(handle: WINDOW, origin: Origin, wchstr: &ComplexString) -> result!(()) {
    match unsafe { ncurses::mvwadd_wchstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wchstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwadd_wchstr", rc))
    }
}

/// Add/Output a ascii character and `normal` attribute/color pair combination on a given window at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char) | attrs;
///
/// mvwaddch(win, origin, chtype_char)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwaddch(handle: WINDOW, origin: Origin, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::mvwaddch(handle, origin.y, origin.x, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddch", rc))
    }
}

/// Add/Output a ascii character string and `normal` attribute/color pair combination of a given length on a given window at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
/// #
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
/// let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;
///
/// // this will output "Testing..Testing.." at line 5, column 10 on the window `win`.
/// mvwaddchnstr(win, origin, &chtype_str, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwaddchnstr(handle: WINDOW, origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwaddchnstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(chstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddchnstr", rc))
    }
}

/// Add/Output a ascii character string and `normal` attribute/color pair combination on a given window at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
/// let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;
///
/// // this will output "Testing..Testing..1..2..3.." at line 5, column 10 on the window `win`.
/// mvwaddchstr(win, origin, &chtype_str)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwaddchstr(handle: WINDOW, origin: Origin, chstr: &ChtypeString) -> result!(()) {
    match unsafe { ncurses::mvwaddchstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(chstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddchstr", rc))
    }
}

/// Add/Output a character string of a given length to a given window at a given origin.
///
/// Note: Originally this function whould just output characters in the ascii character
///       set but as of ABI 6 (and maybe eariler) this function will output any unicode
///       UTF-8 character string.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // this will output "Testing..Testing.." at line 5, column 10 on the window `win`.
/// mvwaddnstr(win, origin, &s, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwaddnstr(handle: WINDOW, origin: Origin, str: &str, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwaddnstr(handle, origin.y, origin.x, c_str_with_nul!(str), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddnstr", rc))
    }
}

/// Add/Output a wide character unicode UTF-8 string of a given length on the given window at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // this will output "Testing..Testing.." at line 5, column 10 on the window `win`
/// mvwaddnwstr(win, origin, &wide_str, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwaddnwstr(handle: WINDOW, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwaddnwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddnwstr", rc))
    }
}

/// Add/Output a character string on a given window at a given origin.
///
/// Note: Originally this function whould just output characters in the ascii character
///       set but as of ABI 6 (and maybe eariler) this function will output any unicode
///       UTF-8 character string.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // this will output "Testing..Testing..1..2..3.." at line 5, column 10 on the window `win`.
/// mvwaddstr(win, origin, &s)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwaddstr(handle: WINDOW, origin: Origin, str: &str) -> result!(()) {
    match unsafe { ncurses::mvwaddstr(handle, origin.y, origin.x, c_str_with_nul!(str)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddstr", rc))
    }
}

/// Add/Output a wide character unicode UTF-8 string on the given window at a given origin.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // this will output "Testing..Testing..1..2..3.." at line 5, column 10 on the window `win`
/// mvwaddwstr(win, origin, &wide_str)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwaddwstr(handle: WINDOW, origin: Origin, wstr: &WideString) -> result!(()) {
    match unsafe { ncurses::mvwaddwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddwstr", rc))
    }
}

pub fn mvwchgat<A, P, T>(handle: WINDOW, origin: Origin, number: i32, attrs: A, color_pair: P) -> result!(())
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::mvwchgat(handle, origin.y, origin.x, number, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_const_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwchgat", rc))
    }
}

pub fn mvwdelch(handle: WINDOW, origin: Origin) -> result!(()) {
    match unsafe { ncurses::mvwdelch(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwdelch", rc))
    }
}

pub fn mvwget_wch(handle: WINDOW, origin: Origin) -> result!(CharacterResult<WideChar>) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::mvwget_wch(handle, origin.y, origin.x, wch.as_mut_ptr()) } {
        EINTR        => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE   => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT    => Err(NCurseswError::KeyEvent),
        KEY_CODE_YES => {
            match wch[0] as i32 {
                #[cfg(feature = "key_resize_as_error")]
                KEY_RESIZE => Err(NCurseswError::KeyResize),
                #[cfg(feature = "key_event_as_error")]
                KEY_EVENT  => Err(NCurseswError::KeyEvent),
                _          => Ok(CharacterResult::Key(KeyBinding::from(wch[0])))
            }
        },
        rc           => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvwget_wch", rc))
            } else {
                Ok(CharacterResult::Character(WideChar::from(wch[0])))
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwgetn_wstr() instead")]
pub fn mvwget_wstr(handle: WINDOW, origin: Origin) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwget_wstr(handle, origin.y, origin.y, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvwget_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::mvwget_wstr() : ptr.is_null()");

                let buf_as_bytes = unsafe { slice::from_raw_parts(ptr as *mut wchar_t, LINE_MAX) };

                for (idx, &byte) in buf_as_bytes.iter().enumerate() {
                    if byte == 0x00 {
                        return Ok(WideString::from(&buf_as_bytes[..idx]));
                    }
                }

                Ok(WideString::from(buf_as_bytes))
            }
        }
    }
}

pub fn mvwgetch(handle: WINDOW, origin: Origin) -> result!(CharacterResult<char>) {
    match unsafe { ncurses::mvwgetch(handle, origin.y, origin.x) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvwgetch", rc))
            } else if rc >= KEY_MIN && rc <= KEY_MAX {
                Ok(CharacterResult::Key(KeyBinding::from(rc)))
            } else {
                Ok(CharacterResult::Character(char::from(rc as i8 as u8)))
            }
        }
    }
}

pub fn mvwgetn_wstr(handle: WINDOW, origin: Origin, number: i32) -> result!(WideString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvwgetn_wstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetn_wstr(handle, origin.y, origin.x, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvwgetn_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::mvwgetn_wstr() : ptr.is_null()");

                let buf_as_bytes = unsafe { slice::from_raw_parts(ptr as *mut wchar_t, LINE_MAX) };

                for (idx, &byte) in buf_as_bytes.iter().enumerate() {
                    if byte == 0x00 {
                        return Ok(WideString::from(&buf_as_bytes[..idx]));
                    }
                }

                Ok(WideString::from(buf_as_bytes))
            }
        }
    }
}

pub fn mvwgetnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(String) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvwgetnstr() : number={}, LINE_MAX{}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetnstr(handle, origin.y, origin.x, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvwgetnstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::mvwgetnstr() : ptr.is_null()");

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwgetnstr() instead")]
pub fn mvwgetstr(handle: WINDOW, origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetstr(handle, origin.y, origin.x, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("mvwgetstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::mvwgetstr() : ptr.is_null()");

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

pub fn mvwhline(handle: WINDOW, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwhline(handle, origin.y, origin.x, ChtypeChar::into(ch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwhline", rc))
    }
}

pub fn mvwhline_set(handle: WINDOW, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwhline_set(handle, origin.y, origin.x, &ComplexChar::into(wch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwhline_set", rc))
    }
}

pub fn mvwin(handle: WINDOW, origin: Origin) -> result!(()) {
    match unsafe { ncurses::mvwin(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwin", rc))
    }
}

pub fn mvwin_wch(handle: WINDOW, origin: Origin) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::mvwin_wch(handle, origin.y, origin.x, wcval.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wcval[0])),
        rc => Err(ncurses_function_error_with_rc!("mvwin_wch", rc))
    }
}

pub fn mvwin_wchnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(ComplexString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvwin_wchnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwin_wchnstr(handle, origin.y, origin.x, ptr, number) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::mvwin_wchnstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, number as usize) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvwin_wchnstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwin_wchnstr() instead")]
pub fn mvwin_wchstr(handle: WINDOW, origin: Origin) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwin_wchstr(handle, origin.y, origin.x, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::mvwin_wchstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvwin_wchstr", rc))
    }
}

pub fn mvwinch(handle: WINDOW, origin: Origin) -> ChtypeChar {
    ChtypeChar::from(unsafe { ncurses::mvwinch(handle, origin.y, origin.x) })
}

pub fn mvwinchnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(ChtypeString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvwinchnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinchnstr(handle, origin.y, origin.x, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvwinchnstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvwinchnstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvwinchnstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwinchnstr() instead")]
pub fn mvwinchstr(handle: WINDOW, origin: Origin) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinchstr(handle, origin.y, origin.x, ptr) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvwinchstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvwinchstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvwinchstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

pub fn mvwinnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(String) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvwinnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinnstr(handle, origin.y, origin.x, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvwinnstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvwinnstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvwinnstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

pub fn mvwinnwstr(handle: WINDOW, origin: Origin, number: i32) -> result!(WideString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::mvwinnwstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinnwstr(handle, origin.y, origin.x, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvwinnwstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvwinnwstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvwinnwstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

/// Insert a wide character string (unicode UTF-8) of a given length on the given window at a given origin.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // insert "Testing..Testing.." at line 5, column 10 on the window `win`
/// mvwins_nwstr(win, origin, &wide_str, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwins_nwstr(handle: WINDOW, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwins_nwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwins_nwstr", rc))
    }
}

/// Insert a complex character on the given window at the given origin.
///
/// Insert the complex character with rendition before the character under the cursor.
/// All characters to the right of the cursor are moved one space to the right, with
/// the possibility of the rightmost character on the line being lost. The insertion
/// operation does not change the cursor position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;
///
/// mvwins_wch(win, origin, complex_char)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwins_wch(handle: WINDOW, origin: Origin, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::mvwins_wch(handle, origin.y, origin.x, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwins_wch", rc))
    }
}

/// Insert a wide character string (unicode UTF-8) on the given window at a given origin.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // insert "Testing..Testing..1..2.3.." at line 5, column 10 on the window `win`
/// mvwins_wstr(win, origin, &wide_str)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwins_wstr(handle: WINDOW, origin: Origin, wstr: &WideString) -> result!(()) {
    match unsafe { ncurses::mvwins_wstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwins_wstr", rc))
    }
}

/// Insert a ascii character and `normal` attribute/color pair combination on the given window at a given origin.
///
/// Insert the character with rendition before the character under the cursor.
/// All characters to the right of the cursor are moved one space to the right, with
/// the possibility of the rightmost character on the line being lost. The insertion
/// operation does not change the cursor position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char) | attrs;
///
/// mvwinsch(win, Origin { y: 5, x: 10 }, chtype_char)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwinsch(handle: WINDOW, origin: Origin, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::mvwinsch(handle, origin.y, origin.x, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwinsch", rc))
    }
}

/// Insert a string of a given length on the given window at the given origin.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // insert "Testing..Testing.." at line 5, column 10
/// mvwinsnstr(win, Origin { y: 5, x: 10 }, &s, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwinsnstr(handle: WINDOW, origin: Origin, str: &str, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwinsnstr(handle, origin.y, origin.x, c_str_with_nul!(str), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwinsnstr", rc))
    }
}

/// Insert a string on the given window at the given origin.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // insert "Testing..Testing..1..2..3.." at line 5, column 10
/// mvwinsstr(win, Origin { y: 5, x: 10 }, &s)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn mvwinsstr(handle: WINDOW, origin: Origin, str: &str) -> result!(()) {
    match unsafe { ncurses::mvwinsstr(handle, origin.y, origin.x, c_str_with_nul!(str)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwinsstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwinnstr() instead")]
pub fn mvwinstr(handle: WINDOW, origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinstr(handle, origin.y, origin.x, ptr) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("mvwinstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::mvwinstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::mvwinstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwinnwstr() instead")]
pub fn mvwinwstr(handle: WINDOW, origin: Origin) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwinwstr(handle, origin.y, origin.x, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::mvwinwstr() : ptr.is_null()");

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvwinwstr", rc))
    }
}

pub fn mvwvline(handle: WINDOW, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwvline(handle, origin.y, origin.x, ChtypeChar::into(ch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwvline", rc))
    }
}

pub fn mvwvline_set(handle: WINDOW, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwvline_set(handle, origin.y, origin.x, &ComplexChar::into(wch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwvline_set", rc))
    }
}

#[deprecated(since = "0.3.2", note = "ncurses library call superseeded by native rust call. Use std::thread::sleep(dur: std::time::Duration) instead")]
pub fn napms(ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    match ncurses::napms(ms) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("napms", rc))
    }
}

pub fn newpad(size: Size) -> result!(WINDOW) {
    match unsafe { ncurses::newpad(size.lines, size.columns) } {
        None      => Err(ncurses_function_error!("newpad")),
        Some(win) => Ok(win)
    }
}

pub fn newterm(_ty: Option<&str>, _outfd: crate::shims::bindings::FILE, _infd: crate::shims::bindings::FILE) -> Option<SCREEN> {
    unimplemented!();
}

pub fn newwin(size: Size, origin: Origin) -> result!(WINDOW) {
    match unsafe { ncurses::newwin(size.lines, size.columns, origin.y, origin.x) } {
        None      => Err(ncurses_function_error!("newwin")),
        Some(win) => Ok(win)
    }
}

basic_ncurses_function!(nl, "nl");

basic_ncurses_function!(nocbreak, "nocbreak");

pub fn nodelay(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::nodelay(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("nodelay", rc))
    }
}

basic_ncurses_function!(noecho, "noecho");

simple_ncurses_function!(nofilter);

basic_ncurses_function!(nonl, "nonl");

simple_ncurses_function!(noqiflush);

basic_ncurses_function!(noraw, "noraw");

pub fn notimeout(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::notimeout(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("notimeout", rc))
    }
}

pub fn overlay(src_handle: WINDOW, dst_handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::overlay(src_handle, dst_handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("overlay", rc))
    }
}

pub fn overwrite(src_handle: WINDOW, dst_handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::overwrite(src_handle, dst_handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("overwrite", rc))
    }
}

pub fn pair_content(color_pair: normal::ColorPair) -> result!(normal::Colors) {
    let mut fg: [short_t; 1] = [0];
    let mut bg: [short_t; 1] = [0];

    match unsafe { ncurses::pair_content(normal::ColorPair::into(color_pair), fg.as_mut_ptr(), bg.as_mut_ptr()) } {
        OK => Ok(normal::Colors::new(normal::Color::from(fg[0]), normal::Color::from(bg[0]))),
        rc => Err(ncurses_function_error_with_rc!("pair_content", rc))
    }
}

pub fn pechochar(pad: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::pechochar(pad, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("pechochar", rc))
    }
}

pub fn pecho_wchar(pad: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::pecho_wchar(pad, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("pecho_wchar", rc))
    }
}

pub fn pnoutrefresh(pad: WINDOW, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
    match unsafe { ncurses::pnoutrefresh(pad, pmin.y, pmin.x, smin.y, smin.x, smax.y, smax.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("pnoutrefresh", rc))
    }
}

pub fn prefresh(pad: WINDOW, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
    match unsafe { ncurses::prefresh(pad, pmin.y, pmin.x, smin.y, smin.x, smax.y, smax.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("prefresh", rc))
    }
}

pub fn putp(_str: &str) -> i32 {
    unimplemented!();
}

pub fn putwin(handle: WINDOW, path: &path::Path) -> result!(()) {
    let mode = "w";

    match crate::shims::utils::fopen(path, mode) {
        None     => Err(NCurseswError::FOpen { fname: path.display().to_string(), mode:  mode.to_string() }),
        Some(fp) => {
            match unsafe { ncurses::putwin(handle, fp) } {
                OK => Ok(()),
                rc => Err(ncurses_function_error_with_rc!("putwin", rc))
            }
        }
    }
}

simple_ncurses_function!(qiflush);

basic_ncurses_function!(raw, "raw");

basic_ncurses_function_with_window!(redrawwin, "redrawwin");

basic_ncurses_function!(refresh, "refresh");

simple_ncurses_function!(reset_color_pairs);

basic_ncurses_function!(reset_prog_mode, "reset_prog_mode");

basic_ncurses_function!(reset_shell_mode, "reset_shell_mode");

basic_ncurses_function!(resetty, "resetty");

pub fn resize_term(size: Size) -> result!(()) {
    match ncurses::resize_term(size.lines, size.columns) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("resize_term", rc))
    }
}

pub fn resizeterm(size: Size) -> result!(()) {
    match ncurses::resizeterm(size.lines, size.columns) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("resizeterm", rc))
    }
}

pub fn ripoffline(line: Orientation, init: RipoffInit) -> result!(()) {
    match ncurses::ripoffline(
        match line {
            Orientation::Top    => 1,
            Orientation::Bottom => -1
        },
        init
    ) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ripoffline", rc))
    }
}

basic_ncurses_function!(savetty, "savetty");

pub fn scr_dump(_filename: &str) -> i32 {
    unimplemented!();
}

pub fn scr_init(_filename: &str) -> i32 {
    unimplemented!();
}

pub fn scr_restore(_filename: &str) -> i32 {
    unimplemented!();
}

pub fn scr_set(_filename: &str) -> i32 {
    unimplemented!();
}

pub fn scrl(n: i32) -> result!(()) {
    match ncurses::scrl(n) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scrl", rc))
    }
}

basic_ncurses_function_with_window!(scroll, "scroll");

pub fn scrollok(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::scrollok(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scrollok", rc))
    }
}

pub fn set_escdelay(size: time::Duration) -> result!(()) {
    let ms = i32::try_from(size.as_millis())?;

    match ncurses::set_escdelay(ms) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("set_escdelay", rc))
    }
}

pub fn set_tabsize(size: i32) -> result!(()) {
    match ncurses::set_tabsize(size) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("set_tabsize", rc))
    }
}

pub fn set_term(scr: SCREEN) -> result!(SCREEN) {
    match unsafe { ncurses::set_term(scr) } {
        None    => Err(ncurses_function_error!("set_term")),
        Some(s) => Ok(s)
    }
}

pub fn setcchar<A, P, T>(ch: char, attrs: &A, color_pair: &P) -> result!(ComplexChar)
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    let mut cchar_buf: [cchar_t; 1] = unsafe { mem::zeroed() };
    let wchar_buf: [wchar_t; 2] = [u32::from(ch) as wchar_t, 0x00];

    let cchar_ptr: *mut cchar_t = cchar_buf.as_mut_ptr();

    match unsafe { ncurses::setcchar(cchar_ptr, wchar_buf.as_ptr(), attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => {
            assert!(!cchar_ptr.is_null(), "ncursesw::setcchar() : cchar_ptr.is_null()");

            Ok(ComplexChar::from(unsafe { slice::from_raw_parts(cchar_ptr, 1)[0] }))
        },
        rc => Err(ncurses_function_error_with_rc!("setcchar", rc))
    }
}

pub fn setscrreg(region: Region) -> result!(()) {
    match ncurses::setscrreg(region.top, region.bottom) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("setscrreg", rc))
    }
}

// convert into the attributes type of your choice with
//     normal::Attributes::from(slk_attr()) or
//     extend::Attributes::from(slk_attr())
pub fn slk_attr() -> attr_t {
    ncurses::slk_attr()
}

pub fn slk_attr_off<A, T>(attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::slk_attr_off(attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attr_off", rc))
    }
}

pub fn slk_attr_on<A, T>(attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::slk_attr_on(attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attr_on", rc))
    }
}

pub fn slk_attr_set<A, P, T>(attrs: A, color_pair: P) -> result!(())
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::slk_attr_set(attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attr_set", rc))
    }
}

pub fn slk_attroff(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attroff(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attroff", rc))
    }
}

pub fn slk_attron(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attron(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attron", rc))
    }
}

pub fn slk_attrset(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attrset(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attrset", rc))
    }
}

basic_ncurses_function!(slk_clear, "slk_clear");

pub fn slk_color(color_pair: normal::ColorPair) -> result!(()) {
    match ncurses::slk_color(color_pair.number()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_color", rc))
    }
}

pub fn slk_init(fmt: SoftLabelType) -> result!(()) {
    match ncurses::slk_init(match fmt {
        SoftLabelType::ThreeTwoThree => 0,
        SoftLabelType::FourFour      => 1,
        SoftLabelType::FourFourIndex => 2
    }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_init", rc))
    }
}

pub fn slk_label(number: i32) -> String {
    ncurses::slk_label(number)
}

basic_ncurses_function!(slk_noutrefresh, "slk_noutrefresh");

basic_ncurses_function!(slk_refresh, "slk_refresh");

basic_ncurses_function!(slk_restore, "slk_restore");

pub fn slk_set(label_number: i32, label: &str, fmt: Justification) -> result!(()) {
    match ncurses::slk_set(label_number, label, fmt.value()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_set", rc))
    }
}

basic_ncurses_function!(slk_touch, "slk_touch");

pub fn slk_wset(label_number: i32, label: &WideString, fmt: Justification) -> result!(()) {
    match ncurses::slk_wset(label_number, raw_with_nul_as_slice!(label), fmt.value()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_set", rc))
    }
}

basic_ncurses_function!(standend, "standend");

basic_ncurses_function!(standout, "standout");

basic_ncurses_function!(start_color, "start_color");

pub fn subpad(handle: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    match unsafe { ncurses::subpad(handle, size.lines, size.columns, origin.y, origin.x) } {
        None      => Err(ncurses_function_error!("subpad")),
        Some(win) => Ok(win)
    }
}

pub fn subwin(handle: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    match unsafe { ncurses::subwin(handle, size.lines, size.columns, origin.y, origin.x) } {
        None         => Err(ncurses_function_error!("subwin")),
        Some(handle) => Ok(handle)
    }
}

pub fn syncok(handle: WINDOW, bf: bool) -> result!(()) {
    match unsafe { ncurses::syncok(handle, bf) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("syncok", rc))
    }
}

pub fn term_attrs() -> attr_t {
    unimplemented!();
}

pub fn termattrs() -> chtype {
    unimplemented!();
}

pub fn termname() -> result!(String) {
    match ncurses::termname() {
        None    => Err(ncurses_function_error!("termname")),
        Some(s) => Ok(s)
    }
}

pub fn tigetflag(_capname: &str) -> i32 {
    unimplemented!();
}

pub fn tigetnum(_capname: &str) -> i32 {
    unimplemented!();
}

pub fn tigetstr(_capname: &str) -> String {
    unimplemented!();
}

pub fn timeout(ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    ncurses::timeout(ms);

    Ok(())
}

pub fn touchline(handle: WINDOW, start: i32, count: i32) -> result!(()) {
    match unsafe { ncurses::touchline(handle, start, count) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("touchline", rc))
    }
}

basic_ncurses_function_with_window!(touchwin, "touchwin");

pub fn tparm(_s: &str) -> String {
    unimplemented!();
}

pub fn typeahead(_fd: i32) -> i32 {
    unimplemented!();
}

pub fn unctrl(c: ChtypeChar) -> String {
    ncurses::unctrl(ChtypeChar::into(c))
}

pub fn unget_wch(ch: WideChar) -> result!(()) {
    match ncurses::unget_wch(WideChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("unget_wch", rc))
    }
}

pub fn ungetch(ch: char) -> result!(()) {
    match ncurses::ungetch(i32::from(ch as u8)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ungetch", rc))
    }
}

basic_ncurses_function_with_window!(untouchwin, "untouchwin");

basic_ncurses_function!(use_default_colors, "use_default_colors");

pub fn use_env(f: bool) {
    ncurses::use_env(f)
}

pub fn use_extended_names(enable: bool) -> bool {
    ncurses::use_extended_names(enable) == TRUE
}

pub fn use_legacy_coding(level: Legacy) -> result!(Legacy) {
    match ncurses::use_legacy_coding(match level {
        Legacy::Level0 => 0,
        Legacy::Level1 => 1,
        Legacy::Level2 => 2
    }) {
        0  => Ok(Legacy::Level0),
        1  => Ok(Legacy::Level1),
        2  => Ok(Legacy::Level2),
        rc => Err(ncurses_function_error_with_rc!("use_legacy_coding", rc))
    }
}

pub fn use_tioctl(f: bool) {
    ncurses::use_tioctl(f)
}

pub fn vid_attr(_attrs: attr_t, _pair: short_t) -> i32 {
    unimplemented!();
}

pub fn vidattr(_attrs: chtype) -> i32 {
    unimplemented!();
}

pub fn vline(ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::vline(ChtypeChar::into(ch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("vline", rc))
    }
}

pub fn vline_set(wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::vline_set(&ComplexChar::into(wch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("vline_set", rc))
    }
}

/// Add/Output a complex character on a given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;
///
/// wadd_wch(win, complex_char)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wadd_wch(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::wadd_wch(handle, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wadd_wch", rc))
    }
}

/// Add/Output a complex character string of a given length on a given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair0)?;
///
/// // this will output "Testing..Testing.." on the window `win`.
/// wadd_wchnstr(win, &complex_str, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wadd_wchnstr(handle: WINDOW, wchstr: &ComplexString, number: i32) -> result!(()) {
    match unsafe { ncurses::wadd_wchnstr(handle, raw_with_nul_as_slice!(wchstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wadd_wchnstr", rc))
    }
}

/// Add/Output a complex character string on a given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair0)?;
///
/// // this will output "Testing..Testing..1..2..3.." on the window `win`.
/// wadd_wchstr(win, &complex_str)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wadd_wchstr(handle: WINDOW, wchstr: &ComplexString) -> result!(()) {
    match unsafe { ncurses::wadd_wchstr(handle, raw_with_nul_as_slice!(wchstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wadd_wchstr", rc))
    }
}

/// Add/Output a ascii character and `normal` attribute/color pair combination on a given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char) | attrs;
///
/// waddch(win, chtype_char)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn waddch(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::waddch(handle, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddch", rc))
    }
}

/// Add/Output a ascii character string and `normal` attribute/color pair combination of a given length on a given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
/// let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;
///
/// // this will output "Testing..Testing.." on the window `win`.
/// waddchnstr(win, &chtype_str, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn waddchnstr(handle: WINDOW, chstr: &ChtypeString, number: i32) -> result!(()) {
    match unsafe { ncurses::waddchnstr(handle, raw_with_nul_as_slice!(chstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddchnstr", rc))
    }
}

/// Add/Output a ascii character string and `normal` attribute/color pair combination on a given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// # use std::error::Error;
/// use ascii::*;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default() | color_pair0;
///
/// let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
/// let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;
///
/// // this will output "Testing..Testing..1..2..3.." on the window `win`.
/// waddchstr(win, &chtype_str)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn waddchstr(handle: WINDOW, chstr: &ChtypeString) -> result!(()) {
    match unsafe { ncurses::waddchstr(handle, raw_with_nul_as_slice!(chstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddchstr", rc))
    }
}

/// Add/Output a character string of a given length to a given window.
///
/// Note: Originally this function whould just output characters in the ascii character
///       set but as of ABI 6 (and maybe eariler) this function will output any unicode
///       UTF-8 character string.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // this will output "Testing..Testing.." at line 5, column 10 on the window `win`.
/// waddnstr(win, &s, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn waddnstr(handle: WINDOW, str: &str, number: i32) -> result!(()) {
    match unsafe { ncurses::waddnstr(handle, c_str_with_nul!(str), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddnstr", rc))
    }
}

/// Add/Output a wide character unicode UTF-8 string of a given length on the given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // this will output "Testing..Testing.." on the window `win`
/// waddnwstr(win, &wide_str, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn waddnwstr(handle: WINDOW, wstr: &WideString, number: i32) -> result!(()) {
    match unsafe { ncurses::waddnwstr(handle, raw_with_nul_as_slice!(wstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddnwstr", rc))
    }
}

/// Add/Output a character string on a given window.
///
/// Note: Originally this function whould just output characters in the ascii character
///       set but as of ABI 6 (and maybe eariler) this function will output any unicode
///       UTF-8 character string.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // this will output "Testing..Testing..1..2..3.." at line 5, column 10 on the window `win`.
/// waddstr(win, &s)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn waddstr(handle: WINDOW, str: &str) -> result!(()) {
    match unsafe { ncurses::waddstr(handle, c_str_with_nul!(str)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddstr", rc))
    }
}

/// Add/Output a wide character unicode UTF-8 string on the given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let origin = Origin { y: 5, x: 10 };
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // this will output "Testing..Testing..1..2..3.." on the window `win`
/// waddwstr(win, &wide_str)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn waddwstr(handle: WINDOW, wstr: &WideString) -> result!(()) {
    match unsafe { ncurses::waddwstr(handle, raw_with_nul_as_slice!(wstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddwstr", rc))
    }
}

/// Return the current attributes and color pair on the given window.
///
/// Notes: This does *NOT* return the attribute and color pair rendition when defined
///        by `chtype` and/or `cchar` type add/insert functions as these are cell based
///        but when set by functions such as `wattr_set`.
///        When returning a `normal` attribute and color pair the attribute does *NOT*
///        contain the color pair so this must be OR'd to back for some functions.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// wattr_set(win, attrs1, color_pair1)?;
/// waddch(win, chtype_char | attrs0)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s) => {
///         assert!(s.attributes().is_bold());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wattr_get(handle: WINDOW) -> result!(AttributesColorPairSet) {
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let mut opts: [i32; 1] = [0];

    match unsafe { ncurses::wattr_get(handle, attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts.as_mut_ptr() as *mut c_void) } {
        OK => Ok(match ncurses_colortype() {
                     NCursesColorType::Normal => {
                         AttributesColorPairSet::Normal(
                             normal::AttributesColorPair::new(
                                 normal::Attributes::from(attrs[0]),
                                 normal::ColorPair::from(color_pair[0])
                             )
                         )
                     },
                     NCursesColorType::Extended => {
                         AttributesColorPairSet::Extended(
                             extend::AttributesColorPair::new(
                                 extend::Attributes::from(attrs[0]),
                                 extend::ColorPair::from(opts[0])
                             )
                         )
                     }
              }),
        rc => Err(ncurses_function_error_with_rc!("wattr_get", rc))
    }
}

/// Switch off the given attributes on the given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | Attribute::Dim | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// wattr_set(win, attrs1, color_pair1)?;
/// waddch(win, chtype_char | attrs0)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// wattr_off(win, Attributes::default() | Attribute::Dim)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(!s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wattr_off<A, T>(handle: WINDOW, attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::wattr_off(handle, attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattr_off", rc))
    }
}

/// Switch on the given attributes on the given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// wattr_set(win, attrs1, color_pair1)?;
/// waddch(win, chtype_char | attrs0)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(!s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// wattr_on(win, Attributes::default() | Attribute::Dim)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wattr_on<A, T>(handle: WINDOW, attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::wattr_on(handle, attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattr_on", rc))
    }
}

/// Set the current attributes and color pair on the given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// wattr_set(win, attrs1, color_pair1)?;
/// waddch(win, chtype_char | attrs0)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wattr_set<A, P, T>(handle: WINDOW, attrs: A, color_pair: P) -> result!(())
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::wattr_set(handle, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattr_set", rc))
    }
}

/// Switch off the given `normal` attributes on the given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | Attribute::Dim | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// wattrset(win, attrs1)?;
/// waddch(win, chtype_char | attrs0)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// wattroff(win, Attributes::default() | Attribute::Dim)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(!s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wattroff(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match unsafe { ncurses::wattroff(handle, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattroff", rc))
    }
}

/// Switch on the given `normal` attributes on the given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// wattrset(win, attrs1)?;
/// waddch(win, chtype_char | attrs0)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(!s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// wattron(win, Attribute::Dim | color_pair1)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.attributes().is_dim());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wattron(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match unsafe { ncurses::wattron(handle, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattron", rc))
    }
}

/// Set the current `normal` attributes and color pair on the given window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// use_default_colors()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let color_pair1 = ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Blue)))?;
///
/// let attrs0 = Attribute::Dim | color_pair0;
/// let attrs1 = Attribute::Bold | color_pair1;
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char);
///
/// wattrset(win, attrs1)?;
/// waddch(win, chtype_char | attrs0)?;
///
/// match wattr_get(win)? {
///     AttributesColorPairSet::Normal(s)   => {
///         assert!(s.attributes().is_bold());
///         assert!(s.color_pair() == color_pair1);
///     },
///     AttributesColorPairSet::Extended(_) => {
///         panic!("not a extended attributes/color pair!");
///     }
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wattrset(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match unsafe { ncurses::wattrset(handle, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattrset", rc))
    }
}

/// Set the background property on the given window and then apply this setting to every character position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use shims::ncurses::ACS_CKBOARD;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// wbkgd(win, ChtypeChar::from_chtype(ACS_CKBOARD()))?;
///
/// delwin(win)?;
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wbkgd(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::wbkgd(handle, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wbkgd", rc))
    }
}

/// Manipulate the background of the given window.
///
/// The window background is a `chtype` consisting of any combination of attributes
/// (i.e., rendition) and a character. The attribute part of the background is
/// combined (OR'ed) with all non-blank characters that are written into the window
/// with waddch. Both the character and attribute parts of the background are combined
/// with the blank characters. The background becomes a property of the character and
/// moves with the character through any scrolling and insert/delete line/character operations.
///
/// To the extent possible on a particular terminal, the attribute part of the
/// background is displayed as the graphic rendition of the character put on the screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use shims::ncurses::ACS_CKBOARD;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// wbkgdset(win, ChtypeChar::from_chtype(ACS_CKBOARD()));
///
/// delwin(win)?;
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wbkgdset(handle: WINDOW, ch: ChtypeChar) {
    unsafe {
        ncurses::wbkgdset(handle, ChtypeChar::into(ch))
    }
}

/// Set the background property on the given window and then apply this setting to every character position in that window.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::extend::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let yellow = Color::Dark(BaseColor::Yellow);
/// let blue = Color::Dark(BaseColor::Blue);
///
/// let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
/// let mut attrs = Attributes::default();
/// attrs.set_dim(true);
///
/// match std::char::from_u32(0x20) {
///     Some(c) => {
///         let background_char = ComplexChar::from_char(c, &attrs, &color_pair1)?;
///         wbkgrnd(win, background_char)?;
///     },
///     None    => panic!("unable to convert to character!")
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wbkgrnd(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::wbkgrnd(handle, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wbkgrnd", rc))
    }
}

/// Manipulate the background on the given window.
///
/// The window background is a `cchar_t` consisting of any combination of attributes
/// (i.e., rendition) and a complex character. The attribute part of the background
/// is combined (OR'ed) with all non-blank characters that are written into the window
/// with `waddch`. Both the character and attribute parts of the background are combined
/// with the blank characters. The background becomes a property of the character and moves
/// with the character through any scrolling and insert/delete line/character operations.
///
/// To the extent possible on a particular terminal, the attribute part of the background
/// is displayed as the graphic rendition of the character put on the screen.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::extend::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let yellow = Color::Dark(BaseColor::Yellow);
/// let blue = Color::Dark(BaseColor::Blue);
///
/// let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
/// let mut attrs = Attributes::default();
/// attrs.set_dim(true);
///
/// match std::char::from_u32(0x20) {
///     Some(c) => {
///         let background_char = ComplexChar::from_char(c, &attrs, &color_pair1)?;
///         wbkgrndset(win, background_char);
///     },
///     None    => panic!("unable to convert to character!")
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wbkgrndset(handle: WINDOW, wch: ComplexChar) {
    unsafe {
        ncurses::wbkgrndset(handle, &ComplexChar::into(wch))
    }
}

/// Draw a box around the edges of the given window.
///
/// ls - left side,
/// rs - right side,
/// ts - top side,
/// bs - bottom side,
/// tl - top left-hand corner,
/// tr - top right-hand corner,
/// bl - bottom left-hand corner, and
/// br - bottom right-hand corner.
///
/// If any of these arguments is zero, then the corresponding
/// default values are used instead:
///     ACS_VLINE,
///     ACS_VLINE,
///     ACS_HLINE,
///     ACS_HLINE,
///     ACS_ULCORNER,
///     ACS_URCORNER,
///     ACS_LLCORNER,
///     ACS_LRCORNER.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use shims::ncurses::{
///     ACS_VLINE, ACS_HLINE, ACS_ULCORNER,
///     ACS_URCORNER, ACS_LLCORNER, ACS_LRCORNER
/// };
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let ls = ChtypeChar::from_chtype(ACS_VLINE());
/// let rs = ChtypeChar::from_chtype(ACS_VLINE());
/// let ts = ChtypeChar::from_chtype(ACS_HLINE());
/// let bs = ChtypeChar::from_chtype(ACS_HLINE());
/// let tl = ChtypeChar::from_chtype(ACS_ULCORNER());
/// let tr = ChtypeChar::from_chtype(ACS_URCORNER());
/// let bl = ChtypeChar::from_chtype(ACS_LLCORNER());
/// let br = ChtypeChar::from_chtype(ACS_LRCORNER());
///
/// wborder(win, ls, rs, ts, bs, tl, tr, bl, br)?;
///
/// delwin(win)?;
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wborder(
    handle: WINDOW,
    ls: ChtypeChar,
    rs: ChtypeChar,
    ts: ChtypeChar,
    bs: ChtypeChar,
    tl: ChtypeChar,
    tr: ChtypeChar,
    bl: ChtypeChar,
    br: ChtypeChar) -> result!(())
{
    match unsafe { ncurses::wborder(
        handle,
        ChtypeChar::into(ls),
        ChtypeChar::into(rs),
        ChtypeChar::into(ts),
        ChtypeChar::into(bs),
        ChtypeChar::into(tl),
        ChtypeChar::into(tr),
        ChtypeChar::into(bl),
        ChtypeChar::into(br)
    ) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wborder", rc)),
    }
}

pub fn wborder_set(
    handle: WINDOW,
    ls: ComplexChar,
    rs: ComplexChar,
    ts: ComplexChar,
    bs: ComplexChar,
    tl: ComplexChar,
    tr: ComplexChar,
    bl: ComplexChar,
    br: ComplexChar) -> result!(())
{
    match unsafe { ncurses::wborder_set(
        handle,
        &ComplexChar::into(ls),
        &ComplexChar::into(rs),
        &ComplexChar::into(ts),
        &ComplexChar::into(bs),
        &ComplexChar::into(tl),
        &ComplexChar::into(tr),
        &ComplexChar::into(bl),
        &ComplexChar::into(br)
    ) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wborder_set", rc))
    }
}

pub fn wchgat<A, P, T>(handle: WINDOW, number: i32, attrs: A, color_pair: P) -> result!(())
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::wchgat(handle, number, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_const_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wchgat", rc))
    }
}

basic_ncurses_function_with_window!(wclear, "wclear");

basic_ncurses_function_with_window!(wclrtobot, "wclrtobot");

basic_ncurses_function_with_window!(wclrtoeol, "wclrtoeol");

pub fn wcolor_set<P, T>(handle: WINDOW, color_pair: P) -> result!(())
    where P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::wcolor_set(handle, color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wcolor_set", rc))
    }
}

pub fn wcursyncup(handle: WINDOW) {
    unsafe {
        ncurses::wcursyncup(handle)
    }
}

basic_ncurses_function_with_window!(wdelch, "wdelch");

pub fn wecho_wchar(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::wecho_wchar(handle, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wecho_wchar", rc))
    }
}

pub fn wechochar(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::wechochar(handle, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wechochar", rc))
    }
}

basic_ncurses_function_with_window!(werase, "werase");

pub fn wget_wch(handle: WINDOW) -> result!(CharacterResult<WideChar>) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::wget_wch(handle, wch.as_mut_ptr()) } {
        EINTR        => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE   => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT    => Err(NCurseswError::KeyEvent),
        KEY_CODE_YES => {
            match wch[0] as i32 {
                #[cfg(feature = "key_resize_as_error")]
                KEY_RESIZE => Err(NCurseswError::KeyResize),
                #[cfg(feature = "key_event_as_error")]
                KEY_EVENT  => Err(NCurseswError::KeyEvent),
                _          => Ok(CharacterResult::Key(KeyBinding::from(wch[0])))
            }
        },
        rc           => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("wget_wch", rc))
            } else {
                Ok(CharacterResult::Character(WideChar::from(wch[0])))
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use wgetn_wstr() instead")]
pub fn wget_wstr(handle: WINDOW) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::wget_wstr(handle, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("wget_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::wget_wstr() : ptr.is_null()");

                let buf_as_bytes = unsafe { slice::from_raw_parts(ptr as *mut wchar_t, LINE_MAX) };

                for (idx, &byte) in buf_as_bytes.iter().enumerate() {
                    if byte == 0x00 {
                        return Ok(WideString::from(&buf_as_bytes[..idx]));
                    }
                }

                Ok(WideString::from(buf_as_bytes))
            }
        }
    }
}

/// Returns the given window's current background character/attribute pair.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::extend::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// #     if has_colors() {
/// start_color()?;
///
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let yellow = Color::Dark(BaseColor::Yellow);
/// let blue = Color::Dark(BaseColor::Blue);
///
/// let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
/// let mut attrs = Attributes::default();
/// attrs.set_dim(true);
///
/// match std::char::from_u32(0x2764) {
///     Some(c) => {
///         let background_char = ComplexChar::from_char(c, &attrs, &color_pair1)?;
///         wbkgrndset(win, background_char);
///
///         assert!(wgetbkgrnd(win)? == background_char);
///     },
///     None    => panic!("unable to convert to character!")
/// }
///
/// delwin(win)?;
/// #     }
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wgetbkgrnd(handle: WINDOW) -> result!(ComplexChar) {
    let mut wch: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::wgetbkgrnd(handle, wch.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wch[0])),
        rc => Err(ncurses_function_error_with_rc!("wgetbkgrnd", rc))
    }
}

pub fn wgetch(handle: WINDOW) -> result!(CharacterResult<char>) {
    match unsafe { ncurses::wgetch(handle) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("wgetch", rc))
            } else if rc >= KEY_MIN && rc <= KEY_MAX {
                Ok(CharacterResult::Key(KeyBinding::from(rc)))
            } else {
                Ok(CharacterResult::Character(char::from(rc as i8 as u8)))
            }
        }
    }
}

pub fn wgetdelay(handle: WINDOW) -> result!(time::Duration) {
    let delay = time::Duration::from_millis(u64::try_from(unsafe { ncurses::wgetdelay(handle) })?);

    Ok(delay)
}

pub fn wgetn_wstr(handle: WINDOW, number: i32) -> result!(WideString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::wgetn_wstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::wgetn_wstr(handle, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("wgetn_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::wgetn_wstr() : ptr.is_null()");

                let buf_as_bytes = unsafe { slice::from_raw_parts(ptr as *mut wchar_t, LINE_MAX) };

                for (idx, &byte) in buf_as_bytes.iter().enumerate() {
                    if byte == 0x00 {
                        return Ok(WideString::from(&buf_as_bytes[..idx]));
                    }
                }

                Ok(WideString::from(buf_as_bytes))
            }
        }
    }
}

pub fn wgetnstr(handle: WINDOW, number: i32) -> result!(String) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::wgetnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::wgetnstr(handle, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("wgetnstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::wgetnstr() : ptr.is_null()");

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

pub fn wgetparent(handle: WINDOW) -> Option<WINDOW> {
    unsafe {
        ncurses::wgetparent(handle)
    }
}

pub fn wgetscrreg(handle: WINDOW) -> result!(Region) {
    let mut top: [i32; 1] = [0];
    let mut bottom: [i32; 1] = [0];

    match unsafe { ncurses::wgetscrreg(handle, top.as_mut_ptr(), bottom.as_mut_ptr()) } {
        OK => Ok(Region { top: top[0], bottom: bottom[0] }),
        rc => Err(ncurses_function_error_with_rc!("wgetscrreg", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use wgetnstr() instead")]
pub fn wgetstr(handle: WINDOW) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::wgetstr(handle, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc < 0 {
                Err(ncurses_function_error_with_rc!("wgetstr", rc))
            } else {
                assert!(!ptr.is_null(), "ncursesw::wgetstr() : ptr.is_null()");

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

pub fn whline(handle: WINDOW, ch: ChtypeChar, number: i32) -> result!(()) {
    match unsafe { ncurses::whline(handle, ChtypeChar::into(ch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("whline", rc))
    }
}

pub fn whline_set(handle: WINDOW, wch: ComplexChar, number: i32) -> result!(()) {
    match unsafe { ncurses::whline_set(handle, &ComplexChar::into(wch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("whline_set", rc))
    }
}

pub fn win_wch(handle: WINDOW) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::win_wch(handle, wcval.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wcval[0])),
        rc => Err(ncurses_function_error_with_rc!("win_wch", rc))
    }
}

pub fn win_wchnstr(handle: WINDOW, number: i32) -> result!(ComplexString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::win_wchnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::win_wchnstr(handle, ptr, number) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::win_wchnstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, number as usize) }))
        },
        rc => Err(ncurses_function_error_with_rc!("win_wchnstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use win_wchnstr() instead")]
pub fn win_wchstr(handle: WINDOW) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::win_wchstr(handle, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::win_wchstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("win_wchstr", rc))
    }
}

pub fn winch(handle: WINDOW) -> ChtypeChar {
    ChtypeChar::from(unsafe { ncurses::winch(handle) })
}

pub fn winchnstr(handle: WINDOW, number: i32) -> result!(ChtypeString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::winchnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::winchnstr(handle, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("winchnstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::winchnstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::winchnstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use winchnstr() instead")]
pub fn winchstr(handle: WINDOW) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::winchstr(handle, ptr) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("inchstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::winchstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::winchstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

pub fn winnstr(handle: WINDOW, number: i32) -> result!(String) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::winnstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::winnstr(handle, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("winnstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::winnstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::winnstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

pub fn winnwstr(handle: WINDOW, number: i32) -> result!(WideString) {
    assert!(number > 0 && number <= LINE_MAX as i32, "ncursesw::winnwstr() : number={}, LINE_MAX={}", number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    let len = unsafe { ncurses::winnwstr(handle, ptr, number) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("winnwstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::winnwstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::winnwstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
    }
}

/// Insert a wide character string (unicode UTF-8) of a given length on the given window.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // insert "Testing..Testing.." on the window `win`
/// wins_nwstr(win, &wide_str, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wins_nwstr(handle: WINDOW, wstr: &WideString, number: i32) -> result!(()) {
    match unsafe { ncurses::wins_nwstr(handle, raw_with_nul_as_slice!(wstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wins_nwstr", rc))
    }
}

/// Insert a complex character on the given window.
///
/// Insert the complex character with rendition before the character under the cursor.
/// All characters to the right of the cursor are moved one space to the right, with
/// the possibility of the rightmost character on the line being lost. The insertion
/// operation does not change the cursor position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;
///
/// wins_wch(win, complex_char)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wins_wch(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::wins_wch(handle, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wins_wch", rc))
    }
}

/// Insert a wide character string (unicode UTF-8) on the given window.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let wide_str = WideString::from_str("Testing..Testing..1..2..3..");
///
/// // insert "Testing..Testing..1..2..3.." on the window `win`
/// wins_wstr(win, &wide_str)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn wins_wstr(handle: WINDOW, wstr: &WideString) -> result!(()) {
    match unsafe { ncurses::wins_wstr(handle, raw_with_nul_as_slice!(wstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wins_wstr", rc))
    }
}

/// Insert a ascii character and `normal` attribute/color pair combination on the given window.
///
/// Insert the character with rendition before the character under the cursor.
/// All characters to the right of the cursor are moved one space to the right, with
/// the possibility of the rightmost character on the line being lost. The insertion
/// operation does not change the cursor position.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
/// extern crate ascii;
///
/// use ascii::*;
/// # use std::error::Error;
/// use ncursesw::*;
/// use ncursesw::normal::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let color_pair0 = ColorPair::default();
/// let attrs = Attributes::default();
///
/// let ascii_char = AsciiChar::A;
/// let chtype_char = ChtypeChar::new(ascii_char) | attrs;
///
/// winsch(win, chtype_char)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn winsch(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::winsch(handle, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsch", rc))
    }
}

pub fn winsdelln(handle: WINDOW, n: i32) -> result!(()) {
    match unsafe { ncurses::winsdelln(handle, n) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsdelln", rc))
    }
}

basic_ncurses_function_with_window!(winsertln, "winsertln");

/// Insert a string of a given length on the given window.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // insert "Testing..Testing.."
/// winsnstr(win, &s, 18)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn winsnstr(handle: WINDOW, str: &str, number: i32) -> result!(()) {
    match unsafe { ncurses::winsnstr(handle, c_str_with_nul!(str), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsnstr", rc))
    }
}

/// Insert a string on the given window.
///
/// All characters to the right of the cursor are shifted right, with the possibility
/// of the rightmost characters on the line being lost. No wrapping is performed.
///
/// ## Example
/// ```rust
/// extern crate ncursesw;
///
/// # use std::error::Error;
/// use ncursesw::*;
///
/// # fn main() -> Result<(), Box<Error>> {
/// #     let h = initscr()?;
/// let win_size = Size { lines: 10, columns: 50 };
/// let win_origin = Origin { y: 5, x: 5 };
///
/// let win = newwin(win_size, win_origin)?;
///
/// let s = "Testing..Testing..1..2..3..";
///
/// // insert "Testing..Testing..1..2..3.."
/// winsstr(win, &s)?;
///
/// delwin(win)?;
/// #
/// #     delwin(h)?;
/// #     // endwin()?;
/// #     Ok(())
/// # }
/// ```
pub fn winsstr(handle: WINDOW, str: &str) -> result!(()) {
    match unsafe { ncurses::winsstr(handle, c_str_with_nul!(str)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use winnstr() instead")]
pub fn winstr(handle: WINDOW) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::winstr(handle, ptr) };

    if len < 0 {
        Err(ncurses_function_error_with_rc!("winstr", len))
    } else {
        assert!(!ptr.is_null(), "ncursesw::winstr() : ptr.is_null()");
        assert!(len > 0 && len <= LINE_MAX as i32, "ncursesw::winstr() : len={}, LINE_MAX={}", len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use winnwstr() instead")]
pub fn winwstr(handle: WINDOW) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::winwstr(handle, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "ncursesw::winwstr() : ptr.is_null()");

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("winwstr", rc))
    }
}

pub fn wmove(handle: WINDOW, origin: Origin) -> result!(()) {
    match unsafe { ncurses::wmove(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wmove", rc))
    }
}

basic_ncurses_function_with_window!(wnoutrefresh, "wnoutrefresh");

pub fn wredrawln(handle: WINDOW, beg_line: i32, num_lines: i32) -> result!(()) {
    match unsafe { ncurses::wredrawln(handle, beg_line, num_lines) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wredrawln", rc))
    }
}

basic_ncurses_function_with_window!(wrefresh, "wrefresh");

pub fn wresize(handle: WINDOW, size: Size) -> result!(()) {
    match unsafe { ncurses::wresize(handle, size.lines, size.columns) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wresize", rc))
    }
}

pub fn wscrl(handle: WINDOW, n: i32) -> result!(()) {
    match unsafe { ncurses::wscrl(handle, n) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wscrl", rc))
    }
}

pub fn wsetscrreg(handle: WINDOW, region: Region) -> result!(()) {
    match unsafe { ncurses::wsetscrreg(handle, region.top, region.bottom) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wsetscrreg", rc))
    }
}

basic_ncurses_function_with_window!(wstandend, "wstandend");

basic_ncurses_function_with_window!(wstandout, "wstandout");

pub fn wsyncdown(handle: WINDOW) {
    unsafe {
        ncurses::wsyncdown(handle)
    }
}

pub fn wsyncup(handle: WINDOW) {
    unsafe {
        ncurses::wsyncup(handle)
    }
}

pub fn wtimeout(handle: WINDOW, ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    unsafe {
        ncurses::wtimeout(handle, ms);
    }

    Ok(())
}

pub fn wtouchln(handle: WINDOW, line: i32, n: i32, changed: Changed) -> result!(()) {
    match unsafe { ncurses::wtouchln(handle, line, n, match changed {
        Changed::True  => 1,
        Changed::False => 0
    }) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wtouchln", rc))
    }
}

pub fn wunctrl(ch: ComplexChar) -> result!(WideChar) {
    let mut wch: [cchar_t; 1] = [ComplexChar::into(ch)];

    let ptr = unsafe { ncurses::wunctrl(wch.as_mut_ptr()) };

    if ptr.is_null() {
        Err(ncurses_function_error!("wunctrl"))
    } else {
        let wc = WideChar::from(unsafe { slice::from_raw_parts(ptr, 1)[0] as wchar_t });

        Ok(wc)
    }
}

pub fn wvline(handle: WINDOW, ch: ChtypeChar, number: i32) -> result!(()) {
    match unsafe { ncurses::wvline(handle, ChtypeChar::into(ch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wvline", rc))
    }
}

pub fn wvline_set(handle: WINDOW, wch: ComplexChar, number: i32) -> result!(()) {
    match unsafe { ncurses::wvline_set(handle, &ComplexChar::into(wch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wvline_set", rc))
    }
}
