/*
    src/panels/funcs.rs

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

use origin::Origin;
use shims::{ncurses, npanels, constants::{OK, ERR}};
use panels::{NCurseswPanelsError, PanelUserPtr};

type WINDOW = ncurses::WINDOW;

pub type PANEL = npanels::PANEL;

/// allocates a PANEL structure, associates it with win, places the panel on
/// the top of the stack (causes it to be displayed above any other panel)
/// and returns a pointer to the new panel.
pub fn new_panel(window_handle: WINDOW) -> panels_result!(PANEL) {
    match unsafe { npanels::new_panel(window_handle) } {
        None         => Err(panels_function_error!("new_panel")),
        Some(handle) => Ok(handle)
    }
} 

/// puts panel at the bottom of all panels.
pub fn bottom_panel(handle: PANEL) -> panels_result!(()) {
    match unsafe { npanels::bottom_panel(handle) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("bottom_panel", rc))
    }
}

/// puts the given visible panel on top of all panels in the stack.
pub fn top_panel(handle: PANEL) -> panels_result!(()) {
    match unsafe { npanels::top_panel(handle) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("top_panel", rc))
    }
}

/// makes a hidden panel visible by placing it on top of the panels in the panel stack.
pub fn show_panel(handle: PANEL) -> panels_result!(()) {
    match unsafe { npanels::show_panel(handle) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("show_panel", rc))
    }
}

/// refreshes the virtual screen to reflect the relations between the panels in the stack.
///
/// Does not call doupdate() to refresh the physical screen. Use this function and not wrefresh() or wnoutrefresh().
/// update_panels() may be called more than once before a call to doupdate(), but doupdate() is the function
/// responsible for updating the physical screen.
pub fn update_panels() {
    npanels::update_panels();
} 

/// removes the given panel from the panel stack and thus hides it from view.
/// The PANEL structure is not lost, merely removed from the stack.
pub fn hide_panel(handle: PANEL) -> panels_result!(()) {
    match unsafe { npanels::hide_panel(handle) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("hide_panel", rc))
    }
}

/// returns a pointer to the window of the given panel.
pub fn panel_window(handle: PANEL) -> panels_result!(WINDOW) {
    match unsafe { npanels::panel_window(handle) } {
        None                => Err(panels_function_error!("panel_window")),
        Some(window_handle) => Ok(window_handle)
    }
} 

/// replaces the current window of panel with window (useful, for example if you
/// want to resize a panel; if you're using ncurses, you can call replace_panel()
/// on the output of wresize(3x)). It does not change the position of the panel
/// in the stack.
pub fn replace_panel(handle: PANEL, window_handle: WINDOW) -> panels_result!(()) {
    match unsafe { npanels::replace_panel(handle, window_handle) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("replace_panel", rc))
    }
} 

/// moves the given panel window so that its upper-left corner is at origin.y, origin.x.
/// It does not change the position of the panel in the stack. Be sure to use this
/// function, not mvwin(), to move a panel window.
pub fn move_panel(handle: PANEL, origin: Origin) -> panels_result!(()) {
    match unsafe { npanels::move_panel(handle, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("move_panel", rc))
    }
} 

/// returns true if the panel is in the panel stack, false if it is not.
pub fn panel_hidden(handle: PANEL) -> panels_result!(bool) {
    match unsafe { npanels::panel_hidden(handle) } {
        None    => Err(panels_function_error!("panel_hidden")),
        Some(v) => Ok(v)
    }
}

/// returns a pointer to the panel above pan.
/// If the panel argument is None, it returns a pointer to the bottom panel in the stack.
pub fn panel_above(handle: Option<PANEL>) -> panels_result!(PANEL) {
    match unsafe { npanels::panel_above(handle) } {
        None    => Err(panels_function_error!("panel_above")),
        Some(p) => Ok(p)
    }
} 

/// returns a pointer to the panel just below pan.
/// If the panel argument is None, it returns a pointer to the top panel in the stack.
pub fn panel_below(handle: Option<PANEL>) -> panels_result!(PANEL) {
    match unsafe { npanels::panel_below(handle) } {
        None    => Err(panels_function_error!("panel_below")),
        Some(p) => Ok(p)
    }
} 

/// sets the panel's user pointer.
pub fn set_panel_userptr(handle: PANEL, ptr: PanelUserPtr) -> panels_result!(()) {
    match unsafe { npanels::set_panel_userptr(handle, ptr) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("set_panel_userptr", rc))
    }
}

/// returns the user pointer for a given panel.
pub fn panel_userptr(handle: PANEL) -> PanelUserPtr {
    unsafe {
        npanels::panel_userptr(handle)
    }
}

/// removes the given panel from the stack and deallocates the PANEL structure
/// (but not its associated window).
pub fn del_panel(handle: PANEL) -> panels_result!(()) {
    match unsafe { npanels::del_panel(handle) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("del_panel", rc))
    }
}