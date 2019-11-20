/*
    src/mouse/funcs.rs

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

use std::time;
use std::convert::TryFrom;

use mouse::{NCurseswMouseError, OriginResult, constants::NCURSES_MOUSE_VERSION};
use origin::Origin;
use shims::{ncurses, nmouse, bindings};
use shims::constants::{OK, ERR};

type WINDOW = ncurses::WINDOW;
pub type MEVENT = bindings::MEVENT;
pub type mmask_t = nmouse::mmask_t;

pub fn has_mouse() -> bool {
    nmouse::has_mouse()
}

pub fn getmouse(event: nmouse::MEVENT) -> mouse_result!(()) {
    match unsafe { nmouse::getmouse(event) } {
        OK => Ok(()),
        rc => Err(mouse_function_error_with_rc!("getmouse", rc))
    }
}

pub fn ungetmouse(event: nmouse::MEVENT) -> mouse_result!(()) {
    match unsafe { nmouse::ungetmouse(event) } {
        OK => Ok(()),
        rc => Err(mouse_function_error_with_rc!("ungetmouse", rc))
    }
}

pub fn mousemask(newmask: mmask_t, oldmask: Option<*mut mmask_t>) -> mouse_result!(mmask_t) {
    let mask = unsafe { nmouse::mousemask(newmask, oldmask) };

    if mask == 0 {
        Err(mouse_function_error!("mousemask"))
    } else {
        Ok(mask)
    }
}

pub fn mouseinterval() -> mouse_result!(time::Duration) {
    let rc = nmouse::mouseinterval(-1);

    if rc < 0 {
        Err(mouse_function_error_with_rc!("mouseinterval", rc))
    } else {
        Ok(time::Duration::from_millis(u64::try_from(rc)?))
    }
}

pub fn set_mouseinterval(delay: time::Duration) -> mouse_result!(()) {
    let ms = i32::try_from(delay.as_millis())?;

    match nmouse::mouseinterval(ms) {
        OK => Ok(()),
        rc => Err(mouse_function_error_with_rc!("mouseinterval", rc))
    }
}

pub fn wenclose(win: WINDOW, origin: Origin) -> bool {
    unsafe { nmouse::wenclose(win, origin.y, origin.x) }
}

pub fn wmouse_trafo(win: WINDOW, origin: Origin, to_screen: bool) -> OriginResult {
    let mut y: [i32; 1] = [origin.y];
    let mut x: [i32; 1] = [origin.x];

    let result = unsafe { nmouse::wmouse_trafo(win, y.as_mut_ptr(), x.as_mut_ptr(), to_screen) };

    OriginResult::new(y[0], x[0], to_screen, result)
}

pub fn mouse_trafo(origin: Origin, to_screen: bool) -> OriginResult {
    let mut y: [i32; 1] = [origin.y];
    let mut x: [i32; 1] = [origin.x];

    let result = unsafe { nmouse::mouse_trafo(y.as_mut_ptr(), x.as_mut_ptr(), to_screen) };

    OriginResult::new(y[0], x[0], to_screen, result)
}

pub fn mouse_version() -> i32 {
    NCURSES_MOUSE_VERSION
}

pub fn has_mouse_interface() -> bool {
    mouse_version() > 0
}
