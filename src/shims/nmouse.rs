/*
    src/shims/nmouse.rs

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

use bindings;
use shims;

type WINDOW = shims::ncurses::WINDOW;

pub type mmask_t = bindings::mmask_t;
pub type MEVENT = *mut bindings::MEVENT;

static MODULE_PATH: &str = "ncursesw::shims::nmouse::";

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn getmouse(event: MEVENT) -> i32 {
    assert!(!event.is_null(), "{}getmouse() : event.is_null()", MODULE_PATH);

    bindings::getmouse(event)
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn has_mouse() -> bool {
    unsafe {
        bindings::has_mouse()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn mouse_trafo(py: *mut i32, px: *mut i32, to_screen: bool) -> bool {
    assert!(!py.is_null(), "{}mouse_trafo : py.is_null()", MODULE_PATH);
    assert!(!px.is_null(), "{}mouse_trafo : px.is_null()", MODULE_PATH);

    bindings::mouse_trafo(py, px, to_screen)
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn mouseinterval(erval: i32) -> i32 {
    unsafe {
        bindings::mouseinterval(erval)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn mousemask(newmask: mmask_t, oldmask: Option<*mut mmask_t>) -> mmask_t {
    bindings::mousemask(newmask, return_mut_ptr!(oldmask))
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn ungetmouse(event: MEVENT) -> i32 {
    assert!(!event.is_null(), "{}ungetmouse() : event.is_null()", MODULE_PATH);

    bindings::ungetmouse(event)
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn wenclose(win: WINDOW, y: i32, x: i32) -> bool {
    assert!(!win.is_null(), "{}wenclose() : win.is_null()", MODULE_PATH);

    bindings::wenclose(win, y, x)
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn wmouse_trafo(win: WINDOW, py: *mut i32, px: *mut i32, to_screen: bool) -> bool {
    assert!(!win.is_null(), "{}wmouse_trafo() : win.is_null()", MODULE_PATH);
    assert!(!py.is_null(), "{}wmouse_trafo : py.is_null()", MODULE_PATH);
    assert!(!px.is_null(), "{}wmouse_trafo : px.is_null()", MODULE_PATH);

    bindings::wmouse_trafo(win, py, px, to_screen)
}
