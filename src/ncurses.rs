/*
    src/ncurses.rs

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

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use libc::{c_void, EINTR};
use std::{
    convert::{TryFrom, TryInto}, char, ptr, slice, time, mem,
    ffi, path::Path, os::unix::io::AsRawFd, io::{Write, Read}
};

use crate::{
    constants::{
        ERR, OK, KEY_MIN, KEY_MAX, KEY_CODE_YES, KEY_RESIZE,
        KEY_EVENT, TRUE, FALSE
    },
    normal, extend,
    attributescolorpairset::*, changed::*, characterresult::*,
    chtypet::*, complex::*, cursortype::*, cstring::*, gen::*,
    keybinding::*, legacy::*, origin::*, orientation::*,
    justification::*, wide::*, ncursescolortype::*,
    ncurseswerror::*, region::*, size::*, softlabeltype::*,
    shims::{funcs, ncurses, bindings}
};

macro_rules! path_as_slice { ($name: ident) => { &*(path_as_vec($name)?.as_slice() as *const [u8] as *const [i8]) } }

static MODULE_PATH: &str = "ncursesw::ncurses::";

// The maximum buffer size used in a variety of functions.
const LINE_MAX: usize = 4096;

/// NCurses window raw pointer.
type WINDOW = ncurses::WINDOW;
/// NCurses screen raw pointer.
type SCREEN = ncurses::SCREEN;
/// Ripoff line callback function signature.
type RipoffInit = crate::shims::bindings::RipoffInit;

// Raw attribute type value.
type attr_t = ncurses::attr_t;
type cchar_t = ncurses::cchar_t;
type chtype = ncurses::chtype;
type short_t = ncurses::short_t;
type wchar_t = ncurses::wchar_t;
type wint_t = ncurses::wint_t;

/// Return the raw pointer to the current screen.
pub fn curscr() -> WINDOW {
    unsafe { ncurses::curscr() }
}

/// Return the raw pointer to the new screen.
pub fn newscr() -> WINDOW {
    unsafe { ncurses::newscr() }
}

/// Return the raw pointer to the standard screen.
pub fn stdscr() -> WINDOW {
    unsafe { ncurses::stdscr() }
}

pub fn ttytype() -> result!(String) {
    unsafe { ncurses::ttytype().ok_or(ncurses_function_error!("ttytype")) }
}

/// Return the number of colors available.
pub fn COLORS() -> i32 {
    ncurses::COLORS()
}

#[deprecated(since = "0.4.0", note = "use shims::ncurses::COLOR_PAIR() instead")]
/// Return the attribute value of a given `normal` color pair.
pub fn COLOR_PAIR(color_pair: i32) -> attr_t {
    ncurses::COLOR_PAIR(color_pair) as attr_t
}

#[deprecated(since = "0.4.0", note = "use normal::Attributes::color_pair() instead")]
/// Return the color pair number from  given `normal` attributes value.
pub fn PAIR_NUMBER(attrs: attr_t) -> short_t {
    ncurses::PAIR_NUMBER(attrs.try_into().unwrap()) as short_t
}

/// Return the number of color pairs available.
pub fn COLOR_PAIRS() -> i32 {
    ncurses::COLOR_PAIRS()
}

/// Return the number of columns (x-axis) available on the terminal.
pub fn COLS() -> i32 {
    ncurses::COLS()
}

/// Return the delay used to interpret termianl keyboard escape sequences.
pub fn ESCDELAY() -> result!(time::Duration) {
    Ok(time::Duration::from_millis(u64::try_from(ncurses::ESCDELAY())?))
}

/// Return the number of lines (y-axis) available on the terminal.
pub fn LINES() -> i32 {
    ncurses::LINES()
}

/// Return the number of columns a tab represents on the terminal.
pub fn TABSIZE() -> i32 {
    ncurses::TABSIZE()
}

/// Equivalent of `wadd_wch()` using `stdscr()` as window `handle`.
pub fn add_wch(wch: ComplexChar) -> result!(()) {
    match ncurses::add_wch(&ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("add_wch", rc))
    }
}

/// Equivalent of `wadd_wchnstr()` using `stdscr()` as window `handle`.
pub fn add_wchnstr(wchstr: &ComplexString, number: i32) -> result!(()) {
    match ncurses::add_wchnstr(raw_with_nul_as_slice!(wchstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("add_wchnstr", rc))
    }
}

/// Equivalent of `wadd_wchstr()` using `stdscr()` as window `handle`.
pub fn add_wchstr(wchstr: &ComplexString) -> result!(()) {
    match ncurses::add_wchstr(raw_with_nul_as_slice!(wchstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("add_wchstr", rc))
    }
}

/// Equivalent of `waddch()` using `stdscr()` as window `handle`.
pub fn addch(ch: ChtypeChar) -> result!(()) {
    match ncurses::addch(ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addch", rc))
    }
}

/// Equivalent of `waddchnstr()` using `stdscr()` as window `handle`.
pub fn addchnstr(chstr: &ChtypeString, number: i32) -> result!(()) {
    match ncurses::addchnstr(raw_with_nul_as_slice!(chstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addchnstr", rc))
    }
}

/// Equivalent of `waddchstr()` using `stdscr()` as window `handle`.
pub fn addchstr(chstr: &ChtypeString) -> result!(()) {
    match ncurses::addchstr(raw_with_nul_as_slice!(chstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addchstr", rc))
    }
}

/// Equivalent of `waddnstr()` using `stdscr()` as window `handle`.
pub fn addnstr(str: &str, number: i32) -> result!(()) {
    match ncurses::addnstr(unsafe { c_str_with_nul!(str) }, number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addnstr", rc)),
    }
}

/// Equivalent of `waddnwstr()` using `stdscr()` as window `handle`.
pub fn addnwstr(wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::addnwstr(raw_with_nul_as_slice!(wstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addnwstr", rc))
    }
}

/// Equivalent of `waddstr()` using `stdscr()` as window `handle`.
pub fn addstr(str: &str) -> result!(()) {
    match ncurses::addstr(unsafe { c_str_with_nul!(str) }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addstr", rc))
    }
}

/// Equivalent of `waddwstr()` using `stdscr()` as window `handle`.
pub fn addwstr(wstr: &WideString) -> result!(()) {
    match ncurses::addwstr(raw_with_nul_as_slice!(wstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("addwstr", rc))
    }
}

/// This defines which colors to paint for color pair 0. This function
/// recognizes a special color `Color::TerminalDefault`, which denotes
/// the default terminal color.
///
/// The following are equivalent:
/// ```text
/// use_default_colors()?;
/// assume_default_colors(Colors::new(Color::new(ColorPalette::default()), Color::new(ColorPalette::default())))?;
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

/// Equivalent of `wattr_get()` using `stdscr()` as window `handle`.
pub fn attr_get() -> result!(AttributesColorPairSet) {
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let mut opts: [i32; 1] = [0];

    match unsafe { ncurses::attr_get(attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts.as_mut_ptr() as *mut c_void) } {
        OK => Ok(match ncurses_colortype() {
            NCursesColorType::Normal => {
                AttributesColorPairSet::Normal(
                    normal::AttributesColorPair::new(
                        normal::Attributes::_from(None, attrs[0]),
                        normal::ColorPair::_from(None, color_pair[0])
                    )
                )
            },
            NCursesColorType::Extend => {
                AttributesColorPairSet::Extend(
                    extend::AttributesColorPair::new(
                        extend::Attributes::_from(None, attrs[0]),
                        extend::ColorPair::_from(None, opts[0])
                    )
                )
            }
        }),
        rc => Err(ncurses_function_error_with_rc!("attr_get", rc))
    }
}

/// Equivalent of `wattr_off()` using `stdscr()` as window `handle`.
pub fn attr_off<A, T>(attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::attr_off(attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attr_off", rc))
    }
}

/// Equivalent of `wattr_on()` using `stdscr()` as window `handle`.
pub fn attr_on<A, T>(attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::attr_on(attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attr_on", rc))
    }
}

/// Equivalent of `wattr_set()` using `stdscr()` as window `handle`.
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

/// Equivalent of `wattroff()` using `stdscr()` as window `handle`.
pub fn attroff(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attroff(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attroff", rc))
    }
}

/// Equivalent of `wattron()` using `stdscr()` as window `handle`.
pub fn attron(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attron(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attron", rc))
    }
}

/// Equivalent of `wattron()` using `stdscr()` as window `handle`.
pub fn attrset(attrs: normal::Attributes) -> result!(()) {
    match ncurses::attrset(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("attrset", rc))
    }
}

/// Return the output speed of the terminal in bits per second.
/// On software terminal emulators it will have a fixed high value.
/// Included for historical reasons; in former times, it was used
/// to write output loops for time delays and occasionally to
/// change interfaces depending on the line speed.
pub fn baudrate() -> i32 {
    ncurses::baudrate()
}

/// Emit a short attention sound.
pub fn beep() -> result!(()) {
    match ncurses::beep() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("beep", rc))
    }
}

/// Set the background property of the window to the character `ch`.
/// The change is then applied to every character position in that window:
///
/// - The attribute of every character in the window is changed to the new background attribute.
/// - Wherever the former background character appears, it is changed to the new background character.
pub fn bkgd(ch: ChtypeChar) -> result!(()) {
    match ncurses::bkgd(ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("bkgd", rc))
    }
}

/// Set the window’s background. A window’s background consists of a character
/// and it's combination of attributes. The attribute part of the background is
/// combined (OR’ed) with all non-blank characters that are written into the
/// window. Both the character and attribute parts of the background are combined
/// with the blank characters. The background becomes a property of the character
/// and moves with the character through any scrolling and
/// insert/delete line/character operations.
pub fn bkgdset(ch: ChtypeChar) {
    ncurses::bkgdset(ChtypeChar::into(ch))
}

/// Equivalent of `wbkgrnd()` using `stdscr()` as window `handle`.
pub fn bkgrnd(wch: ComplexChar) -> result!(()) {
    match ncurses::bkgrnd(&ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("bkgrnd", rc))
    }
}

/// Equivalent of `wbkgrndset()` using `stdscr()` as window `handle`.
pub fn bkgrndset(wch: ComplexChar) {
    ncurses::bkgrndset(&ComplexChar::into(wch))
}

/// Equivalent of `wborder()` using `stdscr()` as window `handle`.
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

/// Equivalent of `wborder_set()` using `stdscr()` as window `handle`.
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

/// Similar to `border()`, but both ls and rs are vertch and both ts and
/// bs are horch. The default corner characters are always used by this function.
pub fn r#box(handle: WINDOW, verch: ChtypeChar, horch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::r#box(handle, ChtypeChar::into(verch), ChtypeChar::into(horch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("box", rc))
    }
}

/// Similar to `border_set()`, but both ls and rs are vertch and both ts and
/// bs are horch. The default corner characters are always used by this function.
pub fn box_set(handle: WINDOW, verch: ComplexChar, horch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::box_set(handle, &ComplexChar::into(verch), &ComplexChar::into(horch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("box_set", rc))
    }
}

/// Return `true` or `false`, depending on whether the programmer
/// can change the colors displayed by the terminal.
pub fn can_change_color() -> bool {
    ncurses::can_change_color()
}

/// Enter cbreak mode. In cbreak mode (sometimes called “rare” mode)
/// normal tty line buffering is turned off and characters are available
/// to be read one by one. However, unlike raw mode, special characters
/// (interrupt, quit, suspend, and flow control) retain their effects
/// on the tty driver and calling program. Calling first raw() then
/// cbreak() leaves the terminal in cbreak mode.
pub fn cbreak() -> result!(()) {
    match ncurses::cbreak() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("cbreak", rc))
    }
}

/// Equivalent of `wchgat()` using `stdscr()` as window `handle`.
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

/// Equivalent of `wclear()` using `stdscr()` as window `handle`.
pub fn clear() -> result!(()) {
    match ncurses::clear() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("clear", rc))
    }
}

/// If flag is `true`, the next call to `refresh()` will clear the window completely.
pub fn clearok(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::clearok(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("clearok", rc))
    }
}

/// Equivalent of `wclrtobot()` using `stdscr()` as window `handle`.
pub fn clrtobot() -> result!(()) {
    match ncurses::clrtobot() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("clrtobot", rc))
    }
}

/// Equivalent of `wclrtoeol()` using `stdscr()` as window `handle`.
pub fn clrtoeol() -> result!(()) {
    match ncurses::clrtoeol() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("clrtoeol", rc))
    }
}

#[deprecated(since = "0.4.0", note = "Use normal::Color::rgb() or shims::ncurses::color_content() instead")]
/// Return the intensity of the red, green, and blue (RGB) components in
/// the color, which must be between 0 and COLORS. Return a structure,
/// containing the R,G,B values for the given color, which will be
/// between 0 (no component) and 1000 (maximum amount of component).
pub fn color_content(color_number: short_t) -> result!(normal::RGB) {
    let mut r: [short_t; 1] = [0];
    let mut g: [short_t; 1] = [0];
    let mut b: [short_t; 1] = [0];

    match unsafe { ncurses::color_content(color_number, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()) } {
        OK => Ok(normal::RGB::new(r[0], g[0], b[0])),
        rc => Err(ncurses_function_error_with_rc!("color_content", rc))
    }
}

/// Equivalent of `wcolor_set()` using `stdscr()` as window `handle`.
pub fn color_set<P, T>(color_pair: P) -> result!(())
    where P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::color_set(color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("color_set", rc))
    }
}

/// The `copywin()` routine provides a finer granularity of control over
/// the `overlay()` and `overwrite()` routines. As in the `prefresh()`
/// routine, a rectangle is specified in the destination  window, (`dmin`)
/// and (`dmax`), and the upper-left-corner coordinates of the source window,
/// (`smin`). If the argument `overlay` is `true`, then copying is
/// non-destructive, as in `overlay()`.
pub fn copywin(
    src_handle: WINDOW,
    dst_handle: WINDOW,
    smin:       Origin,
    dmin:       Origin,
    dmax:       Origin,
    overlay:    bool) -> result!(())
{
    let olay = if overlay {
        TRUE
    } else {
        FALSE
    };

    match unsafe { ncurses::copywin(src_handle, dst_handle, smin.y, smin.x, dmin.y, dmin.x, dmax.y, dmax.x, olay) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("copywin", rc))
    }
}

/// Set the cursor state. visibility can be set to invisible, normal,
/// or very visible. If the terminal supports the visibility requested,
/// return the previous cursor state; otherwise raise an exception.
/// On many terminals, the “visible” mode is an underline cursor
/// and the “very visible” mode is a block cursor.
pub fn curs_set(cursor: CursorType) -> result!(CursorType) {
    let rc = ncurses::curs_set(cursor.value());

    CursorType::new(rc).ok_or(ncurses_function_error_with_rc!("curs_set", rc))
}

/// Return the version number, including patch level of the underlying
/// library, e.g., 6.1.20180127.
pub fn curses_version() -> result!(String) {
    ncurses::curses_version().ok_or(ncurses_function_error!("curses_version"))
}

/// Save the current terminal mode as the “program” mode, the mode when
/// the running program is using NCurses. (Its counterpart is the “shell”
/// mode, for when the program is not in NCurses.) Subsequent calls to
/// `reset_prog_mode()` will restore this mode.
pub fn def_prog_mode() -> result!(()) {
    match ncurses::def_prog_mode() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("def_prog_mode", rc))
    }
}

/// Save the current terminal mode as the “shell” mode, the mode when
/// the running program is not using NCurses. (Its counterpart is the
/// “program” mode, when the program is using NCurses capabilities.)
/// Subsequent calls to reset_shell_mode() will restore this mode.
pub fn def_shell_mode() -> result!(()) {
    match ncurses::def_shell_mode() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("def_shell_mode", rc))
    }
}

/// Permits an application to define keycodes with their corresponding
/// control strings, so that the NCurses library will interpret them
/// just as it would the predefined codes in the terminfo database.
///
/// If the given `definition` is `None`, any existing definition for
/// the keycode is removed. Similarly, if the given `KeyBinding::Unknown`
/// is negative or zero, any existing string for the given definition
/// is removed.
pub fn define_key(definition: Option<&str>, keycode: KeyBinding) -> result!(()) {
    match unsafe { ncurses::define_key(option_str_as_ptr!(definition), KeyBinding::into(keycode)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("define_key", rc))
    }
}

/// Insert an ms millisecond pause in output.
pub fn delay_output(ms: time::Duration) -> result!(()) {
    match ncurses::delay_output(i32::try_from(ms.as_millis())?) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("delay_output", rc))
    }
}

/// Equivalent of `wdelch()` using `stdscr()` as window `handle`.
pub fn delch() -> result!(()) {
    match ncurses::delch() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("delch", rc))
    }
}

/// Delete the line under the cursor. All following lines are moved up by one line.
pub fn deleteln() -> result!(()) {
    match ncurses::deleteln() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("deleteln", rc))
    }
}

/// The `delscreen()` routine frees storage associated with the `SCREEN`
/// data structure. The `endwin()` routine does not do this, so `delscreen()` should
/// be called after `endwin()` if a particular `SCREEN` is no longer needed.
pub fn delscreen(screen: SCREEN) {
    unsafe { ncurses::delscreen(screen) }
}

/// Deletes the named window, freeing all memory associated with it
/// (it does not actually erase the window's screen image). Sub-windows
/// must be deleted before the main window can be deleted.
pub fn delwin(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::delwin(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("delwin", rc))
    }
}

/// An abbreviation for “derive window”, `derwin()` is the same as calling `subwin()`,
/// except that `origin` are relative to the origin of the window, rather than
/// relative to the entire screen. Return a window object for the derived window.
pub fn derwin(orig: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    unsafe { ncurses::derwin(orig, size.lines, size.columns, origin.y, origin.x).ok_or(ncurses_function_error!("derwin")) }
}

/// Update the physical screen. The NCurses library keeps two data structures,
/// one representing the current physical screen contents and a virtual screen
/// representing the desired next state. The doupdate() ground updates the
/// physical screen to match the virtual screen.
///
/// The virtual screen may be updated by a `noutrefresh()` call after write
/// operations such as `addstr()` have been performed on a window. The normal
/// `refresh()` call is simply `noutrefresh()` followed by `doupdate()`; if
/// you have to update multiple windows, you can speed performance and perhaps
/// reduce screen flicker by issuing `noutrefresh()` calls on all windows,
/// followed by a single `doupdate()`.
pub fn doupdate() -> result!(()) {
    match ncurses::doupdate() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("doupdate", rc))
    }
}

/// Creates an exact duplicate of the window `handle`.
pub fn dupwin(handle: WINDOW) -> result!(WINDOW) {
    unsafe { ncurses::dupwin(handle).ok_or(ncurses_function_error!("dupwin")) }
}

/// Enter echo mode. In echo mode, each character input is echoed to the screen
/// as it is entered.
pub fn echo() -> result!(()) {
    match ncurses::echo() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("echo", rc))
    }
}

/// Equivalent of `wecho_wchar()` using `stdscr()` as window `handle`.
pub fn echo_wchar(wch: ComplexChar) -> result!(()) {
    match ncurses::echo_wchar(&ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("echo_wchar", rc))
    }
}

/// Equivalent of `wechochar()` using `stdscr()` as window `handle`.
pub fn echochar(ch: ChtypeChar) -> result!(()) {
    match ncurses::echochar(ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("echochar", rc))
    }
}

/// De-initialize the library, and return terminal to normal status.
pub fn endwin() -> result!(()) {
    match ncurses::endwin() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("endwin", rc))
    }
}

/// Equivalent of `werase()` using `stdscr()` as window `handle`.
pub fn erase() -> result!(()) {
    match ncurses::erase() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("erase", rc))
    }
}

/// Return the user’s current erase character as a one-byte bytes object.
/// Under Unix operating systems this is a property of the controlling
/// tty of the NCurses program, and is not set by the NCurses library itself.
pub fn erasechar() -> result!(char) {
    let rc = ncurses::erasechar();

    if rc.is_negative() {
        Err(ncurses_function_error_with_rc!("erasechar", i32::from(rc)))
    } else {
        Ok(char::from(u8::try_from(rc)?))
    }
}

/// Returns the current erase character as a wide character.
pub fn erasewchar() -> result!(WideChar) {
    let mut wch: [wchar_t; 1] = [0];

    match unsafe { ncurses::erasewchar(wch.as_mut_ptr()) } {
        OK => Ok(WideChar::from(wch[0])),
        rc => Err(ncurses_function_error_with_rc!("erasewchar", rc))
    }
}

#[deprecated(since = "0.4.0", note = "Use extend::Color::rgb() or shims::ncurses::extended_color_content() instead")]
/// The extended color version of the `color_content()` routine.
pub fn extended_color_content(color_number: i32) -> result!(extend::RGB) {
    let mut r: [i32; 1] = [0];
    let mut g: [i32; 1] = [0];
    let mut b: [i32; 1] = [0];

    match unsafe { ncurses::extended_color_content(color_number, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()) } {
        OK => Ok(extend::RGB::new(r[0], g[0], b[0])),
        rc => Err(ncurses_function_error_with_rc!("extended_color_content", rc))
    }
}

#[deprecated(since = "0.4.0", note = "Use extend::ColorPair::colors() or shims::ncurses::extended_pair_content() instead")]
/// The extended color version of the `pair_content()` routine.
pub fn extended_pair_content(color_pair: i32) -> result!(extend::Colors) {
    let mut fg: [i32; 1] = [0];
    let mut bg: [i32; 1] = [0];

    let color_palette = |color_number: i32| extend::ColorPalette::_from(color_number);

    match unsafe { ncurses::extended_pair_content(color_pair, fg.as_mut_ptr(), bg.as_mut_ptr()) } {
        OK => Ok(extend::Colors::new(extend::Color::_from(None, color_palette(fg[0])), extend::Color::_from(None, color_palette(bg[0])))),
        rc => Err(ncurses_function_error_with_rc!("extended_pair_content", rc))
    }
}

/// The extended color version of the `slk_color()` routine.
pub fn extended_slk_color(color_pair: extend::ColorPair) -> result!(()) {
    match ncurses::extended_slk_color(color_pair.number()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("extended_slk_color", rc))
    }
}

/// The `filter()` routine, if used, must be called before `initscr()` is
/// called. The effect is that, during those calls, `LINES` is set to 1;
/// the capabilities clear, cup, cud, cud1, cuu1, cuu, vpa are disabled;
/// and the home string is set to the value of cr. The effect is that the
/// cursor is confined to the current line, and so are screen updates.
/// This may be used for enabling character-at-a-time line editing without
/// touching the rest of the screen.
pub fn filter() {
    ncurses::filter()
}

/// Flash the screen. That is, change it to reverse-video and then change
/// it back in a short interval. Some people prefer such as ‘visible bell’
/// to the audible attention signal produced by `beep()`.
pub fn flash() -> result!(()) {
    match ncurses::flash() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("flash", rc))
    }
}

/// Flush all input buffers. This throws away any typeahead that has been
/// typed by the user and has not yet been processed by the program.
pub fn flushinp() -> result!(()) {
    match ncurses::flushinp() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("flushinp", rc))
    }
}

#[deprecated(since = "0.1.3", note = "use with caution as the specified color_pair must go out of scope before reuse of it's color pair number otherwise the color pair will default to terminal default foreground and backgound colors.")]
/// Marks the given color pair as unused, i.e., like color pair 0.
pub fn free_pair<P, T>(color_pair: P) -> result!(())
    where P:   ColorPairType<T>,
          i32: From<T>,
          T:   ColorAttributeTypes
{
    match ncurses::free_pair(i32::from(color_pair.number())) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("free_pair", rc))
    }
}

/// Returns the escape delay.
pub fn get_escdelay() -> result!(time::Duration) {
    Ok(time::Duration::from_millis(u64::try_from(ncurses::get_escdelay())?))
}

/// Equivalent of `wget_wch()` using `stdscr()` as window `handle`.
pub fn get_wch() -> result!(CharacterResult<WideChar>) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::get_wch(wch.as_mut_ptr()) } {
        EINTR        => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE   => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT    => Err(NCurseswError::KeyEvent),
        KEY_CODE_YES => {
            match i32::try_from(wch[0])? {
                #[cfg(feature = "key_resize_as_error")]
                KEY_RESIZE => Err(NCurseswError::KeyResize),
                #[cfg(feature = "key_event_as_error")]
                KEY_EVENT  => Err(NCurseswError::KeyEvent),
                _          => Ok(CharacterResult::Key(KeyBinding::try_from(wch[0])?))
            }
        },
        rc           => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("get_wch", rc))
            } else {
                Ok(CharacterResult::Character(WideChar::from(wch[0])))
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use getn_wstr() instead")]
/// Equivalent of `wget_wstr()` using `stdscr()` as window `handle`.
pub fn get_wstr() -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::get_wstr(ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("get_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}get_wstr() : ptr.is_null()", MODULE_PATH);

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

/// Returns the same attribute data as `wattr_get()`. However, `getattrs()`
/// internally returns an integer (actually a chtype), while `wattr_get()`
/// returns the current color pair in a separate parameter. In the
/// wide-character library configuration, color pairs may not fit into
/// a chtype, so `wattr_get()` is the only way to obtain the color information.
pub fn getattrs(handle: WINDOW) -> normal::Attributes {
    normal::Attributes::_from(None, unsafe { ncurses::getattrs(handle) as attr_t })
}

/// Return a `x` of co-ordinates of upper-left corner.
pub fn getbegx(handle: WINDOW) -> result!(i32) {
    let x = unsafe { ncurses::getbegx(handle) };

    if x.is_negative() {
        Err(ncurses_function_error_with_rc!("getbegx", x))
    } else {
        Ok(x)
    }
}

/// Return a `y` of co-ordinates of upper-left corner.
pub fn getbegy(handle: WINDOW) -> result!(i32) {
    let y = unsafe { ncurses::getbegy(handle) };

    if y.is_negative() {
        Err(ncurses_function_error_with_rc!("getbegy", y))
    } else {
        Ok(y)
    }
}

/// Return a `origin` of co-ordinates of upper-left corner.
pub fn getbegyx(handle: WINDOW) -> result!(Origin) {
    let y = unsafe { ncurses::getbegy(handle) };
    let x = unsafe { ncurses::getbegx(handle) };

    if y.is_negative() {
        Err(ncurses_function_error_with_rc!("getbegyx (y)", y))
    } else if x.is_negative() {
        Err(ncurses_function_error_with_rc!("getbegyx (x)", x))
    } else {
        Ok(Origin { y, x })
    }
}

/// Return the given window’s current background character (with rendition).
pub fn getbkgd(handle: WINDOW) -> ChtypeChar {
    ChtypeChar::from(unsafe { ncurses::getbkgd(handle) })
}

/// Equivalent of `wgetbkgrnd()` using `stdscr()` as window `handle`.
pub fn getbkgrnd() -> result!(ComplexChar) {
    let mut wch: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::getbkgrnd(wch.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wch[0])),
        rc => Err(ncurses_function_error_with_rc!("getbkgrnd", rc))
    }
}

/// Get a widecharacter string and rendition from a complex character.
pub fn getcchar(wcval: ComplexChar) -> result!(WideCharAndAttributes) {
    let mut wch: [wchar_t; bindings::CCHARW_MAX as usize] = [0; bindings::CCHARW_MAX as usize];
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let opts: *mut i32 = ptr::null_mut();

    let attribute_colorpair_set = |attrs: attr_t, color_pair: short_t, ext_color_pair: i32| -> AttributesColorPairSet {
        match ncurses_colortype() {
            NCursesColorType::Normal   => {
                AttributesColorPairSet::Normal(
                    normal::AttributesColorPair::new(
                        normal::Attributes::_from(None, attrs),
                        normal::ColorPair::_from(None, color_pair)
                    )
                )
            },
            NCursesColorType::Extend => {
                AttributesColorPairSet::Extend(
                    extend::AttributesColorPair::new(
                        extend::Attributes::_from(None, attrs),
                        extend::ColorPair::_from(None, ext_color_pair)
                    )
                )
            }
        }
    };

    match unsafe { ncurses::getcchar(&ComplexChar::into(wcval), wch.as_mut_ptr(), attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts) } {
        OK => {
            // TODO : get opts working correct so not to rely on bodge!
            //assert!(!opts.is_null(), "{}getcchar() : opts.is_null()", MODULE_PATH);
            //
            //Ok(WideCharAndAttributes::new(WideChar::from(wch[0]), attribute_colorpair_set(attrs[0], color_pair[0], unsafe { ptr::read(opts) })))

            let c: cchar_t = ComplexChar::into(wcval); // bodge to get extended color pair.

            Ok(WideCharAndAttributes::new(WideChar::from(wch[0]), attribute_colorpair_set(attrs[0], color_pair[0], c.ext_color)))
        },
        rc => Err(ncurses_function_error_with_rc!("getcchar", rc))
    }
}

/// Equivalent of `wgetch()` using `stdscr()` as window `handle`.
pub fn getch() -> result!(CharacterResult<char>) {
    match ncurses::getch() {
        EINTR      => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("getch", rc))
            } else if rc >= KEY_MIN && rc <= KEY_MAX {
                Ok(CharacterResult::Key(KeyBinding::from(rc)))
            } else {
                Ok(CharacterResult::Character(char::from(u8::try_from(i8::try_from(rc)?)?)))
            }
        }
    }
}

/// Return the `x` coordinate of the current cursor position.
pub fn getcurx(handle: WINDOW) -> result!(i32) {
    let x = unsafe { ncurses::getcurx(handle) };

    if x.is_negative() {
        Err(ncurses_function_error_with_rc!("getcurx", x))
    } else {
        Ok(x)
    }
}

/// Return the `y` coordinate of the current cursor position.
pub fn getcury(handle: WINDOW) -> result!(i32) {
    let y = unsafe { ncurses::getcury(handle) };

    if y.is_negative() {
        Err(ncurses_function_error_with_rc!("getcury", y))
    } else {
        Ok(y)
    }
}

/// Return the `origin` coordinates of the current cursor position.
pub fn getcuryx(handle: WINDOW) -> result!(Origin) {
    Ok(Origin { y: getcury(handle)?, x: getcurx(handle)? })
}

/// Return the height of the window.
pub fn getmaxx(handle: WINDOW) -> result!(i32) {
    let x = unsafe { ncurses::getmaxx(handle) };

    if x.is_negative() {
        Err(ncurses_function_error_with_rc!("getmaxx", x))
    } else {
        Ok(x)
    }
}

/// Return the width of the window.
pub fn getmaxy(handle: WINDOW) -> result!(i32) {
    let y = unsafe { ncurses::getmaxy(handle) };

    if y.is_negative() {
        Err(ncurses_function_error_with_rc!("getmaxy", y))
    } else {
        Ok(y)
    }
}

/// Return the height and width of the window.
pub fn getmaxyx(handle: WINDOW) -> result!(Size) {
    Ok(Size { lines: getmaxy(handle)?, columns: getmaxx(handle)? })
}

/// Equivalent of `wgetn_wstr()` using `stdscr()` as window `handle`.
pub fn getn_wstr(number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "{}getn_wstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::getn_wstr(ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("getn_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}getn_wstr() : ptr.is_null()", MODULE_PATH);

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

/// Equivalent of `wgetnstr()` using `stdscr()` as window `handle`.
pub fn getnstr(number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "{}getnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::getnstr(ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("getnstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}getnstr() : ptr.is_null()", MODULE_PATH);

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

/// Return the `x` coordinate of this window relative to its parent window.
pub fn getparx(handle: WINDOW) -> result!(i32) {
    let x = unsafe { ncurses::getparx(handle) };

    if x.is_negative() {
        Err(ncurses_function_error_with_rc!("getparx", x))
    } else {
        Ok(x)
    }
}

/// Return the `y` coordinate of this window relative to its parent window.
pub fn getpary(handle: WINDOW) -> result!(i32) {
    let y = unsafe { ncurses::getpary(handle) };

    if y.is_negative() {
        Err(ncurses_function_error_with_rc!("getpary", y))
    } else {
        Ok(y)
    }
}

/// Return the beginning coordinates of this window relative to its parent window.
pub fn getparyx(handle: WINDOW) -> result!(Origin) {
    Ok(Origin { y: getpary(handle)?, x: getparx(handle)? })
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use getnstr() instead")]
/// Equivalent of `wgetstr()` using `stdscr()` as window `handle`.
pub fn getstr() -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::getstr(ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("getstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}getstr() : ptr.is_null()", MODULE_PATH);

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

/// Return the current coordinates of the virtual screen cursor.
/// If `leaveok()` is currently `true`, then return `None`.
pub fn getsyx() -> result!(Option<Origin>) {
    if is_leaveok(newscr()) {
        Ok(None)
    } else {
        Ok(Some(getcuryx(newscr())?))
    }
}

/// Read window related data stored in the file by an earlier `putwin()` call.
/// The routine then creates and initializes a new window using that data,
/// returning the new window object.
pub fn getwin<I: AsRawFd + Read>(file: &I) -> result!(WINDOW) {
    unsafe { ncurses::getwin(fdopen(file, "r")?).ok_or(ncurses_function_error!("getwin")) }
}

/// Used for half-delay mode, which is similar to cbreak mode in that characters
/// typed by the user are immediately available to the program. However, after
/// blocking for tenths tenths of seconds, raise an exception if nothing has
/// been typed. The value of tenths must be a number between 1 and 255.
/// Use `nocbreak()` to leave half-delay mode.
pub fn halfdelay(tenths: time::Duration) -> result!(()) {
    match ncurses::halfdelay(i32::try_from(tenths.as_secs())? / 10) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("halfdelay", rc))
    }
}

/// Return `true` if the terminal can display colors; otherwise, return `false`.
pub fn has_colors() -> bool {
    ncurses::has_colors()
}

/// Return `true` if the terminal has insert- and delete-character capabilities.
/// This function is included for historical reasons only, as all modern
/// software terminal emulators have such capabilities.
pub fn has_ic() -> bool {
    ncurses::has_ic()
}

/// Return `true` if the terminal has insert- and delete-line capabilities,
/// or can simulate them using scrolling regions. This function is included
/// for historical reasons only, as all modern software terminal emulators
/// have such capabilities.
pub fn has_il() -> bool {
    ncurses::has_il()
}

/// Take a key value `ch`, and return `true` if the current terminal type
/// recognizes a key with that value.
pub fn has_key(ch: KeyBinding) -> bool {
    ncurses::has_key(KeyBinding::into(ch)) == TRUE
}

/// Equivalent of `whline()` using `stdscr()` as window `handle`.
pub fn hline(ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::hline(ChtypeChar::into(ch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("hline", rc))
    }
}

/// Equivalent of `whline_set()` using `stdscr()` as window `handle`.
pub fn hline_set(wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::hline_set(&ComplexChar::into(wch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("hline_set", rc))
    }
}

/// If flag is `false`, NCurses no longer considers using the hardware
/// insert/delete character feature of the terminal; if flag is `true`, use
/// of character insertion and deletion is enabled. When NCurses is first
/// initialized, use of character insert/delete is enabled by default.
pub fn idcok(handle: WINDOW, flag: bool) {
    unsafe { ncurses::idcok(handle, flag) }
}

/// If flag is `true`, NCurses will try and use hardware line editing facilities.
/// Otherwise, line insertion/deletion are disabled.
pub fn idlok(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::idlok(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("idlok", rc))
    }
}

/// If flag is `true`, any change in the window image automatically causes the
/// window to be refreshed; you no longer have to call `refresh()` yourself.
/// However, it may degrade performance considerably, due to repeated calls
/// to wrefresh. This option is disabled by default.
pub fn immedok(handle: WINDOW, flag: bool) {
    unsafe { ncurses::immedok(handle, flag) }
}

/// Equivalent of `win_wch()` using `stdscr()` as window `handle`.
pub fn in_wch() -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::in_wch(wcval.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wcval[0])),
        rc => Err(ncurses_function_error_with_rc!("in_wch", rc))
    }
}

/// Equivalent of `win_wchnstr()` using `stdscr()` as window `handle`.
pub fn in_wchnstr(number: i32) -> result!(ComplexString) {
    assert!(number <= LINE_MAX as i32, "{}in_wchnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::in_wchnstr(ptr, number) } {
        OK => {
            assert!(!ptr.is_null(), "{}in_wchnstr() : ptr.is_null()", MODULE_PATH);

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(number)?) }))
        },
        rc => Err(ncurses_function_error_with_rc!("in_wchnstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use in_wchnstr() instead")]
/// Equivalent of `win_wchstr()` using `stdscr()` as window `handle`.
pub fn in_wchstr() -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::in_wchstr(ptr) } {
        OK => {
            assert!(!ptr.is_null(), "{}in_wchstr() : ptr.is_null()", MODULE_PATH);

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("in_wchstr", rc))
    }
}

/// Equivalent of `winch()` using `stdscr()` as window `handle`.
pub fn inch() -> ChtypeChar {
    ChtypeChar::from(ncurses::inch())
}

/// Equivalent of `winchnstr()` using `stdscr()` as window `handle`.
pub fn inchnstr(number: i32) -> result!(ChtypeString) {
    assert!(number <= LINE_MAX as i32, "{}inchnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::inchnstr(ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("inchnstr", len))
    } else {
        assert!(!ptr.is_null(), "{}inchnstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}inchnstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use inchnstr() instead")]
/// Equivalent of `winchstr()` using `stdscr()` as window `handle`.
pub fn inchstr() -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::inchstr(ptr) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("inchstr", len))
    } else {
        assert!(!ptr.is_null(), "{}inchstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}inchstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

#[deprecated(since = "0.4.0", note = "Use normal::Color::set_rgb() or shims::ncurses::init_color() instead")]
/// Change the definition of a color, taking the number of the color to be
/// changed followed by three RGB values (for the amounts of red, green,
/// and blue components). The value of color_number must be between 0 and
/// COLORS. Each of r, g, b, must be a value between 0 and 1000. When
/// `init_color()` is used, all occurrences of that color on the screen
/// immediately change to the new definition. This function is a no-op on
/// most terminals; it is active only if `can_change_color()` returns `true`.
pub fn init_color(color_number: short_t, rgb: normal::RGB) -> result!(()) {
    if i32::from(color_number) >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match ncurses::init_color(color_number, rgb.red(), rgb.green(), rgb.blue()) {
            OK => {
                set_ncurses_colortype(NCursesColorType::Normal);

                Ok(())
            },
            rc => Err(ncurses_function_error_with_rc!("init_color", rc))
        }
    }
}

#[deprecated(since = "0.4.0", note = "Use extend::Color::set_rgb() or shims::ncurses::init_extended_color() instead")]
/// The extended color version of the `init_color()` routine.
pub fn init_extended_color(color_number: i32, rgb: extend::RGB) -> result!(()) {
    if color_number >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match ncurses::init_extended_color(color_number, rgb.red(), rgb.green(), rgb.blue()) {
            OK => {
                set_ncurses_colortype(NCursesColorType::Extend);

                Ok(())
            },
            rc => Err(ncurses_function_error_with_rc!("init_extended_color", rc))
        }
    }
}

#[deprecated(since = "0.4.0", note = "Use extend::ColorPair::new() or shims::ncurses::init_extended_pair() instead")]
/// The extended color version of the `init_pair()` routine.
pub fn init_extended_pair(color_pair: i32, colors: extend::Colors) -> result!(extend::ColorPair) {
    if color_pair >= COLOR_PAIRS() {
        Err(NCurseswError::ColorPairLimit)
    } else if colors.foreground().number() >= COLORS() || colors.background().number() >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match ncurses::init_extended_pair(color_pair, colors.foreground().number(), colors.background().number()) {
            OK => Ok(extend::ColorPair::_from(None, color_pair)),
            rc => Err(ncurses_function_error_with_rc!("init_extended_pair", rc))
        }
    }
}

#[deprecated(since = "0.4.0", note = "Use normal::ColorPair::new() or shims::ncurses::init_pair() instead")]
/// Change the definition of a color-pair. It takes two arguments: the number
/// of the color-pair to be changed, and the foreground and background colors.
/// The value of color_pair must be between 1 and COLOR_PAIRS - 1 (the 0 color
/// pair is wired to white on black and cannot be changed).
/// If the color-pair was previously initialized, the screen is refreshed and
/// all occurrences of that color-pair are changed to the new definition.
pub fn init_pair(color_pair: short_t, colors: normal::Colors) -> result!(normal::ColorPair) {
    if i32::from(color_pair) >= COLOR_PAIRS() {
        Err(NCurseswError::ColorPairLimit)
    } else if colors.foreground().number() >= COLORS() || colors.background().number() >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match ncurses::init_pair(color_pair, short_t::try_from(colors.foreground().number())?, short_t::try_from(colors.background().number())?) {
            OK => Ok(normal::ColorPair::_from(None, color_pair)),
            rc => Err(ncurses_function_error_with_rc!("init_pair", rc))
        }
    }
}

/// Initialize the NCurses data structures and return the standard screen.
///
/// `initscr()` is normally the first NCurses routine to call when initializing
/// a program. A few special routines sometimes need to be called before it; these
/// are `slk_init()`, `filter()`, `ripoffline()`, `use_env()`.  For multiple-terminal
/// applications, `newterm()` may be called before `initscr()`.
///
/// The `initscr()` code determines the terminal type and initializes all NCurses data
/// structures. `initscr()` also causes the first call to `refresh()` to clear the
/// screen. If errors occur, `initscr()` writes an appropriate error message to
/// standard error and exits; otherwise, a pointer is returned to `stdscr()`.
pub fn initscr() -> result!(WINDOW) {
    unsafe { ncurses::initscr().ok_or(ncurses_function_error!("initscr")) }
}

/// Equivalent of `winnstr()` using `stdscr()` as window `handle`.
pub fn innstr(number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "{}innstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::innstr(ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("innstr", len))
    } else {
        assert!(!ptr.is_null(), "{}innstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}innstr() : len={}, LINEMAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

/// Equivalent of `winnwstr()` using `stdscr()` as window `handle`.
pub fn innwstr(number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "{}innwstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    let len = unsafe { ncurses::innwstr(ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("innwstr", len))
    } else {
        assert!(!ptr.is_null(), "{}innwstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}innwstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

/// Equivalent of `ins_nwstr()` using `stdscr()` as window `handle`.
pub fn ins_nwstr(wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::ins_nwstr(raw_with_nul_as_slice!(wstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ins_nwstr", rc))
    }
}

/// Equivalent of `ins_wch()` using `stdscr()` as window `handle`.
pub fn ins_wch(wch: ComplexChar) -> result!(()) {
    match ncurses::ins_wch(&ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ins_wch", rc))
    }
}

/// Equivalent of `ins_wstr()` using `stdscr()` as window `handle`.
pub fn ins_wstr(wstr: &WideString) -> result!(()) {
    match ncurses::ins_wstr(raw_with_nul_as_slice!(wstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ins_wstr", rc))
    }
}

/// Equivalent of `insch()` using `stdscr()` as window `handle`.
pub fn insch(ch: ChtypeChar) -> result!(()) {
    match ncurses::insch(ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insch", rc))
    }
}

/// Equivalent of `insdelln()` using `stdscr()` as window `handle`.
pub fn insdelln(n: i32) -> result!(()) {
    match ncurses::insdelln(n) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insdelln", rc))
    }
}

/// Equivalent of `insertln()` using `stdscr()` as window `handle`.
pub fn insertln() -> result!(()) {
    match ncurses::insertln() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insertln", rc))
    }
}

/// Equivalent of `insnstr()` using `stdscr()` as window `handle`.
pub fn insnstr(str: &str, number: i32) -> result!(()) {
    match ncurses::insnstr(unsafe { c_str_with_nul!(str) }, number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insnstr", rc))
    }
}

/// Equivalent of `insstr()` using `stdscr()` as window `handle`.
pub fn insstr(str: &str) -> result!(()) {
    match ncurses::insstr(unsafe { c_str_with_nul!(str) }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("insstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use innstr() instead")]
/// Equivalent of `instr()` using `stdscr()` as window `handle`.
pub fn instr() -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::instr(ptr) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("instr", len))
    } else {
        assert!(!ptr.is_null(), "{}instr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}instr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

/// If the `intrflush()` option is enabled (`flag` is `true`), and an interrupt
/// key is pressed on the keyboard (interrupt, break, quit), all output in the
/// tty driver queue will be flushed, giving the effect of faster response to
/// the interrupt, but causing NCurses to have the wrong idea of what is on the
/// screen. Disabling the option (`flag` is `false`) prevents the flush.
/// The default for the option is inherited from the tty driver settings.
pub fn intrflush(flag: bool) -> result!(()) {
    match unsafe { ncurses::intrflush(ptr::null_mut(), flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("intrflush", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use innwstr() instead")]
/// Equivalent of `inwstr()` using `stdscr()` as window `handle`.
pub fn inwstr() -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::inwstr(ptr) } {
        OK => {
            assert!(!ptr.is_null(), "{}inwstr() : ptr.is_null()", MODULE_PATH);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("inwstr", rc))
    }
}

/// Returns the value set in `clearok()`.
pub fn is_cleared(handle: WINDOW) -> bool {
    unsafe { ncurses::is_cleared(handle) }
}

/// Returns the value set in `idcok()`.
pub fn is_idcok(handle: WINDOW) -> bool {
    unsafe { ncurses::is_idcok(handle) }
}

/// Returns the value set in `idlok()`.
pub fn is_idlok(handle: WINDOW) -> bool {
    unsafe { ncurses::is_idlok(handle) }
}

/// Returns the value set in `immedok()`.
pub fn is_immedok(handle: WINDOW) -> bool {
    unsafe { ncurses::is_immedok(handle) }
}

/// Returns the value set in `keypad()`.
pub fn is_keypad(handle: WINDOW) -> bool {
    unsafe { ncurses::is_keypad(handle) }
}

/// Returns the value set in `leaveok()`.
pub fn is_leaveok(handle: WINDOW) -> bool {
    unsafe { ncurses::is_leaveok(handle) }
}

/// Return `true` if the specified line was modified since the last call to
/// `refresh()`; otherwise return `false`.
pub fn is_linetouched(handle: WINDOW, line: i32) -> bool {
    unsafe { ncurses::is_linetouched(handle, line) }
}

/// Returns the value set in `nodelay()`.
pub fn is_nodelay(handle: WINDOW) -> bool {
    unsafe { ncurses::is_nodelay(handle) }
}

/// Returns the value set in `notimeout()`.
pub fn is_notimeout(handle: WINDOW) -> bool {
    unsafe { ncurses::is_notimeout(handle) }
}

/// Returns `true` if the window is a pad i.e., created by `newpad()`.
pub fn is_pad(handle: WINDOW) -> bool {
    unsafe { ncurses::is_pad(handle) }
}

/// Returns the value set in `scrollok()`.
pub fn is_scrollok(handle: WINDOW) -> bool {
    unsafe { ncurses::is_scrollok(handle) }
}

/// Returns `true` if the window is a sub-window, i.e., created by
/// `subwin()` or `derwin()`.
pub fn is_subwin(handle: WINDOW) -> bool {
    unsafe { ncurses::is_subwin(handle) }
}

/// Returns the value set in `syncok()`.
pub fn is_syncok(handle: WINDOW) -> bool {
    unsafe { ncurses::is_syncok(handle) }
}

/// Return `true` if `resize_term()` would modify the window structure,
/// `false` otherwise.
pub fn is_term_resized(size: Size) -> bool {
    ncurses::is_term_resized(size.lines, size.columns)
}

/// Return `true` if the specified window was modified since the last call
/// to `refresh()`; otherwise return `false`.
pub fn is_wintouched(handle: WINDOW) -> bool {
    unsafe { ncurses::is_wintouched(handle) }
}

/// Return `true` if `endwin()` has been called (that is, the
/// NCurses library has been deinitialized).
pub fn isendwin() -> bool {
    ncurses::isendwin()
}

/// Permits an application to determine if a string is currently bound
/// to any `KeyBindind`.
pub fn key_defined(definition: &str) -> result!(KeyBinding) {
    let c = ncurses::key_defined(unsafe { c_str_with_nul!(definition) });

    if c.is_negative() {
        Err(ncurses_function_error_with_rc!("key_defined", c))
    } else {
        Ok(KeyBinding::from(c))
    }
}

/// Returns a string corresponding to a given `KeyBinding`.
pub fn key_name(w: KeyBinding) -> result!(String) {
    ncurses::key_name(KeyBinding::into(w)).ok_or(ncurses_function_error!("key_name"))
}

/// Permits an application to determine the string which is defined
/// in the terminfo for specific keycodes.
pub fn keybound(keycode: KeyBinding, count: i32) -> result!(String) {
    ncurses::keybound(KeyBinding::into(keycode), count).ok_or(ncurses_function_error!("keybound"))
}

/// Return the name of the key binding c. The name of a key generating
/// printable ASCII character is the key’s character. The name of a
/// control-key combination is a two-byte bytes object consisting of a
/// caret (b'^') followed by the corresponding printable ASCII character.
/// The name of an alt-key combination (128–255) is a bytes object
/// consisting of the prefix b'M-' followed by the name of the
/// corresponding ASCII character.
pub fn keyname(c: KeyBinding) -> result!(String) {
    ncurses::keyname(KeyBinding::into(c)).ok_or(ncurses_function_error!("keyname"))
}

/// Permits an application to disable specific `KeyBinding`, rather than use
/// the keypad function to disable all keycodes. Keys that have been disabled
/// can be re-enabled.
pub fn keyok(keycode: KeyBinding, enable: bool) -> result!(()) {
    match ncurses::keyok(KeyBinding::into(keycode), enable) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("keyok", rc))
    }
}

/// If flag is `true`, escape sequences generated by some keys (keypad, function keys)
/// will be interpreted by NCurses. If flag is `false`, escape sequences will be left
/// as is in the input stream.
pub fn keypad(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::keypad(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("keypad", rc))
    }
}

/// Return the user’s current line kill character. Under Unix operating
/// systems this is a property of the controlling tty of the NCurses
/// program, and is not set by the NCurses library itself.
pub fn killchar() -> result!(char) {
    let rc = ncurses::killchar();

    if rc.is_negative() {
        Err(ncurses_function_error_with_rc!("killchar", i32::from(rc)))
    } else {
        Ok(char::from(u8::try_from(rc)?))
    }
}

/// Return the user’s current line kill character as a wide character.
/// Under Unix operating systems this is a property of the controlling
/// tty of the NCurses program, and is not set by the NCurses library itself.
pub fn killwchar() -> result!(WideChar) {
    let mut wch: [wchar_t; 1] = [0];

    match unsafe { ncurses::killwchar(wch.as_mut_ptr()) } {
        OK => Ok(WideChar::from(wch[0])),
        rc => Err(ncurses_function_error_with_rc!("killwchar", rc))
    }
}

/// If flag is `true`, cursor is left where it is on update, instead of being
/// at “cursor position.” This reduces cursor movement where possible.
/// If possible the cursor will be made invisible.
///
/// If flag is `false`, cursor will always be at “cursor position” after an update.
pub fn leaveok(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::leaveok(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("leaveok", rc))
    }
}

/// Return the terminfo long name field describing the current terminal.
/// The maximum length of a verbose description is 128 characters.
/// It is defined only after the call to `initscr()`.
pub fn longname() -> result!(String) {
    ncurses::longname().ok_or(ncurses_function_error!("longname"))
}

/// Ship binary data to printer. Returns the number of characters
/// actually sent to the printer.
///
/// This function uses the mc5p or mc4 and mc5 capabilities, if they are
/// present, to ship given data to a printer attached to the terminal.
///
/// Note that the `mcprint()` code has no way to do flow control with the
/// printer or to know how much buffering it has. Your application is
/// responsible for keeping the rate of writes to the printer below its
/// continuous throughput rate (typically about half of its nominal cps
/// rating). Dot-matrix printers and 6-page-per-minute lasers can typically
/// handle 80cps, so a good conservative rule of thumb is to sleep for a
/// second after shipping each 80-character line.
pub fn mcprint(data: &[i8], len: i32) -> result!(i32) {
    match unsafe { ncurses::mcprint(data.as_ptr() as *mut i8, len) } {
        ERR => Err(ncurses_os_error!("mcprint")),
        rc  => Ok(rc)
    }
}

/// If flag is `true`, allow 8-bit characters to be input. If flag is
/// `false`, allow only 7-bit chars.
pub fn meta(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::meta(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("meta", rc))
    }
}

/// Equivalent of `wmove()` using `stdscr()` as window `handle`.
pub fn r#move(origin: Origin) -> result!(()) {
    match ncurses::r#move(origin.y, origin.x) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("move", rc))
    }
}

/// Equivalent of `mvwadd_wch()` using `stdscr()` as window `handle`.
pub fn mvadd_wch(origin: Origin, wch: ComplexChar) -> result!(()) {
    match ncurses::mvadd_wch(origin.y, origin.x, &ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvadd_wch", rc))
    }
}

/// Equivalent of `mvwadd_wchnstr()` using `stdscr()` as window `handle`.
pub fn mvadd_wchnstr(origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
    match ncurses::mvadd_wchnstr(origin.y, origin.x, raw_with_nul_as_slice!(wchstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvadd_wchnstr", rc))
    }
}

/// Equivalent of `mvwadd_wchstr()` using `stdscr()` as window `handle`.
pub fn mvadd_wchstr(origin: Origin, wchstr: &ComplexString) -> result!(()) {
    match ncurses::mvadd_wchstr(origin.y, origin.x, raw_with_nul_as_slice!(wchstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvadd_wchstr", rc))
    }
}

/// Equivalent of `mvwaddch()` using `stdscr()` as window `handle`.
pub fn mvaddch(origin: Origin, ch: ChtypeChar) -> result!(()) {
    match ncurses::mvaddch(origin.y, origin.x, ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddch", rc))
    }
}

/// Equivalent of `mvwaddchnstr()` using `stdscr()` as window `handle`.
pub fn mvaddchnstr(origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
    match ncurses::mvaddchnstr(origin.y, origin.x, raw_with_nul_as_slice!(chstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddchnstr", rc))
    }
}

/// Equivalent of `mvwaddchstr()` using `stdscr()` as window `handle`.
pub fn mvaddchstr(origin: Origin, chstr: &ChtypeString) -> result!(()) {
    match ncurses::mvaddchstr(origin.y, origin.x, raw_with_nul_as_slice!(chstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddchstr", rc))
    }
}

/// Equivalent of `mvwaddnstr()` using `stdscr()` as window `handle`.
pub fn mvaddnstr(origin: Origin, str: &str, number: i32) -> result!(()) {
    match ncurses::mvaddnstr(origin.y, origin.x, unsafe { c_str_with_nul!(str) }, number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddnstr", rc))
    }
}

/// Equivalent of `mvwaddnwstr()` using `stdscr()` as window `handle`.
pub fn mvaddnwstr(origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::mvaddnwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddnwstr", rc))
    }
}

/// Equivalent of `mvwaddstr()` using `stdscr()` as window `handle`.
pub fn mvaddstr(origin: Origin, str: &str) -> result!(()) {
    match ncurses::mvaddstr(origin.y, origin.x, unsafe { c_str_with_nul!(str) }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddstr", rc))
    }
}

/// Equivalent of `mvwaddwstr()` using `stdscr()` as window `handle`.
pub fn mvaddwstr(origin: Origin, wstr: &WideString) -> result!(()) {
    match ncurses::mvaddwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvaddwstr", rc))
    }
}

/// Equivalent of `mvwchgat()` using `stdscr()` as window `handle`.
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

/// The `mvcur()` routine provides low-level cursor motion.
/// It takes effect immediately (rather than at the next refresh).
pub fn mvcur(old: Origin, new: Origin) -> result!(()) {
    match ncurses::mvcur(old.y, old.x, new.y, new.x) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvcur", rc))
    }
}

/// Delete any character at origin.
pub fn mvdelch(origin: Origin) -> result!(()) {
    match ncurses::mvdelch(origin.y, origin.x) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvdelch", rc))
    }
}

/// Move the window inside its parent window. The screen-relative parameters
/// of the window are not changed. This routine is used to display different
/// parts of the parent window at the same physical position on the screen.
pub fn mvderwin(handle: WINDOW, origin: Origin) -> result!(()) {
    match unsafe { ncurses::mvderwin(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvderwin", rc))
    }
}

/// Equivalent of `mvwget_wch()` using `stdscr()` as window `handle`.
pub fn mvget_wch(origin: Origin) -> result!(CharacterResult<WideChar>) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::mvget_wch(origin.y, origin.x, wch.as_mut_ptr()) } {
        EINTR        => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE   => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT    => Err(NCurseswError::KeyEvent),
        KEY_CODE_YES => {
            match i32::try_from(wch[0])? {
                #[cfg(feature = "key_resize_as_error")]
                KEY_RESIZE => Err(NCurseswError::KeyResize),
                #[cfg(feature = "key_event_as_error")]
                KEY_EVENT  => Err(NCurseswError::KeyEvent),
                _          => Ok(CharacterResult::Key(KeyBinding::try_from(wch[0])?))
            }
        },
        rc           => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvget_wch", rc))
            } else {
                Ok(CharacterResult::Character(WideChar::from(wch[0])))
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvgetn_wstr() instead")]
/// Equivalent of `mvwget_wstr()` using `stdscr()` as window `handle`.
pub fn mvget_wstr(origin: Origin) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvget_wstr(origin.y, origin.x, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvget_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}mvget_wstr() : ptr.is_null()", MODULE_PATH);

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

/// Equivalent of `mvwgetch()` using `stdscr()` as window `handle`.
pub fn mvgetch(origin: Origin) -> result!(CharacterResult<char>) {
    match ncurses::mvgetch(origin.y, origin.x) {
        EINTR      => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvgetch", rc))
            } else if rc >= KEY_MIN && rc <= KEY_MAX {
                Ok(CharacterResult::Key(KeyBinding::from(rc)))
            } else {
                Ok(CharacterResult::Character(char::from(u8::try_from(i8::try_from(rc)?)?)))
            }
        }
    }
}

/// Equivalent of `mvwgetn_wstr()` using `stdscr()` as window `handle`.
pub fn mvgetn_wstr(origin: Origin, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "{}mvgetn_wstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetn_wstr(origin.y, origin.x, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvgetn_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}mvgetn_wstr() : ptr.is_null()", MODULE_PATH);

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

/// Equivalent of `mvwgetnstr()` using `stdscr()` as window `handle`.
pub fn mvgetnstr(origin: Origin, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "{}mvgetnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetnstr(origin.y, origin.x, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvgetnstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}mvgetnstr() : ptr.is_null()", MODULE_PATH);

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvgetnstr() instead")]
/// Equivalent of `mvwgetstr()` using `stdscr()` as window `handle`.
pub fn mvgetstr(origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvgetstr(origin.y, origin.x, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvgetstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}mvgetstr() : ptr.is_null()", MODULE_PATH);

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

/// Equivalent of `mvwhline()` using `stdscr()` as window `handle`.
pub fn mvhline(origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::mvhline(origin.y, origin.x, ChtypeChar::into(ch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvhline", rc))
    }
}

/// Equivalent of `mvwhline_set()` using `stdscr()` as window `handle`.
pub fn mvhline_set(origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::mvhline_set(origin.y, origin.x, &ComplexChar::into(wch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvhline_set", rc))
    }
}

/// Equivalent of `mvwin_wch()` using `stdscr()` as window `handle`.
pub fn mvin_wch(origin: Origin) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::mvin_wch(origin.y, origin.x, wcval.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wcval[0])),
        rc => Err(ncurses_function_error_with_rc!("mvin_wch", rc))
    }
}

/// Equivalent of `mvwin_wchnstr()` using `stdscr()` as window `handle`.
pub fn mvin_wchnstr(origin: Origin, number: i32) -> result!(ComplexString) {
    assert!(number <= LINE_MAX as i32, "{}mvin_wchnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvin_wchnstr(origin.y, origin.x, ptr, number) } {
        OK => {
            assert!(!ptr.is_null(), "{}mvin_wchnstr() : ptr.is_null()", MODULE_PATH);

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(number)?) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvin_wchnstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvin_wchnstr() instead")]
/// Equivalent of `mvwin_wchstr()` using `stdscr()` as window `handle`.
pub fn mvin_wchstr(origin: Origin) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvin_wchstr(origin.y, origin.x, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "{}mvin_wchstr() : ptr.is_null()", MODULE_PATH);

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvin_wchstr", rc))
    }
}

/// Equivalent of `mvwinch()` using `stdscr()` as window `handle`.
pub fn mvinch(origin: Origin) -> ChtypeChar {
    ChtypeChar::from(ncurses::mvinch(origin.y, origin.x))
}

/// Equivalent of `mvwinchnstr()` using `stdscr()` as window `handle`.
pub fn mvinchnstr(origin: Origin, number: i32) -> result!(ChtypeString) {
    assert!(number <= LINE_MAX as i32, "{}mvinchnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinchnstr(origin.y, origin.x, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvinchnstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvinchnstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvinchnstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvinchnstr() instead")]
/// Equivalent of `mvwinchstr()` using `stdscr()` as window `handle`.
pub fn mvinchstr(origin: Origin) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinchstr(origin.y, origin.x, ptr) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvinchstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvinchstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvinchstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

/// Equivalent of `mvwinnstr()` using `stdscr()` as window `handle`.
pub fn mvinnstr(origin: Origin, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "{}mvinnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinnstr(origin.y, origin.x, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvinnstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvinnstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvinnstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

/// Equivalent of `mvwinnwstr()` using `stdscr()` as window `handle`.
pub fn mvinnwstr(origin: Origin, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "{}mvinnwstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinnwstr(origin.y, origin.x, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvinnwstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvinnwstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvinnwstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

/// Equivalent of `mvwins_nwstr()` using `stdscr()` as window `handle`.
pub fn mvins_nwstr(origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match ncurses::mvins_nwstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvins_nwstr", rc))
    }
}

/// Equivalent of `mvwins_wch()` using `stdscr()` as window `handle`.
pub fn mvins_wch(origin: Origin, wch: ComplexChar) -> result!(()) {
    match ncurses::mvins_wch(origin.y, origin.x, &ComplexChar::into(wch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvins_wch", rc))
    }
}

/// Equivalent of `mvwins_wstr()` using `stdscr()` as window `handle`.
pub fn mvins_wstr(origin: Origin, wstr: &WideString) -> result!(()) {
    match ncurses::mvins_wstr(origin.y, origin.x, raw_with_nul_as_slice!(wstr)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvins_wstr", rc))
    }
}

/// Equivalent of `mvwinsch()` using `stdscr()` as window `handle`.
pub fn mvinsch(origin: Origin, ch: ChtypeChar) -> result!(()) {
    match ncurses::mvinsch(origin.y, origin.x, ChtypeChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvinsch", rc))
    }
}

/// Equivalent of `mvwinsnstr()` using `stdscr()` as window `handle`.
pub fn mvinsnstr(origin: Origin, str: &str, number: i32) -> result!(()) {
    match ncurses::mvinsnstr(origin.y, origin.x, unsafe { c_str_with_nul!(str) }, number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvinsnstr", rc))
    }
}

/// Equivalent of `mvwinsstr()` using `stdscr()` as window `handle`.
pub fn mvinsstr(origin: Origin, str: &str) -> result!(()) {
    match ncurses::mvinsstr(origin.y, origin.x, unsafe { c_str_with_nul!(str) }) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvinsstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvinnstr() instead")]
/// Equivalent of `mvwinstr()` using `stdscr()` as window `handle`.
pub fn mvinstr(origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvinstr(origin.y, origin.x, ptr) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvinstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvinstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvinstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvinnwstr() instead")]
/// Equivalent of `mvwinwstr()` using `stdscr()` as window `handle`.
pub fn mvinwstr(origin: Origin) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvinwstr(origin.y, origin.x, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "{}mvinwstr() : ptr.is_null()", MODULE_PATH);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvinwstr", rc))
    }
}

/// Equivalent of `mvwvline()` using `stdscr()` as window `handle`.
pub fn mvvline(origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::mvvline(origin.y, origin.x, ChtypeChar::into(ch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvvline", rc))
    }
}

/// Equivalent of `mvwvline_set()` using `stdscr()` as window `handle`.
pub fn mvvline_set(origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::mvvline_set(origin.y, origin.x, &ComplexChar::into(wch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvvline_set", rc))
    }
}

/// Paint a complex character `wch` at `origin`, overwriting any character
/// previously painted at that location.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwadd_wch(handle: WINDOW, origin: Origin, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::mvwadd_wch(handle, origin.y, origin.x, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwadd_wch", rc))
    }
}

/// Paint a complex character string of at most `number` characters of `wchstr`
/// at `origin`, overwriting anything previously on the window.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwadd_wchnstr(handle: WINDOW, origin: Origin, wchstr: &ComplexString, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwadd_wchnstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wchstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwadd_wchnstr", rc))
    }
}

/// Paint a complex character string of `wchstr` at `origin`,
/// overwriting anything previously on the window.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwadd_wchstr(handle: WINDOW, origin: Origin, wchstr: &ComplexString) -> result!(()) {
    match unsafe { ncurses::mvwadd_wchstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wchstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwadd_wchstr", rc))
    }
}

/// Paint character `ch` at `origin`, overwriting any character previously
/// painted at that location.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwaddch(handle: WINDOW, origin: Origin, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::mvwaddch(handle, origin.y, origin.x, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddch", rc))
    }
}

/// Paint a character string (with rendition) of `chstr` of at most `number`
/// characters at `origin`, overwriting anything previously on the window.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwaddchnstr(handle: WINDOW, origin: Origin, chstr: &ChtypeString, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwaddchnstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(chstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddchnstr", rc))
    }
}

/// Paint a character string (with rendition) of `chstr` at `origin`,
/// overwriting anything previously on the window.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwaddchstr(handle: WINDOW, origin: Origin, chstr: &ChtypeString) -> result!(()) {
    match unsafe { ncurses::mvwaddchstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(chstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddchstr", rc))
    }
}

/// Paint a string of `str` with at most `number` characters` at `origin`,
/// overwriting anything previously on the window.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwaddnstr(handle: WINDOW, origin: Origin, str: &str, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwaddnstr(handle, origin.y, origin.x, c_str_with_nul!(str), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddnstr", rc))
    }
}

/// Paint a wide string of `wstr` with at most `number` characters at `origin`,
/// overwriting anything previously on the window.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwaddnwstr(handle: WINDOW, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwaddnwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddnwstr", rc))
    }
}

/// Paint a string of `str` at `origin`, overwriting anything previously
/// on the window.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwaddstr(handle: WINDOW, origin: Origin, str: &str) -> result!(()) {
    match unsafe { ncurses::mvwaddstr(handle, origin.y, origin.x, c_str_with_nul!(str)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddstr", rc))
    }
}

/// Paint a wide string of `wstr` at `origin`, overwriting anything previously
/// on the window.
///
/// Writing outside the window, sub-window, or pad raises a `NCurseswError`.
/// Attempting to write to the lower right corner of a window, sub-window,
/// or pad will cause an `NCurseswError` to be raised after the character
/// is printed.
pub fn mvwaddwstr(handle: WINDOW, origin: Origin, wstr: &WideString) -> result!(()) {
    match unsafe { ncurses::mvwaddwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwaddwstr", rc))
    }
}

/// Set the attributes of `number` characters at the position `origin`.
/// This routine moves cursor to position `origin`. The changed line
/// will be touched using the `touchline()` method so that the contents
/// will be redisplayed by the next window `refresh()`.
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

/// Delete any character at `origin` on window.
pub fn mvwdelch(handle: WINDOW, origin: Origin) -> result!(()) {
    match unsafe { ncurses::mvwdelch(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwdelch", rc))
    }
}

/// Return an enum of `CharacterResult::Character(WideChar)` for most keys, or
/// a `CharacterResult::Key(KeyBinding)` for function keys, keypad keys, and
/// other special keys. In no-delay mode, raise a `NCurseswError` if there is
/// no input.
///
/// If the `keypad()` function has been called with a `flag` of `true` the
/// NCurses library will interpret some keys such as function keys, keypad
/// keys and other special keys (this may also include such things as mouse
/// and resizing events) and return these wrapped in the enum `CharacterResult::Key()`
/// or a `CharacterResult::Character()` for non-interpreted keys.
pub fn mvwget_wch(handle: WINDOW, origin: Origin) -> result!(CharacterResult<WideChar>) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::mvwget_wch(handle, origin.y, origin.x, wch.as_mut_ptr()) } {
        EINTR        => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE   => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT    => Err(NCurseswError::KeyEvent),
        KEY_CODE_YES => {
            match i32::try_from(wch[0])? {
                #[cfg(feature = "key_resize_as_error")]
                KEY_RESIZE => Err(NCurseswError::KeyResize),
                #[cfg(feature = "key_event_as_error")]
                KEY_EVENT  => Err(NCurseswError::KeyEvent),
                _          => Ok(CharacterResult::Key(KeyBinding::try_from(wch[0])?))
            }
        },
        rc           => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvwget_wch", rc))
            } else {
                Ok(CharacterResult::Character(WideChar::from(wch[0])))
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvwgetn_wstr() instead")]
/// Read a wide string from the user at `origin`, with primitive line editing capacity.
pub fn mvwget_wstr(handle: WINDOW, origin: Origin) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwget_wstr(handle, origin.y, origin.y, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvwget_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}mvwget_wstr() : ptr.is_null()", MODULE_PATH);

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

/// Return an enum of `CharacterResult::Character(char)` for most keys, or a
/// `CharacterResult::Key(KeyBinding)` for function keys, keypad keys, and
/// other special keys. In no-delay mode, raise a `NCurseswError` if there is
/// no input.
///
/// If the `keypad()` function has been called with a `flag` of `true` the
/// NCurses library will interpret some keys such as function keys, keypad
/// keys and other special keys (this may also include such things as mouse
/// and resizing events) and return these wrapped in the enum `CharacterResult::Key()`
/// or a `CharacterResult::Character()` for non-interpreted keys.
pub fn mvwgetch(handle: WINDOW, origin: Origin) -> result!(CharacterResult<char>) {
    match unsafe { ncurses::mvwgetch(handle, origin.y, origin.x) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvwgetch", rc))
            } else if rc >= KEY_MIN && rc <= KEY_MAX {
                Ok(CharacterResult::Key(KeyBinding::from(rc)))
            } else {
                Ok(CharacterResult::Character(char::from(u8::try_from(i8::try_from(rc)?)?)))
            }
        }
    }
}

/// Read a wide string of at most `number` characters from the user at
/// `origin`, with primitive line editing capacity.
pub fn mvwgetn_wstr(handle: WINDOW, origin: Origin, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "{}mvwgetn_wstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetn_wstr(handle, origin.y, origin.x, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvwgetn_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}mvwgetn_wstr() : ptr.is_null()", MODULE_PATH);

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

/// Read a string of at most `number` characters from the user at `origin`,
/// with primitive line editing capacity.
pub fn mvwgetnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "{}mvwgetnstr() : number={}, LINE_MAX{}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetnstr(handle, origin.y, origin.x, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvwgetnstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}mvwgetnstr() : ptr.is_null()", MODULE_PATH);

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvwgetnstr() instead")]
/// Read a string from the user at `origin`, with primitive line editing capacity.
pub fn mvwgetstr(handle: WINDOW, origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::mvwgetstr(handle, origin.y, origin.x, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("mvwgetstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}mvwgetstr() : ptr.is_null()", MODULE_PATH);

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

/// Display a horizontal line with length `number` consisting of the
/// character `ch` at `origin`.
pub fn mvwhline(handle: WINDOW, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwhline(handle, origin.y, origin.x, ChtypeChar::into(ch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwhline", rc))
    }
}

/// Display a horizontal line with length `number` consisting of the
/// character `wch` at `origin`.
pub fn mvwhline_set(handle: WINDOW, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwhline_set(handle, origin.y, origin.x, &ComplexChar::into(wch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwhline_set", rc))
    }
}

/// Move the window so its upper-left corner is at `origin`.
///
/// If the move would cause the window to be off the screen, it is an error
/// and the window is not moved. Moving sub-windows is allowed, but should
/// be avoided.
pub fn mvwin(handle: WINDOW, origin: Origin) -> result!(()) {
    match unsafe { ncurses::mvwin(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwin", rc))
    }
}

/// Return the complex character at the given `origin` in the window.
pub fn mvwin_wch(handle: WINDOW, origin: Origin) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::mvwin_wch(handle, origin.y, origin.x, wcval.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wcval[0])),
        rc => Err(ncurses_function_error_with_rc!("mvwin_wch", rc))
    }
}

/// Return the complex character string of length `number` at the
/// given `origin` in the window.
pub fn mvwin_wchnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(ComplexString) {
    assert!(number <= LINE_MAX as i32, "{}mvwin_wchnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwin_wchnstr(handle, origin.y, origin.x, ptr, number) } {
        OK => {
            assert!(!ptr.is_null(), "{}mvwin_wchnstr() : ptr.is_null()", MODULE_PATH);

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(number)?) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvwin_wchnstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvwin_wchnstr() instead")]
/// Return the complex character string given `origin` in the window.
pub fn mvwin_wchstr(handle: WINDOW, origin: Origin) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwin_wchstr(handle, origin.y, origin.x, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "{}mvwin_wchstr() : ptr.is_null()", MODULE_PATH);

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvwin_wchstr", rc))
    }
}

/// Return the character and rendition at the given `origin` in the window.
pub fn mvwinch(handle: WINDOW, origin: Origin) -> ChtypeChar {
    ChtypeChar::from(unsafe { ncurses::mvwinch(handle, origin.y, origin.x) })
}

/// Return the character string and rendition of length `number` at the
/// given `origin` in the window.
pub fn mvwinchnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(ChtypeString) {
    assert!(number <= LINE_MAX as i32, "{}mvwinchnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinchnstr(handle, origin.y, origin.x, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvwinchnstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvwinchnstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvwinchnstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvwinchnstr() instead")]
/// Return the character string and rendion at the given `origin` in the window.
pub fn mvwinchstr(handle: WINDOW, origin: Origin) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinchstr(handle, origin.y, origin.x, ptr) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvwinchstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvwinchstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvwinchstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

/// Return the character string of length `number` at the given `origin` in the window.
pub fn mvwinnstr(handle: WINDOW, origin: Origin, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "{}mvwinnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinnstr(handle, origin.y, origin.x, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvwinnstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvwinnstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvwinnstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

/// Return the wide character string of length `number` at the given `origin` in the window.
pub fn mvwinnwstr(handle: WINDOW, origin: Origin, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "{}mvwinnwstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinnwstr(handle, origin.y, origin.x, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvwinnwstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvwinnwstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvwinnwstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

/// Insert a wide string (as many characters as will fit on the line) before the
/// character at `origin`, up to `number` characters. If `number` is zero or
/// negative, the entire string is inserted. All characters to the right of
/// the cursor are shifted right, with the rightmost characters on the line
/// being lost. The cursor position does not change (after moving to `origin`).
pub fn mvwins_nwstr(handle: WINDOW, origin: Origin, wstr: &WideString, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwins_nwstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwins_nwstr", rc))
    }
}

/// Insert the complex character `wch` at `origin`, moving the cursor position from
/// `origin.x` right by one character.
pub fn mvwins_wch(handle: WINDOW, origin: Origin, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::mvwins_wch(handle, origin.y, origin.x, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwins_wch", rc))
    }
}

/// Insert a wide string (as many characters as will fit on the line) before the
/// character at `origin`. All characters to the right of the cursor are shifted
/// right, with the rightmost characters on the line being lost. The cursor
/// position does not change (after moving to `origin`).
pub fn mvwins_wstr(handle: WINDOW, origin: Origin, wstr: &WideString) -> result!(()) {
    match unsafe { ncurses::mvwins_wstr(handle, origin.y, origin.x, raw_with_nul_as_slice!(wstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwins_wstr", rc))
    }
}

/// Insert character `ch` at `origin`, moving the cursor position from
/// `origin.x` right by one character.
pub fn mvwinsch(handle: WINDOW, origin: Origin, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::mvwinsch(handle, origin.y, origin.x, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwinsch", rc))
    }
}

/// Insert a string (as many characters as will fit on the line) before the
/// character at `origin`, up to `number` characters. If `number` is zero or
/// negative, the entire string is inserted. All characters to the right of
/// the cursor are shifted right, with the rightmost characters on the line
/// being lost. The cursor position does not change (after moving to `origin`).
pub fn mvwinsnstr(handle: WINDOW, origin: Origin, str: &str, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwinsnstr(handle, origin.y, origin.x, c_str_with_nul!(str), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwinsnstr", rc))
    }
}

/// Insert a string (as many characters as will fit on the line) before the
/// character at `origin`. All characters to the right of the cursor are shifted
/// right, with the rightmost characters on the line being lost. The cursor
/// position does not change (after moving to `origin`).
pub fn mvwinsstr(handle: WINDOW, origin: Origin, str: &str) -> result!(()) {
    match unsafe { ncurses::mvwinsstr(handle, origin.y, origin.x, c_str_with_nul!(str)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwinsstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvwinnstr() instead")]
/// Return the string at the given `origin` in the window.
pub fn mvwinstr(handle: WINDOW, origin: Origin) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::mvwinstr(handle, origin.y, origin.x, ptr) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("mvwinstr", len))
    } else {
        assert!(!ptr.is_null(), "{}mvwinstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}mvwinstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use mvwinnwstr() instead")]
/// Return the wide string at the given `origin` in the window.
pub fn mvwinwstr(handle: WINDOW, origin: Origin) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::mvwinwstr(handle, origin.y, origin.x, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "{}mvwinwstr() : ptr.is_null()", MODULE_PATH);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("mvwinwstr", rc))
    }
}

/// Display a vertical line with length `number` consisting of the character `ch`.
pub fn mvwvline(handle: WINDOW, origin: Origin, ch: ChtypeChar, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwvline(handle, origin.y, origin.x, ChtypeChar::into(ch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwvline", rc))
    }
}

/// Display a vertical line with length `number` consisting of the character `wch`.
pub fn mvwvline_set(handle: WINDOW, origin: Origin, wch: ComplexChar, number: i32) -> result!(()) {
    match unsafe { ncurses::mvwvline_set(handle, origin.y, origin.x, &ComplexChar::into(wch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvwvline_set", rc))
    }
}

#[deprecated(since = "0.3.2", note = "ncurses library call superseeded by native rust call. Use std::thread::sleep(dur: std::time::Duration) instead")]
/// Sleep for ms milliseconds.
pub fn napms(ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    match ncurses::napms(ms) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("napms", rc))
    }
}

/// The `newpad()` routine creates and returns a pointer to a new pad data
/// structure with the given `size`. A pad is like a window, except that it
/// is not restricted by the screen size, and is not necessarily associated
/// with a particular part of the screen. Pads can be used when a large
/// window is needed, and only a part of the window will be on the screen at
/// one time. Automatic refreshes of pads (e.g., from scrolling or echoing
/// of input) do not occur.
///
/// It is not legal to call `wrefresh()` with a pad as an argument; the
/// routines `prefresh()` or `pnoutrefresh()` should be called instead.
/// Note that these routines require additional parameters to specify the
/// part of the pad to be displayed and the location on the screen to be
/// used for the display.
pub fn newpad(size: Size) -> result!(WINDOW) {
    unsafe { ncurses::newpad(size.lines, size.columns).ok_or(ncurses_function_error!("newpad")) }
}

/// A program that outputs to more than one terminal should use the `newterm()`
/// routine for each terminal instead of `initscr()`. A program that needs to
/// inspect capabilities, so it can continue to run in a line-oriented mode
/// if the terminal cannot support a screen-oriented program, would also use
/// `newterm()`. The routine `newterm()` should be called once for each terminal.
/// It returns a pointer of type SCREEN which should be saved as a reference
/// to that terminal. newterm's arguments are:
///
/// - the type of the terminal to be used in place of $TERM,
/// - a file descriptor for output to the terminal, and...
/// - another file descriptor for input from the terminal
///
/// If the `term_type` parameter is `None`, $TERM will be used.
pub fn newterm<O, I>(term: Option<&str>, output: &O, input: &I) -> result!(SCREEN)
    where O: AsRawFd + Write,
          I: AsRawFd + Read
{
    unsafe {
        ncurses::newterm(
            option_str_as_ptr!(term),
            fdopen(output, "wb+")?,
            fdopen(input, "rb+")?
        ).ok_or(ncurses_function_error!("newterm"))
    }
}

/// Return a new window, whose left-upper corner is at origin,
/// and whose height/width is size.
pub fn newwin(size: Size, origin: Origin) -> result!(WINDOW) {
    unsafe { ncurses::newwin(size.lines, size.columns, origin.y, origin.x).ok_or(ncurses_function_error!("newwin")) }
}

/// Enter newline mode. This mode translates the return key into newline on
/// input, and translates newline into return and line-feed on output.
/// Newline mode is initially on.
pub fn nl() -> result!(()) {
    match ncurses::nl() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("nl", rc))
    }
}

/// Leave cbreak mode. Return to normal “cooked” mode with line buffering.
pub fn nocbreak() -> result!(()) {
    match ncurses::nocbreak() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("nocbreak", rc))
    }
}

/// If flag is `true`, `getch()` will be non-blocking.
pub fn nodelay(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::nodelay(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("nodelay", rc))
    }
}

/// Leave echo mode. Echoing of input characters is turned off.
pub fn noecho() -> result!(()) {
    match ncurses::noecho() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("noecho", rc))
    }
}

/// The `nofilter()` routine cancels the effect of a preceding `filter()` call.
/// That allows the caller to initialize a screen on a different device, using
/// a different value of $TERM. The limitation arises because the `filter()`
/// routine modifies the in-memory copy of the terminal information.
pub fn nofilter() {
    ncurses::nofilter()
}

/// Leave newline mode. Disable translation of return into newline on input,
/// and disable low-level translation of newline into newline/return on output
/// (but this does not change the behavior of `addch('\n')`, which always does
/// the equivalent of return and line feed on the virtual screen).
/// With translation off, NCurses can sometimes speed up vertical motion a
/// little; also, it will be able to detect the return key on input.
pub fn nonl() -> result!(()) {
    match ncurses::nonl() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("nonl", rc))
    }
}

/// When the `noqiflush()` routine is used, normal flush of input and output
/// queues associated with the INTR, QUIT and SUSP characters will not be done.
/// You may want to call `noqiflush()` in a signal handler if you want output
/// to continue as though the interrupt had not occurred, after the handler exits.
pub fn noqiflush() {
    ncurses::noqiflush()
}

/// Leave raw mode. Return to normal “cooked” mode with line buffering.
pub fn noraw() -> result!(()) {
    match ncurses::noraw() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("noraw", rc))
    }
}

/// If flag is `true`, escape sequences will not be timed out.
///
/// If flag is `false`, after a few milliseconds, an escape sequence will not be
/// interpreted, and will be left in the input stream as is.
pub fn notimeout(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::notimeout(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("notimeout", rc))
    }
}

/// Overlay the window on top of `destwin`. The windows need not be the same size,
/// only the overlapping region is copied. This copy is non-destructive, which
/// means that the current background character does not overwrite the old
/// contents of destwin.
///
/// To get fine-grained control over the copied region, the second form of
/// `overlay()` can be used. sminrow and smincol are the upper-left coordinates
/// of the source window, and the other variables mark a rectangle in the
/// destination window.
pub fn overlay(src_handle: WINDOW, dst_handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::overlay(src_handle, dst_handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("overlay", rc))
    }
}

/// Overlay the window on top of `destwin` in the same way as `overlay()`
/// but in a destructive manner.
pub fn overwrite(src_handle: WINDOW, dst_handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::overwrite(src_handle, dst_handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("overwrite", rc))
    }
}

#[deprecated(since = "0.4.0", note = "Use normal::ColorPair::colors() or shims::ncurses::pair_content() instead")]
/// Return a structure containing the colors for the requested color pair.
/// The value of `color_pair` must be between 1 and COLOR_PAIRS - 1.
pub fn pair_content(color_pair: short_t) -> result!(normal::Colors) {
    let mut fg: [short_t; 1] = [0];
    let mut bg: [short_t; 1] = [0];

    let color_palette = |color_number: short_t| normal::ColorPalette::_from(color_number);

    match unsafe { ncurses::pair_content(color_pair, fg.as_mut_ptr(), bg.as_mut_ptr()) } {
        OK => Ok(normal::Colors::new(normal::Color::_from(None, color_palette(fg[0])), normal::Color::_from(None, color_palette(bg[0])))),
        rc => Err(ncurses_function_error_with_rc!("pair_content", rc))
    }
}

/// The `pechochar()` routine is functionally equivalent to a call to `addch()`
/// followed by a call to `refresh()`, a call to `waddch()` followed by a call
/// to `wrefresh()`, or a call to `waddch()` followed by a call to `prefresh()`.
/// The knowledge that only a single character is being output is taken into
/// consideration and, for non-control characters, a considerable performance
/// gain might be seen by using these routines instead of their equivalents.
/// In the case of `pechochar()`, the last location of the pad on the screen
/// is reused for the arguments to `prefresh()`.
pub fn pechochar(pad: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::pechochar(pad, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("pechochar", rc))
    }
}

/// The `pecho_wchar()` function is the analogous wide-character form of `pechochar()`.
/// It outputs one character to a pad and immediately refreshes the pad. It does this
/// by a call to `wadd_wch()` followed by a call to `prefresh()`.
pub fn pecho_wchar(pad: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::pecho_wchar(pad, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("pecho_wchar", rc))
    }
}

/// The `pnoutrefresh()` routine is analogous to `wnoutrefresh()` except that they relate
/// to pads instead of windows. The additional parameters are needed to indicate what part
/// of the pad and screen are involved.
///
/// - The `pmin.y`, `pmin.x` parameters specify the upper left-hand corner of the rectangle
///   to be displayed in the pad.
/// - The `smin.y`, `smin.y`, `smax.y`, and `smax.x` parameters specify the edges of the
///   rectangle to be displayed on the screen.
///
/// The lower right-hand corner of the rectangle to be displayed in the pad is calculated
/// from the screen coordinates, since the rectangles must be the same size. Both rectangles
/// must be entirely contained within their respective structures. Negative values of
/// pmin.{y,x}, smin.{y,x} are treated as if they were zero.
pub fn pnoutrefresh(pad: WINDOW, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
    match unsafe { ncurses::pnoutrefresh(pad, pmin.y, pmin.x, smin.y, smin.x, smax.y, smax.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("pnoutrefresh", rc))
    }
}

/// The `prefresh()` routine is analogous to `wrefresh()` except that they relate to pads
/// instead of windows. The additional parameters are needed to indicate what part of the
/// pad and screen are involved.
///
/// - The `pmin.y`, `pmin.x` parameters specify the upper left-hand corner of the rectangle
///   to be displayed in the pad.
/// - The `smin.y`, `smin.y`, `smax.y`, and `smax.x` parameters specify the edges of the
///   rectangle to be displayed on the screen.
///
/// The lower right-hand corner of the rectangle to be displayed in the pad is calculated
/// from the screen coordinates, since the rectangles must be the same size. Both rectangles
/// must be entirely contained within their respective structures. Negative values of
/// pmin.{y,x}, smin.{y,x} are treated as if they were zero.
pub fn prefresh(pad: WINDOW, pmin: Origin, smin: Origin, smax: Origin) -> result!(()) {
    match unsafe { ncurses::prefresh(pad, pmin.y, pmin.x, smin.y, smin.x, smax.y, smax.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("prefresh", rc))
    }
}

/// At present this function is unimplemented.
pub fn putp(_str: &str) -> i32 {
    unimplemented!();
}

/// Write all data associated with the window into the provided file.
/// This information can be later retrieved using the `getwin()` function.
pub fn putwin<O: AsRawFd + Write>(handle: WINDOW, file: &O) -> result!(()) {
    match unsafe { ncurses::putwin(handle, fdopen(file, "w")?) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("putwin", rc))
    }
}

/// When the `noqiflush()` routine is used, normal flush of input and output
/// queues associated with the INTR, QUIT and SUSP characters will be done.
pub fn qiflush() {
    ncurses::qiflush()
}

/// Enter raw mode. In raw mode, normal line buffering and processing of
/// interrupt, quit, suspend, and flow control keys are turned off;
/// characters are presented to NCurses input functions one by one.
pub fn raw() -> result!(()) {
    match ncurses::raw() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("raw", rc))
    }
}

/// Touch the entire window, causing it to be completely redrawn on the
/// next call to `refresh()`.
pub fn redrawwin(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::redrawwin(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("redrawwin", rc))
    }
}

/// Update the display immediately (sync actual screen with previous
/// drawing/deleting methods).
pub fn refresh() -> result!(()) {
    match ncurses::refresh() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("refresh", rc))
    }
}

#[deprecated(since = "0.5.0", note = "use with caution as this routine will reset all color pairs potentially before they go out of scope and the color pairs will default to terminal default foreground and backgound colors.")]
/// Reset all defined color pairs.
pub fn reset_color_pairs() {
    ncurses::reset_color_pairs()
}

/// Restore the terminal to “program” mode, as previously saved by `def_prog_mode()`.
pub fn reset_prog_mode() -> result!(()) {
    match ncurses::reset_prog_mode() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("reset_prog_mode", rc))
    }
}

/// Restore the terminal to “shell” mode, as previously saved by `def_shell_mode()`.
pub fn reset_shell_mode() -> result!(()) {
    match ncurses::reset_shell_mode() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("reset_shell_mode", rc))
    }
}

/// Restore the state of the terminal modes to what it was at the last call to `savetty()`.
pub fn resetty() -> result!(()) {
    match ncurses::resetty() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("resetty", rc))
    }
}

/// Backend function used by `resizeterm()`, performing most of the work; when
/// resizing the windows, `resize_term()` blank-fills the areas that are extended.
/// The calling application should fill in these areas with appropriate data.
/// The `resize_term()` function attempts to resize all windows. However, due to
/// the calling convention of pads, it is not possible to resize these without
/// additional interaction with the application.
pub fn resize_term(size: Size) -> result!(()) {
    match ncurses::resize_term(size.lines, size.columns) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("resize_term", rc))
    }
}

/// Resize the standard and current windows to the specified dimensions, and
/// adjusts other bookkeeping data used by the NCurses library that record the
/// window dimensions (in particular the SIGWINCH handler).
pub fn resizeterm(size: Size) -> result!(()) {
    match ncurses::resizeterm(size.lines, size.columns) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("resizeterm", rc))
    }
}

/// The `ripoffline()` routine provides access to the same facility that `slk_init()`
/// uses to reduce the size of the screen. `ripoffline()` must be called before
/// `initscr()` or `newterm()` is called.
///
/// When the resulting initialization is done inside `initscr()`, the routine init
/// (supplied by the user) is called with two arguments:
/// - a window pointer to the one-line window that has been allocated and
/// - an `i32` with the number of columns in the window.
///
/// Inside this initialization routine, the `i32` variables `LINE`S and `COLS` are
/// not guaranteed to be accurate and `wrefresh()` or `doupdate()` must not be called.
/// It is allowable to call `wnoutrefresh()` during the initialization routine.
///
/// `ripoffline()` can be called up to five times before calling `initscr()` or `newterm()`.
pub fn ripoffline(line: Orientation, init: RipoffInit) -> result!(()) {
    match ncurses::ripoffline(line.value(), init) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ripoffline", rc))
    }
}

/// Save the current state of the terminal modes in a buffer, usable by `resetty()`.
pub fn savetty() -> result!(()) {
    match ncurses::savetty() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("savetty", rc))
    }
}

/// The `scr_dump()` routine dumps the current contents of the virtual screen
/// to the file specificed by `path`.
pub fn scr_dump<P: AsRef<Path>>(path: P) -> result!(()) {
    match unsafe { ncurses::scr_dump(path_as_slice!(path)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scr_dump", rc))
    }
}

/// The `scr_init()` routine reads in the contents of `path` and uses them to
/// initialize the curses data structures about what the terminal currently has
/// on its screen. If the data is determined to be valid, curses bases its next
/// update of the screen on this information rather than clearing the screen
/// and starting from scratch.
pub fn scr_init<P: AsRef<Path>>(path: P) -> result!(()) {
    match unsafe { ncurses::scr_init(path_as_slice!(path)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scr_init", rc))
    }
}

/// The `scr_restore()` routine sets the virtual screen to the contents of the file
/// specificed by `path`, which must have been written using `scr_dump()`. The next call
/// to `doupdate()` restores the physical screen to the way it looked in the
/// dump file.
pub fn scr_restore<P: AsRef<Path>>(path: P) -> result!(()) {
    match unsafe { ncurses::scr_restore(path_as_slice!(path)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scr_restore", rc))
    }
}

/// The `scr_set()` routine is a combination of `scr_restore()` and `scr_init()`.
/// It tells the program that the information in `path` is what is currently
/// on the screen, and also what the program wants on the screen. This can be
/// thought of as a screen inheritance function.
pub fn scr_set<P: AsRef<Path>>(path: P) -> result!(()) {
    match unsafe { ncurses::scr_set(path_as_slice!(path)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scr_set", rc))
    }
}

/// For positive n, the `scrl()` routine scroll the window up `n` lines
/// (line i+n becomes i); otherwise scroll the window down `n` lines.
/// This involves moving the lines in the window character image structure.
/// The current cursor position is not changed.
///
/// For these functions to work, scrolling must be enabled via `scrollok()`.
pub fn scrl(n: i32) -> result!(()) {
    match ncurses::scrl(n) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scrl", rc))
    }
}

/// Scroll the screen upward by 1 lines.
pub fn scroll(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::scroll(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scroll", rc))
    }
}

/// Control what happens when the cursor of a window is moved off the edge of the
/// window or scrolling region, either as a result of a newline action on the
/// bottom line, or typing the last character of the last line. If flag is `false`,
/// the cursor is left on the bottom line. If flag is `true`, the window is scrolled
/// up one line. Note that in order to get the physical scrolling effect on the
/// terminal, it is also necessary to call `idlok()`.
pub fn scrollok(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::scrollok(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scrollok", rc))
    }
}

/// Sets the escape delay delay.
pub fn set_escdelay(ms: time::Duration) -> result!(()) {
    match ncurses::set_escdelay(i32::try_from(ms.as_millis())?) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("set_escdelay", rc))
    }
}

/// Sets the tab size.
pub fn set_tabsize(size: i32) -> result!(()) {
    match ncurses::set_tabsize(size) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("set_tabsize", rc))
    }
}

/// The `set_term()` routine is used to switch between different terminals.
/// The screen reference new becomes the new current terminal. The previous
/// terminal is returned by the routine. This is the only routine which
/// manipulates SCREEN pointers; all other routines affect only the current
/// terminal.
pub fn set_term(screen: SCREEN) -> result!(SCREEN) {
    unsafe { ncurses::set_term(screen) }.ok_or(ncurses_function_error!("set_term"))
}

/// Sets a complex character from a character and rendition.
pub fn setcchar<A, P, T>(ch: char, attrs: &A, color_pair: &P) -> result!(ComplexChar)
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    let mut cchar_buf: [cchar_t; 1] = unsafe { mem::zeroed() };
    let wchar_buf: [wchar_t; 2] = [wchar_t::try_from(u32::from(ch))?, 0x00];

    let cchar_ptr: *mut cchar_t = cchar_buf.as_mut_ptr();

    match unsafe { ncurses::setcchar(cchar_ptr, wchar_buf.as_ptr(), attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => {
            assert!(!cchar_ptr.is_null(), "{}setcchar() : cchar_ptr.is_null()", MODULE_PATH);

            Ok(ComplexChar::from(unsafe { slice::from_raw_parts(cchar_ptr, 1)[0] }))
        },
        rc => Err(ncurses_function_error_with_rc!("setcchar", rc))
    }
}

/// Set the scrolling region ro `region`. All scrolling actions will take place in this region.
pub fn setscrreg(region: Region) -> result!(()) {
    match ncurses::setscrreg(region.top, region.bottom) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("setscrreg", rc))
    }
}

/// Set the virtual screen cursor to `origin`. If `None` then `leaveok()` is set `true`.
pub fn setsyx(origin: Option<Origin>) -> result!(()) {
    if let Some(origin) = origin {
        leaveok(newscr(), false)?;
        wmove(newscr(), origin)
    } else {
        leaveok(newscr(), true)
    }
}

/// Retrieve attributes of soft label.
pub fn slk_attr() -> normal::Attributes {
    normal::Attributes::_from(None, ncurses::slk_attr())
}

/// Turn off soft label attributes, without affecting other attributes.
pub fn slk_attr_off<A, T>(attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::slk_attr_off(attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attr_off", rc))
    }
}

/// Turn on soft label attributes, without affecting other attributes.
pub fn slk_attr_on<A, T>(attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::slk_attr_on(attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attr_on", rc))
    }
}

/// Sets the soft label attributes to `attrs`, with color specified by `color_pair`.
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

/// Turn off attribute `attrs` of the soft labels.
pub fn slk_attroff(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attroff(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attroff", rc))
    }
}

/// Turn on attribute `attrs` of the soft labels.
pub fn slk_attron(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attron(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attron", rc))
    }
}

/// Sets attribute `attrs` of the soft labels.
pub fn slk_attrset(attrs: normal::Attributes) -> result!(()) {
    match ncurses::slk_attrset(normal::Attributes::into(attrs)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attrset", rc))
    }
}

/// Clears the soft labels from the screen.
pub fn slk_clear() -> result!(()) {
    match ncurses::slk_clear() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_clear", rc))
    }
}

/// Corresponds to `color_set()` routine. It has an effect only if
/// soft labels are simulated on the bottom line of the screen.
pub fn slk_color(color_pair: normal::ColorPair) -> result!(()) {
    match ncurses::slk_color(color_pair.number()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_color", rc))
    }
}

/// The `slk_init()` routine must be called before `initscr()` or `newterm()`
/// is called. `initscr()` eventually uses a line from `stdscr()` to emulate
/// the soft labels, then `fmt` determines how the labels are arranged on
/// the screen.
pub fn slk_init(fmt: SoftLabelType) -> result!(()) {
    match ncurses::slk_init(fmt.value()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_init", rc))
    }
}

/// Returns the current label for label number `labnum`, with leading and
/// trailing blanks stripped.
pub fn slk_label(labnum: i32) -> Option<String> {
    ncurses::slk_label(labnum)
}

/// Mark for refresh but wait. This function updates the data structure representing
/// the desired state of the soft labels, but does not force an update of
/// the physical screen. To accomplish that, call `doupdate()`.
pub fn slk_noutrefresh() -> result!(()) {
    match ncurses::slk_noutrefresh() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_noutrefresh", rc))
    }
}

/// Update the soft labels immediately.
pub fn slk_refresh() -> result!(()) {
    match ncurses::slk_refresh() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_refresh", rc))
    }
}

/// Restores the soft labels to the screen after a `slk_clear()` has
/// been performed.
pub fn slk_restore() -> result!(()) {
    match ncurses::slk_restore() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_restore", rc))
    }
}

/// The `slk_set()` routine sets a soft label.
///
/// - labnum: is the label number, from 1 to 8 (12 if `slk_init()` was called
///           with `SoftLabelType::{FourFourFour,FourFourFourIndex}`);
/// - label:  is be the string to put on the label, up to eight (five if
///           `slk_init() was called with `SoftLabelType::{FourFour,FourFourIndex}`)
///           characters in length.
/// - fmt:    indicating whether the label is to be left-justified, centered
///           or right-justified.
pub fn slk_set(labnum: i32, label: Option<&str>, fmt: Justification) -> result!(()) {
    match unsafe { ncurses::slk_set(labnum, option_str_as_ptr!(label), fmt.value()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_set", rc))
    }
}

/// Forces all the soft labels to be output the next time a `slk_noutrefresh()`
/// is performed.
pub fn slk_touch() -> result!(()) {
    match ncurses::slk_touch() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_touch", rc))
    }
}

/// The wide character version of the `slk_set()` routine.
pub fn slk_wset(labnum: i32, label: &WideString, fmt: Justification) -> result!(()) {
    match ncurses::slk_wset(labnum, raw_with_nul_as_slice!(label), fmt.value()) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_set", rc))
    }
}

/// Turn off the standout attribute. On some terminals this has the side
/// effect of turning off all attributes.
pub fn standend() -> result!(()) {
    match ncurses::standend() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("standend", rc))
    }
}

/// Turn on attribute A_STANDOUT.
pub fn standout() -> result!(()) {
    match ncurses::standout() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("standout", rc))
    }
}

/// Must be called if the programmer wants to use colors, and before any other
/// color manipulation routine is called. It is good practice to call this
/// routine right after `initscr()`.
///
/// `start_color()` initializes eight basic colors (black, red, green, yellow,
/// blue, magenta, cyan, and white), and two global variables in the NCurses
/// module, COLORS and COLOR_PAIRS, containing the maximum number of colors
/// and color-pairs the terminal can support. It also restores the colors on
/// the terminal to the values they had when the terminal was just turned on.
pub fn start_color() -> result!(()) {
    match ncurses::start_color() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("start_color", rc))
    }
}

/// Return a sub-window, whose upper-left corner is at `origin`, and whose width/height is `size`.
pub fn subpad(handle: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    unsafe { ncurses::subpad(handle, size.lines, size.columns, origin.y, origin.x).ok_or(ncurses_function_error!("subpad")) }
}

/// Return a sub-window, whose upper-left corner is at `origin`, and whose width/height is `size`.
///
/// By default, the sub-window will extend from the specified position to the
/// lower right corner of the window.
///
/// The sub-window shares memory with the window orig, so that changes made to
/// one window will affect both windows. When using this routine, it is necessary
/// to call `touchwin()` or `touchline()` on orig before calling `wrefresh()`
/// on the sub-window.
pub fn subwin(handle: WINDOW, size: Size, origin: Origin) -> result!(WINDOW) {
    unsafe { ncurses::subwin(handle, size.lines, size.columns, origin.y, origin.x).ok_or(ncurses_function_error!("subwin")) }
}

/// If flag is `true`, then `syncup()` is called automatically whenever there is a change in the window.
pub fn syncok(handle: WINDOW, flag: bool) -> result!(()) {
    match unsafe { ncurses::syncok(handle, flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("syncok", rc))
    }
}

/// At present this function is unimplemented.
pub fn term_attrs() -> attr_t {
    unimplemented!();
}

/// At present this function is unimplemented.
///
/// Return a logical OR of all video attributes supported by the terminal.
/// This information is useful when a NCurses program needs complete control
/// over the appearance of the screen.
pub fn termattrs() -> chtype {
    unimplemented!();
}

/// Return the value of the environment variable TERM, as a string,
/// truncated to 14 characters.
pub fn termname() -> result!(String) {
    ncurses::termname().ok_or(ncurses_function_error!("termname"))
}

/// At present this function is unimplemented.
///
/// Return the value of the Boolean capability corresponding to the terminfo
/// capability name capname as an integer. Return the value -1 if capname is
/// not a Boolean capability, or 0 if it is canceled or absent from the
/// terminal description.
pub fn tigetflag(_capname: &str) -> i32 {
    unimplemented!();
}

/// At present this function is unimplemented.
///
/// Return the value of the numeric capability corresponding to the terminfo
/// capability name capname as an integer. Return the value -2 if capname is
/// not a numeric capability, or -1 if it is canceled or absent from the
/// terminal description.
pub fn tigetnum(_capname: &str) -> i32 {
    unimplemented!();
}

/// At present this function is unimplemented.
///
/// Return the value of the string capability corresponding to the terminfo
/// capability name capname as a bytes object. Return None if capname is not
/// a terminfo “string capability”, or is canceled or absent from the terminal
/// description.
pub fn tigetstr(_capname: &str) -> String {
    unimplemented!();
}

/// Set blocking or non-blocking read behavior for the window.
pub fn timeout(ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    ncurses::timeout(ms);

    Ok(())
}

/// Pretend that `count` lines have been changed, beginning with line `start`.
///
/// This routines throw away all optimization information about which parts
/// of the window have been touched, by pretending that the entire window
/// has been drawn on. This is sometimes necessary when using overlapping
/// windows, since a change to one window affects the other window, but the
/// records of which lines have been changed in the other window do not
/// reflect the change.
pub fn touchline(handle: WINDOW, count: i32, start: i32) -> result!(()) {
    match unsafe { ncurses::touchline(handle, count, start) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("touchline", rc))
    }
}

/// Pretend the whole window has been changed, for purposes of drawing optimizations.
///
/// This routines throw away all optimization information about which parts
/// of the window have been touched, by pretending that the entire window
/// has been drawn on. This is sometimes necessary when using overlapping
/// windows, since a change to one window affects the other window, but the
/// records of which lines have been changed in the other window do not
/// reflect the change.
pub fn touchwin(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::touchwin(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("touchwin", rc))
    }
}

/// At present this function is unimplemented.
pub fn tparm(_s: &str) -> String {
    unimplemented!();
}

/// Specify that the file descriptor `file` be used for typeahead checking.
/// If `file` is None, then no typeahead checking is done.
///
/// The NCurses library does “line-breakout optimization” by looking for
/// typeahead periodically while updating the screen. If input is found,
/// and it is coming from a tty, the current update is postponed until
/// refresh or doupdate is called again, allowing faster response to
/// commands typed in advance. This function allows specifying a different
/// file descriptor for typeahead checking.
pub fn typeahead<FD: AsRawFd + Read>(file: Option<FD>) -> result!(()) {
    match ncurses::typeahead(file.map_or_else(|| -1, |file| file.as_raw_fd())) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("typeahead", rc))
    }
}

/// Return a string which is a printable representation of the character
/// c. Control characters are represented as a caret followed by the character,
/// for example as b'^C'. Printing characters are left as they are.
pub fn unctrl(c: ChtypeChar) -> result!(String) {
    ncurses::unctrl(ChtypeChar::into(c)).ok_or(ncurses_function_error!("unctrl"))
}

/// Push ch so the next `get_wch()` will return it.
///
/// Note: Only one ch can be pushed before `get_wch()` is called.
pub fn unget_wch(ch: WideChar) -> result!(()) {
    match ncurses::unget_wch(WideChar::into(ch)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("unget_wch", rc))
    }
}

/// Push `ch` so the next `getch()` will return it.
///
/// Note: Only one `ch` can be pushed before `getch()` is called.
pub fn ungetch(ch: char) -> result!(()) {
    match ncurses::ungetch(i32::from(ch as u8)) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ungetch", rc))
    }
}

/// Mark all lines in the window as unchanged since the last call to `refresh()`.
pub fn untouchwin(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::untouchwin(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("untouchwin", rc))
    }
}

/// Allow use of default values for colors on terminals supporting this
/// feature. Use this to support transparency in your application.
/// The default color is assigned to the color `Color::TerminalDefault`.
///
/// The following are equivalent:
/// ```text
/// use_default_colors()?;
/// assume_default_colors(Colors::new(Color::TerminalDefault, Color::TerminalDefault));
/// ```
pub fn use_default_colors() -> result!(()) {
    match ncurses::use_default_colors() {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("use_default_colors", rc))
    }
}

/// If used, this function should be called before `initscr()` or `newterm()`
/// are called. When flag is `false`, the values of lines and columns specified
/// in the terminfo database will be used, even if environment variables
/// LINES and COLUMNS (used by default) are set, or if NCurses is running
/// in a window (in which case default behavior would be to use the window
/// size if LINES and COLUMNS are not set).
pub fn use_env(flag: bool) {
    ncurses::use_env(flag)
}

/// Controls whether the calling application is able to use user-defined or
/// nonstandard names which may be compiled into the terminfo description,
/// i.e., via the terminfo or termcap interfaces.
/// Normally these names are available for use, since the essential decision
/// is made by using the -x option of tic to compile extended terminal
/// definitions. However you can disable this feature to ensure compatibility
/// with other implementations of curses.
pub fn use_extended_names(enable: bool) -> bool {
    ncurses::use_extended_names(enable) == TRUE
}

/// Override locale-encoding checks.
///
/// The `use_legacy_coding()` function is an extension to the NCurses library.
/// It allows the caller to change the result of `unctrl()`, and suppress
/// related checks within the library that would normally cause nonprinting
/// characters to be rendered in visible form. This affects only 8-bit characters.
pub fn use_legacy_coding(level: Legacy) -> result!(Legacy) {
    let rc = ncurses::use_legacy_coding(level.value());

    Legacy::new(rc).ok_or(ncurses_function_error_with_rc!("use_legacy_coding", rc))
}

/// Determine how to compute terminal size. The `use_tioctl()` routine,
/// if used, should be called before `initscr()` or `newterm()` are called
/// (because those compute the screen size).
///
/// After `use_tioctl()` is called with `true` as an argument, NCurses
/// modifies the last step in its computation of screen size as follows:
/// - Checks if the $LINES and $COLUMNS environment variables are set to
///   a number greater than zero.
/// - for each, NCurses updates the corresponding environment variable
///   with the value that it has obtained via operating system call or
///   from the terminal database.
/// - NCurses re-fetches the value of the environment variables so that
///   it is still the environment variables which set the screen size.
///
/// The `use_env()` and `use_tioctl()` routines combine as summarized here:
///
/// use_env   use_tioctl   Summary
/// ----------------------------------------------------------------
/// true      false        This is the default behavior. NCurses uses operating
///                        system calls unless overridden by $LINES or $COLUMNS
///                        environment variables.
/// true      true         NCurses updates $LINES and $COLUMNS based on operating
///                        system calls.
/// false     true         NCurses ignores $LINES and $COLUMNS, uses operating
///                        system calls to obtain size.
/// false     false        NCurses relies on the terminal database to determine size.
pub fn use_tioctl(flag: bool) {
    ncurses::use_tioctl(flag)
}

/// At present this function is unimplemented.
pub fn vid_attr(_attrs: attr_t, _pair: short_t) -> i32 {
    unimplemented!();
}

/// At present this function is unimplemented.
pub fn vidattr(_attrs: chtype) -> i32 {
    unimplemented!();
}

/// Display a vertical line with length `number` consisting of the character `ch`.
pub fn vline(ch: ChtypeChar, number: i32) -> result!(()) {
    match ncurses::vline(ChtypeChar::into(ch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("vline", rc))
    }
}

/// Display a vertical line with length `number` consisting of the character `ch`.
pub fn vline_set(wch: ComplexChar, number: i32) -> result!(()) {
    match ncurses::vline_set(&ComplexChar::into(wch), number) {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("vline_set", rc))
    }
}

/// Equivalent of `mvwadd_wch()` using `getcuryx()` as `origin`.
pub fn wadd_wch(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::wadd_wch(handle, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wadd_wch", rc))
    }
}

/// Equivalent of `mvwadd_wchnstr()` using `getcuryx()` as `origin`.
pub fn wadd_wchnstr(handle: WINDOW, wchstr: &ComplexString, number: i32) -> result!(()) {
    match unsafe { ncurses::wadd_wchnstr(handle, raw_with_nul_as_slice!(wchstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wadd_wchnstr", rc))
    }
}

/// Equivalent of `mvwadd_wchstr()` using `getcuryx()` as `origin`.
pub fn wadd_wchstr(handle: WINDOW, wchstr: &ComplexString) -> result!(()) {
    match unsafe { ncurses::wadd_wchstr(handle, raw_with_nul_as_slice!(wchstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wadd_wchstr", rc))
    }
}

/// Equivalent of `mvwaddch()` using `getcuryx()` as `origin`.
pub fn waddch(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::waddch(handle, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddch", rc))
    }
}

/// Equivalent of `mvwaddchnstr()` using `getcuryx()` as `origin`.
pub fn waddchnstr(handle: WINDOW, chstr: &ChtypeString, number: i32) -> result!(()) {
    match unsafe { ncurses::waddchnstr(handle, raw_with_nul_as_slice!(chstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddchnstr", rc))
    }
}

/// Equivalent of `mvwaddchstr()` using `getcuryx()` as `origin`.
pub fn waddchstr(handle: WINDOW, chstr: &ChtypeString) -> result!(()) {
    match unsafe { ncurses::waddchstr(handle, raw_with_nul_as_slice!(chstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddchstr", rc))
    }
}

/// Equivalent of `mvwaddnstr()` using `getcuryx()` as `origin`.
pub fn waddnstr(handle: WINDOW, str: &str, number: i32) -> result!(()) {
    match unsafe { ncurses::waddnstr(handle, c_str_with_nul!(str), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddnstr", rc))
    }
}

/// Equivalent of `mvwaddnwstr()` using `getcuryx()` as `origin`.
pub fn waddnwstr(handle: WINDOW, wstr: &WideString, number: i32) -> result!(()) {
    match unsafe { ncurses::waddnwstr(handle, raw_with_nul_as_slice!(wstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddnwstr", rc))
    }
}

/// Equivalent of `mvwaddstr()` using `getcuryx()` as `origin`.
pub fn waddstr(handle: WINDOW, str: &str) -> result!(()) {
    match unsafe { ncurses::waddstr(handle, c_str_with_nul!(str)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddstr", rc))
    }
}

/// Equivalent of `mvwaddwstr()` using `getcuryx()` as `origin`.
pub fn waddwstr(handle: WINDOW, wstr: &WideString) -> result!(()) {
    match unsafe { ncurses::waddwstr(handle, raw_with_nul_as_slice!(wstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("waddwstr", rc))
    }
}

/// Retrieve attributes for the given window.
pub fn wattr_get(handle: WINDOW) -> result!(AttributesColorPairSet) {
    let mut attrs: [attr_t; 1] = [0];
    let mut color_pair: [short_t; 1] = [0];
    let mut opts: [i32; 1] = [0];

    match unsafe { ncurses::wattr_get(handle, attrs.as_mut_ptr(), color_pair.as_mut_ptr(), opts.as_mut_ptr() as *mut c_void) } {
        OK => Ok(match ncurses_colortype() {
                     NCursesColorType::Normal => {
                         AttributesColorPairSet::Normal(
                             normal::AttributesColorPair::new(
                                 normal::Attributes::_from(None, attrs[0]),
                                 normal::ColorPair::_from(None, color_pair[0])
                             )
                         )
                     },
                     NCursesColorType::Extend => {
                         AttributesColorPairSet::Extend(
                             extend::AttributesColorPair::new(
                                 extend::Attributes::_from(None, attrs[0]),
                                 extend::ColorPair::_from(None, opts[0])
                             )
                         )
                     }
              }),
        rc => Err(ncurses_function_error_with_rc!("wattr_get", rc))
    }
}

/// Turn off window attributes, without affecting other attributes.
pub fn wattr_off<A, T>(handle: WINDOW, attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::wattr_off(handle, attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattr_off", rc))
    }
}

/// Turn on window attributes, without affecting other attributes.
pub fn wattr_on<A, T>(handle: WINDOW, attrs: A) -> result!(())
    where A: AttributesType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::wattr_on(handle, attrs.as_attr_t(), ptr::null_mut()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattr_on", rc))
    }
}

/// Sets the current attributes of the given window to `attrs`,
/// with color specified by `color_pair`.
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

/// Remove attribute attrs from the “background” set applied to all writes
/// to the current window.
pub fn wattroff(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match unsafe { ncurses::wattroff(handle, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattroff", rc))
    }
}

/// Add attribute attrs from the “background” set applied to all writes to
/// the current window.
pub fn wattron(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match unsafe { ncurses::wattron(handle, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattron", rc))
    }
}

/// Set the “background” set of attributes to attrs. This set is initially
/// no attributes.
pub fn wattrset(handle: WINDOW, attrs: normal::Attributes) -> result!(()) {
    match unsafe { ncurses::wattrset(handle, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wattrset", rc))
    }
}

/// Set the background property of the specified window and then apply this setting
/// to every character position in that window.
///
/// According to X/Open Curses, it should do this:
/// - The rendition of every character on the screen is changed to the new background rendition.
/// - Wherever the former background character appears, it is changed to the new background character.
pub fn wbkgd(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::wbkgd(handle, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wbkgd", rc))
    }
}

/// Manipulate the background of the specified window. The window background is a
/// character (with rendition). The attribute part of the background is combined
/// (OR'ed) with all non-blank characters that are written into the window with
/// `waddch()`. Both the character and attribute parts of the background are
/// combined with the blank characters. The background becomes a property of the
/// character and moves with the character through any scrolling and insert/delete
/// line/character operations.
///
/// To the extent possible on a particular terminal, the attribute part of the
/// background is displayed as the graphic rendition of the character put on the screen.
pub fn wbkgdset(handle: WINDOW, ch: ChtypeChar) {
    unsafe { ncurses::wbkgdset(handle, ChtypeChar::into(ch)) }
}

/// Set the background property of the window to the complex character `wch`.
/// The change is then applied to every character position in that window:
/// - The attribute of every character in the window is changed to the new
///   background attribute.
/// - Wherever the former background character appears, it is changed to
///   the new background character.
pub fn wbkgrnd(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::wbkgrnd(handle, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wbkgrnd", rc))
    }
}

/// Set the window’s background. A window’s background consists of a character
/// and it's combination of attributes as a complex character. The attribute
/// part of the background is combined (OR’ed) with all non-blank characters
/// that are written into the window. Both the character and attribute parts
/// of the background are combined with the blank characters. The background
/// becomes a property of the character and moves with the character through
/// any scrolling and insert/delete line/character operations.
pub fn wbkgrndset(handle: WINDOW, wch: ComplexChar) {
    unsafe { ncurses::wbkgrndset(handle, &ComplexChar::into(wch)) }
}

/// Draw a border around the edges of the window. Each parameter specifies the
/// character to use for a specific part of the border.
///
/// See the table below for more details.
///
/// ls : Left side
/// rs : Right side
/// ts : Top
/// bs : Bottom
/// tl : Upper-left corner
/// tr : Upper-right corner
/// bl : Bottom-left corner
/// br : Bottom-right corner
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

/// Draw a border around the edges of the window. Each parameter specifies the
/// character to use for a specific part of the border.
///
/// See the table below for more details.
///
/// ls : Left side
/// rs : Right side
/// ts : Top
/// bs : Bottom
/// tl : Upper-left corner
/// tr : Upper-right corner
/// bl : Bottom-left corner
/// br : Bottom-right corner
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

/// Equivalent of `mvwchgat()` using `getcuryx()` as `origin`.
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

/// Like `erase()`, but also cause the whole window to be repainted upon
/// next call to `refresh()`.
pub fn wclear(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::wclear(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wclear", rc))
    }
}

/// Erase from cursor to the end of the window: all lines below the cursor
/// are deleted, and then the equivalent of `clrtoeol()` is performed.
pub fn wclrtobot(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::wclrtobot(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wclrtobot", rc))
    }
}

/// Erase from cursor to the end of the line.
pub fn wclrtoeol(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::wclrtoeol(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wclrtoeol", rc))
    }
}

/// Sets the current color of the given window to the foreground/background
/// combination described by the color_pair parameter.
pub fn wcolor_set<P, T>(handle: WINDOW, color_pair: P) -> result!(())
    where P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    match unsafe { ncurses::wcolor_set(handle, color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wcolor_set", rc))
    }
}

/// Update the current cursor position of all the ancestors of the window
/// to reflect the current cursor position of the window.
pub fn wcursyncup(handle: WINDOW) {
    unsafe { ncurses::wcursyncup(handle) }
}

/// Delete any character at current position.
pub fn wdelch(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::wdelch(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wdelch", rc))
    }
}

/// Add a complex character `wch`, and immediately call `refresh()` on the window.
pub fn wecho_wchar(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::wecho_wchar(handle, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wecho_wchar", rc))
    }
}

/// Add a character (withrendition) `ch`, and immediately call `refresh()` on the window.
pub fn wechochar(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::wechochar(handle, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wechochar", rc))
    }
}

/// Clear the window.
pub fn werase(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::werase(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("werase", rc))
    }
}

/// Equivalent of `mvwget_wch()` using `getcuryx()` as `origin`.
pub fn wget_wch(handle: WINDOW) -> result!(CharacterResult<WideChar>) {
    let mut wch: [wint_t; 1] = [0];

    match unsafe { ncurses::wget_wch(handle, wch.as_mut_ptr()) } {
        EINTR        => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE   => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT    => Err(NCurseswError::KeyEvent),
        KEY_CODE_YES => {
            match i32::try_from(wch[0])? {
                #[cfg(feature = "key_resize_as_error")]
                KEY_RESIZE => Err(NCurseswError::KeyResize),
                #[cfg(feature = "key_event_as_error")]
                KEY_EVENT  => Err(NCurseswError::KeyEvent),
                _          => Ok(CharacterResult::Key(KeyBinding::try_from(wch[0])?))
            }
        },
        rc           => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("wget_wch", rc))
            } else {
                Ok(CharacterResult::Character(WideChar::from(wch[0])))
            }
        }
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use wgetn_wstr() instead")]
/// Equivalent of `mvwget_wstr()` using `getcuryx()` as `origin`.
pub fn wget_wstr(handle: WINDOW) -> result!(WideString) {
    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::wget_wstr(handle, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("wget_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}wget_wstr() : ptr.is_null()", MODULE_PATH);

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

/// Returns the specified window's current background character as a complex character.
pub fn wgetbkgrnd(handle: WINDOW) -> result!(ComplexChar) {
    let mut wch: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::wgetbkgrnd(handle, wch.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wch[0])),
        rc => Err(ncurses_function_error_with_rc!("wgetbkgrnd", rc))
    }
}

/// Equivalent of `mvwgetch()` using `getcuryx()` as `origin`.
pub fn wgetch(handle: WINDOW) -> result!(CharacterResult<char>) {
    match unsafe { ncurses::wgetch(handle) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        #[cfg(feature = "key_resize_as_error")]
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        #[cfg(feature = "key_event_as_error")]
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("wgetch", rc))
            } else if rc >= KEY_MIN && rc <= KEY_MAX {
                Ok(CharacterResult::Key(KeyBinding::from(rc)))
            } else {
                Ok(CharacterResult::Character(char::from(u8::try_from(i8::try_from(rc)?)?)))
            }
        }
    }
}

/// Returns the delay timeout as set in `wtimeout()`.
pub fn wgetdelay(handle: WINDOW) -> result!(time::Duration) {
    Ok(time::Duration::from_millis(u64::try_from(unsafe { ncurses::wgetdelay(handle) })?))
}

/// Equivalent of `mvwgetn_wstr()` using `getcuryx()` as `origin`.
pub fn wgetn_wstr(handle: WINDOW, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "{}wgetn_wstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [wint_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wint_t = buf.as_mut_ptr();

    match unsafe { ncurses::wgetn_wstr(handle, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("wgetn_wstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}wgetn_wstr() : ptr.is_null()", MODULE_PATH);

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

/// Equivalent of `mvwgetnstr()` using `getcuryx()` as `origin`.
pub fn wgetnstr(handle: WINDOW, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "{}wgetnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::wgetnstr(handle, ptr, number) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("wgetnstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}wgetnstr() : ptr.is_null()", MODULE_PATH);

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

/// Returns the parent WINDOW pointer for sub-windows, or `None` for
/// windows having no parent.
pub fn wgetparent(handle: WINDOW) -> Option<WINDOW> {
    unsafe { ncurses::wgetparent(handle) }
}

/// Returns the top and bottom rows for the scrolling margin as set
/// by `wsetscrreg()`.
pub fn wgetscrreg(handle: WINDOW) -> result!(Region) {
    let mut top: [i32; 1] = [0];
    let mut bottom: [i32; 1] = [0];

    match unsafe { ncurses::wgetscrreg(handle, top.as_mut_ptr(), bottom.as_mut_ptr()) } {
        OK => Ok(Region { top: top[0], bottom: bottom[0] }),
        rc => Err(ncurses_function_error_with_rc!("wgetscrreg", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use wgetnstr() instead")]
/// Equivalent of `mvwgetstr()` using `getcuryx()` as `origin`.
pub fn wgetstr(handle: WINDOW) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    match unsafe { ncurses::wgetstr(handle, ptr) } {
        EINTR      => Err(NCurseswError::InterruptedCall),
        KEY_RESIZE => Err(NCurseswError::KeyResize),
        KEY_EVENT  => Err(NCurseswError::KeyEvent),
        rc         => {
            if rc.is_negative() {
                Err(ncurses_function_error_with_rc!("wgetstr", rc))
            } else {
                assert!(!ptr.is_null(), "{}wgetstr() : ptr.is_null()", MODULE_PATH);

                Ok(unsafe { FromCStr::from_c_str(ptr) })
            }
        }
    }
}

/// Equivalent of `mvwhline()` using `getcuryx()` as `origin`.
pub fn whline(handle: WINDOW, ch: ChtypeChar, number: i32) -> result!(()) {
    match unsafe { ncurses::whline(handle, ChtypeChar::into(ch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("whline", rc))
    }
}

/// Equivalent of `mvwhline_set()` using `getcuryx()` as `origin`.
pub fn whline_set(handle: WINDOW, wch: ComplexChar, number: i32) -> result!(()) {
    match unsafe { ncurses::whline_set(handle, &ComplexChar::into(wch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("whline_set", rc))
    }
}

/// Equivalent of `mvwin_wch()` using `getcuryx()` as `origin`.
pub fn win_wch(handle: WINDOW) -> result!(ComplexChar) {
    let mut wcval: [cchar_t; 1] = unsafe { mem::zeroed() };

    match unsafe { ncurses::win_wch(handle, wcval.as_mut_ptr()) } {
        OK => Ok(ComplexChar::from(wcval[0])),
        rc => Err(ncurses_function_error_with_rc!("win_wch", rc))
    }
}

/// Equivalent of `mvwin_wchnstr()` using `getcuryx()` as `origin`.
pub fn win_wchnstr(handle: WINDOW, number: i32) -> result!(ComplexString) {
    assert!(number <= LINE_MAX as i32, "{}win_wchnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::win_wchnstr(handle, ptr, number) } {
        OK => {
            assert!(!ptr.is_null(), "{}win_wchnstr() : ptr.is_null()", MODULE_PATH);

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(number)?) }))
        },
        rc => Err(ncurses_function_error_with_rc!("win_wchnstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use win_wchnstr() instead")]
/// Equivalent of `mvwin_wchstr()` using `getcuryx()` as `origin`.
pub fn win_wchstr(handle: WINDOW) -> result!(ComplexString) {
    let mut buf: [cchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut cchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::win_wchstr(handle, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "{}win_wchstr() : ptr.is_null()", MODULE_PATH);

            Ok(ComplexString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("win_wchstr", rc))
    }
}

/// Equivalent of `mvwinch()` using `getcuryx()` as `origin`.
pub fn winch(handle: WINDOW) -> ChtypeChar {
    ChtypeChar::from(unsafe { ncurses::winch(handle) })
}

/// Equivalent of `mvwinchnstr()` using `getcuryx()` as `origin`.
pub fn winchnstr(handle: WINDOW, number: i32) -> result!(ChtypeString) {
    assert!(number <= LINE_MAX as i32, "{}winchnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::winchnstr(handle, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("winchnstr", len))
    } else {
        assert!(!ptr.is_null(), "{}winchnstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}winchnstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use winchnstr() instead")]
/// Equivalent of `mvwinchstr()` using `getcuryx()` as `origin`.
pub fn winchstr(handle: WINDOW) -> result!(ChtypeString) {
    let mut buf: [chtype; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut chtype = buf.as_mut_ptr();

    let len = unsafe { ncurses::winchstr(handle, ptr) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("inchstr", len))
    } else {
        assert!(!ptr.is_null(), "{}winchstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}winchstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(ChtypeString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

/// Equivalent of `mvwinnstr()` using `getcuryx()` as `origin`.
pub fn winnstr(handle: WINDOW, number: i32) -> result!(String) {
    assert!(number <= LINE_MAX as i32, "{}winnstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::winnstr(handle, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("winnstr", len))
    } else {
        assert!(!ptr.is_null(), "{}winnstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}winnstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

/// Equivalent of `mvwinnwstr()` using `getcuryx()` as `origin`.
pub fn winnwstr(handle: WINDOW, number: i32) -> result!(WideString) {
    assert!(number <= LINE_MAX as i32, "{}winnwstr() : number={}, LINE_MAX={}", MODULE_PATH, number, LINE_MAX);

    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    let len = unsafe { ncurses::winnwstr(handle, ptr, number) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("winnwstr", len))
    } else {
        assert!(!ptr.is_null(), "{}winnwstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}winnwstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, usize::try_from(len)?) }))
    }
}

/// Equivalent of `mvwins_nwstr()` using `getcuryx()` as `origin`.
pub fn wins_nwstr(handle: WINDOW, wstr: &WideString, number: i32) -> result!(()) {
    match unsafe { ncurses::wins_nwstr(handle, raw_with_nul_as_slice!(wstr), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wins_nwstr", rc))
    }
}

/// Equivalent of `mvwins_wch()` using `getcuryx()` as `origin`.
pub fn wins_wch(handle: WINDOW, wch: ComplexChar) -> result!(()) {
    match unsafe { ncurses::wins_wch(handle, &ComplexChar::into(wch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wins_wch", rc))
    }
}

/// Equivalent of `mvwins_wstr()` using `getcuryx()` as `origin`.
pub fn wins_wstr(handle: WINDOW, wstr: &WideString) -> result!(()) {
    match unsafe { ncurses::wins_wstr(handle, raw_with_nul_as_slice!(wstr)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wins_wstr", rc))
    }
}

/// Equivalent of `mvwinsch()` using `getcuryx()` as `origin`.
pub fn winsch(handle: WINDOW, ch: ChtypeChar) -> result!(()) {
    match unsafe { ncurses::winsch(handle, ChtypeChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsch", rc))
    }
}

/// For positive `n`, insert `n` lines into the specified window above the
/// current line. The `n` bottom lines are lost. For negative `n`, delete
/// `n` lines (starting with the one under the cursor), and move the
/// remaining lines up. The bottom `n` lines are cleared. The current
/// cursor position remains the same.
pub fn winsdelln(handle: WINDOW, n: i32) -> result!(()) {
    match unsafe { ncurses::winsdelln(handle, n) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsdelln", rc))
    }
}

/// Insert a blank line under the cursor. All following lines are moved down by one line.
pub fn winsertln(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::winsertln(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsertln", rc))
    }
}

/// Equivalent of `mvwinsnstr()` using `getcuryx()` as `origin`.
pub fn winsnstr(handle: WINDOW, str: &str, number: i32) -> result!(()) {
    match unsafe { ncurses::winsnstr(handle, c_str_with_nul!(str), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsnstr", rc))
    }
}

/// Equivalent of `mvwinsstr()` using `getcuryx()` as `origin`.
pub fn winsstr(handle: WINDOW, str: &str) -> result!(()) {
    match unsafe { ncurses::winsstr(handle, c_str_with_nul!(str)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("winsstr", rc))
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use winnstr() instead")]
/// Equivalent of `mvwinstr()` using `getcuryx()` as `origin`.
pub fn winstr(handle: WINDOW) -> result!(String) {
    let mut buf: [i8; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut i8 = buf.as_mut_ptr();

    let len = unsafe { ncurses::winstr(handle, ptr) };

    if len.is_negative() {
        Err(ncurses_function_error_with_rc!("winstr", len))
    } else {
        assert!(!ptr.is_null(), "{}winstr() : ptr.is_null()", MODULE_PATH);
        assert!(len > 0 && len <= LINE_MAX as i32, "{}winstr() : len={}, LINE_MAX={}", MODULE_PATH, len, LINE_MAX);

        Ok(unsafe { FromCStr::from_c_str(ptr) })
    }
}

#[deprecated(since = "0.1.2", note = "underlying native function can cause undefined behaviour. Use winnwstr() instead")]
/// Equivalent of `mvwinwstr()` using `getcuryx()` as `origin`.
pub fn winwstr(handle: WINDOW) -> result!(WideString) {
    let mut buf: [wchar_t; LINE_MAX] = unsafe { mem::zeroed() };
    let ptr: *mut wchar_t = buf.as_mut_ptr();

    match unsafe { ncurses::winwstr(handle, ptr) } {
        OK => {
            assert!(!ptr.is_null(), "{}winwstr() : ptr.is_null()", MODULE_PATH);

            Ok(WideString::from(unsafe { slice::from_raw_parts(ptr, LINE_MAX) }))
        },
        rc => Err(ncurses_function_error_with_rc!("winwstr", rc))
    }
}

/// Move cursor to `origin`.
pub fn wmove(handle: WINDOW, origin: Origin) -> result!(()) {
    match unsafe { ncurses::wmove(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wmove", rc))
    }
}

/// Mark for refresh but wait. This function updates the data structure representing
/// the desired state of the window, but does not force an update of the physical
/// screen. To accomplish that, call `doupdate()`.
pub fn wnoutrefresh(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::wnoutrefresh(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wnoutrefresh", rc))
    }
}

/// Indicate that `region` are corrupted and should be completely redrawn on the
/// next `refresh()` call.
pub fn wredrawln(handle: WINDOW, region: Region) -> result!(()) {
    match unsafe { ncurses::wredrawln(handle, region.top, region.bottom) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wredrawln", rc))
    }
}

/// Update the display immediately (sync actual screen with previous
/// drawing/deleting methods).
pub fn wrefresh(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::wrefresh(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wrefresh", rc))
    }
}

/// Reallocate storage for a NCurses window to adjust its dimensions to the
/// specified values. If either dimension is larger than the current values,
/// the window’s data is filled with blanks that have the current background
/// rendition (as set by `bkgdset()`) merged into them.
pub fn wresize(handle: WINDOW, size: Size) -> result!(()) {
    match unsafe { ncurses::wresize(handle, size.lines, size.columns) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wresize", rc))
    }
}

/// For positive n, the `wscrl()` routine scroll the window up `n` lines
/// (line i+n becomes i); otherwise scroll the window down `n` lines.
/// This involves moving the lines in the window character image structure.
/// The current cursor position is not changed.
///
/// For these functions to work, scrolling must be enabled via `scrollok()`.
pub fn wscrl(handle: WINDOW, n: i32) -> result!(()) {
    match unsafe { ncurses::wscrl(handle, n) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wscrl", rc))
    }
}

/// Set the scrolling region ro `region`. All scrolling actions will take place in this region.
pub fn wsetscrreg(handle: WINDOW, region: Region) -> result!(()) {
    match unsafe { ncurses::wsetscrreg(handle, region.top, region.bottom) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wsetscrreg", rc))
    }
}

/// Turn off the standout attribute. On some terminals this has the side
/// effect of turning off all attributes.
pub fn wstandend(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::wstandend(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wstandend", rc))
    }
}

/// Turn on attribute A_STANDOUT.
pub fn wstandout(handle: WINDOW) -> result!(()) {
    match unsafe { ncurses::wstandout(handle) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wstandout", rc))
    }
}

/// Touch each location in the window that has been touched in any of its
/// ancestor windows. This routine is called by `refresh()`, so it should
/// almost never be necessary to call it manually.
pub fn wsyncdown(handle: WINDOW) {
    unsafe { ncurses::wsyncdown(handle) }
}

/// Touch all locations in ancestors of the window that have been changed in the window.
pub fn wsyncup(handle: WINDOW) {
    unsafe { ncurses::wsyncup(handle) }
}

/// Set blocking or non-blocking read behavior for the window.
pub fn wtimeout(handle: WINDOW, ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    unsafe { ncurses::wtimeout(handle, ms) };

    Ok(())
}

/// The `wtouchln()` routine makes `n` lines in the window, starting at `line`,
/// look as if they have (Changed::True) or have not (Changed::False) been
/// changed since the last call to `wrefresh()`.
pub fn wtouchln(handle: WINDOW, line: i32, n: i32, changed: Changed) -> result!(()) {
    let change = match changed {
        Changed::True  => 1,
        Changed::False => 0
    };

    match unsafe { ncurses::wtouchln(handle, line, n, change) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wtouchln", rc))
    }
}

/// The `wunctrl()` routine returns a wide character string which is a printable
/// representation of the character `ch`, ignoring attributes. Control characters
/// are displayed in the ^X notation. Printing characters are displayed as is.
pub fn wunctrl(ch: ComplexChar) -> result!(WideChar) {
    let mut wch: [cchar_t; 1] = [ComplexChar::into(ch)];

    match unsafe { ncurses::wunctrl(wch.as_mut_ptr()) } {
        Some(ptr) => Ok(WideChar::from(unsafe { wchar_t::try_from(slice::from_raw_parts(ptr, 1)[0])? })),
        None      => Err(ncurses_function_error!("wunctrl"))
    }
}

/// Equivalent of `mvwvline()` using `getcuryx()` as `origin`.
pub fn wvline(handle: WINDOW, ch: ChtypeChar, number: i32) -> result!(()) {
    match unsafe { ncurses::wvline(handle, ChtypeChar::into(ch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wvline", rc))
    }
}

/// Equivalent of `mvwvline_set()` using `getcuryx()` as `origin`.
pub fn wvline_set(handle: WINDOW, wch: ComplexChar, number: i32) -> result!(()) {
    match unsafe { ncurses::wvline_set(handle, &ComplexChar::into(wch), number) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("wvline_set", rc))
    }
}

// screen `_sp` functions.

/// Screen function of `assume_default_colors()`.
pub fn assume_default_colors_sp<S, C, T>(screen: SCREEN, colors: S) -> result!(())
    where S: ColorsType<C, T>,
          C: ColorType<T>,
          T: ColorAttributeTypes
{
    assert!(screen == colors.screen().map_or_else(|| ptr::null_mut(), |screen| screen));

    match unsafe { ncurses::assume_default_colors_sp(screen, colors.foreground().number(), colors.background().number()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("assume_default_colors_sp", rc))
    }
}

/// Screen function of `baudrate()`.
pub fn baudrate_sp(screen: SCREEN) -> i32 {
    unsafe { ncurses::baudrate_sp(screen) }
}

/// Screen function of `beep()`.
pub fn beep_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::beep_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("beep_sp", rc))
    }
}

/// Screen function of `can_change_color()`.
pub fn can_change_color_sp(screen: SCREEN) -> bool {
    unsafe { ncurses::can_change_color_sp(screen) }
}

/// Screen function of `cbreak()`.
pub fn cbreak_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::cbreak_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("cbreak_sp", rc))
    }
}

#[deprecated(since = "0.5.0", note = "Use normal::Color::rgb() or shims::ncurses::color_content_sp() instead")]
/// Screen function of `color_content()`.
pub fn color_content_sp(screen: SCREEN, color_number: short_t) -> result!(normal::RGB) {
    let mut r: [short_t; 1] = [0];
    let mut g: [short_t; 1] = [0];
    let mut b: [short_t; 1] = [0];

    match unsafe { ncurses::color_content_sp(screen, color_number, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()) } {
        OK => Ok(normal::RGB::new(r[0], g[0], b[0])),
        rc => Err(ncurses_function_error_with_rc!("color_content_sp", rc))
    }
}

/// Screen function of `curs_set()`.
pub fn curs_set_sp(screen: SCREEN, cursor: CursorType) -> result!(CursorType) {
    let rc = unsafe { ncurses::curs_set_sp(screen, cursor.value()) };

    CursorType::new(rc).ok_or(ncurses_function_error_with_rc!("curs_set_sp", rc))
}

/// Screen function of `define_key()`.
pub fn define_key_sp(screen: SCREEN, definition: Option<&str>, keycode: KeyBinding) -> result!(()) {
    match unsafe { ncurses::define_key_sp(screen, option_str_as_ptr!(definition), KeyBinding::into(keycode)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("define_key_sp", rc))
    }
}

/// Screen function of `def_prog_mode()`.
pub fn def_prog_mode_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::def_prog_mode_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("def_prog_mode_sp", rc))
    }
}

/// Screen function of `def_shell_mode()`.
pub fn def_shell_mode_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::def_shell_mode_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("def_shell_mode_sp", rc))
    }
}

/// Screen function of `delay_output()`.
pub fn delay_output_sp(screen: SCREEN, ms: time::Duration) -> result!(()) {
    match unsafe { ncurses::delay_output_sp(screen, i32::try_from(ms.as_millis())?) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("delay_output_sp", rc))
    }
}

/// Screen function of `doupdate()`.
pub fn doupdate_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::doupdate_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("doupdate_sp", rc))
    }
}

/// Screen function of `echo()`.
pub fn echo_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::echo_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("echo_sp", rc))
    }
}

/// Screen function of `endwin()`.
pub fn endwin_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::endwin_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("endwin_sp", rc))
    }
}

/// Screen function of `erasechar()`.
pub fn erasechar_sp(screen: SCREEN) -> result!(char) {
    let rc = unsafe { ncurses::erasechar_sp(screen) };

    if rc.is_negative() {
        Err(ncurses_function_error_with_rc!("erasechar_sp", i32::from(rc)))
    } else {
        Ok(char::from(u8::try_from(rc)?))
    }
}

#[deprecated(since = "0.5.0", note = "Use extend::Color::rgb() or shims::ncurses::extended_color_content_sp() instead")]
/// Screen function of `extended_color_content()`.
pub fn extended_color_content_sp(screen: SCREEN, color_number: i32) -> result!(extend::RGB) {
    let mut r: [i32; 1] = [0];
    let mut g: [i32; 1] = [0];
    let mut b: [i32; 1] = [0];

    match unsafe { ncurses::extended_color_content_sp(screen, color_number, r.as_mut_ptr(), g.as_mut_ptr(), b.as_mut_ptr()) } {
        OK => Ok(extend::RGB::new(r[0], g[0], b[0])),
        rc => Err(ncurses_function_error_with_rc!("extended_color_content_sp", rc))
    }
}

#[deprecated(since = "0.5.0", note = "Use extend::ColorPair::colors() or shims::ncurses::extended_pair_content_sp() instead")]
/// Screen function of `extended_pair_content()`.
pub fn extended_pair_content_sp(screen: SCREEN, color_pair: i32) -> result!(extend::Colors) {
    let mut fg: [i32; 1] = [0];
    let mut bg: [i32; 1] = [0];

    let color_palette = |color_number: i32| extend::ColorPalette::_from(color_number);

    match unsafe { ncurses::extended_pair_content_sp(screen, color_pair, fg.as_mut_ptr(), bg.as_mut_ptr()) } {
        OK => Ok(extend::Colors::new(extend::Color::_from(Some(screen), color_palette(fg[0])), extend::Color::_from(Some(screen), color_palette(bg[0])))),
        rc => Err(ncurses_function_error_with_rc!("extended_pair_content_sp", rc))
    }
}

/// Screen function of `extended_slk_color()`.
pub fn extended_slk_color_sp(screen: SCREEN, color_pair: extend::ColorPair) -> result!(()) {
    match unsafe { ncurses::extended_slk_color_sp(screen, color_pair.number()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("extended_slk_color_sp", rc))
    }
}

/// Screen function of `filter()`.
pub fn filter_sp(screen: SCREEN) {
    unsafe { ncurses::filter_sp(screen) }
}

#[deprecated(since = "0.5.0", note = "specified color_pair must go out of scope before reuse of it's color pair number otherwise unpredicable results may occur.")]
/// Screen function of `free_pair()`.
pub fn free_pair_sp<P, T>(screen: SCREEN, color_pair: P) -> result!(())
    where P:   ColorPairType<T>,
          i32: From<T>,
          T:   ColorAttributeTypes
{
    assert!(screen == color_pair.screen().map_or_else(|| ptr::null_mut(), |screen| screen));

    match unsafe { ncurses::free_pair_sp(screen, i32::from(color_pair.number())) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("free_pair_sp", rc))
    }
}

/// Screen function of `flash()`.
pub fn flash_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::flash_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("flash_sp", rc))
    }
}

/// Screen function of `flushinp()`.
pub fn flushinp_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::flushinp_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("flushinp_sp", rc))
    }
}

/// Screen function of `get_escdelay()`.
pub fn get_escdelay_sp(screen: SCREEN) -> result!(time::Duration) {
    Ok(time::Duration::from_millis(u64::try_from(unsafe { ncurses::get_escdelay_sp(screen) })?))
}

/// Screen function of `getwin()`.
pub fn getwin_sp<I: AsRawFd + Read>(screen: SCREEN, file: &I) -> result!(WINDOW) {
    unsafe { ncurses::getwin_sp(screen, fdopen(file, "r")?).ok_or(ncurses_function_error!("getwin_sp")) }
}

/// Screen function of `halfdelay()`.
pub fn halfdelay_sp(screen: SCREEN, tenths: time::Duration) -> result!(()) {
    match unsafe { ncurses::halfdelay_sp(screen, i32::try_from(tenths.as_secs())? / 10) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("halfdelay_sp", rc))
    }
}

/// Screen function of `has_colors()`.
pub fn has_colors_sp(screen: SCREEN) -> bool {
    unsafe { ncurses::has_colors_sp(screen) }
}

/// Screen function of `has_ic()`.
pub fn has_ic_sp(screen: SCREEN) -> bool {
    unsafe { ncurses::has_ic_sp(screen) }
}

/// Screen function of `has_il()`.
pub fn has_il_sp(screen: SCREEN) -> bool {
    unsafe { ncurses::has_il_sp(screen) }
}

/// Screen function of `has_key()`.
pub fn has_key_sp(screen: SCREEN, ch: KeyBinding) -> bool {
    unsafe { ncurses::has_key_sp(screen, KeyBinding::into(ch)) == TRUE }
}

#[deprecated(since = "0.5.0", note = "Use normal::Color::set_rgb() or shims::ncurses::init_color_sp() instead")]
/// Screen function of `init_color()`.
pub fn init_color_sp(screen: SCREEN, color_number: short_t, rgb: normal::RGB) -> result!(()) {
    if i32::from(color_number) >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match unsafe { ncurses::init_color_sp(screen, color_number, rgb.red(), rgb.green(), rgb.blue()) } {
            OK => {
                set_ncurses_colortype(NCursesColorType::Normal);

                Ok(())
            },
            rc => Err(ncurses_function_error_with_rc!("init_color_sp", rc))
        }
    }
}

#[deprecated(since = "0.4.0", note = "Use extend::Color::set_rgb() or shims::ncurses::init_extended_color_sp() instead")]
/// Screen function of `init_extended_color()`.
pub fn init_extended_color_sp(screen: SCREEN, color_number: i32, rgb: extend::RGB) -> result!(()) {
    if color_number >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match unsafe { ncurses::init_extended_color_sp(screen, color_number, rgb.red(), rgb.green(), rgb.blue()) } {
            OK => {
                set_ncurses_colortype(NCursesColorType::Extend);

                Ok(())
            },
            rc => Err(ncurses_function_error_with_rc!("init_extended_color_sp", rc))
        }
    }
}

#[deprecated(since = "0.5.0", note = "Use extend::ColorPair::new_sp() or shims::ncurses::init_extended_pair_sp() instead")]
/// Screen function of `init_extended_pair()`.
pub fn init_extended_pair_sp(screen: SCREEN, color_pair: i32, colors: extend::Colors) -> result!(extend::ColorPair) {
    if color_pair >= COLOR_PAIRS() {
        Err(NCurseswError::ColorPairLimit)
    } else if colors.foreground().number() >= COLORS() || colors.background().number() >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match unsafe { ncurses::init_extended_pair_sp(screen, color_pair, colors.foreground().number(), colors.background().number()) } {
            OK => Ok(extend::ColorPair::_from(Some(screen), color_pair)),
            rc => Err(ncurses_function_error_with_rc!("init_extended_pair_sp", rc))
        }
    }
}

#[deprecated(since = "0.5.0", note = "Use normal::ColorPair::new_sp() or shims::ncurses::init_pair_sp() instead")]
/// Screen function of `init_pair()`.
pub fn init_pair_sp(screen: SCREEN, color_pair: short_t, colors: normal::Colors) -> result!(normal::ColorPair) {
    if i32::from(color_pair) >= COLOR_PAIRS() {
        Err(NCurseswError::ColorPairLimit)
    } else if colors.foreground().number() >= COLORS() || colors.background().number() >= COLORS() {
        Err(NCurseswError::ColorLimit)
    } else {
        match unsafe { ncurses::init_pair_sp(screen, color_pair, short_t::try_from(colors.foreground().number())?, short_t::try_from(colors.background().number())?) } {
            OK => Ok(normal::ColorPair::_from(Some(screen), color_pair)),
            rc => Err(ncurses_function_error_with_rc!("init_pair_sp", rc))
        }
    }
}

/// Screen function of `intrflush()`.
pub fn intrflush_sp(screen: SCREEN, flag: bool) -> result!(()) {
    match unsafe { ncurses::intrflush_sp(screen, ptr::null_mut(), flag) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("intrflush_sp", rc))
    }
}

/// Screen function of `isendwin()`.
pub fn isendwin_sp(screen: SCREEN) -> bool {
    unsafe { ncurses::isendwin_sp(screen) }
}

/// Screen function of `is_term_resized()`.
pub fn is_term_resized_sp(screen: SCREEN, size: Size) -> bool {
    unsafe { ncurses::is_term_resized_sp(screen, size.lines, size.columns) }
}

/// Screen function of `keybound()`.
pub fn keybound_sp(screen: SCREEN, keycode: KeyBinding, count: i32) -> result!(String) {
    unsafe { ncurses::keybound_sp(screen, KeyBinding::into(keycode), count).ok_or(ncurses_function_error!("keybound_sp")) }
}

/// Screen function of `key_defined()`.
pub fn key_defined_sp(screen: SCREEN, definition: &str) -> result!(KeyBinding) {
    let c = unsafe { ncurses::key_defined_sp(screen, c_str_with_nul!(definition)) };

    if c.is_negative() {
        Err(ncurses_function_error_with_rc!("key_defined_sp", c))
    } else {
        Ok(KeyBinding::from(c))
    }
}

/// Screen function of `keyname()`.
pub fn keyname_sp(screen: SCREEN, c: KeyBinding) -> result!(String) {
    unsafe { ncurses::keyname_sp(screen, KeyBinding::into(c)).ok_or(ncurses_function_error!("keyname_sp")) }
}

/// Screen function of `keyok()`.
pub fn keyok_sp(screen: SCREEN, keycode: KeyBinding, enable: bool) -> result!(()) {
    match unsafe { ncurses::keyok_sp(screen, KeyBinding::into(keycode), enable) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("keyok_sp", rc))
    }
}

/// Screen function of `killchar()`.
pub fn killchar_sp(screen: SCREEN) -> result!(char) {
    let rc = unsafe { ncurses::killchar_sp(screen) };

    if rc.is_negative() {
        Err(ncurses_function_error_with_rc!("killchar_sp", i32::from(rc)))
    } else {
        Ok(char::from(u8::try_from(rc)?))
    }
}

/// Screen function of `longname()`.
pub fn longname_sp(screen: SCREEN) -> result!(String) {
    unsafe { ncurses::longname_sp(screen).ok_or(ncurses_function_error!("longname_sp")) }
}

/// Screen function of `mcprint()`.
pub fn mcprint_sp(screen: SCREEN, data: &[i8], len: i32) -> result!(i32) {
    match unsafe { ncurses::mcprint_sp(screen, data.as_ptr() as *mut i8, len) } {
        ERR => Err(ncurses_os_error!("mcprint_sp")),
        rc  => Ok(rc)
    }
}

/// Screen function of `mvcur()`.
pub fn mvcur_sp(screen: SCREEN, old: Origin, new: Origin) -> result!(()) {
    match unsafe { ncurses::mvcur_sp(screen, old.y, old.x, new.y, new.x) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("mvcur_sp", rc))
    }
}

#[deprecated(since = "0.5.0", note = "ncurses library call superseeded by native rust call. Use std::thread::sleep(dur: std::time::Duration) instead")]
/// Screen function of `namps()`.
pub fn napms_sp(screen: SCREEN, ms: time::Duration) -> result!(()) {
    let ms = i32::try_from(ms.as_millis())?;

    match unsafe { ncurses::napms_sp(screen, ms) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("napms_sp", rc))
    }
}

/// Screen function of `newpad()`.
pub fn newpad_sp(screen: SCREEN, size: Size) -> result!(WINDOW) {
    unsafe { ncurses::newpad_sp(screen, size.lines, size.columns).ok_or(ncurses_function_error!("newpad_sp")) }
}

/// When creating a new screen, the library uses static variables which have
/// been preset, e.g. by `use_env()`, `filter()` etc. With the screen-pointer
/// extension, there are situations where it must create a current screen
/// before the unextended library does. The `new_prescr()` function is used
/// internally to handle these cases. It is also provided as an entrypoint
/// to allow applications to customize the library initialization.
pub fn new_prescr() -> result!(SCREEN) {
    unsafe { ncurses::new_prescr().ok_or(ncurses_function_error!("new_prescr")) }
}

/// Screen function of `newterm()`.
pub fn newterm_sp<O, I>(screen: SCREEN, term: Option<&str>, output: &O, input: &I) -> result!(SCREEN)
    where O: AsRawFd + Write,
          I: AsRawFd + Read
{
    unsafe {
        ncurses::newterm_sp(
            screen,
            option_str_as_ptr!(term),
            fdopen(output, "wb+")?,
            fdopen(input, "rb+")?
        ).ok_or(ncurses_function_error!("newterm_sp"))
    }
}

/// Screen function of `newwin()`.
pub fn newwin_sp(screen: SCREEN, size: Size, origin: Origin) -> result!(WINDOW) {
    unsafe { ncurses::newwin_sp(screen, size.lines, size.columns, origin.y, origin.x).ok_or(ncurses_function_error!("newwin_sp")) }
}

/// Screen function of `nl()`.
pub fn nl_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::nl_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("nl_sp", rc))
    }
}

/// Screen function of `nocbreak()`.
pub fn nocbreak_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::nocbreak_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("nocbreak_sp", rc))
    }
}

/// Screen function of `noecho()`.
pub fn noecho_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::noecho_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("noecho_sp", rc))
    }
}

/// Screen function of `nofilter()`.
pub fn nofilter_sp(screen: SCREEN) {
    unsafe { ncurses::nofilter_sp(screen) }
}

/// Screen function of `nonl()`.
pub fn nonl_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::nonl_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("nonl_sp", rc))
    }
}

/// Screen function of `noqiflush()`.
pub fn noqiflush_sp(screen: SCREEN) {
    unsafe { ncurses::noqiflush_sp(screen) }
}

/// Screen function of `noraw()`.
pub fn noraw_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::noraw_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("noraw_sp", rc))
    }
}

#[deprecated(since = "0.5.0", note = "Use normal::ColorPair::colors() or shims::ncurses::pair_content_sp() instead")]
/// Screen function of `pair_content()`.
pub fn pair_content_sp(screen: SCREEN, color_pair: short_t) -> result!(normal::Colors) {
    let mut fg: [short_t; 1] = [0];
    let mut bg: [short_t; 1] = [0];

    let color_palette = |color_number: short_t| normal::ColorPalette::_from(color_number);

    match unsafe { ncurses::pair_content_sp(screen, color_pair, fg.as_mut_ptr(), bg.as_mut_ptr()) } {
        OK => Ok(normal::Colors::new(normal::Color::_from(Some(screen), color_palette(fg[0])), normal::Color::_from(Some(screen), color_palette(bg[0])))),
        rc => Err(ncurses_function_error_with_rc!("pair_content_sp", rc))
    }
}

/// Screen function of `qiflush()`.
pub fn qiflush_sp(screen: SCREEN) {
    unsafe { ncurses::qiflush_sp(screen) }
}

/// Screen function of `raw()`.
pub fn raw_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::raw_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("raw_sp", rc))
    }
}

#[deprecated(since = "0.5.0", note = "use with caution as this routine will reset all color pairs potentially before they go out of scope and the color pairs will default to terminal default foreground and backgound colors.")]
/// Screen function of `reset_color_pairs()`.
pub fn reset_color_pairs_sp(screen: SCREEN) {
    unsafe { ncurses::reset_color_pairs_sp(screen) }
}

/// Screen function of `reset_prog_mode()`.
pub fn reset_prog_mode_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::reset_prog_mode_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("reset_prog_mode_sp", rc))
    }
}

/// Screen function of `reset_shell_mode()`.
pub fn reset_shell_mode_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::reset_shell_mode_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("reset_shell_mode_sp", rc))
    }
}

/// Screen function of `resetty()`.
pub fn resetty_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::resetty_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("resetty_sp", rc))
    }
}

/// Screen function of `resize_term()`.
pub fn resize_term_sp(screen: SCREEN, size: Size) -> result!(()) {
    match unsafe { ncurses::resize_term_sp(screen, size.lines, size.columns) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("resize_term_sp", rc))
    }
}

/// Screen function of `resizeterm()`.
pub fn resizeterm_sp(screen: SCREEN, size: Size) -> result!(()) {
    match unsafe { ncurses::resizeterm_sp(screen, size.lines, size.columns) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("resizeterm_sp", rc))
    }
}

// int restartterm_sp(SCREEN*, NCURSES_CONST char*, int, int *);

/// Screen function of `ripoffline()`.
pub fn ripoffline_sp(screen: SCREEN, line: Orientation, init: RipoffInit) -> result!(()) {
    match unsafe { ncurses::ripoffline_sp(screen, line.value(), init) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ripoffline_sp", rc))
    }
}

/// Screen function of `savetty()`.
pub fn savetty_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::savetty_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("savetty_sp", rc))
    }
}

/// Screen function of `scr_init()`.
pub fn scr_init_sp<P: AsRef<Path>>(screen: SCREEN, path: P) -> result!(()) {
    match unsafe { ncurses::scr_init_sp(screen, path_as_slice!(path)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scr_init_sp", rc))
    }
}

/// Screen function of `scr_restore()`.
pub fn scr_restore_sp<P: AsRef<Path>>(screen: SCREEN, path: P) -> result!(()) {
    match unsafe { ncurses::scr_restore_sp(screen, path_as_slice!(path)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scr_restore_sp", rc))
    }
}

/// Screen function of `scr_set()`.
pub fn scr_set_sp<P: AsRef<Path>>(screen: SCREEN, path: P) -> result!(()) {
    match unsafe { ncurses::scr_set_sp(screen, path_as_slice!(path)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("scr_set_sp", rc))
    }
}

// TERMINAL* set_curterm_sp(SCREEN*, TERMINAL*);

/// Screen function of `set_escdelay()`.
pub fn set_escdelay_sp(screen: SCREEN, ms: time::Duration) -> result!(()) {
    match unsafe { ncurses::set_escdelay_sp(screen, i32::try_from(ms.as_millis())?) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("set_escdelay_sp", rc))
    }
}

/// Screen function of `set_tabsize()`.
pub fn set_tabsize_sp(screen: SCREEN, size: i32) -> result!(()) {
    match unsafe { ncurses::set_tabsize_sp(screen, size) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("set_tabsize_sp", rc))
    }
}

/// Screen function of `slk_attroff()`.
pub fn slk_attroff_sp(screen: SCREEN, attrs: normal::Attributes) -> result!(()) {
    assert!(screen == attrs.screen().map_or_else(|| ptr::null_mut(), |screen| screen));

    match unsafe { ncurses::slk_attroff_sp(screen, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attroff_sp", rc))
    }
}

/// Screen function of `slk_attron()`.
pub fn slk_attron_sp(screen: SCREEN, attrs: normal::Attributes) -> result!(()) {
    assert!(screen == attrs.screen().map_or_else(|| ptr::null_mut(), |screen| screen));

    match unsafe { ncurses::slk_attron_sp(screen, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attron_sp", rc))
    }
}

/// Screen function of `slk_attr_set()`.
pub fn slk_attr_set_sp<A, P, T>(screen: SCREEN, attrs: A, color_pair: P) -> result!(())
    where A: AttributesType<T>,
          P: ColorPairType<T>,
          T: ColorAttributeTypes
{
    assert!(screen == attrs.screen().map_or_else(|| ptr::null_mut(), |screen| screen));
    assert!(screen == color_pair.screen().map_or_else(|| ptr::null_mut(), |screen| screen));

    match unsafe { ncurses::slk_attr_set_sp(screen, attrs.as_attr_t(), color_pair.as_short_t(), color_pair.as_mut_ptr()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attr_set_sp", rc))
    }
}

/// Screen function of `slk_attrset()`.
pub fn slk_attrset_sp(screen: SCREEN, attrs: normal::Attributes) -> result!(()) {
    assert!(screen == attrs.screen().map_or_else(|| ptr::null_mut(), |screen| screen));

    match unsafe { ncurses::slk_attrset_sp(screen, normal::Attributes::into(attrs)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_attrset_sp", rc))
    }
}

/// Screen function of `slk_attr()`.
pub fn slk_attr_sp(screen: SCREEN) -> normal::Attributes {
    normal::Attributes::_from(Some(screen), unsafe { ncurses::slk_attr_sp(screen) })
}

/// Screen function of `slk_clear()`.
pub fn slk_clear_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::slk_clear_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_clear_sp", rc))
    }
}

/// Screen function of `slk_color()`.
pub fn slk_color_sp(screen: SCREEN, color_pair: normal::ColorPair) -> result!(()) {
    assert!(screen == color_pair.screen().map_or_else(|| ptr::null_mut(), |screen| screen));

    match unsafe { ncurses::slk_color_sp(screen, color_pair.number()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_color_sp", rc))
    }
}

/// Screen function of `slk_init()`.
pub fn slk_init_sp(screen: SCREEN, fmt: SoftLabelType) -> result!(()) {
    match unsafe { ncurses::slk_init_sp(screen, fmt.value()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_init_sp", rc))
    }
}

/// Screen function of `slk_label()`.
pub fn slk_label_sp(screen: SCREEN, labnum: i32) -> Option<String> {
    unsafe { ncurses::slk_label_sp(screen, labnum) }
}

/// Screen function of `slk_noutrefresh()`.
pub fn slk_noutrefresh_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::slk_noutrefresh_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_noutrefresh_sp", rc))
    }
}

/// Screen function of `slk_refresh()`.
pub fn slk_refresh_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::slk_refresh_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_refresh_sp", rc))
    }
}

/// Screen function of `slk_restore()`.
pub fn slk_restore_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::slk_restore_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_restore_sp", rc))
    }
}

/// Screen function of `slk_set()`.
pub fn slk_set_sp(screen: SCREEN, label_number: i32, label: Option<&str>, fmt: Justification) -> result!(()) {
    match unsafe { ncurses::slk_set_sp(screen, label_number, option_str_as_ptr!(label), fmt.value()) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_set_sp", rc))
    }
}

/// Screen function of `slk_touch()`.
pub fn slk_touch_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::slk_touch_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("slk_touch_sp", rc))
    }
}

/// Screen function of `start_color()`.
pub fn start_color_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::start_color_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("start_color_sp", rc))
    }
}

/// Screen function of `term_attrs()`.
pub fn term_attrs_sp(_screen: SCREEN) -> attr_t {
    unimplemented!();
}

/// Screen function of `termattrs()`.
pub fn termattrs_sp(_screen: SCREEN) -> chtype {
    unimplemented!();
}

/// Screen function of `termname()`.
pub fn termname_sp(screen: SCREEN) -> result!(String) {
    unsafe { ncurses::termname_sp(screen).ok_or(ncurses_function_error!("termname_sp")) }
}

/// Screen function of `typeahead()`.
pub fn typeahead_sp<FD: AsRawFd + Read>(screen: SCREEN, file: Option<FD>) -> result!(()) {
    match unsafe { ncurses::typeahead_sp(screen, file.map_or_else(|| -1, |file| file.as_raw_fd())) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("typeahead_sp", rc))
    }
}

/// Screen function of `unctrl()`.
pub fn unctrl_sp(screen: SCREEN, c: ChtypeChar) -> result!(String) {
    unsafe { ncurses::unctrl_sp(screen, ChtypeChar::into(c)).ok_or(ncurses_function_error!("unctrl_sp")) }
}

/// Screen function of `ungetch()`.
pub fn ungetch_sp(screen: SCREEN, ch: char) -> result!(()) {
    match unsafe { ncurses::ungetch_sp(screen, i32::from(ch as u8)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("ungetch_sp", rc))
    }
}

/// Screen function of `unget_wch()`.
pub fn unget_wch_sp(screen: SCREEN, ch: WideChar) -> result!(()) {
    match unsafe { ncurses::unget_wch_sp(screen, WideChar::into(ch)) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("unget_wch_sp", rc))
    }
}

/// Screen function of `use_default_colors()`.
pub fn use_default_colors_sp(screen: SCREEN) -> result!(()) {
    match unsafe { ncurses::use_default_colors_sp(screen) } {
        OK => Ok(()),
        rc => Err(ncurses_function_error_with_rc!("use_default_colors_sp", rc))
    }
}

/// Screen function of `use_env()`.
pub fn use_env_sp(screen: SCREEN, f: bool) {
    unsafe { ncurses::use_env_sp(screen, f) }
}

/// Screen function of `use_tioctl()`.
pub fn use_tioctl_sp(screen: SCREEN, f: bool) {
    unsafe { ncurses::use_tioctl_sp(screen, f) }
}

/// Screen function of `use_legacy_coding()`.
pub fn use_legacy_coding_sp(screen: SCREEN, level: Legacy) -> result!(Legacy) {
    let rc = unsafe { ncurses::use_legacy_coding_sp(screen, level.value()) };

    Legacy::new(rc).ok_or(ncurses_function_error_with_rc!("use_legacy_coding_sp", rc))
}

/// Screen function of `vid_attr()`.
pub fn vid_attr_sp(_screen: SCREEN, _attrs: attr_t, _pair: short_t) -> i32 {
    unimplemented!();
}

/// Screen function of `vidattr()`.
pub fn vidattr_sp(_screen: SCREEN, _attrs: chtype) -> i32 {
    unimplemented!();
}

// int vid_puts_sp(SCREEN*, attr_t, short, void *, NCURSES_SP_OUTC);

// int vidputs_sp(SCREEN*, chtype, NCURSES_SP_OUTC);

/// Screen function of `wunctrl()`.
pub fn wunctrl_sp(screen: SCREEN, ch: ComplexChar) -> result!(WideChar) {
    let mut wch: [cchar_t; 1] = [ComplexChar::into(ch)];

    match unsafe { ncurses::wunctrl_sp(screen, wch.as_mut_ptr()) } {
        Some(ptr) => Ok(WideChar::from(unsafe { wchar_t::try_from(slice::from_raw_parts(ptr, 1)[0])? })),
        None      => Err(ncurses_function_error!("wunctrl_sp"))
    }
}

// private functions.

// get a file stream from a file descriptor.
fn fdopen<FD: AsRawFd>(file: &FD, mode: &str) -> result!(ncurses::FILE) {
    unsafe { funcs::fdopen(file, c_str_with_nul!(mode)).ok_or(ncurses_os_error!("fdopen")) }
}

fn path_as_vec<P: AsRef<Path>>(path: P) -> result!(Vec<u8>) {
    Ok(ffi::CString::new(path.as_ref().to_str().expect("path is invalid!!!"))?.into_bytes_with_nul())
}
