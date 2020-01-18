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
#![allow(clippy::missing_safety_doc)]

use bindings;
use shims;

type WINDOW = shims::ncurses::WINDOW;
type SCREEN = shims::ncurses::SCREEN;

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
    unsafe { bindings::has_mouse() }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn mouse_trafo(py: *mut i32, px: *mut i32, to_screen: bool) -> bool {
    assert!(!py.is_null(), "{}mouse_trafo : py.is_null()", MODULE_PATH);
    assert!(!px.is_null(), "{}mouse_trafo : px.is_null()", MODULE_PATH);

    bindings::mouse_trafo(py, px, to_screen)
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn mouseinterval(erval: i32) -> i32 {
    unsafe { bindings::mouseinterval(erval) }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn mousemask(newmask: mmask_t, oldmask: Option<*mut mmask_t>) -> mmask_t {
    if let Some(mask) = oldmask {
        assert!(!mask.is_null(), "{}mousemask() : oldmask.is_null()", MODULE_PATH);
    }

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
    assert!(y >= 0, "{}wenclose() : y = {}", MODULE_PATH, y);
    assert!(x >= 0, "{}wenclose() : x = {}", MODULE_PATH, x);

    bindings::wenclose(win, y, x)
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub unsafe fn wmouse_trafo(win: WINDOW, py: *mut i32, px: *mut i32, to_screen: bool) -> bool {
    assert!(!win.is_null(), "{}wmouse_trafo() : win.is_null()", MODULE_PATH);
    assert!(!py.is_null(), "{}wmouse_trafo : py.is_null()", MODULE_PATH);
    assert!(!px.is_null(), "{}wmouse_trafo : px.is_null()", MODULE_PATH);

    bindings::wmouse_trafo(win, py, px, to_screen)
}

/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn getmouse_sp(sp: SCREEN, event: MEVENT) -> i32 {
    assert!(!sp.is_null(), "{}getmouse_sp() : sp.is_null()", MODULE_PATH);
    assert!(!event.is_null(), "{}getmouse_sp() : event.is_null()", MODULE_PATH);

    bindings::getmouse_sp(sp, event)
}

/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn has_mouse_sp(sp: SCREEN) -> bool {
    assert!(!sp.is_null(), "{}has_mouse_sp() : sp.is_null()", MODULE_PATH);

    bindings::has_mouse_sp(sp)
}

/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn mouseinterval_sp(sp: SCREEN, erval: i32) -> i32 {
    assert!(!sp.is_null(), "{}mouseinterval_sp() : sp.is_null()", MODULE_PATH);

    bindings::mouseinterval_sp(sp, erval)
}

/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn mousemask_sp(sp: SCREEN, newmask: mmask_t, oldmask: Option<*mut mmask_t>) -> mmask_t {
    assert!(!sp.is_null(), "{}mousemask_sp() : sp.is_null()", MODULE_PATH);
    if let Some(mask) = oldmask {
        assert!(!mask.is_null(), "{}mousemask_sp() : oldmask.is_null()", MODULE_PATH);
    }

    bindings::mousemask_sp(sp, newmask, return_mut_ptr!(oldmask))
}

/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn ungetmouse_sp(sp: SCREEN, event: MEVENT) -> i32 {
    assert!(!sp.is_null(), "{}ungetmouse_sp() : sp.is_null()", MODULE_PATH);
    assert!(!event.is_null(), "{}ungetmouse_sp() : event.is_null()", MODULE_PATH);

    bindings::ungetmouse_sp(sp, event)
}
