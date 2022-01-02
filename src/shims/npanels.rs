/*
    src/shims/npanels.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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
#![allow(missing_debug_implementations)]
#![allow(clippy::upper_case_acronyms)]

use crate::shims::{
    bindings, ncurses,
    constants::{TRUE, FALSE}
};

pub type PANEL = *mut bindings::panel;
pub type PANEL_USERPTR = *const libc::c_void;

type WINDOW = ncurses::WINDOW;
type SCREEN = ncurses::SCREEN;

static MODULE_PATH: &str = "ncursesw::shims::npanels::";

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn new_panel(win: WINDOW) -> Option<PANEL> {
    assert!(!win.is_null(), "{}new_panel() : win.is_null()", MODULE_PATH);

    bindings::new_panel(win).as_mut().map(|ptr| ptr as PANEL)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn bottom_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}bottom_panel() : pan.is_null()", MODULE_PATH);

    bindings::bottom_panel(pan)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn top_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}top_panel() : pan.is_null()", MODULE_PATH);

    bindings::top_panel(pan)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn show_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}show_panel() : pan.is_null()", MODULE_PATH);

    bindings::show_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub fn update_panels() {
    unsafe { bindings::update_panels() }
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn hide_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}hide_panel() : pan.is_null()", MODULE_PATH);

    bindings::hide_panel(pan)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_window(pan: PANEL) -> Option<WINDOW> {
    assert!(!pan.is_null(), "{}panel_window() : pan.is_null()", MODULE_PATH);

    bindings::panel_window(pan).as_mut().map(|ptr| ptr as WINDOW)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn replace_panel(pan: PANEL, win: WINDOW) -> i32 {
    assert!(!pan.is_null(), "{}replace_panel() : pan.is_null()", MODULE_PATH);
    assert!(!win.is_null(), "{}replace_panel() : win.is_null()", MODULE_PATH);

    bindings::replace_panel(pan, win)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn move_panel(pan: PANEL, starty: i32, startx: i32) -> i32 {
    assert!(!pan.is_null(), "{}move_panel() : pan.is_null()", MODULE_PATH);
    assert!(starty >= 0, "{}move_panel() : starty = {}", MODULE_PATH, starty);
    assert!(startx >= 0, "{}move_panel() : startx = {}", MODULE_PATH, startx);

    bindings::move_panel(pan, starty, startx)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_hidden(pan: PANEL) -> Option<bool> {
    assert!(!pan.is_null(), "{}panel_hidden() : pan.is_null()", MODULE_PATH);

    match bindings::panel_hidden(pan) {
        TRUE  => Some(true),
        FALSE => Some(false),
        _     => None
    }
}

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_above(pan: Option<PANEL>) -> Option<PANEL> {
    bindings::panel_above(return_mut_ptr!(pan)).as_mut().map(|ptr| ptr as PANEL)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_below(pan: Option<PANEL>) -> Option<PANEL> {
    bindings::panel_below(return_mut_ptr!(pan)).as_mut().map(|ptr| ptr as PANEL)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn set_panel_userptr(pan: PANEL, ptr: Option<PANEL_USERPTR>) -> i32 {
    assert!(!pan.is_null(), "{}set_panel_userptr() : pan.is_null()", MODULE_PATH);

    bindings::set_panel_userptr(pan, return_mut_ptr!(ptr))
}

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_userptr(pan: PANEL) -> Option<PANEL_USERPTR> {
    assert!(!pan.is_null(), "{}panel_userptr() : pan.is_null()", MODULE_PATH);

    (bindings::panel_userptr(pan) as *mut libc::c_void).as_mut().map(|ptr| ptr as PANEL_USERPTR)
}

/// # Safety
///
/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn del_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}del_panel() : pan.is_null()", MODULE_PATH);

    bindings::del_panel(pan)
} 

/// # Safety
///
/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn ceiling_panel(sp: SCREEN) -> Option<PANEL> {
    assert!(!sp.is_null(), "{}ceiling_panel() : sp.is_null()", MODULE_PATH);

    bindings::ceiling_panel(sp).as_mut().map(|ptr| ptr as PANEL)
}

/// # Safety
///
/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn ground_panel(sp: SCREEN) -> Option<PANEL> {
    assert!(!sp.is_null(), "{}ground_panel() : sp.is_null()", MODULE_PATH);

    bindings::ground_panel(sp).as_mut().map(|ptr| ptr as PANEL)
}

/// # Safety
///
/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn update_panels_sp(sp: SCREEN) {
    assert!(!sp.is_null(), "{}update_panels_sp() : sp.is_null()", MODULE_PATH);

    bindings::update_panels_sp(sp)
}
