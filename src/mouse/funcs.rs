/*
    src/mouse/funcs.rs

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

use std::time;
use std::convert::TryFrom;

use mouse::{NCurseswMouseError, OriginResult, constants::NCURSES_MOUSE_VERSION};
use crate::Origin;
use shims::{ncurses, nmouse, bindings, constants::{OK, ERR}};

type WINDOW = ncurses::WINDOW;
type SCREEN = ncurses::SCREEN;
/// Mouse event.
pub type MEVENT = bindings::MEVENT;
/// Mouse mask.
pub type mmask_t = nmouse::mmask_t;

/// The `has_mouse()` function returns `true` if the mouse driver has
/// been successfully initialized.
///
/// Note that mouse events will be ignored when input is in cooked mode,
/// and will cause an error beep when cooked mode is being simulated in
/// a window by a function such as `getstr()` that expects a linefeed
/// for input-loop termination.
pub fn has_mouse() -> bool {
    nmouse::has_mouse()
}

/// Once a class of mouse events has been made visible in a window, calling
/// the `wgetch()` function on that window may return `KeyBinding::KeyMouse`
/// as an indicator that a mouse event has been queued. To read the event
/// data and pop the event off the queue, call `getmouse()`. This function
/// will return a `Ok` type `Result` if a mouse event is actually visible
/// in the given window, a `NCurseswMouseError` otherwise. When `getmouse()`
/// returns `Ok`, the data deposited as `y` and `x` in the event structure
/// coordinates will be screen-relative character-cell coordinates. The
/// returned state mask will have exactly one bit set to indicate the event
/// type. The corresponding data in the queue is marked invalid. A subsequent
/// call to `getmouse()` will retrieve the next older item from the queue.
pub fn getmouse(event: nmouse::MEVENT) -> mouse_result!(()) {
    match unsafe { nmouse::getmouse(event) } {
        OK => Ok(()),
        rc => Err(mouse_function_error_with_rc!("getmouse", rc))
    }
}

/// The `ungetmouse()` function behaves analogously to `ungetch()`. It pushes
/// a `KeyBinding::KeyMouse` event onto the input queue, and associates with
/// that event the given state data and screen-relative character-cell coordinates.
pub fn ungetmouse(event: nmouse::MEVENT) -> mouse_result!(()) {
    match unsafe { nmouse::ungetmouse(event) } {
        OK => Ok(()),
        rc => Err(mouse_function_error_with_rc!("ungetmouse", rc))
    }
}

/// To make mouse events visible, use the `mousemask()` function. This will
/// set the mouse events to be reported. By default, no mouse events are
/// reported. The function will return a mask to indicate which of the
/// specified mouse events can be reported; on complete failure it returns 0.
///
/// As a side effect, setting a zero mousemask may turn off the mouse pointer;
/// setting a nonzero mask may turn it on. Whether this happens is device-dependent.
pub fn mousemask(newmask: mmask_t) -> mouse_result!(mmask_t) {
    let mut oldmask: [mmask_t; 1] = [0];

    if unsafe { nmouse::mousemask(newmask, Some(oldmask.as_mut_ptr())) } == 0 {
        Err(mouse_function_error!("mousemask"))
    } else {
        Ok(oldmask[0])
    }
}

/// The `mouseinterval()` function sets the maximum time (in thousands of a second)
/// that can elapse between press and release events for them to be recognized as a
/// click. Use `mouseinterval(Some(time::Duration::from_millis(0)))` to disable click
/// resolution. This function returns the previous interval value.
/// Use `mouseinterval(None)` to obtain the interval without altering it.
/// The default is one sixth of a second.
pub fn mouseinterval(delay: Option<time::Duration>) -> mouse_result!(time::Duration) {
    let rc = if let Some(ms) = delay {
        nmouse::mouseinterval(i32::try_from(ms.as_millis())?)
    } else {
        nmouse::mouseinterval(-1)
    };

    if rc < 0 {
        Err(mouse_function_error_with_rc!("mouseinterval", rc))
    } else {
        Ok(time::Duration::from_millis(u64::try_from(rc)?))
    }
}

/// The `wenclose()` function tests whether a given pair of screen-relative character-cell
/// coordinates is enclosed by a given window, returning `true` if it is and `false`
/// otherwise. It is useful for determining what subset of the screen windows enclose the
/// location of a mouse event.
pub fn wenclose(window: WINDOW, origin: Origin) -> bool {
    unsafe { nmouse::wenclose(window, origin.y, origin.x) }
}

/// The `wmouse_trafo()` function transforms a given pair of coordinates from
/// stdscr-relative coordinates to coordinates relative to the given window or
/// vice versa. The resulting stdscr-relative coordinates are not always identical
/// to window-relative coordinates due to the mechanism to reserve lines on top
/// or bottom of the screen for other purposes (see the `ripoffline()` and
/// `slk_init()` calls, for example).
///
/// - If the parameter `to_screen` is `true`, the `origin` must reference the
///   coordinates of a location inside the `window`. They are converted to
///   window-relative coordinates and returned. If the conversion was successful,
///   the function returns `true` in `OriginResult::result`.
/// - If the `origin` is not inside the window, `false` is returned in `Origin::result`.
/// - If `to_screen` is `false`, the `origin` must reference window-relative coordinates.
///   They are converted to stdscr-relative coordinates if the `window` encloses this
///   point. In this case the function returns `true` in `OriginResult::result`.
/// - If `origin` is not inside the window, `false` is returned in `OriginResult::result`.
///   The referenced coordinates are only returned as the converted coordinates if the
///   transformation was successful.
pub fn wmouse_trafo(window: WINDOW, origin: Origin, to_screen: bool) -> OriginResult {
    let mut y: [i32; 1] = [origin.y];
    let mut x: [i32; 1] = [origin.x];

    let result = unsafe { nmouse::wmouse_trafo(window, y.as_mut_ptr(), x.as_mut_ptr(), to_screen) };

    OriginResult::new(y[0], x[0], to_screen, result)
}

/// The `mouse_trafo()` function performs the same translation as `wmouse_trafo()`,
/// using `stdscr()` for the window.
pub fn mouse_trafo(origin: Origin, to_screen: bool) -> OriginResult {
    let mut y: [i32; 1] = [origin.y];
    let mut x: [i32; 1] = [origin.x];

    let result = unsafe { nmouse::mouse_trafo(y.as_mut_ptr(), x.as_mut_ptr(), to_screen) };

    OriginResult::new(y[0], x[0], to_screen, result)
}

/// Return the NCurses mouse version.
pub fn mouse_version() -> i32 {
    NCURSES_MOUSE_VERSION
}

/// Returns wether a mouse interface is available.
pub fn has_mouse_interface() -> bool {
    mouse_version() > 0
}

// screen `_sp` functions.

/// Screen function of `getmouse()`.
pub fn getmouse_sp(screen: SCREEN, event: nmouse::MEVENT) -> mouse_result!(()) {
    match unsafe { nmouse::getmouse_sp(screen, event) } {
        OK => Ok(()),
        rc => Err(mouse_function_error_with_rc!("getmouse_sp", rc))
    }
}

/// Screen function of `has_mouse()`.
pub fn has_mouse_sp(screen: SCREEN) -> bool {
    unsafe { bindings::has_mouse_sp(screen) }
}

/// Screen function of `set_mouseinterval()`.
pub fn mouseinterval_sp(screen: SCREEN, delay: Option<time::Duration>) -> mouse_result!(time::Duration) {
    let rc = unsafe { if let Some(ms) = delay {
        nmouse::mouseinterval_sp(screen, i32::try_from(ms.as_millis())?)
    } else {
        nmouse::mouseinterval_sp(screen, -1)
    }};

    if rc < 0 {
        Err(mouse_function_error_with_rc!("mouseinterval_sp", rc))
    } else {
        Ok(time::Duration::from_millis(u64::try_from(rc)?))
    }
}

/// Screen function of `mousemask()`.
pub fn mousemask_sp(screen: SCREEN, newmask: mmask_t) -> mouse_result!(mmask_t) {
    let mut oldmask: [mmask_t; 1] = [0];

    if unsafe { nmouse::mousemask_sp(screen, newmask, Some(oldmask.as_mut_ptr())) } == 0 {
        Err(mouse_function_error!("mousemask_sp"))
    } else {
        Ok(oldmask[0])
    }
}

/// Screen function of `ungetmouse()`.
pub fn ungetmouse_sp(screen: SCREEN, event: nmouse::MEVENT) -> mouse_result!(()) {
    match unsafe { nmouse::ungetmouse_sp(screen, event) } {
        OK => Ok(()),
        rc => Err(mouse_function_error_with_rc!("ungetmouse_sp", rc))
    }
}
