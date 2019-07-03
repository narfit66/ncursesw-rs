/*
    src/lib.rs

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
#![warn(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::invalid_ref)]

extern crate libc;
extern crate custom_error;
extern crate ascii;
extern crate semver;

use std::{path, char, ptr, time, mem};
use std::convert::{From, TryFrom};
use std::slice;

mod macros;

pub mod extend;
pub mod gen;
pub mod normal;
pub mod panels;
pub mod shims;

mod chtypet;
mod complex;
mod wide;

mod attributescolorpairset;
mod characterresult;
mod cstring;
mod changed;
mod cursortype;
mod justification;
mod keybinding;
mod lccategory;
mod legacy;
mod ncurseswerror;
mod orientation;
mod origin;
mod region;
mod size;
mod softlabeltype;

pub use attributescolorpairset::*;
pub use characterresult::*;
use cstring::*;
pub use changed::*;
pub use cursortype::*;
pub use gen::*;
pub use justification::*;
pub use keybinding::*;
pub use lccategory::*;
pub use legacy::*;
pub use ncurseswerror::*;
pub use origin::*;
pub use orientation::*;
pub use region::*;
use semver::Version;
use shims::*;
pub use size::*;
pub use softlabeltype::*;

pub use chtypet::*;
pub use complex::*;
pub use wide::*;

use constants::{ERR, KEY_CODE_YES, KEY_MOUSE, KEY_RESIZE, KEY_EVENT, TRUE, FALSE};

const LINE_MAX: usize = 4096;

/// A ncurses window handle
pub type WINDOW = ncurses::WINDOW;
/// A ncurses screen handle
pub type SCREEN = ncurses::SCREEN;
/// Ripoff line callback function signature
pub type RipoffInit = shims::bindings::RipoffInit;

type attr_t = ncurses::attr_t;
type cchar_t = ncurses::cchar_t;
type chtype = ncurses::chtype;
type short_t = ncurses::short_t;
type wchar_t = ncurses::wchar_t;
type wint_t = ncurses::wint_t;

/// The current screen ncurses handle
pub fn curscr() -> WINDOW {
    ncurses::curscr()
}

/// The new screen ncurses handle
pub fn newscr() -> WINDOW {
    ncurses::newscr()
}

/// The standard screen ncurses handle
pub fn stdscr() -> WINDOW {
    ncurses::stdscr()
}

/// The Number of colors available
///
/// # Example
///
/// ```
/// let number_of_colors = COLORS();
///
/// assert!(number_of_colors > 0);
/// ```
pub fn COLORS() -> i32 {
    ncurses::COLORS()
}

/// Extracts the raw color pair attribute value of a color pair
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
///
/// assert!(COLOR_PAIR(color_pair1) > 0);
/// ```
pub fn COLOR_PAIR(color_pair: normal::ColorPair) -> attr_t {
    ncurses::COLOR_PAIR(normal::ColorPair::into(color_pair)) as attr_t
}

/// Extract the color pair from normal attributes.
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// assert!(PAIR_NUMBER(attributes) == color_pair1);
/// ```
pub fn PAIR_NUMBER(attrs: normal::Attributes) -> normal::ColorPair {
    normal::ColorPair::from(ncurses::PAIR_NUMBER(normal::Attributes::into(attrs)))
}

/// The number of color pairs available
///
/// # Example
///
/// ```
/// let number_of_color_pairs = COLOR_PAIRS();
///
/// assert!(number_of_color_pairs > 0);
/// ```
pub fn COLOR_PAIRS() -> i32 {
    ncurses::COLOR_PAIRS()
}

/// The number of columns (x-axis) available on the terminal
///
/// # Example
///
/// ```
/// let number_of_columns = COLS();
///
/// assert!(number_of_columns > 0);
/// ```
pub fn COLS() -> i32 {
    ncurses::COLS()
}

pub fn ESCDELAY() -> result!(time::Duration) {
    let delay = time::Duration::from_millis(u64::try_from(ncurses::ESCDELAY())?);

    Ok(delay)
}

/// The number of lines (y-axis) available on the terminal
///
/// # Example
///
/// ```
/// let number_of_lines = LINES();
///
/// assert!(number_of_lines > 0);
/// ```
pub fn LINES() -> i32 {
    ncurses::LINES()
}

/// The number of columns (x-axis) that a tab represents
///
/// # Example
///
/// ```
/// let tabsize = TABSIZE();
///
/// assert!(tabsize > 0);
/// ```
pub fn TABSIZE() -> i32 {
    ncurses::TABSIZE()
}

/// Add a complex character to the current position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let wch = ComplexChar::from_char('A', color_pair1, attributes)?;
///
/// add_wch(wch)?;
/// ```
pub fn add_wch(wch: ComplexChar) -> result!(()) {
    match ncurses::add_wch(&ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("add_wch")),
        _   => Ok(())
    }
}

/// Add a complex string of a specified length to the current position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let wchstr = ComplexString::from_str("testing, testing... 1..2..3..", color_pair1, attributes)?;
///
/// add_wchnstr(&wchstr, 29)?;
/// ```
pub fn add_wchnstr(wchstr: &ComplexString, number: i32) -> result!(()) {
    match ncurses::add_wchnstr(raw_with_nul_as_slice!(wchstr), number) {
        ERR => Err(ncurses_function_error!("add_wchnstr")),
        _   => Ok(())
    }
}

/// Add a complex string to the current position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let wchstr = ComplexString::from_str("testing, testing 1..2..3..", color_pair1, attributes)?;
///
/// add_wchstr(&wchstr)?;
/// ```
pub fn add_wchstr(wchstr: &ComplexString) -> result!(()) {
    match ncurses::add_wchstr(raw_with_nul_as_slice!(wchstr)) {
        ERR => Err(ncurses_function_error!("add_wchstr")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) character to the current position on the standard screen
///
/// # Example
///
/// ```
/// let ch = ChtypeChar::new(AsciiChar::Asterisk);
///
/// addch(ch)?;
/// ```
pub fn addch(ch: ChtypeChar) -> result!(()) {
    match ncurses::addch(ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("addch")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) type string of a specified length to the current position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let asciistr = AsciiString::from_ascii_str("testing, testing... 1..2..3..")?;
/// let mut chstr = ChtypeString::from_ascii_str(ascii_str);
/// chstr = chstr | attributes;
///
/// addchnstr(&chstr, 29)?;
/// ```
pub fn addchnstr(chstr: &ChtypeString, number: i32) -> result!(()) {
    match ncurses::addchnstr(raw_with_nul_as_slice!(chstr), number) {
        ERR => Err(ncurses_function_error!("addchnstr")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) type string to the current position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let asciistr = AsciiString::from_ascii_str("testing, testing... 1..2..3..")?;
/// let mut chstr = ChtypeString::from_ascii_str(ascii_str);
/// chstr = chstr | attributes;
///
/// addchstr(&chstr)?;
/// ```
pub fn addchstr(chstr: &ChtypeString) -> result!(()) {
    match ncurses::addchstr(raw_with_nul_as_slice!(chstr)) {
        ERR => Err(ncurses_function_error!("addchstr")),
        _   => Ok(())
    }
}

/// Add a ascii string (as of ABI 6 if not before unicode strings are also supported) of a specified length to the current position on the standard screen
///
/// # Example
///
/// ```
/// let string = "testing, testing... 1..2..3..";
///
/// addnstr(&string, 29)?;
/// ```
pub fn addnstr(str: &str, number: i32) -> result!(()) {
    match ncurses::addnstr(c_str_with_nul!(str), number) {
        ERR => Err(ncurses_function_error!("addnstr")),
        _   => Ok(())
    }
}

/// Add a wide character string (unicode) of a specified length to the current position on the standard screen
///
/// # Example
///
/// ```
/// let wide_string = WideString::from_str("testing, testing... 1..2..3..");
///
/// addnwstr(&string, 29)?;
/// ```
pub fn addnwstr(wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::addnwstr(raw_with_nul_as_slice!(wstr), number) {
        ERR => Err(ncurses_function_error!("addnwstr")),
        _   => Ok(())
    }
}

/// Add a ascii string (as of ABI 6 if not before unicode strings are also supported) to the current position on the standard screen
///
/// # Example
///
/// ```
/// let string = "testing, testing... 1..2..3..";
///
/// addstr(&string)?;
/// ```
pub fn addstr(str: &str) -> result!(()) {
    match ncurses::addstr(c_str_with_nul!(str)) {
        ERR => Err(ncurses_function_error!("addstr")),
        _   => Ok(())
    }
}

/// Add a wide character string (unicode) to the current position on the standard screen
///
/// # Example
///
/// ```
/// let wide_string = WideString::from_str("testing, testing... 1..2..3..");
///
/// addwstr(&string)?;
/// ```
pub fn addwstr(wstr: &WideString) -> result!(()) {
    match ncurses::addwstr(raw_with_nul_as_slice!(wstr)) {
        ERR => Err(ncurses_function_error!("addwstr")),
        _   => Ok(())
    }
}

/// Allocate an extended color pair using the passed colors (foreground and background)
///
/// # Example
///
/// ```
/// let colors = Colors::new(Color::Red, Color::Blue);
/// let color_pair = alloc_pair(colors)?;
///
/// assert!(color_pair.foreground() == Color::Red);
/// assert!(color_pair.background() == Color::Blue);
/// ```
pub fn alloc_pair(colors: extend::Colors) -> result!(extend::ColorPair) {
    match ncurses::alloc_pair(colors.foreground().into(), colors.background().into()) {
        ERR  => Err(ncurses_function_error!("alloc_pair")),
        pair => Ok(extend::ColorPair::from(pair))
    }
}

pub fn assume_default_colors<S, C, T>(colors: S) -> result!(()) where S: ColorsType<C, T>, C: ColorType<T>, T: ColorAttributeTypes {
    match ncurses::assume_default_colors(colors.foreground().number(), colors.background().number()) {
        ERR => Err(ncurses_function_error!("assume_default_colors")),
        _   => Ok(())
    }
}

pub fn attr_get() -> result!(AttributesColorPairSet) {
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let opts: *mut libc::c_void = ptr::null_mut();

    match unsafe { ncurses::attr_get(attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts) } {
        ERR => Err(ncurses_function_error!("attr_get")),
        _   => {
            Ok(if opts.is_null() {
                   AttributesColorPairSet::Normal(
                       normal::AttributesColorPair::new(
                           normal::Attributes::from(attrs[0]),
                           normal::ColorPair::from(color_pair[0])
                       )
                   )
               } else {
                   AttributesColorPairSet::Extend(
                       extend::AttributesColorPair::new(
                           extend::Attributes::from(attrs[0]),
                           extend::ColorPair::from(unsafe { slice::from_raw_parts(opts as *mut i32, 1)[0] })
                       )
                   )
               })
        }
    }
}

pub fn attr_off<A, T>(attrs: A) -> result!(()) where A: AttributesType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::attr_off(attrs.as_attr_t(), ptr::null_mut()) } {
        ERR => Err(ncurses_function_error!("attr_off")),
        _   => Ok(())
    }
}

pub fn attr_on<A, T>(attrs: A) -> result!(()) where A: AttributesType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::attr_on(attrs.as_attr_t(), ptr::null_mut()) } {
        ERR => Err(ncurses_function_error!("attr_on")),
        _   => Ok(())
    }
}

pub fn attr_set<A, P, T>(attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::attr_set(attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("attr_set")),
        _   => Ok(())
    }
}

pub fn attroff(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attroff(normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("attroff")),
        _   => Ok(())
    }
}

pub fn attron(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attron(normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("attron")),
        _   => Ok(())
    }
}

pub fn attrset(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attrset(normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("attrset")),
        _   => Ok(())
    }
}

pub fn baudrate() -> i32 {
    ncurses::baudrate()
}

basic_ncurses_function!(beep, "beep");

pub fn bkgd(ch: ChtypeChar) -> result!(()) {
    match ncurses::bkgd(ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("bkgd")),
        _   => Ok(())
    }
}

pub fn bkgdset(ch: ChtypeChar) {
    ncurses::bkgdset(ChtypeChar::into(ch))
}

pub fn bkgrnd(wch: ComplexChar) -> result!(()) {
    match ncurses::bkgrnd(&ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("bkgrnd")),
        _   => Ok(())
    }
}

pub fn bkgrndset(wch: ComplexChar) {
    ncurses::bkgrndset(&ComplexChar::into(wch))
}

pub fn border(ls: ChtypeChar, rs: ChtypeChar, ts: ChtypeChar, bs: ChtypeChar, tl: ChtypeChar, tr: ChtypeChar, bl: ChtypeChar, br: ChtypeChar) -> result!(()) {
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
        ERR => Err(ncurses_function_error!("border")),
        _   => Ok(())
    }
}

pub fn border_set(ls: ComplexChar, rs: ComplexChar, ts: ComplexChar, bs: ComplexChar, tl: ComplexChar, tr: ComplexChar, bl: ComplexChar, br: ComplexChar) -> result!(()) {
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
        ERR => Err(ncurses_function_error!("border_set")),
        _   => Ok(())
    }
}

pub fn r#box(handle: WINDOW, verch: ChtypeChar, horch: ChtypeChar) -> result!(()) {
    match ncurses::r#box(handle, ChtypeChar::into(verch), ChtypeChar::into(horch)) {
        ERR => Err(ncurses_function_error!("box")),
        _   => Ok(())
    }
}

pub fn box_set(handle: WINDOW, verch: ComplexChar, horch: ComplexChar) -> result!(()) {
    match ncurses::box_set(handle, &ComplexChar::into(verch), &ComplexChar::into(horch)) {
        ERR => Err(ncurses_function_error!("box_set")),
        _   => Ok(())
    }
}

pub fn can_change_color() -> bool {
    ncurses::can_change_color()
}

basic_ncurses_function!(cbreak, "cbreak");

pub fn chgat<A, P, T>(number: i32, attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::chgat(number, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_const_ptr()) } {
        ERR => Err(ncurses_function_error!("chgat")),
        _   => Ok(())
    }
}

basic_ncurses_function!(clear, "clear");

pub fn clearok(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::clearok(handle, bf) {
        ERR => Err(ncurses_function_error!("clearok")),
        _   => Ok(())
    }
}

basic_ncurses_function!(clrtobot, "clrtobot");

basic_ncurses_function!(clrtoeol, "clrtoeol");

pub fn color_content(color: normal::Color) -> result!(normal::RGB) {
    let mut r: [short_t; 1] = [0];
    let mut g: [short_t; 1] = [0];
    let mut b: [short_t; 1] = [0];

    match unsafe { ncurses::color_content(normal::Color::into(color), r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("color_content")),
        _   => Ok(normal::RGB::new(r[0], g[0], b[0]))
    }
}

pub fn color_set<P, T>(color_pair: P) -> result!(()) where P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::color_set(color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("color_set")),
        _   => Ok(())
    }
}

pub fn copywin(src_handle: WINDOW, dst_handle: WINDOW, smin: Origin, dmin: Origin, dmax: Origin, overlay: bool) -> result!(()) {
    match ncurses::copywin(src_handle, dst_handle, smin.y, smin.x, dmin.y, dmin.x, dmax.y, dmax.x, if overlay {
        TRUE
    } else {
        FALSE
    }) {
        ERR => Err(ncurses_function_error!("copywin")),
        _   => Ok(())
    }
}

pub fn curs_set(cursor: CursorType) -> result!(CursorType) {
    match ncurses::curs_set(CursorType::into(cursor)) {
        ERR => Err(ncurses_function_error!("curs_set")),
        rc  => {
            let cur = CursorType::try_from(rc)?;

            Ok(cur)
        }
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
        ERR => Err(ncurses_function_error!("define_key")),
        _   => Ok(())
    }
}

pub fn delay_output(ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    match ncurses::delay_output(ms) {
        ERR => Err(ncurses_function_error!("delay_output")),
        _   => Ok(())
    }
}

basic_ncurses_function!(delch, "delch");

basic_ncurses_function!(deleteln, "deleteln");

pub fn delscreen(sp: SCREEN) {
    ncurses::delscreen(sp)
}

basic_ncurses_function_with_window!(delwin, "delwin");

pub fn derwin(orig: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    match ncurses::derwin(orig, size.lines, size.columns, origin.y, origin.x) {
        None      => Err(ncurses_function_error!("derwin")),
        Some(win) => Ok(win)
    }
}

basic_ncurses_function!(doupdate, "doupdate");

pub fn dupwin(handle: WINDOW) -> result!(WINDOW) {
    match ncurses::dupwin(handle) {
        None         => Err(ncurses_function_error!("dupwin")),
        Some(handle) => Ok(handle)
    }
}

basic_ncurses_function!(echo, "echo");

pub fn echo_wchar(wch: ComplexChar) -> result!(()) {
    match ncurses::echo_wchar(&ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("echo_wchar")),
        _   => Ok(())
    }
}

pub fn echochar(ch: ChtypeChar) -> result!(()) {
    match ncurses::echochar(ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("echochar")),
        _   => Ok(())
    }
}

basic_ncurses_function!(endwin, "endwin");

basic_ncurses_function!(erase, "erase");

pub fn erasechar() -> result!(char) {
    let rc = ncurses::erasechar();

    if rc == ERR as i8 {
        Err(ncurses_function_error!("erasechar"))
    } else {
        Ok(char::from(rc as u8))
    }
}

pub fn erasewchar() -> result!(WideChar) {
    let mut wch: [wchar_t; 1] = [0];

    match unsafe { ncurses::erasewchar(wch.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("erasewchar")),
        _   => Ok(WideChar::from(wch[0]))
    }
}

pub fn extended_color_content(color: extend::Color) -> result!(extend::RGB) {
    let mut r: [i32; 1] = [0];
    let mut g: [i32; 1] = [0];
    let mut b: [i32; 1] = [0];

    match unsafe { ncurses::extended_color_content(extend::Color::into(color), r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("extended_color_content")),
        _   => Ok(extend::RGB::new(r[0], g[0], b[0]))
    }
}

pub fn extended_pair_content(pair: extend::ColorPair) -> result!(extend::Colors) {
    let mut fg: [i32; 1] = [0];
    let mut bg: [i32; 1] = [0];

    match unsafe { ncurses::extended_pair_content(extend::ColorPair::into(pair), fg.as_mut_ptr(), bg.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("extended_pair_content")),
        _   => Ok(extend::Colors::new(extend::Color::from(fg[0]), extend::Color::from(bg[0])))
    }
}

pub fn extended_slk_color(color_pair: extend::ColorPair) -> result!(()) {
    match ncurses::extended_slk_color(color_pair.number()) {
        ERR => Err(ncurses_function_error!("extended_slk_color")),
        _   => Ok(())
    }
}

simple_ncurses_function!(filter);

/// Find a previously allocated extended color pair using the passed colors (foreground and background)
///
/// # Example
///
/// ```
/// let colors = Colors::new(Color::Red, Color::Blue);
///
/// match find_pair(colors) {
///     None             => println!("no color pair found for colors {}", colors),
///     Some(color_pair) => {
///         assert!(color_pair.foreground() == Color::Red);
///         assert!(color_pair.background() == Color::Blue);
///     }
/// }
/// ```
pub fn find_pair(colors: extend::Colors) -> Option<extend::ColorPair> {
    match ncurses::find_pair(colors.foreground().into(), colors.background().into()) {
        ERR  => None,
        pair => Some(extend::ColorPair::from(pair))
    }
}

basic_ncurses_function!(flash, "flash");

basic_ncurses_function!(flushinp, "flushinp");

/// Free a previously allocated extended color pair
///
/// Warning : This will de-allocate the color pair internally within the ncurses library
///           but the user must make sure there color_pair goes out of scope otherwise
///           unpredicable results will occur.
///
/// # Example
/// ```
/// let color_pair = ColorPair::new(Colors::new(Color::Red, Color::Yellow))?;
/// free_pair(color_pair)?;
///
/// // warning: even tho color_pair is still in scope it now represents an un-initialised color_pair
/// ```
pub fn free_pair(color_pair: extend::ColorPair) -> result!(()) {
    match ncurses::free_pair(extend::ColorPair::into(color_pair)) {
        ERR => Err(ncurses_function_error!("free_pair")),
        _   => Ok(())
    }
}

pub fn get_escdelay() -> result!(time::Duration) {
    let delay = time::Duration::from_millis(u64::try_from(ncurses::get_escdelay())?);

    Ok(delay)
}

pub fn get_wch() -> result!(WideCharResult) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::get_wch(wch.as_mut_ptr()) } {
        ERR          => Err(ncurses_function_error!("get_wch")),
        KEY_CODE_YES => {
            if wch[0] == KEY_MOUSE as wint_t {
                Err(NCurseswError::KeyMouse)
            } else if wch[0] == KEY_RESIZE as wint_t {
                Err(NCurseswError::KeyReSize)
            } else if wch[0] == KEY_EVENT as wint_t {
                Err(NCurseswError::KeyEvent)
            } else {
                Ok(WideCharResult::Key(KeyBinding::from(wch[0])))
            }
        },
        _            => Ok(WideCharResult::Character(WideChar::from(wch[0])))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use getn_wstr() instead")]
pub fn get_wstr() -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::get_wstr(ptr) } {
        ERR        => Err(ncurses_function_error!("get_wstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
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

pub fn getattrs(handle: WINDOW) -> normal::Attributes {
    normal::Attributes::from(ncurses::getattrs(handle) as attr_t)
}

pub fn getbegx(handle: WINDOW) -> result!(i32) {
    match ncurses::getbegx(handle) {
        ERR => Err(ncurses_function_error!("getbegx")),
        x   => Ok(x)
    }
}

pub fn getbegy(handle: WINDOW) -> result!(i32) {
    match ncurses::getbegy(handle) {
        ERR => Err(ncurses_function_error!("getbegy")),
        y   => Ok(y)
    }
}

pub fn getbegyx(handle: WINDOW) -> result!(Origin) {
    let y = ncurses::getbegy(handle);
    let x = ncurses::getbegx(handle);

    if y == ERR || x == ERR {
        Err(ncurses_function_error!("getbegyx"))
    } else {
        Ok(Origin { y, x })
    }
}

pub fn getbkgd(handle: WINDOW) -> ChtypeChar {
    ChtypeChar::from(ncurses::getbkgd(handle))
}

pub fn getbkgrnd() -> result!(ComplexChar) {
    let mut wch: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::getbkgrnd(wch.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("getbkgd")),
        _   => Ok(ComplexChar::from(wch[0]))
    }
}

pub fn getcchar(wcval: ComplexChar) -> result!(WideCharAndAttributes) {
    let wc: [cchar_t; 1] = [ComplexChar::into(wcval)];
    let mut wch: [wchar_t; 1] = [0];
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let opts: *mut libc::c_void = ptr::null_mut();

    match unsafe { ncurses::getcchar(&wc, wch.as_mut_ptr(), attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts) } {
        ERR => Err(ncurses_function_error!("getcchar")),
        _   => {
            Ok(WideCharAndAttributes::new(
                   WideChar::from(wch[0]),
                   if opts.is_null() {
                       AttributesColorPairSet::Normal(
                           normal::AttributesColorPair::new(
                               normal::Attributes::from(attrs[0]),
                               normal::ColorPair::from(color_pair[0])
                           )
                       )
                   } else {
                       AttributesColorPairSet::Extend(
                           extend::AttributesColorPair::new(
                               extend::Attributes::from(attrs[0]),
                               extend::ColorPair::from(unsafe { slice::from_raw_parts(opts as *mut i32, 1)[0] })
                           )
                       )
                   }
               )
            )
        }
    }
}

pub fn getch() -> result!(CharacterResult) {
    match ncurses::getch() {
        ERR        => Err(ncurses_function_error!("getch")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        ch         => {
            if ch > i32::from(i8::max_value()) {
                Ok(CharacterResult::Key(KeyBinding::from(ch)))
            } else {
                Ok(CharacterResult::Character(char::from(ch as i8 as u8)))
            }
        }
    }
}

pub fn getcurx(handle: WINDOW) -> result!(i32) {
    match ncurses::getcurx(handle) {
        ERR => Err(ncurses_function_error!("getcurx")),
        x   => Ok(x)
    }
}

pub fn getcury(handle: WINDOW) -> result!(i32) {
    match ncurses::getcury(handle) {
        ERR => Err(ncurses_function_error!("getcury")),
        y   => Ok(y)
    }
}

pub fn getcuryx(handle: WINDOW) -> result!(Origin) {
    let y = ncurses::getcury(handle);
    let x = ncurses::getcurx(handle);

    if y == ERR || x == ERR {
        Err(ncurses_function_error!("getcuryx"))
    } else {
        Ok(Origin { y, x })
    }
}

pub fn getmaxx(handle: WINDOW) -> result!(i32) {
    match ncurses::getmaxx(handle) {
        ERR => Err(ncurses_function_error!("getmaxx")),
        x   => Ok(x)
    }
}

pub fn getmaxy(handle: WINDOW) -> result!(i32) {
    match ncurses::getmaxy(handle) {
        ERR => Err(ncurses_function_error!("getmaxy")),
        y   => Ok(y)
    }
}

pub fn getmaxyx(handle: WINDOW) -> result!(Size) {
    let lines = ncurses::getmaxy(handle);
    let columns = ncurses::getmaxx(handle);

    if lines == ERR || columns == ERR {
        Err(ncurses_function_error!("getmaxyx"))
    } else {
        Ok(Size { lines, columns })
    }
}

pub fn getn_wstr(number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::getn_wstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::getn_wstr(ptr, number) } {
        ERR        => Err(ncurses_function_error!("getn_wstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
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

pub fn getnstr(number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "ncursesw::getnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::getnstr(ptr, number) } {
        ERR        => Err(ncurses_function_error!("getnstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
            assert!(!ptr.is_null(), "ncursesw::getnstr() : ptr.is_null()");

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn getparx(handle: WINDOW) -> result!(i32) {
    match ncurses::getparx(handle) {
        ERR => Err(ncurses_function_error!("getparx")),
        x   => Ok(x)
    }
}

pub fn getpary(handle: WINDOW) -> result!(i32) {
    match ncurses::getpary(handle) {
        ERR => Err(ncurses_function_error!("getpary")),
        y   => Ok(y)
    }
}

pub fn getparyx(handle: WINDOW) -> result!(Origin) {
    let y = ncurses::getpary(handle);
    let x = ncurses::getparx(handle);

    if y == ERR || x == ERR {
        Err(ncurses_function_error!("getparyx"))
    } else {
        Ok(Origin { y, x })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use getnstr() instead")]
pub fn getstr() -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::getstr(ptr) } {
        ERR        => Err(ncurses_function_error!("getstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
            assert!(!ptr.is_null(), "ncursesw::getstr() : ptr.is_null()");

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn getwin(path: &path::Path) -> result!(WINDOW) {
    let mode = "r";

    match utils::fopen(path, mode) {
        None     => Err(NCurseswError::FOpen { fname: path.display().to_string(), mode: mode.to_string() }),
        Some(fp) => match ncurses::getwin(fp) {
            None      => Err(ncurses_function_error!("getwin")),
            Some(win) => Ok(win)
        }
    }
}

pub fn halfdelay(tenths: time::Duration) -> result!(()) {
    let delay = i32::try_from(tenths.as_secs())? / 10;

    match ncurses::halfdelay(delay) {
        ERR => Err(ncurses_function_error!("delay_output")),
        _   => Ok(())
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
        ERR => Err(ncurses_function_error!("hline")),
        _   => Ok(())
    }
}

pub fn hline_set(wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::hline_set(&ComplexChar::into(wch), number) {
        ERR => Err(ncurses_function_error!("hline_set")),
        _   => Ok(())
    }
}

pub fn idcok(handle: WINDOW, bf: bool) {
    ncurses::idcok(handle, bf)
}

pub fn idlok(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::idlok(handle, bf) {
        ERR => Err(ncurses_function_error!("idlok")),
        _   => Ok(())
    }
}

pub fn immedok(handle: WINDOW, bf: bool) {
    ncurses::immedok(handle, bf)
}

/// Return a Complex character from the current cursor position from the standard screen
///
/// # Example
///
/// ```
/// let cchar = in_wch()?;
/// ```
pub fn in_wch() -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::in_wch(wcval.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("in_wch")),
        _   => Ok(ComplexChar::from(wcval[0]))
    }
}

/// Return a complex string of characters starting at the current cursor position for a specified number of characters from the standard screen
///
/// # Example
///
/// ```
/// let chstring = in_wchnstr(10)?;
/// ```
pub fn in_wchnstr(number: i32) -> result!(ComplexString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::in_wchnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::in_wchnstr(ptr, number) } {
        ERR => Err(ncurses_function_error!("in_wchnstr")),
        _   => {
            assert!(!ptr.is_null(), "ncursesw::in_wchnstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, number as usize) }))
        }
    }
}

/// Return a Complex string of characters starting at the current cursor position from the standard screen
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let chstring = in_wchstr()?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use in_wchnstr() instead")]
pub fn in_wchstr() -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::in_wchstr(ptr) } {
        ERR => Err(ncurses_function_error!("in_wchstr")),
        _   => {
            assert!(!ptr.is_null(), "ncursesw::in_wchstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        }
    }
}

/// Return a Chtype character from the current cursor position from the standard screen
///
/// # Example
///
/// ```
/// let chchar = inch()?;
/// ```
pub fn inch() -> ChtypeChar {
    ChtypeChar::from(ncurses::inch())
}

/// Return a Chtype string of characters starting at the current cursor position for a specified number of characters from the standard screen
///
/// # Example
///
/// ```
/// let chstring = inchnstr(10)?;
/// ```
pub fn inchnstr(number: i32) -> result!(ChtypeString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::inchnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    match unsafe { ncurses::inchnstr(ptr, number) } {
        ERR => Err(ncurses_function_error!("inchnstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::inchnstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::inchnstr() : len={} > {}", len, LINE_MAX);

            Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

/// Return a Chtype string of characters starting at the current cursor position from the standard screen
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let chstring = inchstr()?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use inchnstr() instead")]
pub fn inchstr() -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    match unsafe { ncurses::inchstr(ptr) } {
        ERR => Err(ncurses_function_error!("inchstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::inchstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::inchstr() : len={} > {}", len, LINE_MAX);

            Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

pub fn init_color(color: short_t, rgb: normal::RGB) -> result!(normal::Color) {
    match ncurses::init_color(color, rgb.red(), rgb.green(), rgb.blue()) {
        ERR => Err(ncurses_function_error!("init_color")),
        _   => Ok(normal::Color::from(color))
    }
}

pub fn init_extended_color(color: i32, rgb: extend::RGB) -> result!(extend::Color) {
    match ncurses::init_extended_color(color, rgb.red(), rgb.green(), rgb.blue()) {
        ERR => Err(ncurses_function_error!("init_extended_color")),
        _   => Ok(extend::Color::from(color))
    }
}

pub fn init_extended_pair(pair: i32, colors: extend::Colors) -> result!(extend::ColorPair) {
    match ncurses::init_extended_pair(pair, extend::Color::into(colors.foreground()), extend::Color::into(colors.background())) {
        ERR => Err(ncurses_function_error!("init_extended_pair")),
        _   => Ok(extend::ColorPair::from(pair))
    }
}

pub fn init_pair(pair: short_t, colors: normal::Colors) -> result!(normal::ColorPair) {
    match ncurses::init_pair(pair, normal::Color::into(colors.foreground()), normal::Color::into(colors.background())) {
        ERR => Err(ncurses_function_error!("init_pair")),
        _   => Ok(normal::ColorPair::from(pair))
    }
}

pub fn initscr() -> result!(WINDOW) {
    match ncurses::initscr() {
        None      => Err(ncurses_function_error!("initscr")),
        Some(win) => Ok(win)
    }
}

/// Return a ascii string of characters starting at the current cursor position for a specified number of characters from the standard screen
///
/// # Example
///
/// ```
/// let string = innstr(10)?;
/// ```
pub fn innstr(number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "ncursesw::innstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::innstr(ptr, number) } {
        ERR => Err(ncurses_function_error!("innstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::innstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::innstr() : len={} > {}", len, LINE_MAX);

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn innwstr(number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::innwstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::innwstr(ptr, number) } {
        ERR => Err(ncurses_function_error!("innwstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::innwstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::innwstr() : len={} > {}", len, LINE_MAX);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

pub fn ins_nwstr(wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::ins_nwstr(raw_with_nul_as_slice!(wstr), number) {
        ERR => Err(ncurses_function_error!("ins_nwstr")),
        _   => Ok(())
    }
}

pub fn ins_wch(wch: &ComplexString) -> result!(()) {
    match ncurses::ins_wch(raw_with_nul_as_slice!(wch)) {
        ERR => Err(ncurses_function_error!("ins_wch")),
        _   => Ok(())
    }
}

pub fn ins_wstr(wstr: &WideString) -> result!(()) {
    match ncurses::ins_wstr(raw_with_nul_as_slice!(wstr)) {
        ERR => Err(ncurses_function_error!("ins_wstr")),
        _   => Ok(())
    }
}

pub fn insch(ch: ChtypeChar) -> result!(()) {
    match ncurses::insch(ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("insch")),
        _   => Ok(())
    }
}

pub fn insdelln(n: i32) -> result!(()) {
    match ncurses::insdelln(n) {
        ERR => Err(ncurses_function_error!("insdelln")),
        _   => Ok(())
    }
}

basic_ncurses_function!(insertln, "insertln");

pub fn insnstr(str: &str, number: i32) -> result!(()) {
    match ncurses::insnstr(c_str_with_nul!(str), number) {
        ERR => Err(ncurses_function_error!("insnstr")),
        _   => Ok(())
    }
}

pub fn insstr(str: &str) -> result!(()) {
    match ncurses::insstr(c_str_with_nul!(str)) {
        ERR => Err(ncurses_function_error!("insstr")),
        _   => Ok(())
    }
}

/// Return an ascii string of characters starting at the current cursor position from the standard screen
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let string = instr()?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use innstr() instead")]
pub fn instr() -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::instr(ptr) } {
        ERR => Err(ncurses_function_error!("instr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::instr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::instr() : len={} > {}", len, LINE_MAX);

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn intrflush(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::intrflush(handle, bf) {
        ERR => Err(ncurses_function_error!("intrflush")),
        _   => Ok(())
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use innwstr() instead")]
pub fn inwstr() -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::inwstr(ptr) } {
        ERR => Err(ncurses_function_error!("inwstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::inwstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::inwstr() : len={} > {}", len, LINE_MAX);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

simple_ncurses_function_with_window_returns_bool!(is_cleared);

simple_ncurses_function_with_window_returns_bool!(is_idcok);

simple_ncurses_function_with_window_returns_bool!(is_idlok);

simple_ncurses_function_with_window_returns_bool!(is_immedok);

simple_ncurses_function_with_window_returns_bool!(is_keypad);

simple_ncurses_function_with_window_returns_bool!(is_leaveok);

pub fn is_linetouched(handle: WINDOW, line: i32) -> bool {
    ncurses::is_linetouched(handle, line)
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
    match ncurses::key_defined(c_str_with_nul!(definition)) {
        ERR => Err(ncurses_function_error!("key_defined")),
        c   => Ok(KeyBinding::from(c))
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
        ERR => Err(ncurses_function_error!("keyok")),
        _   => Ok(())
    }
}

pub fn keypad(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::keypad(handle, bf) {
        ERR => Err(ncurses_function_error!("keypad")),
        _   => Ok(())
    }
}

pub fn killchar() -> result!(char) {
    let rc = ncurses::killchar();

    if rc == ERR as i8 {
        Err(ncurses_function_error!("killchar"))
    } else {
        Ok(char::from(rc as u8))
    }
}

pub fn killwchar() -> result!(WideChar) {
    let mut wch: [wchar_t; 1] = [0];

    match unsafe { ncurses::killwchar(wch.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("killwchar")),
        _   => Ok(WideChar::from(wch[0]))
    }
}

pub fn leaveok(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::leaveok(handle, bf) {
        ERR => Err(ncurses_function_error!("leaveok")),
        _   => Ok(())
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
    match ncurses::meta(handle, bf) {
        ERR => Err(ncurses_function_error!("meta")),
        _   => Ok(())
    }
}

pub fn r#move(origin: Origin) -> result!(()) {
    match ncurses::r#move(origin.y, origin.x) {
        ERR => Err(ncurses_function_error!("move")),
        _   => Ok(())
    }
}

/// Add a complex character to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let origin = Origin { y: 10, x: 15 };
/// let wch = ComplexChar::from_char('A', color_pair1, attributes)?;
///
/// mvadd_wch(origin, wch)?;
/// ```
pub fn mvadd_wch(origin: Origin, wch: ComplexChar) -> result!(()) {
    match ncurses::mvadd_wch(origin.y, origin.x, &ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("mvadd_wch")),
        _   => Ok(())
    }
}

/// Add a complex string of a specified length to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let origin = Origin { y: 10, x: 15 };
/// let wchstr = ComplexString::from_str("testing, testing... 1..2..3..", color_pair1, attributes)?;
///
/// mvadd_wchnstr(origin, &wchstr, 29)?;
/// ```
pub fn mvadd_wchnstr(origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
    match ncurses::mvadd_wchnstr(origin.y, origin.x, raw_with_nul_as_slice!(wchstr), number) {
        ERR => Err(ncurses_function_error!("mvadd_wchnstr")),
        _   => Ok(())
    }
}

/// Add a complex string to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let origin = Origin { y: 10, x: 15 };
/// let wchstr = ComplexString::from_str("testing, testing 1..2..3..", color_pair1, attributes)?;
///
/// mvadd_wchstr(origin, &wchstr)?;
/// ```
pub fn mvadd_wchstr(origin: Origin, wchstr: &ComplexString) -> result!(()) {
    match ncurses::mvadd_wchstr(origin.y, origin.x, raw_with_nul_as_slice!(wchstr)) {
        ERR => Err(ncurses_function_error!("mvadd_wchstr")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) character to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let origin = Origin { y: 10, x: 15 };
/// let ch = ChtypeChar::new(AsciiChar::Asterisk);
///
/// mvaddch(origin, ch)?;
/// ```
pub fn mvaddch(origin: Origin, ch: ChtypeChar) -> result!(()) {
    match ncurses::mvaddch(origin.y, origin.x, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("mvaddch")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) type string of a specified length to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let asciistr = AsciiString::from_ascii_str("testing, testing... 1..2..3..")?;
/// let mut chstr = ChtypeString::from_ascii_str(ascii_str);
/// chstr = chstr | attributes;
/// let origin = Origin { y: 10, x: 15 };
///
/// mvaddchnstr(origin, &chstr, 29)?;
/// ```
pub fn mvaddchnstr(origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
    match ncurses::mvaddchnstr(origin.y, origin.x, raw_with_nul_as_slice!(chstr), number) {
        ERR => Err(ncurses_function_error!("mvaddchnstr")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) type string to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let asciistr = AsciiString::from_ascii_str("testing, testing... 1..2..3..")?;
/// let mut chstr = ChtypeString::from_ascii_str(ascii_str);
/// chstr = chstr | attributes;
/// let origin = Origin { y: 10, x: 15 };
///
/// mvaddchstr(origin, &chstr)?;
/// ```
pub fn mvaddchstr(origin: Origin, chstr: &ChtypeString) -> result!(()) {
    match ncurses::mvaddchstr(origin.y, origin.x, raw_with_nul_as_slice!(chstr)) {
        ERR => Err(ncurses_function_error!("mvaddchstr")),
        _   => Ok(())
    }
}

/// Add a ascii string (as of ABI 6 if not before unicode strings are also supported) of a specified length to the spcified position on the standard screen
///
/// # Example
///
/// ```
/// let string = "testing, testing... 1..2..3..";
/// let origin = Origin { y: 10, x: 15 };
///
/// mvaddnstr(origin, &string, 29)?;
/// ```
pub fn mvaddnstr(origin: Origin, str: &str, number: i32) -> result!(()) {
    match ncurses::mvaddnstr(origin.y, origin.x, c_str_with_nul!(str), number) {
        ERR => Err(ncurses_function_error!("mvaddnstr")),
        _   => Ok(())
    }
}

/// Add a wide character string (unicode) of a specified length to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let wide_string = WideString::from_str("testing, testing... 1..2..3..");
/// let origin = Origin { y: 10, x: 15 };
///
/// mvaddnwstr(origin, &string, 29)?;
/// ```
pub fn mvaddnwstr(origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::mvaddnwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) {
        ERR => Err(ncurses_function_error!("mvaddnwstr")),
        _   => Ok(())
    }
}

/// Add a ascii string (as of ABI 6 if not before unicode strings are also supported) to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let string = "testing, testing... 1..2..3..";
/// let origin = Origin { y: 10, x: 15 };
///
/// mvaddstr(origin, &string)?;
/// ```
pub fn mvaddstr(origin: Origin, str: &str) -> result!(()) {
    match ncurses::mvaddstr(origin.y, origin.x, c_str_with_nul!(str)) {
        ERR => Err(ncurses_function_error!("mvaddstr")),
        _   => Ok(())
    }
}

/// Add a wide character string (unicode) to the specified position on the standard screen
///
/// # Example
///
/// ```
/// let wide_string = WideString::from_str("testing, testing... 1..2..3..");
/// let origin = Origin { y: 10, x: 15 };
///
/// mvaddwstr(origin, &string)?;
/// ```
pub fn mvaddwstr(origin: Origin, wstr: &WideString) -> result!(()) {
    match ncurses::mvaddwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr)) {
        ERR => Err(ncurses_function_error!("mvaddwstr")),
        _   => Ok(())
    }
}

pub fn mvchgat<A, P, T>(origin: Origin, number: i32, attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::mvchgat(origin.y, origin.x, number, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_const_ptr()) } {
        ERR => Err(ncurses_function_error!("mvchgat")),
        _   => Ok(())
    }
}

pub fn mvcur(old: Origin, new: Origin) -> result!(()) {
    match ncurses::mvcur(old.y, old.x, new.y, new.x) {
        ERR => Err(ncurses_function_error!("mvcur")),
        _   => Ok(())
    }
}

pub fn mvdelch(origin: Origin) -> result!(()) {
    match ncurses::mvdelch(origin.y, origin.x) {
        ERR => Err(ncurses_function_error!("mvdelch")),
        _   => Ok(())
    }
}

pub fn mvderwin(handle: WINDOW, origin: Origin) -> result!(()) {
    match ncurses::mvderwin(handle, origin.y, origin.x) {
        ERR => Err(ncurses_function_error!("mvderwin")),
        _   => Ok(())
    }
}

pub fn mvget_wch(origin: Origin) -> result!(WideCharResult) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::mvget_wch(origin.y, origin.x, wch.as_mut_ptr()) } {
        ERR          => Err(ncurses_function_error!("mvget_wch")),
        KEY_CODE_YES => {
            if wch[0] == KEY_MOUSE as wint_t {
                Err(NCurseswError::KeyMouse)
            } else if wch[0] == KEY_RESIZE as wint_t {
                Err(NCurseswError::KeyReSize)
            } else if wch[0] == KEY_EVENT as wint_t {
                Err(NCurseswError::KeyEvent)
            } else {
                Ok(WideCharResult::Key(KeyBinding::from(wch[0])))
            }
        },
        _            => Ok(WideCharResult::Character(WideChar::from(wch[0])))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvgetn_wstr() instead")]
pub fn mvget_wstr(origin: Origin) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvget_wstr(origin.y, origin.x, ptr) } {
        ERR        => Err(ncurses_function_error!("mvget_wstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
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

pub fn mvgetch(origin: Origin) -> result!(CharacterResult) {
    match ncurses::mvgetch(origin.y, origin.x) {
        ERR        => Err(ncurses_function_error!("mvgetch")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        ch         => {
            if ch > i32::from(i8::max_value()) {
                Ok(CharacterResult::Key(KeyBinding::from(ch)))
            } else {
                Ok(CharacterResult::Character(char::from(ch as i8 as u8)))
            }
        }
    }
}

pub fn mvgetn_wstr(origin: Origin, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvgetn_wstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetn_wstr(origin.y, origin.x, ptr, number) } {
        ERR        => Err(ncurses_function_error!("mvgetn_wstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
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

pub fn mvgetnstr(origin: Origin, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvgetnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetnstr(origin.y, origin.x, ptr, number) } {
        ERR        => Err(ncurses_function_error!("mvgetnstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
            assert!(!ptr.is_null(), "ncursesw::mvgetnstr() : ptr.is_null()");

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvgetnstr() instead")]
pub fn mvgetstr(origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetstr(origin.y, origin.x, ptr) } {
        ERR        => Err(ncurses_function_error!("mvgetstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
            assert!(!ptr.is_null(), "ncursesw::mvgetstr() : ptr.is_null()");

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn mvhline(origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::mvhline(origin.y, origin.x, ChtypeChar::into(ch), number) {
        ERR => Err(ncurses_function_error!("mvhline")),
        _   => Ok(())
    }
}

pub fn mvhline_set(origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::mvhline_set(origin.y, origin.x, &ComplexChar::into(wch), number) {
        ERR => Err(ncurses_function_error!("mvhline_set")),
        _   => Ok(())
    }
}

/// Return a Complex character from the specified cursor position from the standard screen
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let cchar = mvin_wch(origin)?;
/// ```
pub fn mvin_wch(origin: Origin) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::mvin_wch(origin.y, origin.x, wcval.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("mvin_wch")),
        _   => Ok(ComplexChar::from(wcval[0]))
    }
}

/// Return a complex string of characters starting at the specified cursor position for a specified number of characters from the standard screen
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chstring = mvin_wchnstr(origin, 10)?;
/// ```
pub fn mvin_wchnstr(origin: Origin, number: i32) -> result!(ComplexString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvin_wchnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvin_wchnstr(origin.y, origin.x, ptr, number) } {
        ERR => Err(ncurses_function_error!("mvin_wchnstr")),
        _   => {
            assert!(!ptr.is_null(), "ncursesw::mvin_wchnstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, number as usize) }))
        }
    }
}

/// Return a Complex string of characters starting at the specified cursor position from the standard screen
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chstring = mvin_wchstr(origin)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvin_wchnstr() instead")]
pub fn mvin_wchstr(origin: Origin) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvin_wchstr(origin.y, origin.x, ptr) } {
        ERR => Err(ncurses_function_error!("mvin_wchstr")),
        _   => {
            assert!(!ptr.is_null(), "ncursesw::mvin_wchstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        }
    }
}

/// Return a Chtype character from the specified cursor position from the standard screen
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chchar = mvinch(origin)?;
/// ```
pub fn mvinch(origin: Origin) -> ChtypeChar {
    ChtypeChar::from(ncurses::mvinch(origin.y, origin.x))
}

/// Return a Chtype string of characters starting at the specified cursor position for a specified number of characters from the standard screen
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chstring = mvinchnstr(origin, 10)?;
/// ```
pub fn mvinchnstr(origin: Origin, number: i32) -> result!(ChtypeString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvinchnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    match unsafe { ncurses::mvinchnstr(origin.y, origin.x, ptr, number) } {
        ERR => Err(ncurses_function_error!("mvinchnstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvinchnstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvinchnstr() : len={} > {}", len, LINE_MAX);

            Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

/// Return a Chtype string of characters starting at the specified cursor position from the standard screen
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chstring = mvinchstr(origin)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvinchnstr() instead")]
pub fn mvinchstr(origin: Origin) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    match unsafe { ncurses::mvinchstr(origin.y, origin.x, ptr) } {
        ERR => Err(ncurses_function_error!("mvinchstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvinchstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvinchstr() : len={} > {}", len, LINE_MAX);

            Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

/// Return a ascii string of characters starting at the specified cursor position for a specified number of characters from the standard screen
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let string = mvinnstr(origin, 10)?;
/// ```
pub fn mvinnstr(origin: Origin, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvinnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvinnstr(origin.y, origin.x, ptr, number) } {
        ERR => Err(ncurses_function_error!("mvinnstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvinnstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvinnstr() : len={} > {}", len, LINE_MAX);

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn mvinnwstr(origin: Origin, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvinnwstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvinnwstr(origin.y, origin.x, ptr, number) } {
        ERR => Err(ncurses_function_error!("mvinnwstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvinnwstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvinnwstr() : len={} > {}", len, LINE_MAX);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

pub fn mvins_nwstr(origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::mvins_nwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) {
        ERR => Err(ncurses_function_error!("mvins_nwstr")),
        _   => Ok(())
    }
}

pub fn mvins_wch(origin: Origin, wch: &ComplexString) -> result!(()) {
    match ncurses::mvins_wch(origin.y, origin.x, raw_with_nul_as_slice!(wch)) {
        ERR => Err(ncurses_function_error!("mvins_wch")),
        _   => Ok(())
    }
}

pub fn mvins_wstr(origin: Origin, wstr: &WideString) -> result!(()) {
    match ncurses::mvins_wstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr)) {
        ERR => Err(ncurses_function_error!("mvins_wstr")),
        _   => Ok(())
    }
}

pub fn mvinsch(origin: Origin, ch: ChtypeChar) -> result!(()) {
    match ncurses::mvinsch(origin.y, origin.x, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("mvinsch")),
        _   => Ok(())
    }
}

pub fn mvinsnstr(origin: Origin, str: &str, number: i32) -> result!(()) {
    match ncurses::mvinsnstr(origin.y, origin.x, c_str_with_nul!(str), number) {
        ERR => Err(ncurses_function_error!("mvinsnstr")),
        _   => Ok(())
    }
}

pub fn mvinsstr(origin: Origin, str: &str) -> result!(()) {
    match ncurses::mvinsstr(origin.y, origin.x, c_str_with_nul!(str)) {
        ERR => Err(ncurses_function_error!("mvinsstr")),
        _   => Ok(())
    }
}

/// Return an ascii string of characters starting at the specified cursor position from the standard screen
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let string = mvinstr(origin)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvinnstr() instead")]
pub fn mvinstr(origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvinstr(origin.y, origin.x, ptr) } {
        ERR => Err(ncurses_function_error!("mvinstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvinstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvinstr() : len={} > {}", len, LINE_MAX);

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvinnwstr() instead")]
pub fn mvinwstr(origin: Origin) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvinwstr(origin.y, origin.x, ptr) } {
        ERR => Err(ncurses_function_error!("mvinwstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvinwstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvinwstr() : len={} > {}", len, LINE_MAX);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

pub fn mvvline(origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::mvvline(origin.y, origin.x, ChtypeChar::into(ch), number) {
        ERR => Err(ncurses_function_error!("mvvline")),
        _   => Ok(())
    }
}

pub fn mvvline_set(origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::mvvline_set(origin.y, origin.x, &ComplexChar::into(wch), number) {
        ERR => Err(ncurses_function_error!("mvvline_set")),
        _   => Ok(())
    }
}

/// Add a complex character to the specified position on the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let origin = Origin { y: 10, x: 15 };
/// let wch = ComplexChar::from_char('A', color_pair1, attributes)?;
///
/// mvwadd_wch(window_handle, origin, wch)?;
/// ```
pub fn mvwadd_wch(handle: WINDOW, origin: Origin, wch: ComplexChar) -> result!(()) {
    match ncurses::mvwadd_wch(handle, origin.y, origin.x, &ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("mvwadd_wch")),
        _   => Ok(())
    }
}

/// Add a complex string of a specified length to the specified position on the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let origin = Origin { y: 10, x: 15 };
/// let wchstr = ComplexString::from_str("testing, testing... 1..2..3..", color_pair1, attributes)?;
///
/// mvwadd_wchnstr(window_handle, origin, &wchstr, 29)?;
/// ```
pub fn mvwadd_wchnstr(handle: WINDOW, origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
    match ncurses::mvwadd_wchnstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wchstr), number) {
        ERR => Err(ncurses_function_error!("mvwadd_wchnstr")),
        _   => Ok(())
    }
}

/// Add a complex string to the specified position on the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let wchstr = ComplexString::from_str("testing, testing 1..2..3..", color_pair1, attributes)?;
///
/// mvwadd_wchstr(window_handle, origin, &wchstr)?;
/// ```
pub fn mvwadd_wchstr(handle: WINDOW, origin: Origin, wchstr: &ComplexString) -> result!(()) {
    match ncurses::mvwadd_wchstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wchstr)) {
        ERR => Err(ncurses_function_error!("mvwadd_wchstr")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) character to the specified position on the specified window
///
/// # Example
///
/// ```
/// let origin = Origin { y: 10, x: 15 };
/// let ch = ChtypeChar::new(AsciiChar::Asterisk);
///
/// mvwaddch(window_handle, origin, ch)?;
/// ```
pub fn mvwaddch(handle: WINDOW, origin: Origin, ch: ChtypeChar) -> result!(()) {
    match ncurses::mvwaddch(handle, origin.y, origin.x, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("mvwaddch")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) type string of a specified length to the specified position on the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let asciistr = AsciiString::from_ascii_str("testing, testing... 1..2..3..")?;
/// let mut chstr = ChtypeString::from_ascii_str(ascii_str);
/// chstr = chstr | attributes;
/// let origin = Origin { y: 10, x: 15 };
///
/// mvwaddchnstr(window_handle, origin, &chstr, 29)?;
/// ```
pub fn mvwaddchnstr(handle: WINDOW, origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
    match ncurses::mvwaddchnstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(chstr), number) {
        ERR => Err(ncurses_function_error!("mvwaddchnstr")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) type string to the specified position on the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let asciistr = AsciiString::from_ascii_str("testing, testing... 1..2..3..")?;
/// let mut chstr = ChtypeString::from_ascii_str(ascii_str);
/// chstr = chstr | attributes;
/// let origin = Origin { y: 10, x: 15 };
///
/// mvwaddchstr(window_handle, origin, &chstr)?;
/// ```
pub fn mvwaddchstr(handle: WINDOW, origin: Origin, chstr: &ChtypeString) -> result!(()) {
    match ncurses::mvwaddchstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(chstr)) {
        ERR => Err(ncurses_function_error!("mvwaddchstr")),
        _   => Ok(())
    }
}

/// Add a ascii string (as of ABI 6 if not before unicode strings are also supported) of a specified length to the spcified position on the specified window
///
/// # Example
///
/// ```
/// let string = "testing, testing... 1..2..3..";
/// let origin = Origin { y: 10, x: 15 };
///
/// mvwaddnstr(window_handle, origin, &string, 29)?;
/// ```
pub fn mvwaddnstr(handle: WINDOW, origin: Origin, str: &str, number: i32) -> result!(()) {
    match ncurses::mvwaddnstr(handle, origin.y, origin.x, c_str_with_nul!(str), number) {
        ERR => Err(ncurses_function_error!("mvwaddnstr")),
        _   => Ok(())
    }
}

/// Add a wide character string (unicode) of a specified length to the specified position on the specified window
///
/// # Example
///
/// ```
/// let wide_string = WideString::from_str("testing, testing... 1..2..3..");
/// let origin = Origin { y: 10, x: 15 };
///
/// mvwaddnwstr(window_handle, origin, &string, 29)?;
/// ```
pub fn mvwaddnwstr(handle: WINDOW, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::mvwaddnwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) {
        ERR => Err(ncurses_function_error!("mvwaddnwstr")),
        _   => Ok(())
    }
}

/// Add a ascii string (as of ABI 6 if not before unicode strings are also supported) to the specified position on the specified window
///
/// # Example
///
/// ```
/// let string = "testing, testing... 1..2..3..";
/// let origin = Origin { y: 10, x: 15 };
///
/// mvwaddstr(window_handle, origin, &string)?;
/// ```
pub fn mvwaddstr(handle: WINDOW, origin: Origin, str: &str) -> result!(()) {
    match ncurses::mvwaddstr(handle, origin.y, origin.x, c_str_with_nul!(str)) {
        ERR => Err(ncurses_function_error!("mvwaddstr")),
        _   => Ok(())
    }
}

/// Add a wide character string (unicode) to the specified position on the specified window
///
/// # Example
///
/// ```
/// let wide_string = WideString::from_str("testing, testing... 1..2..3..");
/// let origin = Origin { y: 10, x: 15 };
///
/// mvwaddwstr(window_handle, origin, &string)?;
/// ```
pub fn mvwaddwstr(handle: WINDOW, origin: Origin, wstr: &WideString) -> result!(()) {
    match ncurses::mvwaddwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr)) {
        ERR => Err(ncurses_function_error!("mvwaddwstr")),
        _   => Ok(())
    }
}

pub fn mvwchgat<A, P, T>(handle: WINDOW, origin: Origin, number: i32, attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::mvwchgat(handle, origin.y, origin.x, number, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_const_ptr()) } {
        ERR => Err(ncurses_function_error!("mvwchgat")),
        _   => Ok(())
    }
}

pub fn mvwdelch(handle: WINDOW, origin: Origin) -> result!(()) {
    match ncurses::mvwdelch(handle, origin.y, origin.x) {
        ERR => Err(ncurses_function_error!("mvwdelch")),
        _   => Ok(())
    }
}

pub fn mvwget_wch(handle: WINDOW, origin: Origin) -> result!(WideCharResult) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::mvwget_wch(handle, origin.y, origin.x, wch.as_mut_ptr()) } {
        ERR          => Err(ncurses_function_error!("mvwget_wch")),
        KEY_CODE_YES => {
            if wch[0] == KEY_MOUSE as wint_t {
                Err(NCurseswError::KeyMouse)
            } else if wch[0] == KEY_RESIZE as wint_t {
                Err(NCurseswError::KeyReSize)
            } else if wch[0] == KEY_EVENT as wint_t {
                Err(NCurseswError::KeyEvent)
            } else {
                Ok(WideCharResult::Key(KeyBinding::from(wch[0])))
            }
        },
        _            => Ok(WideCharResult::Character(WideChar::from(wch[0])))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwgetn_wstr() instead")]
pub fn mvwget_wstr(handle: WINDOW, origin: Origin) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwget_wstr(handle, origin.y, origin.y, ptr) } {
        ERR        => Err(ncurses_function_error!("mvwget_wstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
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

pub fn mvwgetch(handle: WINDOW, origin: Origin) -> result!(CharacterResult) {
    match ncurses::mvwgetch(handle, origin.y, origin.x) {
        ERR        => Err(ncurses_function_error!("mvwgetch")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        ch         => {
            if ch > i32::from(i8::max_value()) {
                Ok(CharacterResult::Key(KeyBinding::from(ch)))
            } else {
                Ok(CharacterResult::Character(char::from(ch as i8 as u8)))
            }
        }
    }
}

pub fn mvwgetn_wstr(handle: WINDOW, origin: Origin, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvwgetn_wstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetn_wstr(handle, origin.y, origin.x, ptr, number) } {
        ERR        => Err(ncurses_function_error!("mvwgetn_wstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
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

pub fn mvwgetnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvwgetnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetnstr(handle, origin.y, origin.x, ptr, number) } {
        ERR        => Err(ncurses_function_error!("mvwgetnstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
            assert!(!ptr.is_null(), "ncursesw::mvwgetnstr() : ptr.is_null()");

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwgetnstr() instead")]
pub fn mvwgetstr(handle: WINDOW, origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetstr(handle, origin.y, origin.x, ptr) } {
        ERR        => Err(ncurses_function_error!("mvwgetstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
            assert!(!ptr.is_null(), "ncursesw::mvwgetstr() : ptr.is_null()");

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn mvwhline(handle: WINDOW, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::mvwhline(handle, origin.y, origin.x, ChtypeChar::into(ch), number) {
        ERR => Err(ncurses_function_error!("mvwhline")),
        _   => Ok(())
    }
}

pub fn mvwhline_set(handle: WINDOW, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::mvwhline_set(handle, origin.y, origin.x, &ComplexChar::into(wch), number) {
        ERR => Err(ncurses_function_error!("mvwhline_set")),
        _   => Ok(())
    }
}

pub fn mvwin(handle: WINDOW, origin: Origin) -> result!(()) {
    match ncurses::mvwin(handle, origin.y, origin.x) {
        ERR => Err(ncurses_function_error!("mvwin")),
        _   => Ok(())
    }
}

/// Return a Complex character from the specified cursor position from the specified window
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let cchar = mvwin_wch(window_handle, origin)?;
/// ```
pub fn mvwin_wch(handle: WINDOW, origin: Origin) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::mvwin_wch(handle, origin.y, origin.x, wcval.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("mvwin_wch")),
        _   => Ok(ComplexChar::from(wcval[0]))
    }
}

/// Return a complex string of characters starting at the specified cursor position for a specified number of characters from the specified window
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chstring = mvwin_wchnstr(window_handle, origin, 10)?;
/// ```
pub fn mvwin_wchnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(ComplexString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvwin_wchnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwin_wchnstr(handle, origin.y, origin.x, ptr, number) } {
        ERR => Err(ncurses_function_error!("mvwin_wchnstr")),
        _   => {
            assert!(!ptr.is_null(), "ncursesw::mvwin_wchnstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, number as usize) }))
        }
    }
}

/// Return a Complex string of characters starting at the specified cursor position from the specified window
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chstring = mvwin_wchstr(window_handle, origin)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwin_wchnstr() instead")]
pub fn mvwin_wchstr(handle: WINDOW, origin: Origin) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwin_wchstr(handle, origin.y, origin.x, ptr) } {
        ERR => Err(ncurses_function_error!("mvwin_wchstr")),
        _   => {
            assert!(!ptr.is_null(), "ncursesw::mvwin_wchstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        }
    }
}

/// Return a Chtype character from the specified cursor position from the specified window
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chchar = mvwinch(window_handle, origin)?;
/// ```
pub fn mvwinch(handle: WINDOW, origin: Origin) -> ChtypeChar {
    ChtypeChar::from(ncurses::mvwinch(handle, origin.y, origin.x))
}

/// Return a Chtype string of characters starting at the specified cursor position for a specified number of characters from the specified window
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chstring = mvwinchnstr(window_handle, origin, 10)?;
/// ```
pub fn mvwinchnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(ChtypeString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvwinchnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    match unsafe { ncurses::mvwinchnstr(handle, origin.y, origin.x, ptr, number) } {
        ERR => Err(ncurses_function_error!("mvwinchnstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvwinchnstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvwinchnstr() : len={} > {}", len, LINE_MAX);

            Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

/// Return a Chtype string of characters starting at the specified cursor position from the specified window
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let chstring = mvwinchstr(window_handle, origin)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwinchnstr() instead")]
pub fn mvwinchstr(handle: WINDOW, origin: Origin) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    match unsafe { ncurses::mvwinchstr(handle, origin.y, origin.x, ptr) } {
        ERR => Err(ncurses_function_error!("mvwinchstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvwinchstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvwinchstr() : len={} > {}", len, LINE_MAX);

            Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

/// Return a ascii string of characters starting at the specified cursor position for a specified number of characters from the specified window
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let string = mvwinnstr(window_handle, origin, 10)?;
/// ```
pub fn mvwinnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvwinnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvwinnstr(handle, origin.y, origin.x, ptr, number) } {
        ERR => Err(ncurses_function_error!("mvwinnstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvwinnstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvwinnstr() : len={} > {}", len, LINE_MAX);

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn mvwinnwstr(handle: WINDOW, origin: Origin, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::mvwinnwstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwinnwstr(handle, origin.y, origin.x, ptr, number) } {
        ERR => Err(ncurses_function_error!("mvwinnwstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvwinnwstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvwinnwstr() : len={} > {}", len, LINE_MAX);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

pub fn mvwins_nwstr(handle: WINDOW, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::mvwins_nwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) {
        ERR => Err(ncurses_function_error!("mvwins_nwstr")),
        _   => Ok(())
    }
}

pub fn mvwins_wch(handle: WINDOW, origin: Origin, wch: &ComplexString) -> result!(()) {
    match ncurses::mvwins_wch(handle, origin.y, origin.x, raw_with_nul_as_slice!(wch)) {
        ERR => Err(ncurses_function_error!("mvwins_wch")),
        _   => Ok(())
    }
}

pub fn mvwins_wstr(handle: WINDOW, origin: Origin, wstr: &WideString) -> result!(()) {
    match ncurses::mvwins_wstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr)) {
        ERR => Err(ncurses_function_error!("mvwins_wstr")),
        _   => Ok(())
    }
}

pub fn mvwinsch(handle: WINDOW, origin: Origin, ch: ChtypeChar) -> result!(()) {
    match ncurses::mvwinsch(handle, origin.y, origin.x, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("mvwinsch")),
        _   => Ok(())
    }
}

pub fn mvwinsnstr(handle: WINDOW, origin: Origin, str: &str, number: i32) -> result!(()) {
    match ncurses::mvwinsnstr(handle, origin.y, origin.x, c_str_with_nul!(str), number) {
        ERR => Err(ncurses_function_error!("mvwinsnstr")),
        _   => Ok(())
    }
}

pub fn mvwinsstr(handle: WINDOW, origin: Origin, str: &str) -> result!(()) {
    match ncurses::mvwinsstr(handle, origin.y, origin.x, c_str_with_nul!(str)) {
        ERR => Err(ncurses_function_error!("mvwinsstr")),
        _   => Ok(())
    }
}

/// Return an ascii string of characters starting at the specified cursor position from the specified window
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let origin = Origin { y: 5, x: 15 };
/// let string = mvwinstr(window_handle, origin)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwinnstr() instead")]
pub fn mvwinstr(handle: WINDOW, origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvwinstr(handle, origin.y, origin.x, ptr) } {
        ERR => Err(ncurses_function_error!("mvwinstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvwinstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvwinstr() : len={} > {}", len, LINE_MAX);

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use mvwinnwstr() instead")]
pub fn mvwinwstr(handle: WINDOW, origin: Origin) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwinwstr(handle, origin.y, origin.x, ptr) } {
        ERR => Err(ncurses_function_error!("mvwinwstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::mvwinwstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::mvwinwstr() : len={} > {}", len, LINE_MAX);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

pub fn mvwvline(handle: WINDOW, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::mvwvline(handle, origin.y, origin.x, ChtypeChar::into(ch), number) {
        ERR => Err(ncurses_function_error!("mvwvline")),
        _   => Ok(())
    }
}

pub fn mvwvline_set(handle: WINDOW, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::mvwvline_set(handle, origin.y, origin.x, &ComplexChar::into(wch), number) {
        ERR => Err(ncurses_function_error!("mvwvline_set")),
        _   => Ok(())
    }
}

pub fn napms(ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    match ncurses::napms(ms) {
        ERR => Err(ncurses_function_error!("napms")),
        _   => Ok(())
    }
}

pub fn newpad(size: Size) -> result!(WINDOW) {
    match ncurses::newpad(size.lines, size.columns) {
        None      => Err(ncurses_function_error!("newpad")),
        Some(win) => Ok(win)
    }
}

pub fn newterm(_ty: Option<&str>, _outfd: shims::bindings::FILE, _infd: shims::bindings::FILE) -> Option<SCREEN> {
    unimplemented!();
}

pub fn newwin(size: Size, origin: Origin) -> result!(WINDOW) {
    match ncurses::newwin(size.lines, size.columns, origin.y, origin.x) {
        None      => Err(ncurses_function_error!("newwin")),
        Some(win) => Ok(win)
    }
}

basic_ncurses_function!(nl, "nl");

basic_ncurses_function!(nocbreak, "nocbreak");

pub fn nodelay(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::nodelay(handle, bf) {
        ERR => Err(ncurses_function_error!("nodelay")),
        _   => Ok(())
    }
}

basic_ncurses_function!(noecho, "noecho");

simple_ncurses_function!(nofilter);

basic_ncurses_function!(nonl, "nonl");

simple_ncurses_function!(noqiflush);

basic_ncurses_function!(noraw, "noraw");

pub fn notimeout(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::notimeout(handle, bf) {
        ERR => Err(ncurses_function_error!("notimeout")),
        _   => Ok(())
    }
}

pub fn overlay(src_handle: WINDOW, dst_handle: WINDOW) -> result!(()) {
    match ncurses::overlay(src_handle, dst_handle) {
        ERR => Err(ncurses_function_error!("overlay")),
        _   => Ok(())
    }
}

pub fn overwrite(src_handle: WINDOW, dst_handle: WINDOW) -> result!(()) {
    match ncurses::overwrite(src_handle, dst_handle) {
        ERR => Err(ncurses_function_error!("overwrite")),
        _   => Ok(())
    }
}

pub fn pair_content(pair: normal::ColorPair) -> result!(normal::Colors) {
    let mut fg: [short_t; 1] = [0];
    let mut bg: [short_t; 1] = [0];

    match unsafe { ncurses::pair_content(normal::ColorPair::into(pair), fg.as_mut_ptr(), bg.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("pair_content")),
        _   => Ok(normal::Colors::new(normal::Color::from(fg[0]), normal::Color::from(bg[0])))
    }
}

pub fn pechochar(pad: WINDOW, ch: ChtypeChar) -> result!(()) {
    match ncurses::pechochar(pad, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("pechochar")),
        _   => Ok(())
    }
}

pub fn pecho_wchar(pad: WINDOW, wch: ComplexChar) -> result!(()) {
    match ncurses::pecho_wchar(pad, &ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("pecho_wchar")),
        _   => Ok(())
    }
}

pub fn pnoutrefresh(pad: WINDOW, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
    match ncurses::pnoutrefresh(pad, pmin.y, pmin.x, smin.y, smin.x, smax.y, smax.x) {
        ERR => Err(ncurses_function_error!("pnoutrefresh")),
        _   => Ok(())
    }
}

pub fn prefresh(pad: WINDOW, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
    match ncurses::prefresh(pad, pmin.y, pmin.x, smin.y, smin.x, smax.y, smax.x) {
        ERR => Err(ncurses_function_error!("prefresh")),
        _   => Ok(())
    }
}

pub fn putp(_str: &str) -> i32 {
    unimplemented!();
}

pub fn putwin(handle: WINDOW, path: &path::Path) -> result!(()) {
    let mode = "w";

    match utils::fopen(path, mode) {
        None     => Err(NCurseswError::FOpen { fname: path.display().to_string(), mode:  mode.to_string() }),
        Some(fp) => {
            if ncurses::putwin(handle, fp) == ERR {
                Err(ncurses_function_error!("putwin"))
            } else {
                Ok(())
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
        ERR => Err(ncurses_function_error!("resize_term")),
        _   => Ok(())
    }
}

pub fn resizeterm(size: Size) -> result!(()) {
    match ncurses::resizeterm(size.lines, size.columns) {
        ERR => Err(ncurses_function_error!("resizeterm")),
        _   => Ok(())
    }
}

pub fn ripoffline(line: Orientation, init: RipoffInit) -> result!(()) {
    match ncurses::ripoffline(Orientation::into(line), init) {
        ERR => Err(ncurses_function_error!("ripoffline")),
        _   => Ok(())
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
        ERR => Err(ncurses_function_error!("scrl")),
        _   => Ok(())
    }
}

basic_ncurses_function_with_window!(scroll, "scroll");

pub fn scrollok(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::scrollok(handle, bf) {
        ERR => Err(ncurses_function_error!("scrollok")),
        _   => Ok(())
    }
}

pub fn set_escdelay(size: time::Duration) -> result!(()) {
    let ms = i32::try_from(size.as_millis())?;

    match ncurses::set_escdelay(ms) {
        ERR => Err(ncurses_function_error!("set_escdelay")),
        _   => Ok(())
    }
}

pub fn set_tabsize(size: i32) -> result!(()) {
    match ncurses::set_tabsize(size) {
        ERR => Err(ncurses_function_error!("set_tabsize")),
        _   => Ok(())
    }
}

pub fn set_term(scr: SCREEN) -> result!(SCREEN) {
    match ncurses::set_term(scr) {
        None    => Err(ncurses_function_error!("set_term")),
        Some(s) => Ok(s)
    }
}

pub fn setcchar<A, P, T>(ch: char, attrs: &A, color_pair: &P) -> result!(ComplexChar) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    let mut cchar_buf: [cchar_t; 1] = unsafe { mem::zeroed() };
    let wchar_buf: [wchar_t; 2] = [u32::from(ch) as wchar_t, 0x00];

    let cchar_ptr: *mut cchar_t = cchar_buf.as_mut_ptr();

    match unsafe { ncurses::setcchar(cchar_ptr, wchar_buf.as_ptr(), attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("setcchar")),
        _   => {
            assert!(!cchar_ptr.is_null(), "ncursesw::setcchar() : cchar_ptr.is_null()");

            Ok(ComplexChar::from(unsafe { slice::from_raw_parts(cchar_ptr, 1)[0] }))
        }
    }
}

pub fn setscrreg(region: Region) -> result!(()) {
    match ncurses::setscrreg(region.top, region.bottom) {
        ERR => Err(ncurses_function_error!("setscrreg")),
        _   => Ok(())
    }
}

// convert into the attributes type of your choice with
//     normal::Attributes::from(slk_attr()) or
//     extend::Attributes::from(slk_attr())
pub fn slk_attr() -> attr_t {
    ncurses::slk_attr()
}

pub fn slk_attr_off<A, T>(attrs: A) -> result!(()) where A: AttributesType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::slk_attr_off(attrs.as_attr_t(), ptr::null_mut()) } {
        ERR => Err(ncurses_function_error!("slk_attr_off")),
        _   => Ok(())
    }
}

pub fn slk_attr_on<A, T>(attrs: A) -> result!(()) where A: AttributesType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::slk_attr_on(attrs.as_attr_t(), ptr::null_mut()) } {
        ERR => Err(ncurses_function_error!("slk_attr_on")),
        _   => Ok(())
    }
}

pub fn slk_attr_set<A, P, T>(attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::slk_attr_set(attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("slk_attr_set")),
        _   => Ok(())
    }
}

pub fn slk_attroff(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attroff(normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("slk_attroff")),
        _   => Ok(())
    }
}

pub fn slk_attron(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attron(normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("slk_attron")),
        _   => Ok(())
    }
}

pub fn slk_attrset(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attrset(normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("slk_attrset")),
        _   => Ok(())
    }
}

basic_ncurses_function!(slk_clear, "slk_clear");

pub fn slk_color(color_pair: normal::ColorPair) -> result!(()) {
    match ncurses::slk_color(color_pair.number()) {
        ERR => Err(ncurses_function_error!("slk_color")),
        _   => Ok(())
    }
}

pub fn slk_init(fmt: SoftLabelType) -> result!(()) {
    match ncurses::slk_init(SoftLabelType::into(fmt)) {
        ERR => Err(ncurses_function_error!("slk_init")),
        _   => Ok(())
    }
}

pub fn slk_label(number: i32) -> String {
    ncurses::slk_label(number)
}

basic_ncurses_function!(slk_noutrefresh, "slk_noutrefresh");

basic_ncurses_function!(slk_refresh, "slk_refresh");

basic_ncurses_function!(slk_restore, "slk_restore");

pub fn slk_set(label_number: i32, label: &str, fmt: Justification) -> result!(()) {
    match ncurses::slk_set(label_number, label, Justification::into(fmt)) {
        ERR => Err(ncurses_function_error!("slk_set")),
        _   => Ok(())
    }
}

basic_ncurses_function!(slk_touch, "slk_touch");

pub fn slk_wset(label_number: i32, label: &WideString, fmt: Justification) -> result!(()) {
    match ncurses::slk_wset(label_number, raw_with_nul_as_slice!(label), Justification::into(fmt)) {
        ERR => Err(ncurses_function_error!("slk_set")),
        _   => Ok(())
    }
}

basic_ncurses_function!(standend, "standend");

basic_ncurses_function!(standout, "standout");

basic_ncurses_function!(start_color, "start_color");

pub fn subpad(handle: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    match ncurses::subpad(handle, size.lines, size.columns, origin.y, origin.x) {
        None      => Err(ncurses_function_error!("subpad")),
        Some(win) => Ok(win)
    }
}

pub fn subwin(handle: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    match ncurses::subwin(handle, size.lines, size.columns, origin.y, origin.x) {
        None         => Err(ncurses_function_error!("subwin")),
        Some(handle) => Ok(handle)
    }
}

pub fn syncok(handle: WINDOW, bf: bool) -> result!(()) {
    match ncurses::syncok(handle, bf) {
        ERR => Err(ncurses_function_error!("syncok")),
        _   => Ok(())
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
    match ncurses::touchline(handle, start, count) {
        ERR => Err(ncurses_function_error!("touchline")),
        _   => Ok(())
    }
}

basic_ncurses_function_with_window!(touchwin, "touchwin");

pub fn tparm(_s: &str) -> String {
    unimplemented!();
}

//pub fn tputs

pub fn typeahead(_fd: i32) -> i32 {
    unimplemented!();
}

pub fn unctrl(c: ChtypeChar) -> String {
    ncurses::unctrl(ChtypeChar::into(c))
}

pub fn unget_wch(ch: WideChar) -> result!(()) {
    match ncurses::unget_wch(WideChar::into(ch)) {
        ERR => Err(ncurses_function_error!("unget_wch")),
        _   => Ok(())
    }
}

pub fn ungetch(ch: char) -> result!(()) {
    match ncurses::ungetch(i32::from(ch as u8)) {
        ERR => Err(ncurses_function_error!("ungetch")),
        _   => Ok(())
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
    match ncurses::use_legacy_coding(Legacy::into(level)) {
        ERR => Err(ncurses_function_error!("use_legacy_coding")),
        rc  => {
            let level = Legacy::try_from(rc)?;

            Ok(level)
        }
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
        ERR => Err(ncurses_function_error!("vline")),
        _   => Ok(())
    }
}

pub fn vline_set(wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::vline_set(&ComplexChar::into(wch), number) {
        ERR => Err(ncurses_function_error!("vline_set")),
        _   => Ok(())
    }
}

/// Add a complex character at thecurrent position on the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let wch = ComplexChar::from_char('A', color_pair1, attributes)?;
///
/// wadd_wch(window_handle, wch)?;
/// ```
pub fn wadd_wch(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match ncurses::wadd_wch(handle, &ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("wadd_wch")),
        _   => Ok(())
    }
}

/// Add a complex string of a specified length on the spefied window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let wchstr = ComplexString::from_str("testing, testing... 1..2..3..", color_pair1, attributes)?;
///
/// wadd_wchnstr(window_handle, &wchstr, 29)?;
/// ```
pub fn wadd_wchnstr(handle: WINDOW, wchstr: &ComplexString, number: i32) -> result!(()) {
    match ncurses::wadd_wchnstr(handle, raw_with_nul_as_slice!(wchstr), number) {
        ERR => Err(ncurses_function_error!("wadd_wchnstr")),
        _   => Ok(())
    }
}

/// Add a complex string to the current position on the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let wchstr = ComplexString::from_str("testing, testing 1..2..3..", color_pair1, attributes)?;
///
/// wadd_wchstr(window_handle, &wchstr)?;
/// ```
pub fn wadd_wchstr(handle: WINDOW, wchstr: &ComplexString) -> result!(()) {
    match ncurses::wadd_wchstr(handle, raw_with_nul_as_slice!(wchstr)) {
        ERR => Err(ncurses_function_error!("wadd_wchstr")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) character to the current position on the specified window
///
/// # Example
///
/// ```
/// let ch = ChtypeChar::new(AsciiChar::Asterisk);
///
/// waddch(window_handle, ch)?;
/// ```
pub fn waddch(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match ncurses::waddch(handle, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("waddch")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) type string of a specified length to the current position on the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let asciistr = AsciiString::from_ascii_str("testing, testing... 1..2..3..")?;
/// let mut chstr = ChtypeString::from_ascii_str(ascii_str);
/// chstr = chstr | attributes;
///
/// waddchnstr(window_handle, &chstr, 29)?;
/// ```
pub fn waddchnstr(handle: WINDOW, chstr: &ChtypeString, number: i32) -> result!(()) {
    match ncurses::waddchnstr(handle, raw_with_nul_as_slice!(chstr), number) {
        ERR => Err(ncurses_function_error!("waddchnstr")),
        _   => Ok(())
    }
}

/// Add a chtype (ascii with attributes) type string to the specified window
///
/// # Example
///
/// ```
/// let colors1 = Colors::new(Color::Red, Color::Black);
/// let color_pair1 = ColorPair::new(colors1)?;
/// let attributes = Attribute::Dim | color_pair1;
///
/// let asciistr = AsciiString::from_ascii_str("testing, testing... 1..2..3..")?;
/// let mut chstr = ChtypeString::from_ascii_str(ascii_str);
/// chstr = chstr | attributes;
///
/// waddchstr(window_handle, &chstr)?;
/// ```
pub fn waddchstr(handle: WINDOW, chstr: &ChtypeString) -> result!(()) {
    match ncurses::waddchstr(handle, raw_with_nul_as_slice!(chstr)) {
        ERR => Err(ncurses_function_error!("waddchstr")),
        _   => Ok(())
    }
}

/// Add a ascii string (as of ABI 6 if not before unicode strings are also supported) of a specified length to the current position on the specified window
///
/// # Example
///
/// ```
/// let string = "testing, testing... 1..2..3..";
///
/// waddnstr(window_handle, &string, 29)?;
/// ```
pub fn waddnstr(handle: WINDOW, str: &str, number: i32) -> result!(()) {
    match ncurses::waddnstr(handle, c_str_with_nul!(str), number) {
        ERR => Err(ncurses_function_error!("waddnstr")),
        _   => Ok(())
    }
}

/// Add a wide character string (unicode) of a specified length to the current position on the specified window
///
/// # Example
///
/// ```
/// let wide_string = WideString::from_str("testing, testing... 1..2..3..");
///
/// waddnwstr(window_handle, &string, 29)?;
/// ```
pub fn waddnwstr(handle: WINDOW, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::waddnwstr(handle, raw_with_nul_as_slice!(wstr), number) {
        ERR => Err(ncurses_function_error!("waddnwstr")),
        _   => Ok(())
    }
}

/// Add a ascii string (as of ABI 6 if not before unicode strings are also supported) to the current position on the specified window
///
/// # Example
///
/// ```
/// let string = "testing, testing... 1..2..3..";
///
/// waddstr(window_handle, origin, &string)?;
/// ```
pub fn waddstr(handle: WINDOW, str: &str) -> result!(()) {
    match ncurses::waddstr(handle, c_str_with_nul!(str)) {
        ERR => Err(ncurses_function_error!("waddstr")),
        _   => Ok(())
    }
}

/// Add a wide character string (unicode) to the current position on the specified window
///
/// # Example
///
/// ```
/// let wide_string = WideString::from_str("testing, testing... 1..2..3..");
///
/// waddwstr(window_handle, &string)?;
/// ```
pub fn waddwstr(handle: WINDOW, wstr: &WideString) -> result!(()) {
    match ncurses::waddwstr(handle, raw_with_nul_as_slice!(wstr)) {
        ERR => Err(ncurses_function_error!("waddwstr")),
        _   => Ok(())
    }
}

pub fn wattr_get(handle: WINDOW) -> result!(AttributesColorPairSet) {
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let opts: *mut libc::c_void = ptr::null_mut();

    match unsafe { ncurses::wattr_get(handle, attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts) } {
        ERR => Err(ncurses_function_error!("wattr_get")),
        _   => {
            Ok(if opts.is_null() {
                   AttributesColorPairSet::Normal(
                       normal::AttributesColorPair::new(
                           normal::Attributes::from(attrs[0]),
                           normal::ColorPair::from(color_pair[0])
                       )
                   )
               } else {
                   AttributesColorPairSet::Extend(
                       extend::AttributesColorPair::new(
                           extend::Attributes::from(attrs[0]),
                           extend::ColorPair::from(unsafe { slice::from_raw_parts(opts as *mut i32, 1)[0] })
                       )
                   )
               })
        }
    }
}

pub fn wattr_off<A, T>(handle: WINDOW, attrs: A) -> result!(()) where A: AttributesType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::wattr_off(handle, attrs.as_attr_t(), ptr::null_mut()) } {
        ERR => Err(ncurses_function_error!("wattr_off")),
        _   => Ok(())
    }
}

pub fn wattr_on<A, T>(handle: WINDOW, attrs: A) -> result!(()) where A: AttributesType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::wattr_on(handle, attrs.as_attr_t(), ptr::null_mut()) } {
        ERR => Err(ncurses_function_error!("wattr_on")),
        _   => Ok(())
    }
}

pub fn wattr_set<A, P, T>(handle: WINDOW, attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::wattr_set(handle, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("wattr_set")),
        _   => Ok(())
    }
}

pub fn wattroff(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match ncurses::wattroff(handle, normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("wattroff")),
        _   => Ok(())
    }
}

pub fn wattron(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match ncurses::wattron(handle, normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("wattron")),
        _   => Ok(())
    }
}

pub fn wattrset(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match ncurses::wattrset(handle, normal::Attributes::into(attrs)) {
        ERR => Err(ncurses_function_error!("wattrset")),
        _   => Ok(())
    }
}

pub fn wbkgd(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match ncurses::wbkgd(handle, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("wbkgd")),
        _   => Ok(())
    }
}

pub fn wbkgdset(handle: WINDOW, ch: ChtypeChar) {
    ncurses::wbkgdset(handle, ChtypeChar::into(ch))
}

pub fn wbkgrnd(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match ncurses::wbkgrnd(handle, &ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("wbkgrnd")),
        _   => Ok(())
    }
}

pub fn wbkgrndset(handle: WINDOW, wch: ComplexChar) {
    ncurses::wbkgrndset(handle, &ComplexChar::into(wch))
}

pub fn wborder(handle: WINDOW, ls: ChtypeChar, rs: ChtypeChar, ts: ChtypeChar, bs: ChtypeChar, tl: ChtypeChar, tr: ChtypeChar, bl: ChtypeChar, br: ChtypeChar) -> result!(()) {
    match ncurses::wborder(
        handle,
        ChtypeChar::into(ls),
        ChtypeChar::into(rs),
        ChtypeChar::into(ts),
        ChtypeChar::into(bs),
        ChtypeChar::into(tl),
        ChtypeChar::into(tr),
        ChtypeChar::into(bl),
        ChtypeChar::into(br)
    ) {
        ERR => Err(ncurses_function_error!("wborder")),
        _   => Ok(())
    }
}

pub fn wborder_set(handle: WINDOW, ls: ComplexChar, rs: ComplexChar, ts: ComplexChar, bs: ComplexChar, tl: ComplexChar, tr: ComplexChar, bl: ComplexChar, br: ComplexChar) -> result!(()) {
    match ncurses::wborder_set(
        handle,
        &ComplexChar::into(ls),
        &ComplexChar::into(rs),
        &ComplexChar::into(ts),
        &ComplexChar::into(bs),
        &ComplexChar::into(tl),
        &ComplexChar::into(tr),
        &ComplexChar::into(bl),
        &ComplexChar::into(br)
    ) {
        ERR => Err(ncurses_function_error!("wborder_set")),
        _   => Ok(())
    }
}

pub fn wchgat<A, P, T>(handle: WINDOW, number: i32, attrs: A, color_pair: P) -> result!(()) where A: AttributesType<T>, P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::wchgat(handle, number, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_const_ptr()) } {
        ERR => Err(ncurses_function_error!("wchgat")),
        _   => Ok(())
    }
}

basic_ncurses_function_with_window!(wclear, "wclear");

basic_ncurses_function_with_window!(wclrtobot, "wclrtobot");

basic_ncurses_function_with_window!(wclrtoeol, "wclrtoeol");

pub fn wcolor_set<P, T>(handle: WINDOW, color_pair: P) -> result!(()) where P: ColorPairType<T>, T: ColorAttributeTypes {
    match unsafe { ncurses::wcolor_set(handle, color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("wcolor_set")),
        _   => Ok(())
    }
}

pub fn wcursyncup(handle: WINDOW) {
    ncurses::wcursyncup(handle)
}

basic_ncurses_function_with_window!(wdelch, "wdelch");

pub fn wecho_wchar(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match ncurses::wecho_wchar(handle, &ComplexChar::into(wch)) {
        ERR => Err(ncurses_function_error!("wecho_wchar")),
        _   => Ok(())
    }
}

pub fn wechochar(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match ncurses::wechochar(handle, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("wechochar")),
        _   => Ok(())
    }
}

basic_ncurses_function_with_window!(werase, "werase");

pub fn wget_wch(handle: WINDOW) -> result!(WideCharResult) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::wget_wch(handle, wch.as_mut_ptr()) } {
        ERR          => Err(ncurses_function_error!("wget_wch")),
        KEY_CODE_YES => {
            if wch[0] == KEY_MOUSE as wint_t {
                Err(NCurseswError::KeyMouse)
            } else if wch[0] == KEY_RESIZE as wint_t {
                Err(NCurseswError::KeyReSize)
            } else if wch[0] == KEY_EVENT as wint_t {
                Err(NCurseswError::KeyEvent)
            } else {
                Ok(WideCharResult::Key(KeyBinding::from(wch[0])))
            }
        },
        _            => Ok(WideCharResult::Character(WideChar::from(wch[0])))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use wgetn_wstr() instead")]
pub fn wget_wstr(handle: WINDOW) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::wget_wstr(handle, ptr) } {
        ERR        => Err(ncurses_function_error!("wget_wstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
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

pub fn wgetbkgrnd(handle: WINDOW) -> result!(ComplexChar) {
    let mut wch: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::wgetbkgrnd(handle, wch.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("wgetbkgd")),
        _   => Ok(ComplexChar::from(wch[0]))
    }
}

pub fn wgetch(handle: WINDOW) -> result!(CharacterResult) {
    match ncurses::wgetch(handle) {
        ERR        => Err(ncurses_function_error!("wgetch")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        ch         => {
            if ch > i32::from(i8::max_value()) {
                Ok(CharacterResult::Key(KeyBinding::from(ch)))
            } else {
                Ok(CharacterResult::Character(char::from(ch as i8 as u8)))
            }
        }
    }
}

pub fn wgetdelay(handle: WINDOW) -> result!(time::Duration) {
    let delay = time::Duration::from_millis(u64::try_from(ncurses::wgetdelay(handle))?);

    Ok(delay)
}

pub fn wgetn_wstr(handle: WINDOW, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::wgetn_wstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::wgetn_wstr(handle, ptr, number) } {
        ERR        => Err(ncurses_function_error!("wgetn_wstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
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

pub fn wgetnstr(handle: WINDOW, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "ncursesw::wgetnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::wgetnstr(handle, ptr, number) } {
        ERR        => Err(ncurses_function_error!("wgetnstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
            assert!(!ptr.is_null(), "ncursesw::wgetnstr() : ptr.is_null()");

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn wgetparent(handle: WINDOW) -> Option<WINDOW> {
    ncurses::wgetparent(handle)
}

pub fn wgetscrreg(handle: WINDOW) -> result!(Region) {
    let mut top: [i32; 1] = [0];
    let mut bottom: [i32; 1] = [0];

    match unsafe { ncurses::wgetscrreg(handle, top.as_mut_ptr(), bottom.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("wgetscrreg")),
        _   => Ok(Region { top: top[0], bottom: bottom[0] })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use wgetnstr() instead")]
pub fn wgetstr(handle: WINDOW) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::wgetstr(handle, ptr) } {
        ERR        => Err(ncurses_function_error!("wgetstr")),
        KEY_MOUSE  => Err(NCurseswError::KeyMouse),
        KEY_RESIZE => Err(NCurseswError::KeyReSize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        _          => {
            assert!(!ptr.is_null(), "ncursesw::wgetstr() : ptr.is_null()");

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn whline(handle: WINDOW, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::whline(handle, ChtypeChar::into(ch), number) {
        ERR => Err(ncurses_function_error!("whline")),
        _   => Ok(())
    }
}

pub fn whline_set(handle: WINDOW, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::whline_set(handle, &ComplexChar::into(wch), number) {
        ERR => Err(ncurses_function_error!("whline_set")),
        _   => Ok(())
    }
}

/// Return a Complex character from the current cursor position from the specified window
///
/// # Example
///
/// ```
/// let cchar = win_wch(window_handle)?;
/// ```
pub fn win_wch(handle: WINDOW) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::win_wch(handle, wcval.as_mut_ptr()) } {
        ERR => Err(ncurses_function_error!("win_wch")),
        _   => Ok(ComplexChar::from(wcval[0]))
    }
}

/// Return a complex string of characters starting at the current cursor position for a specified number of characters from the specified window
///
/// # Example
///
/// ```
/// let chstring = win_wchnstr(window_handle, 10)?;
/// ```
pub fn win_wchnstr(handle: WINDOW, number: i32) -> result!(ComplexString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::win_wchnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::win_wchnstr(handle, ptr, number) } {
        ERR => Err(ncurses_function_error!("win_wchnstr")),
        _   => {
            assert!(!ptr.is_null(), "ncursesw::win_wchnstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, number as usize) }))
        }
    }
}

/// Return a Complex string of characters starting at the current cursor position from the specified window
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let chstring = win_wchstr(window_handle)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use win_wchnstr() instead")]
pub fn win_wchstr(handle: WINDOW) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::win_wchstr(handle, ptr) } {
        ERR => Err(ncurses_function_error!("win_wchstr")),
        _   => {
            assert!(!ptr.is_null(), "ncursesw::win_wchstr() : ptr.is_null()");

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        }
    }
}

/// Return a Chtype character from the current cursor position from the specified window
///
/// # Example
///
/// ```
/// let chchar = mvwinch(window_handle)?;
/// ```
pub fn winch(handle: WINDOW) -> ChtypeChar {
    ChtypeChar::from(ncurses::winch(handle))
}

/// Return a Chtype string of characters starting at the current cursor position for a specified number of characters from the specified window
///
/// # Example
///
/// ```
/// let chstring = winchnstr(window_handle, 10)?;
/// ```
pub fn winchnstr(handle: WINDOW, number: i32) -> result!(ChtypeString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::winchnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    match unsafe { ncurses::winchnstr(handle, ptr, number) } {
        ERR => Err(ncurses_function_error!("winchnstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::winchnstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::winchnstr() : len={} > {}", len, LINE_MAX);

            Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

/// Return a Chtype string of characters starting at the current cursor position from the specified window
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let chstring = winchstr(window_handle)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use winchnstr() instead")]
pub fn winchstr(handle: WINDOW) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    match unsafe { ncurses::winchstr(handle, ptr) } {
        ERR => Err(ncurses_function_error!("inchstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::winchstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::winchstr() : len={} > {}", len, LINE_MAX);

            Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

/// Return a ascii string of characters starting at the current cursor position for a specified number of characters from the specified window
///
/// # Example
///
/// ```
/// let string = winnstr(window_handle, 10)?;
/// ```
pub fn winnstr(handle: WINDOW, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "ncursesw::winnstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::winnstr(handle, ptr, number) } {
        ERR => Err(ncurses_function_error!("winnstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::winnstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::winnstr() : len={} > {}", len, LINE_MAX);

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

pub fn winnwstr(handle: WINDOW, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "ncursesw::winnwstr() : number={} > {}", number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::winnwstr(handle, ptr, number) } {
        ERR => Err(ncurses_function_error!("winnwstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::winnwstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::winnwstr() : len={} > {}", len, LINE_MAX);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

pub fn wins_nwstr(handle: WINDOW, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::wins_nwstr(handle, raw_with_nul_as_slice!(wstr), number) {
        ERR => Err(ncurses_function_error!("wins_nwstr")),
        _   => Ok(())
    }
}

pub fn wins_wch(handle: WINDOW, wch: &ComplexString) -> result!(()) {
    match ncurses::wins_wch(handle, raw_with_nul_as_slice!(wch)) {
        ERR => Err(ncurses_function_error!("wins_wch")),
        _   => Ok(())
    }
}

pub fn wins_wstr(handle: WINDOW, wstr: &WideString) -> result!(()) {
    match ncurses::wins_wstr(handle, raw_with_nul_as_slice!(wstr)) {
        ERR => Err(ncurses_function_error!("wins_wstr")),
        _   => Ok(())
    }
}

pub fn winsch(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match ncurses::winsch(handle, ChtypeChar::into(ch)) {
        ERR => Err(ncurses_function_error!("winsch")),
        _   => Ok(())
    }
}

pub fn winsdelln(handle: WINDOW, n: i32) -> result!(()) {
    match ncurses::winsdelln(handle, n) {
        ERR => Err(ncurses_function_error!("winsdelln")),
        _   => Ok(())
    }
}

basic_ncurses_function_with_window!(winsertln, "winsertln");

pub fn winsnstr(handle: WINDOW, str: &str, number: i32) -> result!(()) {
    match ncurses::winsnstr(handle, c_str_with_nul!(str), number) {
        ERR => Err(ncurses_function_error!("winsnstr")),
        _   => Ok(())
    }
}

pub fn winsstr(handle: WINDOW, str: &str) -> result!(()) {
    match ncurses::winsstr(handle, c_str_with_nul!(str)) {
        ERR => Err(ncurses_function_error!("winsstr")),
        _   => Ok(())
    }
}

/// Return an ascii string of characters starting at the current cursor position from the specified window
///
/// Warning : This function is inherently unsafe, the ncurses library may overwrite the maximum buffer size which will cause undefined behaviour
///
/// # Example
///
/// ```
/// let string = winstr(window_handle)?;
/// ```
#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use winnstr() instead")]
pub fn winstr(handle: WINDOW) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::winstr(handle, ptr) } {
        ERR => Err(ncurses_function_error!("winstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::winstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::winstr() : len={} > {}", len, LINE_MAX);

            Ok(unsafe { FromCStr::from_c_str(ptr) })
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause issues. Use winnwstr() instead")]
pub fn winwstr(handle: WINDOW) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::winwstr(handle, ptr) } {
        ERR => Err(ncurses_function_error!("winwstr")),
        len => {
            assert!(!ptr.is_null(), "ncursesw::winwstr() : ptr.is_null()");
            assert!(len <= LINE_MAX as i32, "ncursesw::winwstr() : len={} > {}", len, LINE_MAX);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, len as usize) }))
        }
    }
}

pub fn wmove(handle: WINDOW, origin: Origin) -> result!(()) {
    match ncurses::wmove(handle, origin.y, origin.x) {
        ERR => Err(ncurses_function_error!("wmove")),
        _   => Ok(())
    }
}

basic_ncurses_function_with_window!(wnoutrefresh, "wnoutrefresh");

pub fn wredrawln(handle: WINDOW, beg_line: i32, num_lines: i32) -> result!(()) {
    match ncurses::wredrawln(handle, beg_line, num_lines) {
        ERR => Err(ncurses_function_error!("wredrawln")),
        _   => Ok(())
    }
}

basic_ncurses_function_with_window!(wrefresh, "wrefresh");

pub fn wresize(handle: WINDOW, size: Size) -> result!(()) {
    match ncurses::wresize(handle, size.lines, size.columns) {
        ERR => Err(ncurses_function_error!("wresize")),
        _   => Ok(())
    }
}

pub fn wscrl(handle: WINDOW, n: i32) -> result!(()) {
    match ncurses::wscrl(handle, n) {
        ERR => Err(ncurses_function_error!("wscrl")),
        _   => Ok(())
    }
}

pub fn wsetscrreg(handle: WINDOW, region: Region) -> result!(()) {
    match ncurses::wsetscrreg(handle, region.top, region.bottom) {
        ERR => Err(ncurses_function_error!("wsetscrreg")),
        _   => Ok(())
    }
}

basic_ncurses_function_with_window!(wstandend, "wstandend");

basic_ncurses_function_with_window!(wstandout, "wstandout");

pub fn wsyncdown(handle: WINDOW) {
    ncurses::wsyncdown(handle)
}

pub fn wsyncup(handle: WINDOW) {
    ncurses::wsyncup(handle)
}

pub fn wtimeout(handle: WINDOW, ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    ncurses::wtimeout(handle, ms);

    Ok(())
}

pub fn wtouchln(handle: WINDOW, line: i32, n: i32, changed: Changed) -> result!(()) {
    match ncurses::wtouchln(handle, line, n, Changed::into(changed)) {
        ERR => Err(ncurses_function_error!("wtouchln")),
        _   => Ok(())
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
    match ncurses::wvline(handle, ChtypeChar::into(ch), number) {
        ERR => Err(ncurses_function_error!("wvline")),
        _   => Ok(())
    }
}

pub fn wvline_set(handle: WINDOW, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::wvline_set(handle, &ComplexChar::into(wch), number) {
        ERR => Err(ncurses_function_error!("wvline_set")),
        _   => Ok(())
    }
}

//
// sundry non-ncurses routines.
//

pub fn ncurses_version() -> Version {
    Version {
        major: u64::from(bindings::NCURSES_VERSION_MAJOR),
        minor: u64::from(bindings::NCURSES_VERSION_MINOR),
        patch: u64::from(bindings::NCURSES_VERSION_PATCH),
        pre:   vec!(),
        build: vec!()
    }
}

pub fn setlocale(lc: LcCategory, locale: &str) -> String {
    utils::setlocale(LcCategory::into(lc), locale)
}
