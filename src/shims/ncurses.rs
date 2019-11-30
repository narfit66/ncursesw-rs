/*
    src/shims/ncurses.rs

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
#![warn(missing_debug_implementations)]
#![allow(clippy::too_many_arguments)]

use std::{char, ptr};

use cstring::*;
use shims::bindings;

pub type short_t = i16;
pub type chtype = bindings::chtype;
pub type cchar_t = bindings::cchar_t;
pub type wint_t = bindings::wint_t;
pub type wchar_t = bindings::wchar_t;
pub type attr_t = bindings::attr_t;

pub type FILE = *mut bindings::_IO_FILE;
pub type WINDOW = *mut bindings::_win_st;
pub type SCREEN = *mut bindings::screen;

static MODULE_PATH: &str = "ncursesw::shims::ncurses::";

mod wrapped {
    use libc::c_int;
    use bindings::chtype;
    use bindings::WINDOW;

    extern "C" {
        pub static curscr: *mut WINDOW;
        pub static newscr: *mut WINDOW;
        pub static stdscr: *mut WINDOW;
        pub static COLORS: c_int;
        pub static COLOR_PAIRS: c_int;
        pub static COLS: c_int;
        pub static ESCDELAY: c_int;
        pub static LINES: c_int;
        pub static TABSIZE: c_int;

        /* Line graphics */
        pub static mut acs_map: [chtype; 0];
    }
}

pub unsafe fn curscr() -> WINDOW {
    wrapped::curscr
}

pub unsafe fn newscr() -> WINDOW {
    wrapped::newscr
}

pub unsafe fn stdscr() -> WINDOW {
    wrapped::stdscr
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn COLORS() -> i32 {
    unsafe {
        wrapped::COLORS
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn COLOR_PAIR(n: i32) -> i32 {
    unsafe {
        bindings::COLOR_PAIR(n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn PAIR_NUMBER(attr: i32) -> i32 {
    unsafe {
        bindings::PAIR_NUMBER(attr)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn COLOR_PAIRS() -> i32 {
    unsafe {
        wrapped::COLOR_PAIRS
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn COLS() -> i32 {
    unsafe {
        wrapped::COLS
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn ESCDELAY() -> i32 {
    unsafe {
        wrapped::ESCDELAY
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn LINES() -> i32 {
    unsafe {
        wrapped::LINES
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn TABSIZE() -> i32 {
    unsafe {
        wrapped::TABSIZE
    }
}

pub fn acs_map() -> *const chtype {
    unsafe {
        &wrapped::acs_map as *const chtype
    }
}

/// <https://invisible-island.net/ncurses/man/curs_add_wch.3x.html>
pub fn add_wch(wch: &cchar_t) -> i32 {
    unsafe {
        bindings::add_wch(wch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_add_wchstr.3x.html>
pub fn add_wchnstr(wchstr: &[cchar_t], n: i32) -> i32 {
    unsafe {
        bindings::add_wchnstr(wchstr.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_add_wchstr.3x.html>
pub fn add_wchstr(wchstr: &[cchar_t]) -> i32 {
    unsafe {
        bindings::add_wchstr(wchstr.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addch.3x.html>
pub fn addch(ch: chtype) -> i32 {
    unsafe {
        bindings::addch(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addchstr.3x.html>
pub fn addchnstr(chstr: &[chtype], n: i32) -> i32 {
    unsafe {
        bindings::addchnstr(chstr.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addchstr.3x.html>
pub fn addchstr(chstr: &[chtype]) -> i32 {
    unsafe {
        bindings::addchstr(chstr.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addstr.3x.html>
pub fn addnstr(str: &[i8], n: i32) -> i32 {
    unsafe {
        bindings::addnstr(str.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addwstr.3x.html>
pub fn addnwstr(wstr: &[wchar_t], n: i32) -> i32 {
    unsafe {
        bindings::addnwstr(wstr.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addstr.3x.html>
pub fn addstr(str: &[i8]) -> i32 {
    unsafe {
        bindings::addstr(str.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addwstr.3x.html>
pub fn addwstr(wstr: &[wchar_t]) -> i32 {
    unsafe {
        bindings::addwstr(wstr.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/new_pair.3x.html>
pub fn alloc_pair(fg: i32, bg: i32) -> i32 {
    unsafe {
        bindings::alloc_pair(fg, bg)
    }
}

/// <https://invisible-island.net/ncurses/man/default_colors.3x.html>
pub fn assume_default_colors(fg: i32, bg: i32) -> i32 {
    unsafe {
        bindings::assume_default_colors(fg, bg)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn attr_get(attrs: *mut attr_t, pair: *mut short_t, opts: *mut libc::c_void) -> i32 {
    assert!(!attrs.is_null(), "{}attr_get() : attrs.is_null()", MODULE_PATH);
    assert!(!pair.is_null(), "{}attr_get() : pair.is_null()", MODULE_PATH);

    bindings::attr_get(attrs, pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn attr_off(attrs: attr_t, opts: *mut libc::c_void) -> i32 {
    assert!(opts.is_null(), "{}attr_off() : !opts.is_null()", MODULE_PATH);

    bindings::attr_off(attrs, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn attr_on(attrs: attr_t, opts: *mut libc::c_void) -> i32 {
    assert!(opts.is_null(), "{}attr_on() : !opts.is_null()", MODULE_PATH);

    bindings::attr_on(attrs, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn attr_set(attrs: attr_t, pair: short_t, opts: *mut libc::c_void) -> i32 {
    bindings::attr_set(attrs, pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub fn attroff(attrs: i32) -> i32 {
    unsafe {
        bindings::attroff(attrs)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub fn attron(attrs: i32) -> i32 {
    unsafe {
        bindings::attron(attrs)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub fn attrset(attrs: i32) -> i32 {
    unsafe {
        bindings::attrset(attrs)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn baudrate() -> i32 {
    unsafe {
        bindings::baudrate()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_beep.3x.html>
pub fn beep() -> i32 {
    unsafe {
        bindings::beep()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_bkgd.3x.html>
pub fn bkgd(ch: chtype) -> i32 {
    unsafe {
        bindings::bkgd(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_bkgd.3x.html>
pub fn bkgdset(ch: chtype) {
    unsafe {
        bindings::bkgdset(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_bkgrnd.3x.html>
pub fn bkgrnd(wch: &cchar_t) -> i32 {
    unsafe {
        bindings::bkgrnd(wch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_bkgrnd.3x.html>
pub fn bkgrndset(wch: &cchar_t) {
    unsafe {
        bindings::bkgrndset(wch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub fn border(
    ls: chtype,
    rs: chtype,
    ts: chtype,
    bs: chtype,
    tl: chtype,
    tr: chtype,
    bl: chtype,
    br: chtype
) -> i32 {
    unsafe {
        bindings::border(ls, rs, ts, bs, tl, tr, bl, br)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub fn border_set(
    ls: &cchar_t,
    rs: &cchar_t,
    ts: &cchar_t,
    bs: &cchar_t,
    tl: &cchar_t,
    tr: &cchar_t,
    bl: &cchar_t,
    br: &cchar_t
) -> i32 {
    unsafe {
        bindings::border_set(ls, rs, ts, bs, tl, tr, bl, br)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub unsafe fn r#box(win: WINDOW, verch: chtype, horch: chtype) -> i32 {
    assert!(!win.is_null(), "{}box() : win.is_null()", MODULE_PATH);

    bindings::box_(win, verch, horch)
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub unsafe fn box_set(win: WINDOW, verch: &cchar_t, horch: &cchar_t) -> i32 {
    assert!(!win.is_null(), "{}box_set() : win.is_null()", MODULE_PATH);

    bindings::box_set(win, verch, horch)
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn can_change_color() -> bool {
    unsafe {
        bindings::can_change_color()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn cbreak() -> i32 {
    unsafe {
        bindings::cbreak()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn chgat(n: i32, attr: attr_t, pair: short_t, opts: *const libc::c_void) -> i32 {
    bindings::chgat(n, attr, pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_clear.3x.html>
pub fn clear() -> i32 {
    unsafe {
        bindings::clear()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub unsafe fn clearok(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}clear_ok() : win.is_null()", MODULE_PATH);

    bindings::clearok(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_clear.3x.html>
pub fn clrtobot() -> i32 {
    unsafe {
        bindings::clrtobot()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_clear.3x.html>
pub fn clrtoeol() -> i32 {
    unsafe {
        bindings::clrtoeol()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub unsafe fn color_content(color: short_t, r: *mut short_t, g: *mut short_t, b: *mut short_t) -> i32 {
    assert!(!r.is_null(), "{}color_content() : r.is_null()", MODULE_PATH);
    assert!(!g.is_null(), "{}color_content() : g.is_null()", MODULE_PATH);
    assert!(!b.is_null(), "{}color_content() : b.is_null()", MODULE_PATH);

    bindings::color_content(color, r, g, b)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn color_set(pair: short_t, opts: *mut libc::c_void) -> i32 {
    bindings::color_set(pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_overlay.3x.html>
pub unsafe fn copywin(
    srcwin: WINDOW,
    dstwin: WINDOW,
    sminrow: i32,
    smincol: i32,
    dminrow: i32,
    dmincol: i32,
    dmaxrow: i32,
    dmaxcol: i32,
    overlay: i32
) -> i32 {
    assert!(!srcwin.is_null(), "{}copy_win() : srcwin.is_null()", MODULE_PATH);
    assert!(!dstwin.is_null(), "{}copy_win() : dstwin.is_null()", MODULE_PATH);

    bindings::copywin(srcwin, dstwin, sminrow, smincol, dminrow, dmincol, dmaxrow, dmaxcol, overlay)
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn curs_set(visibility: i32) -> i32 {
    unsafe {
        bindings::curs_set(visibility)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_extend.3x.html>
pub fn curses_version() -> Option<String> {
    unsafe {
        (bindings::curses_version() as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn def_prog_mode() -> i32 {
    unsafe {
        bindings::def_prog_mode()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn def_shell_mode() -> i32 {
    unsafe {
        bindings::def_shell_mode()
    }
}

/// <https://invisible-island.net/ncurses/man/define_key.3x.html>
pub unsafe fn define_key(definition: *mut i8, keycode: i32) -> i32 {
    bindings::define_key(definition, keycode)
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn delay_output(ms: i32) -> i32 {
    unsafe {
        bindings::delay_output(ms)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_delch.3x.html>
pub fn delch() -> i32 {
    unsafe {
        bindings::delch()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_deleteln.3x.html>
pub fn deleteln() -> i32 {
    unsafe {
        bindings::deleteln()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_initscr.3x.html>
pub unsafe fn delscreen(sp: SCREEN) {
    assert!(!sp.is_null(), "{}delscreen() : sp.is_null()", MODULE_PATH);

    bindings::delscreen(sp)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn delwin(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}delwin() : win.is_null()", MODULE_PATH);

    bindings::delwin(win)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn derwin(orig: WINDOW, nlines: i32, ncols: i32, begin_y: i32, begin_x: i32) -> Option<WINDOW> {
    assert!(!orig.is_null(), "{}derwin() : orig.is_null()", MODULE_PATH);

    bindings::derwin(orig, nlines, ncols, begin_y, begin_x).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_refresh.3x.html>
pub fn doupdate() -> i32 {
    unsafe {
        bindings::doupdate()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn dupwin(win: WINDOW) -> Option<WINDOW> {
    assert!(!win.is_null(), "{}dupwin() : win.is_null()", MODULE_PATH);

    bindings::dupwin(win).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn echo() -> i32 {
    unsafe {
        bindings::echo()
    }
}


/// <https://invisible-island.net/ncurses/man/curs_add_wch.3x.html>
pub fn echo_wchar(wch: &cchar_t) -> i32 {
    unsafe {
        bindings::echo_wchar(wch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addch.3x.html>
pub fn echochar(ch: chtype) -> i32 {
    unsafe {
        bindings::echochar(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_initscr.3x.html>
pub fn endwin() -> i32 {
    unsafe {
        bindings::endwin()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_clear.3x.html>
pub fn erase() -> i32 {
    unsafe {
        bindings::erase()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn erasechar() -> i8 {
    unsafe {
        bindings::erasechar()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub unsafe fn erasewchar(ch: *mut wchar_t) -> i32 {
    assert!(!ch.is_null(), "{}erasewchar() : ch.is_null()", MODULE_PATH);

    bindings::erasewchar(ch)
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub unsafe fn extended_color_content(color: i32, r: *mut i32, g: *mut i32, b: *mut i32) -> i32 {
    assert!(!r.is_null(), "{}extended_color_content() : r.is_null()", MODULE_PATH);
    assert!(!g.is_null(), "{}extended_color_content() : g.is_null()", MODULE_PATH);
    assert!(!b.is_null(), "{}extended_color_content() : b.is_null()", MODULE_PATH);

    bindings::extended_color_content(color, r, g, b)
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub unsafe fn extended_pair_content(pair: i32, fg: *mut i32, bg: *mut i32) -> i32 {
    assert!(!fg.is_null(), "{}extended_pair_content() : fg.is_null()", MODULE_PATH);
    assert!(!bg.is_null(), "{}extended_pair_content() : bg.is_null()", MODULE_PATH);

    bindings::extended_pair_content(pair, fg, bg)
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn extended_slk_color(pair: i32) -> i32 {
    unsafe {
        bindings::extended_slk_color(pair)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn filter() {
    unsafe {
        bindings::filter()
    }
}

/// <https://invisible-island.net/ncurses/man/new_pair.3x.html>
pub fn find_pair(fg: i32, bg: i32) -> i32 {
    unsafe {
        bindings::find_pair(fg, bg)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_beep.3x.html>
pub fn flash() -> i32 {
    unsafe {
        bindings::flash()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn flushinp() -> i32 {
    unsafe {
        bindings::flushinp()
    }
}

/// <https://invisible-island.net/ncurses/man/new_pair.3x.html>
pub fn free_pair(pair: i32) -> i32 {
    unsafe {
        bindings::free_pair(pair)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_threads.3x.html>
pub fn get_escdelay() -> i32 {
    unsafe {
        bindings::get_escdelay()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_get_wch.3x.html>
pub unsafe fn get_wch(wch: *mut wint_t) -> i32 {
    assert!(!wch.is_null(), "{}get_wch() : wch.is_null()", MODULE_PATH);

    bindings::get_wch(wch)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wstr.3x.html>
pub unsafe fn get_wstr(wstr: *mut wint_t) -> i32 {
    assert!(!wstr.is_null(), "{}get_wstr() : wstr.is_null()", MODULE_PATH);

    bindings::get_wstr(wstr)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn getattrs(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getattrs() : win.is_null()", MODULE_PATH);

    bindings::getattrs(win)
}

/// <https://invisible-island.net/ncurses/man/curs_legacy.3x.html>
pub unsafe fn getbegx(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getbegx() : win.is_null()", MODULE_PATH);

    bindings::getbegx(win)
}

/// <https://invisible-island.net/ncurses/man/curs_legacy.3x.html>
pub unsafe fn getbegy(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getbegy() : win.is_null()", MODULE_PATH);

    bindings::getbegy(win)
}

/// <https://invisible-island.net/ncurses/man/curs_bkgd.3x.html>
pub unsafe fn getbkgd(win: WINDOW) -> chtype {
    assert!(!win.is_null(), "{}getbkgd() : win.is_null()", MODULE_PATH);

    bindings::getbkgd(win)
}

/// <https://invisible-island.net/ncurses/man/curs_bkgrnd.3x.html>
pub unsafe fn getbkgrnd(wch: *mut cchar_t) -> i32 {
    assert!(!wch.is_null(), "{}getbkgrnd() : wch.is_null()", MODULE_PATH);

    bindings::getbkgrnd(wch)
}

/// <https://invisible-island.net/ncurses/man/curs_getcchar.3x.html>
pub unsafe fn getcchar(wcval: &cchar_t, wch: *mut wchar_t, attrs: *mut attr_t, color_pair: *mut short_t, opts: *mut i32) -> i32 {
    assert!(!wch.is_null(), "{}getcchar() : wch.is_null()", MODULE_PATH);
    assert!(!attrs.is_null(), "{}getcchar() : attrs.is_null()", MODULE_PATH);
    assert!(!color_pair.is_null(), "{}getcchar() : color_pair.is_null()", MODULE_PATH);
    assert!(opts.is_null(), "{}getcchar() : !opts.is_null()", MODULE_PATH);

    bindings::getcchar(wcval, wch, attrs, color_pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_getch.3x.html>
pub fn getch() -> i32 {
    unsafe {
        bindings::getch()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_legacy.3x.html>
pub unsafe fn getcurx(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getcurx() : win.is_null()", MODULE_PATH);

    bindings::getcurx(win)
}

/// <https://invisible-island.net/ncurses/man/curs_legacy.3x.html>
pub unsafe fn getcury(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getcury() : win.is_null()", MODULE_PATH);

    bindings::getcury(win)
}

/// <https://invisible-island.net/ncurses/man/curs_legacy.3x.html>
pub unsafe fn getmaxx(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getmaxx() : win.is_null()", MODULE_PATH);

    bindings::getmaxx(win)
}

/// <https://invisible-island.net/ncurses/man/curs_legacy.3x.html>
pub unsafe fn getmaxy(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getmaxy() : win.is_null()", MODULE_PATH);

    bindings::getmaxy(win)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wstr.3x.html>
pub unsafe fn getn_wstr(wstr: *mut wint_t, n: i32) -> i32 {
    assert!(!wstr.is_null(), "{}getn_wstr() : wstr.is_null()", MODULE_PATH);

    bindings::getn_wstr(wstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_getstr.3x.html>
pub unsafe fn getnstr(str: *mut i8, n: i32) -> i32 {
    assert!(!str.is_null(), "{}getnstr() : str.is_null()", MODULE_PATH);

    bindings::getnstr(str, n)
}

/// <https://invisible-island.net/ncurses/man/curs_legacy.3x.html>
pub unsafe fn getparx(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getparx() : win.is_null()", MODULE_PATH);

    bindings::getparx(win)
}

/// <https://invisible-island.net/ncurses/man/curs_legacy.3x.html>
pub unsafe fn getpary(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}getpary() : win.is_null()", MODULE_PATH);

    bindings::getpary(win)
}

/// <https://invisible-island.net/ncurses/man/curs_getstr.3x.html>
pub unsafe fn getstr(str: *mut i8) -> i32 {
    assert!(!str.is_null(), "{}getstr() : str.is_null()", MODULE_PATH);

    bindings::getstr(str)
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub unsafe fn getwin(filep: FILE) -> Option<WINDOW> {
    bindings::getwin(filep).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn halfdelay(tenths: i32) -> i32 {
    unsafe {
        bindings::halfdelay(tenths)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn has_colors() -> bool {
    unsafe {
        bindings::has_colors()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn has_ic() -> bool {
    unsafe {
        bindings::has_ic()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn has_il() -> bool {
    unsafe {
        bindings::has_il()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_getch.3x.html>
pub fn has_key(ch: i32) -> i32 {
    unsafe {
        bindings::has_key(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub fn hline(ch: chtype, n: i32) -> i32 {
    unsafe {
        bindings::hline(ch, n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub fn hline_set(wch: &cchar_t, n: i32) -> i32 {
    unsafe {
        bindings::hline_set(wch, n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub unsafe fn idcok(win: WINDOW, bf: bool) {
    assert!(!win.is_null(), "{}idcok() : win.is_null()", MODULE_PATH);

    bindings::idcok(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub unsafe fn idlok(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}idlcok() : win.is_null()", MODULE_PATH);

    bindings::idlok(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub unsafe fn immedok(win: WINDOW, bf: bool) {
    assert!(!win.is_null(), "{}immedok() : win.is_null()", MODULE_PATH);

    bindings::immedok(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wch.3x.html>
pub unsafe fn in_wch(wcval: *mut cchar_t) -> i32 {
    assert!(!wcval.is_null(), "{}in_wch() : wcval.is_null()", MODULE_PATH);

    bindings::in_wch(wcval)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wchstr.3x.html>
pub unsafe fn in_wchnstr(wchstr: *mut cchar_t, n: i32) -> i32 {
    assert!(!wchstr.is_null(), "{}in_wchnstr() : wchstr.is_null()", MODULE_PATH);

    bindings::in_wchnstr(wchstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wchstr.3x.html>
pub unsafe fn in_wchstr(wchstr: *mut cchar_t) -> i32 {
    assert!(!wchstr.is_null(), "{}in_wchstr() : wchstr.is_null()", MODULE_PATH);

    bindings::in_wchstr(wchstr)
}

/// <https://invisible-island.net/ncurses/man/curs_inch.3x.html>
pub fn inch() -> chtype {
    unsafe {
        bindings::inch()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inchstr.3x.html>
pub unsafe fn inchnstr(chstr: *mut chtype, n: i32) -> i32 {
    assert!(!chstr.is_null(), "{}inchnstr() : chstr.is_null()", MODULE_PATH);

    bindings::inchnstr(chstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_inchstr.3x.html>
pub unsafe fn inchstr(chstr: *mut chtype) -> i32 {
    assert!(!chstr.is_null(), "{}inchstr() : chstr.is_null()", MODULE_PATH);

    bindings::inchstr(chstr)
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn init_color(color: short_t, r: short_t, g: short_t, b: short_t) -> i32 {
    unsafe {
        bindings::init_color(color, r, g, b)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn init_extended_color(color: i32, r: i32, g: i32, b: i32) -> i32 {
    unsafe {
        bindings::init_extended_color(color, r, g, b)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn init_extended_pair(color: i32, f: i32, b: i32) -> i32 {
    unsafe {
        bindings::init_extended_pair(color, f, b)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn init_pair(pair: short_t, f: short_t, b: short_t) -> i32 {
    unsafe {
        bindings::init_pair(pair, f, b)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_initscr.3x.html>
pub unsafe fn initscr() -> Option<WINDOW> {
    bindings::initscr().as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_instr.3x.html>
pub unsafe fn innstr(str: *mut i8, n: i32) -> i32 {
    assert!(!str.is_null(), "{}innstr() : str.is_null()", MODULE_PATH);

    bindings::innstr(str, n)
}

/// <https://invisible-island.net/ncurses/man/curs_inwstr.3x.html>
pub unsafe fn innwstr(wstr: *mut wchar_t, n: i32) -> i32 {
    assert!(!wstr.is_null(), "{}innwstr() : wstr.is_null()", MODULE_PATH);

    bindings::innwstr(wstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wstr.3x.html>
pub fn ins_nwstr(wstr: &[wchar_t], n: i32) -> i32 {
    unsafe {
        bindings::ins_nwstr(wstr.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wch.3x.html>
pub fn ins_wch(wch: &cchar_t) -> i32 {
    unsafe {
        bindings::ins_wch(wch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wstr.3x.html>
pub fn ins_wstr(wstr: &[wchar_t]) -> i32 {
    unsafe {
        bindings::ins_wstr(wstr.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_insch.3x.html>
pub fn insch(ch: chtype) -> i32 {
    unsafe {
        bindings::insch(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_deleteln.3x.html>
pub fn insdelln(n: i32) -> i32 {
    unsafe {
        bindings::insdelln(n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_deleteln.3x.html>
pub fn insertln() -> i32 {
    unsafe {
        bindings::insertln()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_insstr.3x.html>
pub fn insnstr(str: &[i8], n: i32) -> i32 {
    unsafe {
        bindings::insnstr(str.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_insstr.3x.html>
pub fn insstr(str: &[i8]) -> i32 {
    unsafe {
        bindings::insstr(str.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_instr.3x.html>
pub unsafe fn instr(str: *mut i8) -> i32 {
    assert!(!str.is_null(), "{}instr() : str.is_null()", MODULE_PATH);

    bindings::instr(str)
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub unsafe fn intrflush(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}intrflush() : win.is_null()", MODULE_PATH);

    bindings::intrflush(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_inwstr.3x.html>
pub unsafe fn inwstr(wstr: *mut wchar_t) -> i32 {
    assert!(!wstr.is_null(), "{}inwstr() : wstr.is_null()", MODULE_PATH);

    bindings::inwstr(wstr)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_cleared(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_cleared() : win.is_null()", MODULE_PATH);

    bindings::is_cleared(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_idcok(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_idcok() : win.is_null()", MODULE_PATH);

    bindings::is_idcok(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_idlok(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_idlcok() : win.is_null()", MODULE_PATH);

    bindings::is_idlok(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_immedok(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_immedok() : win.is_null()", MODULE_PATH);

    bindings::is_immedok(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_keypad(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_keypad() : win.is_null()", MODULE_PATH);

    bindings::is_keypad(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_leaveok(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_leaveok() : win.is_null()", MODULE_PATH);

    bindings::is_leaveok(win)
}

/// <https://invisible-island.net/ncurses/man/curs_touch.3x.html>
pub unsafe fn is_linetouched(win: WINDOW, l: i32) -> bool {
    assert!(!win.is_null(), "{}is_linetouched() : win.is_null()", MODULE_PATH);

    bindings::is_linetouched(win, l)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_nodelay(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_nodelay() : win.is_null()", MODULE_PATH);

    bindings::is_nodelay(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_notimeout(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_notimeout() : win.is_null()", MODULE_PATH);

    bindings::is_notimeout(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_pad(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_pad() : win.is_null()", MODULE_PATH);

    bindings::is_pad(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_scrollok(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_scrollok() : win.is_null()", MODULE_PATH);

    bindings::is_scrollok(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn is_syncok(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_syncok() : win.is_null()", MODULE_PATH);

    bindings::is_syncok(win)
}

/// <https://invisible-island.net/ncurses/man/resizeterm.3x.html>
pub fn is_term_resized(lines: i32, cols: i32) -> bool {
    unsafe {
        bindings::is_term_resized(lines, cols)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_touch.3x.html>
pub unsafe fn is_wintouched(win: WINDOW) -> bool {
    assert!(!win.is_null(), "{}is_wintouched() : win.is_null()", MODULE_PATH);

    bindings::is_wintouched(win)
}

/// <https://invisible-island.net/ncurses/man/curs_initscr.3x.html>
pub fn isendwin() -> bool {
    unsafe {
        bindings::isendwin()
    }
}

/// <https://invisible-island.net/ncurses/man/key_defined.3x.html>
pub fn key_defined(definition: &[i8]) -> i32 {
    unsafe {
        bindings::key_defined(definition.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn key_name(w: wchar_t) -> Option<String> {
    unsafe {
        (bindings::key_name(w) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/keybound.3x.html>
pub fn keybound(keycode: i32, count: i32) -> Option<String> {
    unsafe {
        (bindings::keybound(keycode, count) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn keyname(c: i32) -> Option<String> {
    unsafe {
        (bindings::keyname(c) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/keyok.3x.html>
pub fn keyok(keycode: i32, enable: bool) -> i32 {
    unsafe {
        bindings::keyok(keycode, enable)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub unsafe fn keypad(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}keypad() : win.is_null()", MODULE_PATH);

    bindings::keypad(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn killchar() -> i8 {
    unsafe {
        bindings::killchar()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub unsafe fn killwchar(ch: *mut wchar_t) -> i32 {
    assert!(!ch.is_null(), "{}killwchar() : ch.is_null()", MODULE_PATH);

    bindings::killwchar(ch)
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub unsafe fn leaveok(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}leaveok() : win.is_null()", MODULE_PATH);

    bindings::leaveok(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn longname() -> Option<String> {
    unsafe {
        (bindings::longname() as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/curs_print.3x.html>
pub unsafe fn mcprint(data: *mut i8, len: i32) -> i32 {
    assert!(!data.is_null(), "{}mcprint() : data.is_null()", MODULE_PATH);

    bindings::mcprint(data, len)
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub unsafe fn meta(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}meta() : win.is_null()", MODULE_PATH);

    bindings::meta(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_move.3x.html>
pub fn r#move(y: i32, x: i32) -> i32 {
    unsafe {
        bindings::move_(y, x)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_add_wch.3x.html>
pub fn mvadd_wch(y: i32, x: i32, wch: &cchar_t) -> i32 {
    unsafe {
        bindings::mvadd_wch(y, x, wch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_add_wchstr.3x.html>
pub fn mvadd_wchnstr(y: i32, x: i32, wchstr: &[cchar_t], n: i32) -> i32 {
    unsafe {
        bindings::mvadd_wchnstr(y, x, wchstr.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_add_wchstr.3x.html>
pub fn mvadd_wchstr(y: i32, x: i32, wchstr: &[cchar_t]) -> i32 {
    unsafe {
        bindings::mvadd_wchstr(y, x, wchstr.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addch.3x.html>
pub fn mvaddch(y: i32, x: i32, ch: chtype) -> i32 {
    unsafe {
        bindings::mvaddch(y, x, ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addchstr.3x.html>
pub fn mvaddchnstr(y: i32, x: i32, chstr: &[chtype], n: i32) -> i32 {
    unsafe {
        bindings::mvaddchnstr(y, x, chstr.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addchstr.3x.html>
pub fn mvaddchstr(y: i32, x: i32, chstr: &[chtype]) -> i32 {
    unsafe {
        bindings::mvaddchstr(y, x, chstr.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addstr.3x.html>
pub fn mvaddnstr(y: i32, x: i32, str: &[i8], n: i32) -> i32 {
    unsafe {
        bindings::mvaddnstr(y, x, str.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addwstr.3x.html>
pub fn mvaddnwstr(y: i32, x: i32, wstr: &[wchar_t], n: i32) -> i32 {
    unsafe {
        bindings::mvaddnwstr(y, x, wstr.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addstr.3x.html>
pub fn mvaddstr(y: i32, x: i32, str: &[i8]) -> i32 {
    unsafe {
        bindings::mvaddstr(y, x, str.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_addwstr.3x.html>
pub fn mvaddwstr(y: i32, x: i32, wstr: &[wchar_t]) -> i32 {
    unsafe {
        bindings::mvaddwstr(y, x, wstr.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn mvchgat(y: i32, x: i32, n: i32, attr: attr_t, color: short_t, opts: *const libc::c_void) -> i32 {
    bindings::mvchgat(y, x, n, attr, color, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
pub fn mvcur(oldrow: i32, oldcol: i32, newrow: i32, newcol: i32) -> i32 {
    unsafe {
        bindings::mvcur(oldrow, oldcol, newrow, newcol)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_delch.3x.html>
pub fn mvdelch(y: i32, x: i32) -> i32 {
    unsafe {
        bindings::mvdelch(y, x)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn mvderwin(win: WINDOW, y: i32, x: i32) -> i32 {
    assert!(!win.is_null(), "{}mvderwin() : win.is_null()", MODULE_PATH);

    bindings::mvderwin(win, y, x)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wch.3x.html>
pub unsafe fn mvget_wch(y: i32, x: i32, wch: *mut wint_t) -> i32 {
    assert!(!wch.is_null(), "{}mvget_wch() : wch.is_null()", MODULE_PATH);

    bindings::mvget_wch(y, x, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wstr.3x.html>
pub unsafe fn mvget_wstr(y: i32, x: i32, wstr: *mut wint_t) -> i32 {
    assert!(!wstr.is_null(), "{}mvget_wstr() : wstr.is_null()", MODULE_PATH);

    bindings::mvget_wstr(y, x, wstr)
}

/// <https://invisible-island.net/ncurses/man/curs_getch.3x.html>
pub fn mvgetch(y: i32, x: i32) -> i32 {
    unsafe {
        bindings::mvgetch(y, x)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_get_wstr.3x.html>
pub unsafe fn mvgetn_wstr(y: i32, x: i32, wstr: *mut wint_t, n: i32) -> i32 {
    assert!(!wstr.is_null(), "{}mvgetn_wstr() : wstr.is_null()", MODULE_PATH);

    bindings::mvgetn_wstr(y, x, wstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_getstr.3x.html>
pub unsafe fn mvgetnstr(y: i32, x: i32, str: *mut i8, n: i32) -> i32 {
    assert!(!str.is_null(), "{}mvgetnstr() : str.is_null()", MODULE_PATH);

    bindings::mvgetnstr(y, x, str, n)
}

/// <https://invisible-island.net/ncurses/man/curs_getstr.3x.html>
pub unsafe fn mvgetstr(y: i32, x: i32, str: *mut i8) -> i32 {
    assert!(!str.is_null(), "{}mvgetstr() : str.is_null()", MODULE_PATH);

    bindings::mvgetstr(y, x, str)
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub fn mvhline(y: i32, x: i32, ch: chtype, n: i32) -> i32 {
    unsafe {
        bindings::mvhline(y, x, ch, n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub fn mvhline_set(y: i32, x: i32, wch: &cchar_t, n: i32) -> i32 {
    unsafe {
        bindings::mvhline_set(y, x, wch, n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_in_wch.3x.html>
pub unsafe fn mvin_wch(y: i32, x: i32, wcval: *mut cchar_t) -> i32 {
    assert!(!wcval.is_null(), "{}mvin_wch() : wcval.is_null()", MODULE_PATH);

    bindings::mvin_wch(y, x, wcval)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wchstr.3x.html>
pub unsafe fn mvin_wchnstr(y: i32, x: i32, wchstr: *mut cchar_t, n: i32) -> i32 {
    assert!(!wchstr.is_null(), "{}mvin_wchnstr() : wchstr.is_null()", MODULE_PATH);

    bindings::mvin_wchnstr(y, x, wchstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wchstr.3x.html>
pub unsafe fn mvin_wchstr(y: i32, x: i32, wchstr: *mut cchar_t) -> i32 {
    assert!(!wchstr.is_null(), "{}in_mvchstr() : wchstr.is_null()", MODULE_PATH);

    bindings::mvin_wchstr(y, x, wchstr)
}

/// <https://invisible-island.net/ncurses/man/curs_inch.3x.html>
pub fn mvinch(y: i32, x: i32) -> chtype {
    unsafe {
        bindings::mvinch(y, x)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inchstr.3x.html>
pub unsafe fn mvinchnstr(y: i32, x: i32, chstr: *mut chtype, n: i32) -> i32 {
    assert!(!chstr.is_null(), "{}mvinchnstr() : chstr.is_null()", MODULE_PATH);

    bindings::mvinchnstr(y, x, chstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_inchstr.3x.html>
pub unsafe fn mvinchstr(y: i32, x: i32, chstr: *mut chtype) -> i32 {
    assert!(!chstr.is_null(), "{}mvinchstr() : chstr.is_null()", MODULE_PATH);

    bindings::mvinchstr(y, x, chstr)
}

/// <https://invisible-island.net/ncurses/man/curs_instr.3x.html>
pub unsafe fn mvinnstr(y: i32, x: i32, str: *mut i8, n: i32) -> i32 {
    assert!(!str.is_null(), "{}mvinnstr() : str.is_null()", MODULE_PATH);

    bindings::mvinnstr(y, x, str, n)
}

/// <https://invisible-island.net/ncurses/man/curs_inwstr.3x.html>
pub unsafe fn mvinnwstr(y: i32, x: i32, wstr: *mut wchar_t, n: i32) -> i32 {
    assert!(!wstr.is_null(), "{}mvinnwstr() : wstr.is_null()", MODULE_PATH);

    bindings::mvinnwstr(y, x, wstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wstr.3x.html>
pub fn mvins_nwstr(y: i32, x: i32, wstr: &[wchar_t], n: i32) -> i32 {
    unsafe {
        bindings::mvins_nwstr(y, x, wstr.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wch.3x.html>
pub fn mvins_wch(y: i32, x: i32, wch: &cchar_t) -> i32 {
    unsafe {
        bindings::mvins_wch(y, x, wch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wstr.3x.html>
pub fn mvins_wstr(y: i32, x: i32, wstr: &[wchar_t]) -> i32 {
    unsafe {
        bindings::mvins_wstr(y, x, wstr.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_insch.3x.html>
pub fn mvinsch(y: i32, x: i32, ch: chtype) -> i32 {
    unsafe {
        bindings::mvinsch(y, x, ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_insstr.3x.html>
pub fn mvinsnstr(y: i32, x: i32, str: &[i8], n: i32) -> i32 {
    unsafe {
        bindings::mvinsnstr(y, x, str.as_ptr(), n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_insstr.3x.html>
pub fn mvinsstr(y: i32, x: i32, str: &[i8]) -> i32 {
    unsafe {
        bindings::mvinsstr(y, x, str.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_instr.3x.html>
pub unsafe fn mvinstr(y: i32, x: i32, str: *mut i8) -> i32 {
    assert!(!str.is_null(), "{}mvinstr() : str.is_null()", MODULE_PATH);

    bindings::mvinstr(y, x, str)
}

/// <https://invisible-island.net/ncurses/man/curs_inwstr.3x.html>
pub unsafe fn mvinwstr(y: i32, x: i32, wstr: *mut wchar_t) -> i32 {
    assert!(!wstr.is_null(), "{}mvinwstr() : wstr.is_null()", MODULE_PATH);

    bindings::mvinwstr(y, x, wstr)
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub fn mvvline(y: i32, x: i32, ch: chtype, n: i32) -> i32 {
    unsafe {
        bindings::mvvline(y, x, ch, n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub fn mvvline_set(y: i32, x: i32, wch: &cchar_t, n: i32) -> i32 {
    unsafe {
        bindings::mvvline_set(y, x, wch, n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_add_wch.3x.html>
pub unsafe fn mvwadd_wch(win: WINDOW, y: i32, x: i32, wch: &cchar_t) -> i32 {
    assert!(!win.is_null(), "{}mvwadd_wch() : win.is_null()", MODULE_PATH);

    bindings::mvwadd_wch(win, y, x, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_add_wchstr.3x.html>
pub unsafe fn mvwadd_wchnstr(win: WINDOW, y: i32, x: i32, wchstr: &[cchar_t], n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwadd_wchnstr() : win.is_null()", MODULE_PATH);

    bindings::mvwadd_wchnstr(win, y, x, wchstr.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_add_wchstr.3x.html>
pub unsafe fn mvwadd_wchstr(win: WINDOW, y: i32, x: i32, wchstr: &[cchar_t]) -> i32 {
    assert!(!win.is_null(), "{}mvwadd_wchstr() : win.is_null()", MODULE_PATH);

    bindings::mvwadd_wchstr(win, y, x, wchstr.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_addch.3x.html>
pub unsafe fn mvwaddch(win: WINDOW, y: i32, x: i32, ch: chtype) -> i32 {
    assert!(!win.is_null(), "{}mvwaddch() : win.is_null()", MODULE_PATH);

    bindings::mvwaddch(win, y, x, ch)
}

/// <https://invisible-island.net/ncurses/man/curs_addchstr.3x.html>
pub unsafe fn mvwaddchnstr(win: WINDOW, y: i32, x: i32, chstr: &[chtype], n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwaddchnstr() : win.is_null()", MODULE_PATH);

    bindings::mvwaddchnstr(win, y, x, chstr.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_addchstr.3x.html>
pub unsafe fn mvwaddchstr(win: WINDOW, y: i32, x: i32, chstr: &[chtype]) -> i32 {
    assert!(!win.is_null(), "{}mvwaddchstr() : win.is_null()", MODULE_PATH);

    bindings::mvwaddchstr(win, y, x, chstr.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_addstr.3x.html>
pub unsafe fn mvwaddnstr(win: WINDOW, y: i32, x: i32, str: &[i8], n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwaddnstr() : win.is_null()", MODULE_PATH);

    bindings::mvwaddnstr(win, y, x, str.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_addwstr.3x.html>
pub unsafe fn mvwaddnwstr(win: WINDOW, y: i32, x: i32, wstr: &[wchar_t], n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwaddnwstr() : win.is_null()", MODULE_PATH);

    bindings::mvwaddnwstr(win, y, x, wstr.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_addstr.3x.html>
pub unsafe fn mvwaddstr(win: WINDOW, y: i32, x: i32, str: &[i8]) -> i32 {
    assert!(!win.is_null(), "{}mvwaddstr() : win.is_null()", MODULE_PATH);

    bindings::mvwaddstr(win, y, x, str.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_addwstr.3x.html>
pub unsafe fn mvwaddwstr(win: WINDOW, y: i32, x: i32, wstr: &[wchar_t]) -> i32 {
    assert!(!win.is_null(), "{}mvwaddwstr() : win.is_null()", MODULE_PATH);

    bindings::mvwaddwstr(win, y, x, wstr.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn mvwchgat(win: WINDOW, y: i32, x: i32, n: i32, attr: attr_t, color: short_t, opts: *const libc::c_void) -> i32 {
    assert!(!win.is_null(), "{}mvwchgat() : win.is_null()", MODULE_PATH);

    bindings::mvwchgat(win, y, x, n, attr, color, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_delch.3x.html>
pub unsafe fn mvwdelch(win: WINDOW, y: i32, x: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwdelch() : win.is_null()", MODULE_PATH);

    bindings::mvwdelch(win, y, x)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wch.3x.html>
pub unsafe fn mvwget_wch(win: WINDOW, y: i32, x: i32, wch: *mut wint_t) -> i32 {
    assert!(!win.is_null(), "{}mvwget_wch() : win.is_null()", MODULE_PATH);
    assert!(!wch.is_null(), "{}mvwget_wch() : wch.is_null()", MODULE_PATH);

    bindings::mvwget_wch(win, y, x, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wstr.3x.html>
pub unsafe fn mvwget_wstr(win: WINDOW, y: i32, x: i32, wstr: *mut wint_t) -> i32 {
    assert!(!win.is_null(), "{}mvwget_wstr() : win.is_null()", MODULE_PATH);
    assert!(!wstr.is_null(), "{}mvwget_wstr() : wstr.is_null()", MODULE_PATH);

    bindings::mvwget_wstr(win, y, x, wstr)
}

/// <https://invisible-island.net/ncurses/man/curs_getch.3x.html>
pub unsafe fn mvwgetch(win: WINDOW, y: i32, x: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwgetch() : win.is_null()", MODULE_PATH);

    bindings::mvwgetch(win, y, x)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wstr.3x.html>
pub unsafe fn mvwgetn_wstr(win: WINDOW, y: i32, x: i32, wstr: *mut wint_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwgetn_wstr() : win.is_null()", MODULE_PATH);
    assert!(!wstr.is_null(), "{}mvwgetn_wstr() : wstr.is_null()", MODULE_PATH);

    bindings::mvwgetn_wstr(win, y, x, wstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_getstr.3x.html>
pub unsafe fn mvwgetnstr(win: WINDOW, y: i32, x: i32, str: *mut i8, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwgetnstr() : win.is_null()", MODULE_PATH);
    assert!(!str.is_null(), "{}mvwgetnstr() : str.is_null()", MODULE_PATH);

    bindings::mvwgetnstr(win, y, x, str, n)
}

/// <https://invisible-island.net/ncurses/man/curs_getstr.3x.html>
pub unsafe fn mvwgetstr(win: WINDOW, y: i32, x: i32, str: *mut i8) -> i32 {
    assert!(!win.is_null(), "{}mvwgetstr() : win.is_null()", MODULE_PATH);
    assert!(!str.is_null(), "{}mvwgetstr() : str.is_null()", MODULE_PATH);

    bindings::mvwgetstr(win, y, x, str)
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub unsafe fn mvwhline(win: WINDOW, y: i32, x: i32, ch: chtype, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwhline() : win.is_null()", MODULE_PATH);

    bindings::mvwhline(win, y, x, ch, n)
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub unsafe fn mvwhline_set(win: WINDOW, y: i32, x: i32, wch: &cchar_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwhline_set() : win.is_null()", MODULE_PATH);

    bindings::mvwhline_set(win, y, x, wch, n)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn mvwin(win: WINDOW, y: i32, x: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwin() : win.is_null()", MODULE_PATH);

    bindings::mvwin(win, y, x)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wch.3x.html>
pub unsafe fn mvwin_wch(win: WINDOW, y: i32, x: i32, wcval: *mut cchar_t) -> i32 {
    assert!(!win.is_null(), "{}mvwin_wch() : win.is_null()", MODULE_PATH);
    assert!(!wcval.is_null(), "{}mvwin_wch() : wcval.is_null()", MODULE_PATH);

    bindings::mvwin_wch(win, y, x, wcval)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wchstr.3x.html>
pub unsafe fn mvwin_wchnstr(win: WINDOW, y: i32, x: i32, wchstr: *mut cchar_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwin_wchnstr() : win.is_null()", MODULE_PATH);
    assert!(!wchstr.is_null(), "{}mvwin_wchnstr() : wchstr.is_null()", MODULE_PATH);

    bindings::mvwin_wchnstr(win, y, x, wchstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wchstr.3x.html>
pub unsafe fn mvwin_wchstr(win: WINDOW, y: i32, x: i32, wchstr: *mut cchar_t) -> i32 {
    assert!(!win.is_null(), "{}mvwin_wchstr() : win.is_null()", MODULE_PATH);
    assert!(!wchstr.is_null(), "{}mvwin_wchstr() : wchstr.is_null()", MODULE_PATH);

    bindings::mvwin_wchstr(win, y, x, wchstr)
}

/// <https://invisible-island.net/ncurses/man/curs_inch.3x.html>
pub unsafe fn mvwinch(win: WINDOW, y: i32, x: i32) -> chtype {
    assert!(!win.is_null(), "{}mvwinch() : win.is_null()", MODULE_PATH);

    bindings::mvwinch(win, y, x)
}

/// <https://invisible-island.net/ncurses/man/curs_inchstr.3x.html>
pub unsafe fn mvwinchnstr(win: WINDOW, y: i32, x: i32, chstr: *mut chtype, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwinchnstr() : win.is_null()", MODULE_PATH);
    assert!(!chstr.is_null(), "{}mvwinchnstr() : chstr.is_null()", MODULE_PATH);

    bindings::mvwinchnstr(win, y, x, chstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_inchstr.3x.html>
pub unsafe fn mvwinchstr(win: WINDOW, y: i32, x: i32, chstr: *mut chtype) -> i32 {
    assert!(!win.is_null(), "{}mvwinchstr() : win.is_null()", MODULE_PATH);
    assert!(!chstr.is_null(), "{}mvwinchstr() : chstr.is_null()", MODULE_PATH);

    bindings::mvwinchstr(win, y, x, chstr)
}

/// <https://invisible-island.net/ncurses/man/curs_instr.3x.html>
pub unsafe fn mvwinnstr(win: WINDOW, y: i32, x: i32, str: *mut i8, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwinnstr() : win.is_null()", MODULE_PATH);
    assert!(!str.is_null(), "{}mvwinnstr() : str.is_null()", MODULE_PATH);

    bindings::mvwinnstr(win, y, x, str, n)
}

/// <https://invisible-island.net/ncurses/man/curs_inwstr.3x.html>
pub unsafe fn mvwinnwstr(win: WINDOW, y: i32, x: i32, wstr: *mut wchar_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwinnwstr() : win.is_null()", MODULE_PATH);
    assert!(!wstr.is_null(), "{}mvwinnwstr() : wstr.is_null()", MODULE_PATH);

    bindings::mvwinnwstr(win, y, x, wstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wstr.3x.html>
pub unsafe fn mvwins_nwstr(win: WINDOW, y: i32, x: i32, wstr: &[wchar_t], n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwins_nwstr() : win.is_null()", MODULE_PATH);

    bindings::mvwins_nwstr(win, y, x, wstr.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wch.3x.html>
pub unsafe fn mvwins_wch(win: WINDOW, y: i32, x: i32, wch: &cchar_t) -> i32 {
    assert!(!win.is_null(), "{}mvwins_wch() : win.is_null()", MODULE_PATH);

    bindings::mvwins_wch(win, y, x, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wstr.3x.html>
pub unsafe fn mvwins_wstr(win: WINDOW, y: i32, x: i32, wstr: &[wchar_t]) -> i32 {
    assert!(!win.is_null(), "{}mvwins_wstr() : win.is_null()", MODULE_PATH);

    bindings::mvwins_wstr(win, y, x, wstr.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_insch.3x.html>
pub unsafe fn mvwinsch(win: WINDOW, y: i32, x: i32, ch: chtype) -> i32 {
    assert!(!win.is_null(), "{}mvwinsch() : win.is_null()", MODULE_PATH);

    bindings::mvwinsch(win, y, x, ch)
}

/// <https://invisible-island.net/ncurses/man/curs_insstr.3x.html>
pub unsafe fn mvwinsnstr(win: WINDOW, y: i32, x: i32, str: &[i8], n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwinsnstr() : win.is_null()", MODULE_PATH);

    bindings::mvwinsnstr(win, y, x, str.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_insstr.3x.html>
pub unsafe fn mvwinsstr(win: WINDOW, y: i32, x: i32, str: &[i8]) -> i32 {
    assert!(!win.is_null(), "{}mvwinsstr() : win.is_null()", MODULE_PATH);

    bindings::mvwinsstr(win, y, x, str.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_instr.3x.html>
pub unsafe fn mvwinstr(win: WINDOW, y: i32, x: i32, str: *mut i8) -> i32 {
    assert!(!win.is_null(), "{}mvwinstr() : win.is_null()", MODULE_PATH);
    assert!(!str.is_null(), "{}mvwinstr() : str.is_null()", MODULE_PATH);

    bindings::mvwinstr(win, y, x, str)
}

/// <https://invisible-island.net/ncurses/man/curs_inwstr.3x.html>
pub unsafe fn mvwinwstr(win: WINDOW, y: i32, x: i32, wstr: *mut wchar_t) -> i32 {
    assert!(!win.is_null(), "{}mvwinwstr() : win.is_null()", MODULE_PATH);
    assert!(!wstr.is_null(), "{}mvwinwstr() : wstr.is_null()", MODULE_PATH);

    bindings::mvwinwstr(win, y, x, wstr)
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub unsafe fn mvwvline(win: WINDOW, y: i32, x: i32, ch: chtype, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwvline() : win.is_null()", MODULE_PATH);

    bindings::mvwvline(win, y, x, ch, n)
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub unsafe fn mvwvline_set(win: WINDOW, y: i32, x: i32, wch: &cchar_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}mvwvline_set() : win.is_null()", MODULE_PATH);

    bindings::mvwvline_set(win, y, x, wch, n)
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn napms(ms: i32) -> i32 {
    unsafe {
        bindings::napms(ms)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_pad.3x.html>
pub unsafe fn newpad(lines: i32, cols: i32) -> Option<WINDOW> {
    bindings::newpad(lines, cols).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_initscr.3x.html>
pub unsafe fn newterm(ty: Option<&[i8]>, outfd: FILE, infd: FILE) -> Option<SCREEN> {
    bindings::newterm(
        match ty {
            Some(s) => s.as_ptr(),
            None    => ptr::null()
        },
        outfd,
        infd
    ).as_mut().map(|ptr| ptr as SCREEN)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> Option<WINDOW> {
    bindings::newwin(lines, cols, y, x).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub fn nl() -> i32 {
    unsafe {
        bindings::nl()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn nocbreak() -> i32 {
    unsafe {
        bindings::nocbreak()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub unsafe fn nodelay(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}nodelay() : win.is_null()", MODULE_PATH);

    bindings::nodelay(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn noecho() -> i32 {
    unsafe {
        bindings::noecho()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn nofilter() {
    unsafe {
        bindings::nofilter()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub fn nonl() -> i32 {
    unsafe {
        bindings::nonl()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn noqiflush() {
    unsafe {
        bindings::noqiflush()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn noraw() -> i32 {
    unsafe {
        bindings::noraw()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub unsafe fn notimeout(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}notimeout() : win.is_null()", MODULE_PATH);

    bindings::notimeout(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_overlay.3x.html>
pub unsafe fn overlay(srcwin: WINDOW, dstwin: WINDOW) -> i32 {
    assert!(!srcwin.is_null(), "{}overlay() : srcwin.is_null()", MODULE_PATH);
    assert!(!dstwin.is_null(), "{}overlay() : dstwin.is_null()", MODULE_PATH);

    bindings::overlay(srcwin, dstwin)
}

/// <https://invisible-island.net/ncurses/man/curs_overlay.3x.html>
pub unsafe fn overwrite(srcwin: WINDOW, dstwin: WINDOW) -> i32 {
    assert!(!srcwin.is_null(), "{}overwrite() : srcwin.is_null()", MODULE_PATH);
    assert!(!dstwin.is_null(), "{}overwrite() : dstwin.is_null()", MODULE_PATH);

    bindings::overwrite(srcwin, dstwin)
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub unsafe fn pair_content(pair: short_t, fg: *mut short_t, bg: *mut short_t) -> i32 {
    assert!(!fg.is_null(), "{}pair_content() : fg.is_null()", MODULE_PATH);
    assert!(!bg.is_null(), "{}pair_content() : bg.is_null()", MODULE_PATH);

    bindings::pair_content(pair, fg, bg)
}

/// <https://invisible-island.net/ncurses/man/curs_pad.3x.html>
pub unsafe fn pechochar(pad: WINDOW, ch: chtype) -> i32 {
    assert!(!pad.is_null(), "{}pechochar() : pad.is_null()", MODULE_PATH);

    bindings::pechochar(pad, ch)
}

/// <https://invisible-island.net/ncurses/man/curs_pad.3x.html>
pub unsafe fn pecho_wchar(pad: WINDOW, wch: &cchar_t) -> i32 {
    assert!(!pad.is_null(), "{}pecho_wchar() : pad.is_null()", MODULE_PATH);

    bindings::pecho_wchar(pad, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_pad.3x.html>
pub unsafe fn pnoutrefresh(
    pad: WINDOW,
    pminrow: i32,
    pmincol: i32,
    sminrow: i32,
    smincol: i32,
    smaxrow: i32,
    smaxcol: i32
) -> i32 {
    assert!(!pad.is_null(), "{}pnoutrefresh() : pad.is_null()", MODULE_PATH);

    bindings::pnoutrefresh(pad, pminrow, pmincol, sminrow, smincol, smaxrow, smaxcol)
}

/// <https://invisible-island.net/ncurses/man/curs_pad.3x.html>
pub unsafe fn prefresh(
    pad: WINDOW,
    pminrow: i32,
    pmincol: i32,
    sminrow: i32,
    smincol: i32,
    smaxrow: i32,
    smaxcol: i32
) -> i32 {
    assert!(!pad.is_null(), "{}prefresh() : pad.is_null()", MODULE_PATH);

    bindings::prefresh(pad, pminrow, pmincol, sminrow, smincol, smaxrow, smaxcol)
}

/// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
pub fn putp(str: &[i8]) -> i32 {
    unsafe {
        bindings::putp(str.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub unsafe fn putwin(win: WINDOW, filep: FILE) -> i32 {
    assert!(!win.is_null(), "{}putwin() : win.is_null()", MODULE_PATH);
    assert!(!filep.is_null(), "{}putwin() : filep.is_null()", MODULE_PATH);

    bindings::putwin(win, filep)
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn qiflush() {
    unsafe {
        bindings::qiflush()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn raw() -> i32 {
    unsafe {
        bindings::raw()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_refresh.3x.html>
pub unsafe fn redrawwin(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}redrawwin() : win.is_null()", MODULE_PATH);

    bindings::redrawwin(win)
}

/// <https://invisible-island.net/ncurses/man/curs_refresh.3x.html>
pub fn refresh() -> i32 {
    unsafe {
        bindings::refresh()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn reset_color_pairs() {
    unsafe {
        bindings::reset_color_pairs()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn reset_prog_mode() -> i32 {
    unsafe {
        bindings::reset_prog_mode()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn reset_shell_mode() -> i32 {
    unsafe {
        bindings::reset_shell_mode()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn resetty() -> i32 {
    unsafe {
        bindings::resetty()
    }
}

/// <https://invisible-island.net/ncurses/man/resizeterm.3x.html>
pub fn resize_term(lines: i32, cols: i32) -> i32 {
    unsafe {
        bindings::resize_term(lines, cols)
    }
}

/// <https://invisible-island.net/ncurses/man/resizeterm.3x.html>
pub fn resizeterm(lines: i32, cols: i32) -> i32 {
    unsafe {
        bindings::resizeterm(lines, cols)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn ripoffline(line: i32, init: bindings::RipoffInit) -> i32 {
    unsafe {
        bindings::ripoffline(line, init)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_kernel.3x.html>
pub fn savetty() -> i32 {
    unsafe {
        bindings::savetty()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_scr_dump.3x.html>
pub fn scr_dump(filename: &[i8]) -> i32 {
    unsafe {
        bindings::scr_dump(filename.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_scr_dump.3x.html>
pub fn scr_init(filename: &[i8]) -> i32 {
    unsafe {
        bindings::scr_init(filename.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_scr_dump.3x.html>
pub fn scr_restore(filename: &[i8]) -> i32 {
    unsafe {
        bindings::scr_restore(filename.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_scr_dump.3x.html>
pub fn scr_set(filename: &[i8]) -> i32 {
    unsafe {
        bindings::scr_set(filename.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_scroll.3x.html>
pub fn scrl(n: i32) -> i32 {
    unsafe {
        bindings::scrl(n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_scroll.3x.html>
pub unsafe fn scroll(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}scroll() : win.is_null()", MODULE_PATH);

    bindings::scroll(win)
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub unsafe fn scrollok(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}scrollok() : win.is_null()", MODULE_PATH);

    bindings::scrollok(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_threads.3x.html>
pub fn set_escdelay(size: i32) -> i32 {
    unsafe {
        bindings::set_escdelay(size)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_threads.3x.html>
pub fn set_tabsize(size: i32) -> i32 {
    unsafe {
        bindings::set_tabsize(size)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_initscr.3x.html>
pub unsafe fn set_term(scr: SCREEN) -> Option<SCREEN> {
    assert!(!scr.is_null(), "{}set_term() : scr.is_null()", MODULE_PATH);

    bindings::set_term(scr).as_mut().map(|ptr| ptr as SCREEN)
}

/// <https://invisible-island.net/ncurses/man/curs_getcchar.3x.html>
pub unsafe fn setcchar(wcval: *mut cchar_t, wch: *const wchar_t, attrs: attr_t, color_pair: short_t, opts: *const libc::c_void) -> i32 {
    assert!(!wcval.is_null(), "{}setcchar() : wcval.is_null()", MODULE_PATH);
    assert!(!wch.is_null(), "{}setcchar() : wch.is_null()", MODULE_PATH);

    bindings::setcchar(wcval, wch, attrs, color_pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub fn setscrreg(top: i32, bot: i32) -> i32 {
    unsafe {
        bindings::setscrreg(top, bot)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_attr() -> attr_t {
    unsafe {
        bindings::slk_attr()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub unsafe fn slk_attr_off(attrs: attr_t, opts: *mut libc::c_void) -> i32 {
    bindings::slk_attr_off(attrs, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub unsafe fn slk_attr_on(attrs: attr_t, opts: *mut libc::c_void) -> i32 {
    bindings::slk_attr_on(attrs, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub unsafe fn slk_attr_set(attrs: attr_t, pair: short_t, opts: *mut libc::c_void) -> i32 {
    bindings::slk_attr_set(attrs, pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_attroff(ch: chtype) -> i32 {
    unsafe {
        bindings::slk_attroff(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_attron(ch: chtype) -> i32 {
    unsafe {
        bindings::slk_attron(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_attrset(ch: chtype) -> i32 {
    unsafe {
        bindings::slk_attrset(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_clear() -> i32 {
    unsafe {
        bindings::slk_clear()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_color(pair: short_t) -> i32 {
    unsafe {
        bindings::slk_color(pair)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_init(fmt: i32) -> i32 {
    unsafe {
        bindings::slk_init(fmt)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_label(n: i32) -> Option<String> {
    unsafe {
        (bindings::slk_label(n) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_noutrefresh() -> i32 {
    unsafe {
        bindings::slk_noutrefresh()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_refresh() -> i32 {
    unsafe {
        bindings::slk_refresh()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_restore() -> i32 {
    unsafe {
        bindings::slk_restore()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_set(n: i32, label: &[i8], fmt: i32) -> i32 {
    unsafe {
        bindings::slk_set(n, label.as_ptr(), fmt)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_touch() -> i32 {
    unsafe {
        bindings::slk_touch()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_slk.3x.html>
pub fn slk_wset(n: i32, label: &[wchar_t], fmt: i32) -> i32 {
    unsafe {
        bindings::slk_wset(n, label.as_ptr(), fmt)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub fn standend() -> i32 {
    unsafe {
        bindings::standend()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub fn standout() -> i32 {
    unsafe {
        bindings::standout()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_color.3x.html>
pub fn start_color() -> i32 {
    unsafe {
        bindings::start_color()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_pad.3x.html>
pub unsafe fn subpad(win: WINDOW, lines: i32, cols: i32, y: i32, x: i32) -> Option<WINDOW> {
    assert!(!win.is_null(), "{}subpad() : win.is_null()", MODULE_PATH);

    bindings::subpad(win, lines, cols, y, x).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn subwin(win: WINDOW, lines: i32, cols: i32, y: i32, x: i32) -> Option<WINDOW> {
    assert!(!win.is_null(), "{}subwin() : win.is_null()", MODULE_PATH);

    bindings::subwin(win, lines, cols, y, x).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn syncok(win: WINDOW, bf: bool) -> i32 {
    assert!(!win.is_null(), "{}syncok() : win.is_null()", MODULE_PATH);

    bindings::syncok(win, bf)
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn term_attrs() -> attr_t {
    unsafe {
        bindings::term_attrs()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn termattrs() -> chtype {
    unsafe {
        bindings::termattrs()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_termattrs.3x.html>
pub fn termname() -> Option<String> {
    unsafe {
        (bindings::termname() as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
pub fn tigetflag(capname: &[i8]) -> i32 {
    unsafe {
        bindings::tigetflag(capname.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
pub fn tigetnum(capname: &[i8]) -> i32 {
    unsafe {
        bindings::tigetnum(capname.as_ptr())
    }
}

/// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
pub fn tigetstr(capname: &[i8]) -> Option<String> {
    unsafe {
        (bindings::tigetstr(capname.as_ptr()) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn timeout(delay: i32) {
    unsafe {
        bindings::timeout(delay)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_touch.3x.html>
pub unsafe fn touchline(win: WINDOW, start: i32, count: i32) -> i32 {
    assert!(!win.is_null(), "{}touchline() : win.is_null()", MODULE_PATH);

    bindings::touchline(win, start, count)
}

/// <https://invisible-island.net/ncurses/man/curs_touch.3x.html>
pub unsafe fn touchwin(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}touchwin() : win.is_null()", MODULE_PATH);

    bindings::touchwin(win)
}

/// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
pub fn tparm(s: &[i8]) -> Option<String> {
    unsafe {
        (bindings::tparm(s.as_ptr()) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
//pub fn tputs

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub fn typeahead(fd: i32) -> i32 {
    unsafe {
        bindings::typeahead(fd)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn unctrl(c: chtype) -> Option<String> {
    unsafe {
        (bindings::unctrl(c) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/curs_get_wch.3x.html>
pub fn unget_wch(ch: wchar_t) -> i32 {
    unsafe {
        bindings::unget_wch(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_getch.3x.html>
pub fn ungetch(ch: i32) -> i32 {
    unsafe {
        bindings::ungetch(ch)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_touch.3x.html>
pub unsafe fn untouchwin(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}untouchwin() : win.is_null()", MODULE_PATH);

    bindings::untouchwin(win)
}

/// <https://invisible-island.net/ncurses/man/default_colors.3x.html>
pub fn use_default_colors() -> i32 {
    unsafe {
        bindings::use_default_colors()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn use_env(f: bool) {
    unsafe {
        bindings::use_env(f)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_extend.3x.html>
pub fn use_extended_names(enable: bool) -> i32 {
    unsafe {
        bindings::use_extended_names(enable)
    }
}

/// <https://invisible-island.net/ncurses/man/legacy_coding.3x.html>
pub fn use_legacy_coding(level: i32) -> i32 {
    unsafe {
        bindings::use_legacy_coding(level)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub fn use_tioctl(f: bool) {
    unsafe {
        bindings::use_tioctl(f)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
pub fn vid_attr(attrs: attr_t, pair: short_t) -> i32 {
    unsafe {
        bindings::vid_attr(attrs, pair, ptr::null_mut())
    }
}

//int vid_puts(attr_t attrs, short pair, void *opts, int (*putc)(int));

/// <https://invisible-island.net/ncurses/man/curs_terminfo.3x.html>
pub fn vidattr(attrs: chtype) -> i32 {
    unsafe {
        bindings::vidattr(attrs)
    }
}

//int vidputs(chtype attrs, int (*putc)(int));

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub fn vline(ch: chtype, n: i32) -> i32 {
    unsafe {
        bindings::vline(ch, n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub fn vline_set(wch: &cchar_t, n: i32) -> i32 {
    unsafe {
        bindings::vline_set(wch, n)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_add_wch.3x.html>
pub unsafe fn wadd_wch(win: WINDOW, wch: &cchar_t) -> i32 {
    assert!(!win.is_null(), "{}wadd_wch() : win.is_null()", MODULE_PATH);

    bindings::wadd_wch(win, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_add_wchstr.3x.html>
pub unsafe fn wadd_wchnstr(win: WINDOW, wchstr: &[cchar_t], n: i32) -> i32 {
    assert!(!win.is_null(), "{}wadd_wchnstr() : win.is_null()", MODULE_PATH);

    bindings::wadd_wchnstr(win, wchstr.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_add_wchstr.3x.html>
pub unsafe fn wadd_wchstr(win: WINDOW, wchstr: &[cchar_t]) -> i32 {
    assert!(!win.is_null(), "{}wadd_wchstr() : win.is_null()", MODULE_PATH);

    bindings::wadd_wchstr(win, wchstr.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_addch.3x.html>
pub unsafe fn waddch(win: WINDOW, ch: chtype) -> i32 {
    assert!(!win.is_null(), "{}waddch() : win.is_null()", MODULE_PATH);

    bindings::waddch(win, ch)
}

/// <https://invisible-island.net/ncurses/man/curs_addchstr.3x.html>
pub unsafe fn waddchnstr(win: WINDOW, chstr: &[chtype], n: i32) -> i32 {
    assert!(!win.is_null(), "{}waddchnstr() : win.is_null()", MODULE_PATH);

    bindings::waddchnstr(win, chstr.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_addchstr.3x.html>
pub unsafe fn waddchstr(win: WINDOW, chstr: &[chtype]) -> i32 {
    assert!(!win.is_null(), "{}waddchstr() : win.is_null()", MODULE_PATH);

    bindings::waddchstr(win, chstr.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_addstr.3x.html>
pub unsafe fn waddnstr(win: WINDOW, str: &[i8], n: i32) -> i32 {
    assert!(!win.is_null(), "{}waddnstr() : win.is_null()", MODULE_PATH);

    bindings::waddnstr(win, str.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_addwstr.3x.html>
pub unsafe fn waddnwstr(win: WINDOW, wstr: &[wchar_t], n: i32) -> i32 {
    assert!(!win.is_null(), "{}waddnwstr() : win.is_null()", MODULE_PATH);

    bindings::waddnwstr(win, wstr.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_addstr.3x.html>
pub unsafe fn waddstr(win: WINDOW, str: &[i8]) -> i32 {
    assert!(!win.is_null(), "{}waddstr() : win.is_null()", MODULE_PATH);

    bindings::waddstr(win, str.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_addwstr.3x.html>
pub unsafe fn waddwstr(win: WINDOW, wstr: &[wchar_t]) -> i32 {
    assert!(!win.is_null(), "{}waddwstr() : win.is_null()", MODULE_PATH);

    bindings::waddwstr(win, wstr.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wattr_get(win: WINDOW, attrs: *mut attr_t, pair: *mut short_t, opts: *mut libc::c_void) -> i32 {
    assert!(!win.is_null(), "{}wattr_get() : win.is_null()", MODULE_PATH);
    assert!(!attrs.is_null(), "{}wattr_get() : attrs.is_null()", MODULE_PATH);
    assert!(!pair.is_null(), "{}wattr_get() : pair.is_null()", MODULE_PATH);

    bindings::wattr_get(win, attrs, pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wattr_off(win: WINDOW, attrs: attr_t, opts: *mut libc::c_void) -> i32 {
    assert!(!win.is_null(), "{}wattr_off() : win.is_null()", MODULE_PATH);
    assert!(opts.is_null(), "{}wattr_off() : !opts.is_null()", MODULE_PATH);

    bindings::wattr_off(win, attrs, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wattr_on(win: WINDOW, attrs: attr_t, opts: *mut libc::c_void) -> i32 {
    assert!(!win.is_null(), "{}wattr_on() : win.is_null()", MODULE_PATH);
    assert!(opts.is_null(), "{}wattr_on() : !opts.is_null()", MODULE_PATH);

    bindings::wattr_on(win, attrs, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wattr_set(win: WINDOW, attrs: attr_t, pair: short_t, opts: *mut libc::c_void) -> i32 {
    assert!(!win.is_null(), "{}wattr_set() : win.is_null()", MODULE_PATH);

    bindings::wattr_set(win, attrs, pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wattroff(win: WINDOW, attrs: i32) -> i32 {
    assert!(!win.is_null(), "{}wattroff() : win.is_null()", MODULE_PATH);

    bindings::wattroff(win, attrs)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wattron(win: WINDOW, attrs: i32) -> i32 {
    assert!(!win.is_null(), "{}wattron() : win.is_null()", MODULE_PATH);

    bindings::wattron(win, attrs)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wattrset(win: WINDOW, attrs: i32) -> i32 {
    assert!(!win.is_null(), "{}wattrset() : win.is_null()", MODULE_PATH);

    bindings::wattrset(win, attrs)
}

/// <https://invisible-island.net/ncurses/man/curs_bkgd.3x.html>
pub unsafe fn wbkgd(win: WINDOW, ch: chtype) -> i32 {
    assert!(!win.is_null(), "{}wbkgd() : win.is_null()", MODULE_PATH);

    bindings::wbkgd(win, ch)
}

/// <https://invisible-island.net/ncurses/man/curs_bkgd.3x.html>
pub unsafe fn wbkgdset(win: WINDOW, ch: chtype) {
    assert!(!win.is_null(), "{}wbkgdset() : win.is_null()", MODULE_PATH);

    bindings::wbkgdset(win, ch)
}

/// <https://invisible-island.net/ncurses/man/curs_bkgrnd.3x.html>
pub unsafe fn wbkgrnd(win: WINDOW, wch: &cchar_t) -> i32 {
    assert!(!win.is_null(), "{}wbkgrnd() : win.is_null()", MODULE_PATH);

    bindings::wbkgrnd(win, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_bkgrnd.3x.html>
pub unsafe fn wbkgrndset(win: WINDOW, wch: &cchar_t) {
    assert!(!win.is_null(), "{}wbkgrndset() : win.is_null()", MODULE_PATH);

    bindings::wbkgrndset(win, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub unsafe fn wborder(
    win: WINDOW,
    ls: chtype,
    rs: chtype,
    ts: chtype,
    bs: chtype,
    tl: chtype,
    tr: chtype,
    bl: chtype,
    br: chtype
) -> i32 {
    assert!(!win.is_null(), "{}wborder() : win.is_null()", MODULE_PATH);

    bindings::wborder(win, ls, rs, ts, bs, tl, tr, bl, br)
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub unsafe fn wborder_set(
    win: WINDOW,
    ls: &cchar_t,
    rs: &cchar_t,
    ts: &cchar_t,
    bs: &cchar_t,
    tl: &cchar_t,
    tr: &cchar_t,
    bl: &cchar_t,
    br: &cchar_t
) -> i32 {
    assert!(!win.is_null(), "{}wborder_set() : win.is_null()", MODULE_PATH);

    bindings::wborder_set(win, ls, rs, ts, bs, tl, tr, bl, br)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wchgat(win: WINDOW, n: i32, attr: attr_t, color: short_t, opts: *const libc::c_void) -> i32 {
    assert!(!win.is_null(), "{}wchgat() : win.is_null()", MODULE_PATH);

    bindings::wchgat(win, n, attr, color, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_clear.3x.html>
pub unsafe fn wclear(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wclear() : win.is_null()", MODULE_PATH);

    bindings::wclear(win)
}

/// <https://invisible-island.net/ncurses/man/curs_clear.3x.html>
pub unsafe fn wclrtobot(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wclrtobot() : win.is_null()", MODULE_PATH);

    bindings::wclrtobot(win)
}

/// <https://invisible-island.net/ncurses/man/curs_clear.3x.html>
pub unsafe fn wclrtoeol(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wclrtoeol() : win.is_null()", MODULE_PATH);

    bindings::wclrtoeol(win)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wcolor_set(win: WINDOW, pair: short_t, opts: *mut libc::c_void) -> i32 {
    assert!(!win.is_null(), "{}wcolor_set() : win.is_null()", MODULE_PATH);

    bindings::wcolor_set(win, pair, opts)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn wcursyncup(win: WINDOW) {
    assert!(!win.is_null(), "{}wcursyncup() : win.is_null()", MODULE_PATH);

    bindings::wcursyncup(win)
}

/// <https://invisible-island.net/ncurses/man/curs_delch.3x.html>
pub unsafe fn wdelch(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wdelch() : win.is_null()", MODULE_PATH);

    bindings::wdelch(win)
}

/// <https://invisible-island.net/ncurses/man/curs_add_wch.3x.html>
pub unsafe fn wecho_wchar(win: WINDOW, wch: &cchar_t) -> i32 {
    assert!(!win.is_null(), "{}wecho_wchar() : win.is_null()", MODULE_PATH);

    bindings::wecho_wchar(win, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_addch.3x.html>
pub unsafe fn wechochar(win: WINDOW, ch: chtype) -> i32 {
    assert!(!win.is_null(), "{}wechochar() : win.is_null()", MODULE_PATH);

    bindings::wechochar(win, ch)
}

/// <https://invisible-island.net/ncurses/man/curs_clear.3x.html>
pub unsafe fn werase(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}werase() : win.is_null()", MODULE_PATH);

    bindings::werase(win)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wch.3x.html>
pub unsafe fn wget_wch(win: WINDOW, wch: *mut wint_t) -> i32 {
    assert!(!win.is_null(), "{}wget_wch() : win.is_null()", MODULE_PATH);
    assert!(!wch.is_null(), "{}wget_wch() : wch.is_null()", MODULE_PATH);

    bindings::wget_wch(win, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wstr.3x.html>
pub unsafe fn wget_wstr(win: WINDOW, wstr: *mut wint_t) -> i32 {
    assert!(!win.is_null(), "{}wget_wstr() : win.is_null()", MODULE_PATH);
    assert!(!wstr.is_null(), "{}wget_wstr() : wstr.is_null()", MODULE_PATH);

    bindings::wget_wstr(win, wstr)
}

/// <https://invisible-island.net/ncurses/man/curs_bkgrnd.3x.html>
pub unsafe fn wgetbkgrnd(win: WINDOW, wch: *mut cchar_t) -> i32 {
    assert!(!win.is_null(), "{}wgetbkgrnd() : win.is_null()", MODULE_PATH);
    assert!(!wch.is_null(), "{}wgetbkgrnd() : wch.is_null()", MODULE_PATH);

    bindings::wgetbkgrnd(win, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_getch.3x.html>
pub unsafe fn wgetch(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wgetch() : win.is_null()", MODULE_PATH);

    bindings::wgetch(win)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn wgetdelay(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wgetdelay() : win.is_null()", MODULE_PATH);

    bindings::wgetdelay(win)
}

/// <https://invisible-island.net/ncurses/man/curs_get_wstr.3x.html>
pub unsafe fn wgetn_wstr(win: WINDOW, wstr: *mut wint_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}wgetn_wstr() : win.is_null()", MODULE_PATH);
    assert!(!wstr.is_null(), "{}wgetn_wstr() : wstr.is_null()", MODULE_PATH);

    bindings::wgetn_wstr(win, wstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_getstr.3x.html>
pub unsafe fn wgetnstr(win: WINDOW, str: *mut i8, n: i32) -> i32 {
    assert!(!win.is_null(), "{}wgetnstr() : win.is_null()", MODULE_PATH);
    assert!(!str.is_null(), "{}wgetnstr() : str.is_null()", MODULE_PATH);

    bindings::wgetnstr(win, str, n)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn wgetparent(win: WINDOW) -> Option<WINDOW> {
    assert!(!win.is_null(), "{}wgetparent() : win.is_null()", MODULE_PATH);

    bindings::wgetparent(win).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/curs_opaque.3x.html>
pub unsafe fn wgetscrreg(win: WINDOW, top: *mut i32, bot: *mut i32) -> i32 {
    assert!(!win.is_null(), "{}wgetscrreg() : win.is_null()", MODULE_PATH);
    assert!(!top.is_null(), "{}wgetscrreg() : top.is_null()", MODULE_PATH);
    assert!(!bot.is_null(), "{}wgetscrreg() : bot.is_null()", MODULE_PATH);

    bindings::wgetscrreg(win, top, bot)
}

/// <https://invisible-island.net/ncurses/man/curs_getstr.3x.html>
pub unsafe fn wgetstr(win: WINDOW, str: *mut i8) -> i32 {
    assert!(!win.is_null(), "{}wgetstr() : win.is_null()", MODULE_PATH);
    assert!(!str.is_null(), "{}wgetstr() : str.is_null()", MODULE_PATH);

    bindings::wgetstr(win, str)
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub unsafe fn whline(win: WINDOW, ch: chtype, n: i32) -> i32 {
    assert!(!win.is_null(), "{}whline() : win.is_null()", MODULE_PATH);

    bindings::whline(win, ch, n)
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub unsafe fn whline_set(win: WINDOW, wch: &cchar_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}whline_set() : win.is_null()", MODULE_PATH);

    bindings::whline_set(win, wch, n)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wch.3x.html>
pub unsafe fn win_wch(win: WINDOW, wcval: *mut cchar_t) -> i32 {
    assert!(!win.is_null(), "{}win_wch() : win.is_null()", MODULE_PATH);
    assert!(!wcval.is_null(), "{}win_wch() : wcval.is_null()", MODULE_PATH);

    bindings::win_wch(win, wcval)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wchstr.3x.html>
pub unsafe fn win_wchnstr(win: WINDOW, wchstr: *mut cchar_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}win_wchnstr() : win.is_null()", MODULE_PATH);
    assert!(!wchstr.is_null(), "{}win_wchnstr() : wchstr.is_null()", MODULE_PATH);

    bindings::win_wchnstr(win, wchstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_in_wchstr.3x.html>
pub unsafe fn win_wchstr(win: WINDOW, wchstr: *mut cchar_t) -> i32 {
    assert!(!win.is_null(), "{}win_wchstr() : win.is_null()", MODULE_PATH);
    assert!(!wchstr.is_null(), "{}win_wchstr() : wchstr.is_null()", MODULE_PATH);

    bindings::win_wchstr(win, wchstr)
}

/// <https://invisible-island.net/ncurses/man/curs_inch.3x.html>
pub unsafe fn winch(win: WINDOW) -> chtype {
    assert!(!win.is_null(), "{}winch() : win.is_null()", MODULE_PATH);

    bindings::winch(win)
}

/// <https://invisible-island.net/ncurses/man/curs_inchstr.3x.html>
pub unsafe fn winchnstr(win: WINDOW, chstr: *mut chtype, n: i32) -> i32 {
    assert!(!win.is_null(), "{}winchnstr() : win.is_null()", MODULE_PATH);
    assert!(!chstr.is_null(), "{}winchnstr() : chstr.is_null()", MODULE_PATH);

    bindings::winchnstr(win, chstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_inchstr.3x.html>
pub unsafe fn winchstr(win: WINDOW, chstr: *mut chtype) -> i32 {
    assert!(!win.is_null(), "{}winchstr() : win.is_null()", MODULE_PATH);
    assert!(!chstr.is_null(), "{}winchstr() : chstr.is_null()", MODULE_PATH);

    bindings::winchstr(win, chstr)
}

/// <https://invisible-island.net/ncurses/man/curs_instr.3x.html>
pub unsafe fn winnstr(win: WINDOW, str: *mut i8, n: i32) -> i32 {
    assert!(!win.is_null(), "{}winnstr() : win.is_null()", MODULE_PATH);
    assert!(!str.is_null(), "{}winnstr() : str.is_null()", MODULE_PATH);

    bindings::winnstr(win, str, n)
}

/// <https://invisible-island.net/ncurses/man/curs_inwstr.3x.html>
pub unsafe fn winnwstr(win: WINDOW, wstr: *mut wchar_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}winnwstr() : win.is_null()", MODULE_PATH);
    assert!(!wstr.is_null(), "{}winnwstr() : wstr.is_null()", MODULE_PATH);

    bindings::winnwstr(win, wstr, n)
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wstr.3x.html>
pub unsafe fn wins_nwstr(win: WINDOW, wstr: &[wchar_t], n: i32) -> i32 {
    assert!(!win.is_null(), "{}wins_nwstr() : win.is_null()", MODULE_PATH);

    bindings::wins_nwstr(win, wstr.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wch.3x.html>
pub unsafe fn wins_wch(win: WINDOW, wch: &cchar_t) -> i32 {
    assert!(!win.is_null(), "{}wins_wch() : win.is_null()", MODULE_PATH);

    bindings::wins_wch(win, wch)
}

/// <https://invisible-island.net/ncurses/man/curs_ins_wstr.3x.html>
pub unsafe fn wins_wstr(win: WINDOW, wstr: &[wchar_t]) -> i32 {
    assert!(!win.is_null(), "{}wins_wstr() : win.is_null()", MODULE_PATH);

    bindings::wins_wstr(win, wstr.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_insch.3x.html>
pub unsafe fn winsch(win: WINDOW, ch: chtype) -> i32 {
    assert!(!win.is_null(), "{}winsch() : win.is_null()", MODULE_PATH);

    bindings::winsch(win, ch)
}

/// <https://invisible-island.net/ncurses/man/curs_deleteln.3x.html>
pub unsafe fn winsdelln(win: WINDOW, n: i32) -> i32 {
    assert!(!win.is_null(), "{}winsdelln() : win.is_null()", MODULE_PATH);

    bindings::winsdelln(win, n)
}

/// <https://invisible-island.net/ncurses/man/curs_deleteln.3x.html>
pub unsafe fn winsertln(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}winsertln() : win.is_null()", MODULE_PATH);

    bindings::winsertln(win)
}

/// <https://invisible-island.net/ncurses/man/curs_insstr.3x.html>
pub unsafe fn winsnstr(win: WINDOW, str: &[i8], n: i32) -> i32 {
    assert!(!win.is_null(), "{}winsnstr() : win.is_null()", MODULE_PATH);

    bindings::winsnstr(win, str.as_ptr(), n)
}

/// <https://invisible-island.net/ncurses/man/curs_insstr.3x.html>
pub unsafe fn winsstr(win: WINDOW, str: &[i8]) -> i32 {
    assert!(!win.is_null(), "{}winsstr() : win.is_null()", MODULE_PATH);

    bindings::winsstr(win, str.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/curs_instr.3x.html>
pub unsafe fn winstr(win: WINDOW, str: *mut i8) -> i32 {
    assert!(!win.is_null(), "{}winstr() : win.is_null()", MODULE_PATH);
    assert!(!str.is_null(), "{}winstr() : str.is_null()", MODULE_PATH);

    bindings::winstr(win, str)
}

/// <https://invisible-island.net/ncurses/man/curs_inwstr.3x.html>
pub unsafe fn winwstr(win: WINDOW, wstr: *mut wchar_t) -> i32 {
    assert!(!win.is_null(), "{}winwstr() : win.is_null()", MODULE_PATH);
    assert!(!wstr.is_null(), "{}winwstr() : wstr.is_null()", MODULE_PATH);

    bindings::winwstr(win, wstr)
}

/// <https://invisible-island.net/ncurses/man/curs_move.3x.html>
pub unsafe fn wmove(win: WINDOW, y: i32, x: i32) -> i32 {
    assert!(!win.is_null(), "{}wmove() : win.is_null()", MODULE_PATH);

    bindings::wmove(win, y, x)
}

/// <https://invisible-island.net/ncurses/man/curs_refresh.3x.html>
pub unsafe fn wnoutrefresh(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wnoutrefresh() : win.is_null()", MODULE_PATH);

    bindings::wnoutrefresh(win)
}

/// <https://invisible-island.net/ncurses/man/curs_refresh.3x.html>
pub unsafe fn wredrawln(win: WINDOW, beg_line: i32, num_lines: i32) -> i32 {
    assert!(!win.is_null(), "{}wredrawln() : win.is_null()", MODULE_PATH);

    bindings::wredrawln(win, beg_line, num_lines)
}

/// <https://invisible-island.net/ncurses/man/curs_refresh.3x.html>
pub unsafe fn wrefresh(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wrefresh() : win.is_null()", MODULE_PATH);

    bindings::wrefresh(win)
}

/// <https://invisible-island.net/ncurses/man/wresize.3x.html>
pub unsafe fn wresize(win: WINDOW, lines: i32, columns: i32) -> i32 {
    assert!(!win.is_null(), "{}wresize() : win.is_null()", MODULE_PATH);

    bindings::wresize(win, lines, columns)
}

/// <https://invisible-island.net/ncurses/man/curs_scroll.3x.html>
pub unsafe fn wscrl(win: WINDOW, n: i32) -> i32 {
    assert!(!win.is_null(), "{}wscrl() : win.is_null()", MODULE_PATH);

    bindings::wscrl(win, n)
}

/// <https://invisible-island.net/ncurses/man/curs_outopts.3x.html>
pub unsafe fn wsetscrreg(win: WINDOW, top: i32, bot: i32) -> i32 {
    assert!(!win.is_null(), "{}wsetscrreg() : win.is_null()", MODULE_PATH);

    bindings::wsetscrreg(win, top, bot)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wstandend(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wstandend() : win.is_null()", MODULE_PATH);

    bindings::wstandend(win)
}

/// <https://invisible-island.net/ncurses/man/curs_attr.3x.html>
pub unsafe fn wstandout(win: WINDOW) -> i32 {
    assert!(!win.is_null(), "{}wstandout() : win.is_null()", MODULE_PATH);

    bindings::wstandout(win)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn wsyncdown(win: WINDOW) {
    assert!(!win.is_null(), "{}wsyncdown() : win.is_null()", MODULE_PATH);

    bindings::wsyncdown(win)
}

/// <https://invisible-island.net/ncurses/man/curs_window.3x.html>
pub unsafe fn wsyncup(win: WINDOW) {
    assert!(!win.is_null(), "{}wsyncup() : win.is_null()", MODULE_PATH);

    bindings::wsyncup(win)
}

/// <https://invisible-island.net/ncurses/man/curs_inopts.3x.html>
pub unsafe fn wtimeout(win: WINDOW, delay: i32) {
    assert!(!win.is_null(), "{}wtimeout() : win.is_null()", MODULE_PATH);

    bindings::wtimeout(win, delay)
}

/// <https://invisible-island.net/ncurses/man/curs_touch.3x.html>
pub unsafe fn wtouchln(win: WINDOW, y: i32, n: i32, changed: i32) -> i32 {
    assert!(!win.is_null(), "{}wtouchln() : win.is_null()", MODULE_PATH);

    bindings::wtouchln(win, y, n, changed)
}

/// <https://invisible-island.net/ncurses/man/curs_util.3x.html>
pub unsafe fn wunctrl(ch: *mut cchar_t) -> *mut wchar_t {
    assert!(!ch.is_null(), "{}wunctrl() : ch.is_null()", MODULE_PATH);

    bindings::wunctrl(ch)
}

/// <https://invisible-island.net/ncurses/man/curs_border.3x.html>
pub unsafe fn wvline(win: WINDOW, ch: chtype, n: i32) -> i32 {
    assert!(!win.is_null(), "{}wvline() : win.is_null()", MODULE_PATH);

    bindings::wvline(win, ch, n)
}

/// <https://invisible-island.net/ncurses/man/curs_border_set.3x.html>
pub unsafe fn wvline_set(win: WINDOW, wch: &cchar_t, n: i32) -> i32 {
    assert!(!win.is_null(), "{}wvline_set() : win.is_null()", MODULE_PATH);

    bindings::wvline_set(win, wch, n)
}

/* Line graphics */
pub fn NCURSES_ACS(c: char) -> chtype { unsafe { *acs_map().offset((c as libc::c_uchar) as isize) as chtype } }

/* VT100 symbols begin here */
pub fn ACS_ULCORNER() -> chtype { NCURSES_ACS('l') } /* upper left corner */
pub fn ACS_LLCORNER() -> chtype { NCURSES_ACS('m') } /* lower left corner */
pub fn ACS_URCORNER() -> chtype { NCURSES_ACS('k') } /* upper right corner */
pub fn ACS_LRCORNER() -> chtype { NCURSES_ACS('j') } /* lower right corner */
pub fn ACS_LTEE() -> chtype { NCURSES_ACS('t') } /* tee pointing right */
pub fn ACS_RTEE() -> chtype { NCURSES_ACS('u') } /* tee pointing left */
pub fn ACS_BTEE() -> chtype { NCURSES_ACS('v') } /* tee pointing up */
pub fn ACS_TTEE() -> chtype { NCURSES_ACS('w') } /* tee pointing down */
pub fn ACS_HLINE() -> chtype { NCURSES_ACS('q') } /* horizontal line */
pub fn ACS_VLINE() -> chtype { NCURSES_ACS('x') } /* vertical line */
pub fn ACS_PLUS() -> chtype { NCURSES_ACS('n') } /* large plus or crossover */
pub fn ACS_S1() -> chtype { NCURSES_ACS('o') } /* scan line 1 */
pub fn ACS_S9() -> chtype { NCURSES_ACS('s') } /* scan line 9 */
pub fn ACS_DIAMOND() -> chtype { NCURSES_ACS('`') } /* diamond */
pub fn ACS_CKBOARD() -> chtype { NCURSES_ACS('a') } /* checker board(stipple) */
pub fn ACS_DEGREE() -> chtype { NCURSES_ACS('f') } /* degree symbol */
pub fn ACS_PLMINUS() -> chtype { NCURSES_ACS('g') } /* plus/minus */
pub fn ACS_BULLET() -> chtype { NCURSES_ACS('~') } /* bullet */

/* Teletype 5410v1 symbols begin here */
pub fn ACS_LARROW() -> chtype { NCURSES_ACS(',') } /* arrow pointing left */
pub fn ACS_RARROW() -> chtype { NCURSES_ACS('+') } /* arrow pointing right */
pub fn ACS_DARROW() -> chtype { NCURSES_ACS('.') } /* arrow pointing down */
pub fn ACS_UARROW() -> chtype { NCURSES_ACS('-') } /* arrow pointing up */
pub fn ACS_BOARD() -> chtype { NCURSES_ACS('h') } /* board of squares */
pub fn ACS_LANTERN() -> chtype { NCURSES_ACS('i') } /* lantern symbol */
pub fn ACS_BLOCK() -> chtype { NCURSES_ACS('0') } /* solid square block */

/*
 * These aren't documented, but a lot of System Vs have them anyway
 * (you can spot pprryyzz{{||}} in a lot of AT&T terminfo strings).
 * The ACS_names may not match AT&T's, our source didn't know them.
 */
pub fn ACS_S3() -> chtype { NCURSES_ACS('p') } /* scan line 3 */
pub fn ACS_S7() -> chtype { NCURSES_ACS('r') } /* scan line 7 */
pub fn ACS_LEQUAL() -> chtype { NCURSES_ACS('y') } /* less/equal */
pub fn ACS_GEQUAL() -> chtype { NCURSES_ACS('z') } /* greater/equal */
pub fn ACS_PI() -> chtype { NCURSES_ACS('{') } /* Pi */
pub fn ACS_NEQUAL() -> chtype { NCURSES_ACS('|') } /* not equal */
pub fn ACS_STERLING() -> chtype { NCURSES_ACS('}') } /* UK pound sign */

/*
 * Line drawing ACS names are of the form ACS_trbl, where t is the top, r
 * is the right, b is the bottom, and l is the left. t, r, b, and l might
 * be B(blank), S(single), D(double), or T(thick). The subset defined
 * here only uses B and S.
 */
pub fn ACS_BSSB() -> chtype { ACS_ULCORNER() }
pub fn ACS_SSBB() -> chtype { ACS_LLCORNER() }
pub fn ACS_BBSS() -> chtype { ACS_URCORNER() }
pub fn ACS_SBBS() -> chtype { ACS_LRCORNER() }
pub fn ACS_SBSS() -> chtype { ACS_RTEE() }
pub fn ACS_SSSB() -> chtype { ACS_LTEE() }
pub fn ACS_SSBS() -> chtype { ACS_BTEE() }
pub fn ACS_BSSS() -> chtype { ACS_TTEE() }
pub fn ACS_BSBS() -> chtype { ACS_HLINE() }
pub fn ACS_SBSB() -> chtype { ACS_VLINE() }
pub fn ACS_SSSS() -> chtype { ACS_PLUS() }
