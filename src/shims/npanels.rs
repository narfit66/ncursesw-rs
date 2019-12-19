/*
    src/shims/npanels.rs

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

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(missing_debug_implementations)]
#![allow(clippy::missing_safety_doc)]

use bindings;
use shims;

use constants::{TRUE, FALSE};

pub type PANEL = *mut bindings::panel;
pub type PANEL_USERPTR = *const libc::c_void;

type WINDOW = shims::ncurses::WINDOW;

static MODULE_PATH: &str = "ncursesw::shims::npanels::";

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn new_panel(win: WINDOW) -> Option<PANEL> {
    assert!(!win.is_null(), "{}new_panel() : win.is_null()", MODULE_PATH);

    bindings::new_panel(win).as_mut().map(|ptr| ptr as PANEL)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn bottom_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}bottom_panel() : pan.is_null()", MODULE_PATH);

    bindings::bottom_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn top_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}top_panel() : pan.is_null()", MODULE_PATH);

    bindings::top_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn show_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}show_panel() : pan.is_null()", MODULE_PATH);

    bindings::show_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub fn update_panels() {
    unsafe { bindings::update_panels() }
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn hide_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}hide_panel() : pan.is_null()", MODULE_PATH);

    bindings::hide_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_window(pan: PANEL) -> Option<WINDOW> {
    assert!(!pan.is_null(), "{}panel_window() : pan.is_null()", MODULE_PATH);

    bindings::panel_window(pan).as_mut().map(|ptr| ptr as WINDOW)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn replace_panel(pan: PANEL, win: WINDOW) -> i32 {
    assert!(!pan.is_null(), "{}replace_panel() : pan.is_null()", MODULE_PATH);
    assert!(!win.is_null(), "{}replace_panel() : win.is_null()", MODULE_PATH);

    bindings::replace_panel(pan, win)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn move_panel(pan: PANEL, starty: i32, startx: i32) -> i32 {
    assert!(!pan.is_null(), "{}move_panel() : pan.is_null()", MODULE_PATH);

    bindings::move_panel(pan, starty, startx)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_hidden(pan: PANEL) -> Option<bool> {
    assert!(!pan.is_null(), "{}panel_hidden() : pan.is_null()", MODULE_PATH);

    match bindings::panel_hidden(pan) {
        TRUE  => Some(true),
        FALSE => Some(false),
        _     => None
    }
}

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_above(pan: Option<PANEL>) -> Option<PANEL> {
    bindings::panel_above(return_mut_ptr!(pan)).as_mut().map(|ptr| ptr as PANEL)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_below(pan: Option<PANEL>) -> Option<PANEL> {
    bindings::panel_below(return_mut_ptr!(pan)).as_mut().map(|ptr| ptr as PANEL)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn set_panel_userptr(pan: PANEL, ptr: Option<PANEL_USERPTR>) -> i32 {
    assert!(!pan.is_null(), "{}set_panel_userptr() : pan.is_null()", MODULE_PATH);

    bindings::set_panel_userptr(pan, return_mut_ptr!(ptr))
}

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_userptr(pan: PANEL) -> Option<PANEL_USERPTR> {
    assert!(!pan.is_null(), "{}panel_userptr() : pan.is_null()", MODULE_PATH);

    (bindings::panel_userptr(pan) as *mut libc::c_void).as_mut().map(|ptr| ptr as PANEL_USERPTR)
}

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn del_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "{}del_panel() : pan.is_null()", MODULE_PATH);

    bindings::del_panel(pan)
} 
