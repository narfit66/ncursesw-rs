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
#![warn(missing_debug_implementations)]

use bindings;
use shims;

use constants::{TRUE, FALSE};

pub type PANEL = *mut bindings::PANEL;
pub type PANEL_USERPTR = *const libc::c_void;

type WINDOW = shims::ncurses::WINDOW;

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn new_panel(win: WINDOW) -> Option<PANEL> {
    assert!(!win.is_null(), "npanels::new_panel() : win.is_null()");

    let pan = bindings::new_panel(win);

    return_optional_mut_ptr!(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn bottom_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "npanels::bottom_panel() : pan.is_null()");

    bindings::bottom_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn top_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "npanels::top_panel() : pan.is_null()");

    bindings::top_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn show_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "npanels::show_panel() : pan.is_null()");

    bindings::show_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub fn update_panels() {
    unsafe {
        bindings::update_panels();
    }
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn hide_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "npanels::hide_panel() : pan.is_null()");

    bindings::hide_panel(pan)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_window(pan: PANEL) -> Option<WINDOW> {
    assert!(!pan.is_null(), "npanels::panel_window() : pan.is_null()");

    let win = bindings::panel_window(pan);

    return_optional_mut_ptr!(win)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn replace_panel(pan: PANEL, win: WINDOW) -> i32 {
    assert!(!pan.is_null(), "npanels::replace_panel() : pan.is_null()");
    assert!(!win.is_null(), "npanels::replace_panel() : win.is_null()");

    bindings::replace_panel(pan, win)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn move_panel(pan: PANEL, starty: i32, startx: i32) -> i32 {
    assert!(!pan.is_null(), "npanels::move_panel() : pan.is_null()");

    bindings::move_panel(pan, starty, startx)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_hidden(pan: PANEL) -> Option<bool> {
    assert!(!pan.is_null(), "npanels::panel_hidden() : pan.is_null()");

    match bindings::panel_hidden(pan) {
        TRUE  => Some(true),
        FALSE => Some(false),
        _     => None
    }
}

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_above(pan: Option<PANEL>) -> Option<PANEL> {
    let ptr = bindings::panel_above(return_mut_ptr!(pan));

    return_optional_mut_ptr!(ptr)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_below(pan: Option<PANEL>) -> Option<PANEL> {
    let ptr = bindings::panel_below(return_mut_ptr!(pan));

    return_optional_mut_ptr!(ptr)
} 

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn set_panel_userptr(pan: PANEL, ptr: Option<PANEL_USERPTR>) -> i32 {
    assert!(!pan.is_null(), "npanels::set_panel_userptr() : pan.is_null()");

    bindings::set_panel_userptr(pan, return_mut_ptr!(ptr))
}

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn panel_userptr(pan: PANEL) -> Option<PANEL_USERPTR> {
    assert!(!pan.is_null(), "npanels::panel_userptr() : pan.is_null()");

    let ptr = bindings::panel_userptr(pan);

    return_optional_mut_ptr!(ptr)
}

/// <https://invisible-island.net/ncurses//man/panel.3x.html>
pub unsafe fn del_panel(pan: PANEL) -> i32 {
    assert!(!pan.is_null(), "npanels::del_panel() : pan.is_null()");

    bindings::del_panel(pan)
} 
