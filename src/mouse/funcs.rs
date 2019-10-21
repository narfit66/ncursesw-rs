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

use std::time;
use std::convert::TryFrom;

use mouse::constants::NCURSES_MOUSE_VERSION;
use mouse::originresult::OriginResult;
use ncurseswerror::NCurseswError;
use origin::Origin;
use shims::{ncurses, nmouse, bindings};
use shims::constants::ERR;

type WINDOW = ncurses::WINDOW;
pub type MEVENT = bindings::MEVENT;
pub type mmask_t = nmouse::mmask_t;

pub fn has_mouse() -> bool {
    nmouse::has_mouse()
}

pub fn getmouse(event: nmouse::MEVENT) -> result!(()) {
    match unsafe { nmouse::getmouse(event) } {
        ERR => Err(mouse_function_error!("getmouse")),
        _   => Ok(())
    }
}

pub fn ungetmouse(event: nmouse::MEVENT) -> result!(()) {
    match unsafe { nmouse::ungetmouse(event) } {
        ERR => Err(mouse_function_error!("ungetmouse")),
        _   => Ok(())
    }
}

pub fn mousemask(newmask: mmask_t, oldmask: Option<*mut mmask_t>) -> result!(mmask_t) {
    match unsafe { nmouse::mousemask(newmask, oldmask) } {
        0    => Err(mouse_function_error!("mousemask")),
        mask => Ok(mask)
    }
}

pub fn mouseinterval() -> result!(time::Duration) {
    match nmouse::mouseinterval(-1) {
        ERR => Err(mouse_function_error!("mouseinterval")),
        ms  => Ok(time::Duration::from_millis(u64::try_from(ms)?))
    }
}

pub fn set_mouseinterval(delay: time::Duration) -> result!(()) {
    let ms = i32::try_from(delay.as_millis())?;

    match nmouse::mouseinterval(ms) {
        ERR => Err(mouse_function_error!("mouseinterval")),
        _   => Ok(())
    }
}

pub fn wenclose(win: WINDOW, origin: Origin) -> bool {
    unsafe { nmouse::wenclose(win, origin.y, origin.x) }
}

pub fn wmouse_trafo(win: WINDOW, origin: Origin, to_screen: bool) -> result!(OriginResult) {
    let mut y: [i32; 1] = [origin.y];
    let mut x: [i32; 1] = [origin.x];

    let result = unsafe { nmouse::wmouse_trafo(win, y.as_mut_ptr(), x.as_mut_ptr(), to_screen) };

    Ok(OriginResult::new(y[0], x[0], to_screen, result))
}

pub fn mouse_trafo(origin: Origin, to_screen: bool) -> result!(OriginResult) {
    let mut y: [i32; 1] = [origin.y];
    let mut x: [i32; 1] = [origin.x];

    let result = unsafe { nmouse::mouse_trafo(y.as_mut_ptr(), x.as_mut_ptr(), to_screen) };

    Ok(OriginResult::new(y[0], x[0], to_screen, result))
}

pub fn mouse_version() -> i32 {
    NCURSES_MOUSE_VERSION
}
