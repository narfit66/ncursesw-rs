/*
    src/panels/funcs.rs

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

use crate::{
    origin::Origin,
    shims::{ncurses, npanels, constants::{OK, ERR}},
    panels::{NCurseswPanelsError, PanelUserPtr}
};

type WINDOW = ncurses::WINDOW;

type SCREEN = *mut crate::shims::bindings::SCREEN;
pub type PANEL = npanels::PANEL;

/// Allocates a PANEL structure, associates it with win, places the panel on
/// the top of the stack (causes it to be displayed above any other panel)
/// and returns a pointer to the new panel.
pub fn new_panel(window: WINDOW) -> panels_result!(PANEL) {
    unsafe { npanels::new_panel(window).ok_or(panels_function_error!("new_panel")) }
} 

/// Puts panel at the bottom of all panels.
pub fn bottom_panel(panel: PANEL) -> panels_result!(()) {
    match unsafe { npanels::bottom_panel(panel) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("bottom_panel", rc))
    }
}

/// Puts the given visible panel on top of all panels in the stack.
pub fn top_panel(panel: PANEL) -> panels_result!(()) {
    match unsafe { npanels::top_panel(panel) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("top_panel", rc))
    }
}

/// Makes a hidden panel visible by placing it on top of the panels in the panel stack.
pub fn show_panel(panel: PANEL) -> panels_result!(()) {
    match unsafe { npanels::show_panel(panel) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("show_panel", rc))
    }
}

/// Refreshes the virtual screen to reflect the relations between the panels in the stack.
///
/// Does not call doupdate() to refresh the physical screen. Use this function and not wrefresh() or wnoutrefresh().
/// update_panels() may be called more than once before a call to doupdate(), but doupdate() is the function
/// responsible for updating the physical screen.
pub fn update_panels() {
    npanels::update_panels();
} 

/// Removes the given panel from the panel stack and thus hides it from view.
/// The PANEL structure is not lost, merely removed from the stack.
pub fn hide_panel(panel: PANEL) -> panels_result!(()) {
    match unsafe { npanels::hide_panel(panel) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("hide_panel", rc))
    }
}

/// Returns a pointer to the window of the given panel.
pub fn panel_window(panel: PANEL) -> panels_result!(WINDOW) {
    unsafe { npanels::panel_window(panel).ok_or(panels_function_error!("panel_window")) }
} 

/// Replaces the current window of panel with window (useful, for example if you
/// want to resize a panel; if you're using ncurses, you can call replace_panel()
/// on the output of wresize(3x)). It does not change the position of the panel
/// in the stack.
pub fn replace_panel(panel: PANEL, window: WINDOW) -> panels_result!(()) {
    match unsafe { npanels::replace_panel(panel, window) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("replace_panel", rc))
    }
} 

/// Moves the given panel window so that its upper-left corner is at origin.y, origin.x.
/// It does not change the position of the panel in the stack. Be sure to use this
/// function, not mvwin(), to move a panel window.
pub fn move_panel(panel: PANEL, origin: Origin) -> panels_result!(()) {
    match unsafe { npanels::move_panel(panel, origin.y, origin.x) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("move_panel", rc))
    }
} 

/// Returns true if the panel is in the panel stack, false if it is not.
pub fn panel_hidden(panel: PANEL) -> panels_result!(bool) {
    unsafe { npanels::panel_hidden(panel).ok_or(panels_function_error!("panel_hidden")) }
}

/// Returns a pointer to the panel above pan.
/// If the panel argument is None, it returns a pointer to the bottom panel in the stack.
pub fn panel_above(panel: Option<PANEL>) -> panels_result!(PANEL) {
    unsafe { npanels::panel_above(panel).ok_or(panels_function_error!("panel_above")) }
} 

/// Returns a pointer to the panel just below pan.
/// If the panel argument is None, it returns a pointer to the top panel in the stack.
pub fn panel_below(panel: Option<PANEL>) -> panels_result!(PANEL) {
    unsafe { npanels::panel_below(panel).ok_or(panels_function_error!("panel_below")) }
} 

/// Sets the panel's user pointer.
pub fn set_panel_userptr(panel: PANEL, userptr: PanelUserPtr) -> panels_result!(()) {
    match unsafe { npanels::set_panel_userptr(panel, userptr) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("set_panel_userptr", rc))
    }
}

/// Returns the user pointer for a given panel.
pub fn panel_userptr(panel: PANEL) -> PanelUserPtr {
    unsafe { npanels::panel_userptr(panel) }
}

/// Removes the given panel from the stack and deallocates the PANEL structure
/// (but not its associated window).
pub fn del_panel(panel: PANEL) -> panels_result!(()) {
    match unsafe { npanels::del_panel(panel) } {
        OK => Ok(()),
        rc => Err(panels_function_error_with_rc!("del_panel", rc))
    }
}

// screen `_sp` functions.

/// Screen function, returns a pointer to the topmost panel in the given screen.
pub fn ceiling_panel(screen: SCREEN) -> panels_result!(PANEL) {
    unsafe { npanels::ceiling_panel(screen).ok_or(panels_function_error!("ceiling_panel")) }
}

/// Screen function, returns a pointer to the lowest panel in the given screen.
pub fn ground_panel(screen: SCREEN) -> panels_result!(PANEL) {
    unsafe { npanels::ground_panel(screen).ok_or(panels_function_error!("ground_panel")) }
}

/// Screen function of `update_panels()`.
pub fn update_panels_sp(screen: SCREEN) {
    unsafe { npanels::update_panels_sp(screen) }
} 
