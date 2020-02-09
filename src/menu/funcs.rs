/*
    src/menu/func.rs

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

use std::{ptr, ffi::CString, convert::TryFrom};

use errno::errno;

use crate::{
    normal,
    cstring::*,
    shims::{
        nmenu, bindings, ncurses::{SCREEN, WINDOW},
        constants::{E_OK, E_NO_MATCH, E_UNKNOWN_COMMAND}
    },
    menu::{
        ItemOptions, MenuOptions, MenuSpacing, MenuRequest,
        MenuSize, MenuUserPtr,
        ncurseswmenuerror::{
            NCurseswMenuError, ncursesw_menu_error_system_error,
            ncursesw_menu_error_from_rc
        }
    }
};

/// Menu.
pub type MENU = nmenu::MENU;
/// Menu item.
pub type ITEM = nmenu::ITEM;
/// Menu callback function.
pub type Menu_Hook = crate::shims::bindings::Menu_Hook;

static MODULE_PATH: &str = "ncursesw::menu::funcs::";

/// Returns a pointer to the current item in the given menu.
pub fn current_item(menu: MENU) -> menu_result!(ITEM) {
    unsafe { nmenu::current_item(menu).ok_or_else(|| menu_function_error!("current_item")) }
}

/// De-allocates a menu item.
pub fn free_item(item: ITEM) -> menu_result!(()) {
    assert!(!item.is_null(), "{}free_item() : item.is_null()", MODULE_PATH);

    unsafe {
        // if an item name has been defined (and it should be!) then unallocate it.
        let name = bindings::item_name(item) as *mut i8;

        if !name.is_null() {
            let _ = CString::from_raw(name);
        }

        // if an item description has been defined (and it should be!) then unallocate it.
        let description = bindings::item_description(item) as *mut i8;

        if !description.is_null() {
            let _ = CString::from_raw(description);
        }
    }

    match unsafe { nmenu::free_item(item) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("free_item", rc))
    }
}

/// Disconnects menu from its item array and frees the storage allocated for the menu.
///
/// Make sure that `free_menu()` is called before `free_item()` otherwise the menu
/// item will still be connected to the menu.
pub fn free_menu(menu: MENU) -> menu_result!(()) {
    match unsafe { nmenu::free_menu(menu) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("free_menu", rc))
    }
}

/// Returns the count of items in menu.
pub fn item_count(menu: MENU) -> menu_result!(i32) {
    let rc = unsafe { nmenu::item_count(menu) };

    if rc.is_negative() {
        Err(menu_function_unknown_error!("item_count", rc))
    } else {
        Ok(rc)
    }
}

/// Returns the description part of the given menu item.
pub fn item_description(item: ITEM) -> menu_result!(String) {
    unsafe { nmenu::item_description(item).ok_or_else(|| menu_function_error!("item_description")) }
}

/// Returns the (zero-origin) index of item in the menu's item pointer list.
pub fn item_index(item: ITEM) -> menu_result!(i32) {
    let rc = unsafe { nmenu::item_index(item) };

    if rc.is_negative() {
        Err(menu_function_unknown_error!("item_index", rc))
    } else {
        Ok(rc)
    }
}

/// Returns the current menu item init hook.
pub fn item_init(menu: MENU) -> menu_result!(Menu_Hook) {
    unsafe { nmenu::item_init(menu).ok_or_else(|| menu_function_error_with_rc!("item_init", errno().into())) }
}

/// Returns the name part of the given menu item.
pub fn item_name(item: ITEM) -> menu_result!(String) {
    unsafe { nmenu::item_name(item).ok_or_else(|| menu_function_error!("item_name")) }
}

/// Returns the item's current options.
pub fn item_opts(item: ITEM) -> ItemOptions {
    unsafe { ItemOptions::from(nmenu::item_opts(item)) }
}

/// Turns off the given options, and leaves others alone.
pub fn item_opts_off(item: ITEM, opts: ItemOptions) -> menu_result!(()) {
    match unsafe { nmenu::item_opts_off(item, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("item_opts_off", rc))
    }
}

/// Turns on the given options, and leaves others alone.
pub fn item_opts_on(item: ITEM, opts: ItemOptions) -> menu_result!(()) {
    match unsafe { nmenu::item_opts_on(item, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("item_opts_on", rc))
    }
}

/// Returns the current menu item term hook.
pub fn item_term(menu: MENU) -> menu_result!(Menu_Hook) {
    unsafe { nmenu::item_term(menu).ok_or_else(|| menu_function_error_with_rc!("item_term", errno().into())) }
}

/// Returns the menu item user pointer.
pub fn item_userptr(item: ITEM) -> MenuUserPtr {
    unsafe { nmenu::item_userptr(item) }
}

/// Returns the menu items value.
pub fn item_value(item: ITEM) -> bool {
    unsafe { nmenu::item_value(item) }
}

/// Returns if the menu item is visible.
pub fn item_visible(item: ITEM) -> bool {
    unsafe { nmenu::item_visible(item) }
}

/// Returns the background attribute. The default is `normal::Attributes::Normal`.
pub fn menu_back(menu: MENU) -> normal::Attributes {
    unsafe { normal::Attributes::from(nmenu::menu_back(menu)) }
}

/// command-processing loop of the menu system.
///
/// Once a menu has been posted (displayed), you should funnel input events to it
/// through `menu_driver()`. This routine has three major input cases:
///
/// - The input is a form navigation request. Navigation request codes are `MenuRequest`,
///   which are distinct from the key and character codes returned by `wgetch()`.
/// - The input is a printable character. Printable characters (which must be positive,
///   less than 256) are checked according to the program's locale settings.
/// - The input is the `KeyBinding::KeyMouse` special key associated with an mouse event.
pub fn menu_driver(menu: MENU, request: MenuRequest) -> menu_result!(Option<MenuRequest>) {
    match unsafe { nmenu::menu_driver(menu, request.value()?) } {
        E_OK => Ok(None),
        rc   => if request == MenuRequest::Mouse {
            if rc == E_UNKNOWN_COMMAND {
                Ok(None)
            } else {
                let menu_request = MenuRequest::new(rc);

                if menu_request.is_some() {
                    Ok(menu_request)
                } else {
                    Err(menu_function_error_with_rc!("menu_driver", rc))
                }
            }
        } else {
            Err(menu_function_error_with_rc!("menu_driver", rc))
        }
    }
}

/// Returns the foreground attribute. The default is `normal::Attributes::Reverse`.
pub fn menu_fore(menu: MENU) -> normal::Attributes {
    unsafe { normal::Attributes::from(nmenu::menu_fore(menu)) }
}

/// Returns the maximum-size constraints for the given menu into the storage
/// addressed by rows and cols.
pub fn menu_format(menu: MENU) -> MenuSize {
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];

    unsafe { nmenu::menu_format(menu, rows.as_mut_ptr(), cols.as_mut_ptr()) };

    MenuSize { rows: rows[0], columns: cols[0] }
}

/// Returns the grey attribute. The default is `normal::Attributes::Underline`.
pub fn menu_grey(menu: MENU) -> normal::Attributes {
    unsafe { normal::Attributes::from(nmenu::menu_grey(menu)) }
}

/// Returns the current menu init hook.
pub fn menu_init(menu: MENU) -> menu_result!(Menu_Hook) {
    unsafe { nmenu::menu_init(menu).ok_or_else(|| menu_function_error_with_rc!("menu_init", errno().into())) }
}

/// Returns the menu items as a vector of the given menu.
pub fn menu_items(menu: MENU) -> menu_result!(Vec<ITEM>) {
    unsafe { nmenu::menu_items(menu).ok_or_else(|| menu_function_error!("menu_items")) }
}

/// Returns the menu's mark string.
pub fn menu_mark(menu: MENU) -> menu_result!(String) {
    unsafe { nmenu::menu_mark(menu).ok_or_else(|| menu_function_error!("menu_mark")) }
}

/// Returns the menu's current options.
pub fn menu_opts(menu: MENU) -> MenuOptions {
    unsafe { MenuOptions::from(nmenu::menu_opts(menu)) }
}

/// Turns off the given options, and leaves others alone.
pub fn menu_opts_off(menu: MENU, opts: MenuOptions) -> menu_result!(()) {
    match unsafe { nmenu::menu_opts_off(menu, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("menu_opts_off", rc))
    }
}

/// Turns on the given options, and leaves others alone.
pub fn menu_opts_on(menu: MENU, opts: MenuOptions) -> menu_result!(()) {
    match unsafe { nmenu::menu_opts_on(menu, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("menu_opts_on", rc))
    }
}

/// Returns the given menu's pad character. The default is a blank.
pub fn menu_pad(menu: MENU) -> menu_result!(char) {
    Ok(char::from(u8::try_from(u32::try_from(unsafe { nmenu::menu_pad(menu) })?)?))
}

/// Returns the pattern buffer of the given menu.
pub fn menu_pattern(menu: MENU) -> menu_result!(String) {
    unsafe { nmenu::menu_pattern(menu).ok_or_else(|| menu_function_error!("menu_pattern")) }
}

/// Searches in the name-table for a request with the given name and returns
/// its `MenuRequest`. Otherwise `None` is returned.
pub fn menu_request_by_name(name: &str) -> menu_result!(Option<MenuRequest>) {
    match unsafe { nmenu::menu_request_by_name(c_str_with_nul!(name)) } {
        E_NO_MATCH => Ok(None),
        rc         => {
            let menu_request = MenuRequest::new(rc);

            if menu_request.is_some() {
                Ok(menu_request)
            } else {
                Err(menu_function_error_with_rc!("menu_request_by_name", rc))
            }
        }
    }
}

/// Returns the printable name of a menu request code.
pub fn menu_request_name(request: MenuRequest) -> menu_result!(String) {
    nmenu::menu_request_name(request.value()?).ok_or_else(|| menu_function_error_with_rc!("menu_request_name", errno().into()))
}

/// Returns the spacing info for the menu.
pub fn menu_spacing(menu: MENU) -> menu_result!(MenuSpacing) {
    let mut description: [i32; 1] = [0];
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];

    match unsafe { nmenu::menu_spacing(menu, description.as_mut_ptr(), rows.as_mut_ptr(), cols.as_mut_ptr()) } {
        E_OK => Ok(MenuSpacing { description: description[0], menu_size: MenuSize { rows: rows[0], columns: cols[0] }}),
        rc   => Err(menu_function_error_with_rc!("menu_spacing", rc))
    }
}

/// Returns the menus sub-window.
pub fn menu_sub(menu: MENU) -> menu_result!(WINDOW) {
    unsafe { nmenu::menu_sub(menu).ok_or_else(|| menu_function_error!("menu_sub")) }
}

/// Returns the current menu term hook.
pub fn menu_term(menu: MENU) -> menu_result!(Menu_Hook) {
    unsafe { nmenu::menu_term(menu).ok_or_else(|| menu_function_error_with_rc!("menu_term", errno().into())) }
}

/// Returns the menu user pointer.
pub fn menu_userptr(menu: MENU) -> MenuUserPtr {
    unsafe { nmenu::menu_userptr(menu) }
}

/// Returns the menus main-window.
pub fn menu_win(menu: MENU) -> menu_result!(WINDOW) {
    unsafe { nmenu::menu_win(menu).ok_or_else(|| menu_function_error!("menu_win")) }
}

/// Allocates a new item and initializes it from the name and description
pub fn new_item<T>(name: T, description: T) -> menu_result!(ITEM)
    where T: Into<Vec<u8>>
{
    let name = CString::new(name)?.into_raw();
    let description = CString::new(description)?.into_raw();

    unsafe { nmenu::new_item(name, description).ok_or_else(|| menu_function_error_with_rc!("new_item", errno().into())) }
}

/// Creates a new menu connected to a specified vector of menu item.
///
/// When `new_menu()` is called make sure that the memory for the item_handles
/// is contiguous and does not go out of scope until after `free_menu()` has
/// been called otherwise unpredicable results may occur, this is because the
/// underlying ncurses menu functions use this memory directly.
/// See ncursesw-win-rs's Menu::new() <https://github.com/narfit66/ncursesw-win-rs/blob/master/src/menu/menu.rs>
/// as an example of how the `nmenu::new_menu()` function can be called by
/// allocating and keeping the memory required but bypasses this function
/// and calling `nmenu::new_menu()` directly (although you could also call
/// this function directly as long as the underlying memory is contiguous
/// and does not go out of scope).
pub fn new_menu(item_handles: &mut Vec<ITEM>) -> menu_result!(MENU) {
    item_handles.push(ptr::null_mut());
    item_handles.shrink_to_fit();

    let menu = unsafe { nmenu::new_menu(item_handles.as_mut_ptr() as *mut ITEM) };

    item_handles.pop();

    menu.ok_or_else(|| menu_function_error_with_rc!("new_menu", errno().into()))
}

/// Restores the cursor to the current position associated with the menu's
/// selected item. This is useful after NCurses routines have been called
/// to do screen-painting in response to a menu select.
pub fn pos_menu_cursor(menu: MENU) -> menu_result!(()) {
    match unsafe { nmenu::pos_menu_cursor(menu) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("pos_menu_cursor", rc))
    }
}

/// Displays a menu to its associated subwindow. To trigger physical display
/// of the sub-window, use `refresh()` or some equivalent curses routine
/// (the implicit `doupdate()` triggered by an NCurses input request will do).
/// `post_menu()` resets the selection status of all items.
pub fn post_menu(menu: MENU) -> menu_result!(()) {
    match unsafe { nmenu::post_menu(menu) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("post_menu", rc))
    }
}

/// Returns the minimum size required for the sub-window of menu.
pub fn scale_menu(menu: MENU) -> menu_result!(MenuSize) {
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];

    match unsafe { nmenu::scale_menu(menu, rows.as_mut_ptr(), cols.as_mut_ptr()) } {
        E_OK => Ok(MenuSize { rows: rows[0], columns: cols[0] }),
        rc   => Err(menu_function_error_with_rc!("scale_menu", rc))
    }
}

/// Sets the current item (the item on which the menu cursor is positioned).
pub fn set_current_item(menu: MENU, item: ITEM) -> menu_result!(()) {
    match unsafe { nmenu::set_current_item(menu, item) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_current_item", rc))
    }
}

/// Sets a hook to be called at menu-post time and each time the selected
/// item changes (after the change).
pub fn set_item_init(menu: MENU, hook: Menu_Hook) -> menu_result!(()) {
    match unsafe { nmenu::set_item_init(menu, hook) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_item_init", rc))
    }
}

/// Sets all the given item's options.
pub fn set_item_opts(item: ITEM, opts: ItemOptions) -> menu_result!(()) {
    match unsafe { nmenu::set_item_opts(item, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_item_opts", rc))
    }
}

/// Sets a hook to be called at menu-unpost time and each time the selected
/// item changes (before the change).
pub fn set_item_term(menu: MENU, hook: Menu_Hook) -> menu_result!(()) {
    match unsafe { nmenu::set_item_term(menu, hook) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_item_term", rc))
    }
}

/// Sets the menu item user pointer.
pub fn set_item_userptr(item: ITEM, userptr: MenuUserPtr) {
    unsafe { nmenu::set_item_userptr(item, userptr) };
}

/// Sets the menu items value.
pub fn set_item_value(item: ITEM, value: bool) -> menu_result!(()) {
    match unsafe { nmenu::set_item_value(item, value) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_item_value", rc))
    }
}

/// Sets the background attribute of menu. This is the highlight used for
/// selectable (but not currently selected) menu items.
pub fn set_menu_back(menu: MENU, attr: normal::Attributes) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_back(menu, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_back", rc))
    }
}

/// Sets the foreground attribute of menu. This is the highlight used for selected menu items.
pub fn set_menu_fore(menu: MENU, attr: normal::Attributes) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_fore(menu, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_fore", rc))
    }
}

/// Sets the maximum display size of the given menu. If this size is too small to
/// display all menu items, the menu will be made scrollable. If this size is
/// larger than the menus sub-window and the sub-window is too small to display
/// all menu items, `post_menu()` will fail.
///
/// The default format is 16 rows, 1 column. Calling `set_menu_format()` with a
/// menu of `None` will change this default. A zero row or column argument to
/// `set_menu_format()` is interpreted as a request not to change the current value.
pub fn set_menu_format(menu: Option<MENU>, menu_size: MenuSize) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_format(menu, menu_size.rows, menu_size.columns) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_format", rc))
    }
}

/// Sets the grey attribute of menu. This is the highlight used for un-selectable
/// menu items in menus that permit more than one selection.
pub fn set_menu_grey(menu: MENU, attr: normal::Attributes) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_grey(menu, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_grey", rc))
    }
}

/// Sets a hook to be called at menu-post time and just after the top row on the
/// menu changes once it is posted.
pub fn set_menu_init(menu: MENU, hook: Menu_Hook) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_init(menu, hook) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_init", rc))
    }
}

/// Changes the menu items using a vector of menu items for the given menu.
///
/// Please see `new_menu()` for more details on how to implement.
pub fn set_menu_items(menu: MENU, item_handles: &mut Vec<ITEM>) -> menu_result!(()) {
    item_handles.push(ptr::null_mut());
    item_handles.shrink_to_fit();

    let rc = unsafe { nmenu::set_menu_items(menu, item_handles.as_ptr() as *mut ITEM) };

    item_handles.pop();

    match rc {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_items", rc))
    }
}

/// Sets the mark string for the given menu. Note that changing the length of
/// the mark string for a menu while the menu is posted is likely to produce
/// undefined behavior. The default string is "-" (a dash).
pub fn set_menu_mark(menu: MENU, mark: &str) -> menu_result!(()) {
    if menu_mark(menu)? != '-'.to_string() {
        Err(NCurseswMenuError::BadArgument { func: "set_menu_mark".to_string() })
    } else {
        match unsafe { nmenu::set_menu_mark(menu, c_str_with_nul!(mark)) } {
            E_OK => Ok(()),
            rc   => Err(menu_function_error_with_rc!("set_menu_mark", rc))
        }
    }
}

/// Sets all the given item's options.
pub fn set_menu_opts(menu: MENU, opts: MenuOptions) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_opts(menu, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_opts", rc))
    }
}

/// Sets the character used to fill the space between the name and description
/// parts of a menu item.
pub fn set_menu_pad(menu: MENU, pad: char) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_pad(menu, i32::from(u8::try_from(u32::from(pad))?)) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_pad", rc))
    }
}

/// Sets the pattern buffer for the given menu and tries to find the first
/// matching item. If it succeeds, that item becomes current; if not, the
/// current item does not change.
pub fn set_menu_pattern(menu: MENU, pattern: &str) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_pattern(menu, c_str_with_nul!(pattern)) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_pattern", rc))
    }
}

/// Sets the spacing information for the menu. Its parameter `MenuSpacing::spc_description`
/// controls the number of spaces between an item name and an item description. It must
/// not be larger than `TABSIZE`. The menu system puts in the middle of this spacing area
/// the pad character. The remaining parts are filled with spaces. The `MenuSpacing::spc_rows`
/// parameter controls the number of rows that are used for an item. It must not be larger
/// than 3. The menu system inserts the blank lines between item rows, these lines will
/// contain the pad character in the appropriate positions. The `MenuSpacing::spc_columns`
/// parameter controls the number of blanks between columns of items. It must not be larger
/// than `TABSIZE`. A value of 0 for all the spacing values resets them to the default,
/// which is 1 for all of them.
pub fn set_menu_spacing(menu: MENU, menu_spacing: MenuSpacing) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_spacing(
        menu,
        menu_spacing.description,
        menu_spacing.menu_size.rows,
        menu_spacing.menu_size.columns
    ) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_spacing", rc))
    }
}

/// Sets the menus sub-window. if `form` is `None` then `window` is
/// default for all forms, if `window` is `None` the `stdscr()` is used.
pub fn set_menu_sub(menu: Option<MENU>, win: Option<WINDOW>) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_sub(menu, win) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_sub", rc))
    }
}

/// Sets a hook to be called at menu-unpost time and just before the top row
/// on the menu changes once it is posted.
pub fn set_menu_term(menu: MENU, hook: Menu_Hook) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_term(menu, hook) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_term", rc))
    }
}

/// Sets the menu user pointer.
pub fn set_menu_userptr(menu: MENU, userptr: MenuUserPtr) {
    unsafe { nmenu::set_menu_userptr(menu, userptr) };
}

/// Sets the menus main-window. if `form` is `None` then `window` is
/// default for all forms, if `window` is `None` the `stdscr()` is used.
pub fn set_menu_win(menu: Option<MENU>, win: Option<WINDOW>) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_win(menu, win) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_win", rc))
    }
}

/// Sets the top row of the menu to show the given row (the top row is initially 0,
/// and is reset to this value whenever the `MenuOption::RowMajor` option is toggled).
/// The item leftmost on the given row becomes current.
pub fn set_top_row(menu: MENU, row: i32) -> menu_result!(()) {
    assert!(row >= 0, "{}set_top_row() : row={}", MODULE_PATH, row);

    match unsafe { nmenu::set_top_row(menu, row) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_top_row", rc))
    }
}

/// Returns the number of the top menu row being displayed.
pub fn top_row(menu: MENU) -> i32 {
    unsafe { nmenu::top_row(menu) }
}

/// Erases menu from its associated subwindow.
pub fn unpost_menu(menu: MENU) -> menu_result!(()) {
    match unsafe { nmenu::unpost_menu(menu) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("unpost_menu", rc))
    }
}

// screen `_sp` functions.

/// Screen function of `new_menu()`.
pub fn new_menu_sp(screen: SCREEN, item_handles: &mut Vec<ITEM>) -> menu_result!(MENU) {
    item_handles.push(ptr::null_mut());
    item_handles.shrink_to_fit();

    let menu = unsafe { nmenu::new_menu_sp(screen, item_handles.as_mut_ptr() as *mut ITEM) };

    item_handles.pop();

    menu.ok_or_else(|| menu_function_error_with_rc!("new_menu_sp", errno().into()))
}
