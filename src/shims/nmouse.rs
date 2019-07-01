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

use bindings;
use ncurses;

type WINDOW = ncurses::WINDOW;

pub type mmask_t = bindings::mmask_t;
pub type MEVENT = *mut bindings::MEVENT;

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn getmouse(event: MEVENT) -> i32 {
    assert!(!event.is_null(), "nmouse::getmouse() : event.is_null()");

    unsafe {
        bindings::getmouse(event)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn has_mouse() -> bool {
    unsafe {
        bindings::has_mouse()
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn mouse_trafo(py: &mut[i32], px: &mut[i32], to_screen: bool) -> bool {
    unsafe {
        bindings::mouse_trafo(py.as_mut_ptr(), px.as_mut_ptr(), to_screen)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn mouseinterval(erval: i32) -> i32 {
    unsafe {
        bindings::mouseinterval(erval)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn mousemask(newmask: mmask_t, oldmask: Option<&mut mmask_t>) -> mmask_t {
    unsafe {
        bindings::mousemask(newmask, match oldmask {
            None       => std::ptr::null_mut(),
            Some(mask) => mask
        })
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn ungetmouse(event: MEVENT) -> i32 {
    assert!(!event.is_null(), "nmouse::ungetmouse() : event.is_null()");

    unsafe {
        bindings::ungetmouse(event)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn wenclose(win: WINDOW, y: i32, x: i32) -> bool {
    assert!(!win.is_null(), "nmouse::wenclose() : win.is_null()");

    unsafe {
        bindings::wenclose(win, y as libc::c_int, x as libc::c_int)
    }
}

/// <https://invisible-island.net/ncurses/man/curs_mouse.3x.html>
pub fn wmouse_trafo(win: WINDOW, py: &mut[i32], px: &mut[i32], to_screen: bool) -> bool {
    assert!(!win.is_null(), "nmouse::wmouse_trafo() : win.is_null()");

    unsafe {
        bindings::wmouse_trafo(win, py.as_mut_ptr(), px.as_mut_ptr(), to_screen)
    }
}
