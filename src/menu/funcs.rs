/*
    src/menu/func.rs

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

use std::{ptr, ffi::CString};

use errno::errno;

use normal;

use cstring::*;
use shims::{nmenu, bindings, ncurses::WINDOW, constants::{E_OK, E_NO_MATCH, E_UNKNOWN_COMMAND}};
use menu::{
    ItemOptions, MenuOptions, MenuSpacing, MenuRequest, MenuSize, MenuUserPtr,
    ncurseswmenuerror::{
        NCurseswMenuError, ncursesw_menu_error_system_error, ncursesw_menu_error_from_rc
    }
};

pub type MENU = nmenu::MENU;
pub type ITEM = nmenu::ITEM;
pub type Menu_Hook = crate::shims::bindings::Menu_Hook;

static MODULE_PATH: &str = "ncursesw::menu::funcs::";

pub fn current_item(menu: MENU) -> menu_result!(ITEM) {
    unsafe { nmenu::current_item(menu) }.ok_or_else(|| menu_function_error!("current_item"))
}

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

// make sure that free_menu() is called before free_item() otherwise the
// item will still be connected to the menu.
pub fn free_menu(menu: MENU) -> menu_result!(()) {
    match unsafe { nmenu::free_menu(menu) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("free_menu", rc))
    }
} 

pub fn item_count(menu: MENU) -> menu_result!(i32) {
    let rc = unsafe { nmenu::item_count(menu) };

    if rc < 0 {
        Err(NCurseswMenuError::UnknownError { func: "item_count".to_string(), errno: rc })
    } else {
        Ok(rc)
    }
}

pub fn item_description(item: ITEM) -> menu_result!(String) {
    unsafe { nmenu::item_description(item) }.ok_or_else(|| menu_function_error!("item_description"))
}

pub fn item_index(item: ITEM) -> menu_result!(i32) {
    let rc = unsafe { nmenu::item_index(item) };

    if rc < 0 {
        Err(NCurseswMenuError::UnknownError { func: "item_index".to_string(), errno: rc })
    } else {
        Ok(rc)
    }
} 

pub fn item_init(menu: MENU) -> menu_result!(Menu_Hook) {
    unsafe { nmenu::item_init(menu) }.ok_or_else(|| menu_function_error_with_rc!("item_init", errno().into()))
}

pub fn item_name(item: ITEM) -> menu_result!(String) {
    unsafe { nmenu::item_name(item) }.ok_or_else(|| menu_function_error!("item_name"))
}

pub fn item_opts(item: ITEM) -> ItemOptions {
    unsafe { ItemOptions::from(nmenu::item_opts(item)) }
}

pub fn item_opts_off(item: ITEM, opts: ItemOptions) -> menu_result!(()) {
    match unsafe { nmenu::item_opts_off(item, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("item_opts_off", rc))
    }
}

pub fn item_opts_on(item: ITEM, opts: ItemOptions) -> menu_result!(()) {
    match unsafe { nmenu::item_opts_on(item, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("item_opts_on", rc))
    }
}

pub fn item_term(menu: MENU) -> menu_result!(Menu_Hook) {
    unsafe { nmenu::item_term(menu) }.ok_or_else(|| menu_function_error_with_rc!("item_term", errno().into()))
}

pub fn item_userptr(item: ITEM) -> MenuUserPtr {
    unsafe { nmenu::item_userptr(item) }
}

pub fn item_value(item: ITEM) -> bool {
    unsafe { nmenu::item_value(item) }
}

pub fn item_visible(item: ITEM) -> bool {
    unsafe { nmenu::item_visible(item) }
}

pub fn menu_back(menu: MENU) -> normal::Attributes {
    unsafe { normal::Attributes::from(nmenu::menu_back(menu)) }
}

pub fn menu_driver(menu: MENU, request: MenuRequest) -> menu_result!(Option<i32>) {
    match unsafe { nmenu::menu_driver(menu, request.value()) } {
        E_OK => Ok(None),
        rc   => if request == MenuRequest::Mouse {
            if rc == E_UNKNOWN_COMMAND {
                Ok(None)
            } else {
                Ok(Some(rc))
            }
        } else {
            Err(menu_function_error_with_rc!("menu_driver", rc))
        }
    }
}

pub fn menu_fore(menu: MENU) -> normal::Attributes {
    unsafe { normal::Attributes::from(nmenu::menu_fore(menu)) }
}

pub fn menu_format(menu: MENU) -> MenuSize {
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];

    unsafe { nmenu::menu_format(menu, rows.as_mut_ptr(), cols.as_mut_ptr()) };

    MenuSize { rows: rows[0], columns: cols[0] }
}

pub fn menu_grey(menu: MENU) -> normal::Attributes {
    unsafe { normal::Attributes::from(nmenu::menu_grey(menu)) }
}

pub fn menu_init(menu: MENU) -> menu_result!(Menu_Hook) {
    unsafe { nmenu::menu_init(menu) }.ok_or_else(|| menu_function_error_with_rc!("menu_init", errno().into()))
}

pub fn menu_items(menu: MENU) -> menu_result!(Vec<ITEM>) {
    unsafe { nmenu::menu_items(menu) }.ok_or_else(|| menu_function_error!("menu_items"))
}

pub fn menu_mark(menu: MENU) -> menu_result!(String) {
    unsafe { nmenu::menu_mark(menu) }.ok_or_else(|| menu_function_error!("menu_mark"))
}

pub fn menu_opts(menu: MENU) -> MenuOptions {
    unsafe { MenuOptions::from(nmenu::menu_opts(menu)) }
}

pub fn menu_opts_off(menu: MENU, opts: MenuOptions) -> menu_result!(()) {
    match unsafe { nmenu::menu_opts_off(menu, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("menu_opts_off", rc))
    }
}

pub fn menu_opts_on(menu: MENU, opts: MenuOptions) -> menu_result!(()) {
    match unsafe { nmenu::menu_opts_on(menu, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("menu_opts_on", rc))
    }
}

pub fn menu_pad(menu: MENU) -> char {
    unsafe { nmenu::menu_pad(menu) as u8 as char }
}

pub fn menu_pattern(menu: MENU) -> menu_result!(String) {
    unsafe { nmenu::menu_pattern(menu) }.ok_or_else(|| menu_function_error!("menu_pattern"))
}

pub fn menu_request_by_name(name: &str) -> menu_result!(bool) {
    match nmenu::menu_request_by_name(unsafe { c_str_with_nul!(name) }) {
        E_OK       => Ok(true),
        E_NO_MATCH => Ok(false),
        rc         => Err(menu_function_error_with_rc!("menu_request_by_name", rc))
    }
}

pub fn menu_request_name(request: MenuRequest) -> menu_result!(String) {
    nmenu::menu_request_name(request.value()).ok_or_else(|| menu_function_error_with_rc!("menu_request_name", errno().into()))
}

pub fn menu_spacing(menu: MENU) -> menu_result!(MenuSpacing) {
    let mut description: [i32; 1] = [0];
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];

    match unsafe { nmenu::menu_spacing(menu, description.as_mut_ptr(), rows.as_mut_ptr(), cols.as_mut_ptr()) } {
        E_OK => Ok(MenuSpacing { description: description[0], menu_size: MenuSize { rows: rows[0], columns: cols[0] }}),
        rc   => Err(menu_function_error_with_rc!("menu_spacing", rc))
    }
}

pub fn menu_sub(menu: MENU) -> menu_result!(WINDOW) {
    unsafe { nmenu::menu_sub(menu) }.ok_or_else(|| menu_function_error!("menu_sub"))
}

pub fn menu_term(menu: MENU) -> menu_result!(Menu_Hook) {
    unsafe { nmenu::menu_term(menu) }.ok_or_else(|| menu_function_error_with_rc!("menu_term", errno().into()))
}

pub fn menu_userptr(menu: MENU) -> MenuUserPtr {
    unsafe { nmenu::menu_userptr(menu) }
}

pub fn menu_win(menu: MENU) -> menu_result!(WINDOW) {
    unsafe { nmenu::menu_win(menu) }.ok_or_else(|| menu_function_error!("menu_win"))
}

pub fn new_item<T>(name: T, description: T) -> menu_result!(ITEM)
    where T: Into<Vec<u8>>
{
    let name = CString::new(name)?.into_raw();
    let description = CString::new(description)?.into_raw();

    unsafe { nmenu::new_item(name, description) }.ok_or_else(|| menu_function_error_with_rc!("new_item", errno().into()))
}

// when new_menu() is called make sure that the memory for the item_handles
// is contiguous and does not go out of scope until after free_menu() has
// been called otherwise unpredicable results may occur, this is because the
// underlying ncurses menu functions use this memory directly.
// See ncursesw-win-rs's Menu::new() <https://github.com/narfit66/ncursesw-win-rs/blob/master/src/menu/menu.rs>
// as an example of how the nmenu::new_menu() function can be called by
// allocating and keeping the memory required but bypasses this function
// and calling nmenu::new_menu() directly (although you could also call
// this function directly as long as the underlying memory is contiguous
// and does not go out of scope).
pub fn new_menu(item_handles: &mut Vec<ITEM>) -> menu_result!(MENU) {
    item_handles.push(ptr::null_mut());
    item_handles.shrink_to_fit();

    let menu = unsafe { nmenu::new_menu(item_handles.as_mut_ptr() as *mut ITEM) };

    item_handles.pop();

    menu.ok_or_else(|| menu_function_error_with_rc!("new_menu", errno().into()))
}

pub fn pos_menu_cursor(menu: MENU) -> menu_result!(()) {
    match unsafe { nmenu::pos_menu_cursor(menu) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("pos_menu_cursor", rc))
    }
}

pub fn post_menu(menu: MENU) -> menu_result!(()) {
    match unsafe { nmenu::post_menu(menu) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("post_menu", rc))
    }
}

pub fn scale_menu(menu: MENU) -> menu_result!(MenuSize) {
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];

    match unsafe { nmenu::scale_menu(menu, rows.as_mut_ptr(), cols.as_mut_ptr()) } {
        E_OK => Ok(MenuSize { rows: rows[0], columns: cols[0] }),
        rc   => Err(menu_function_error_with_rc!("scale_menu", rc))
    }
}

pub fn set_current_item(menu: MENU, item: ITEM) -> menu_result!(()) {
    match unsafe { nmenu::set_current_item(menu, item) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_current_item", rc))
    }
}

pub fn set_item_init(menu: MENU, hook: Menu_Hook) -> menu_result!(()) {
    match unsafe { nmenu::set_item_init(menu, hook) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_item_init", rc))
    }
}

pub fn set_item_opts(item: ITEM, opts: ItemOptions) -> menu_result!(()) {
    match unsafe { nmenu::set_item_opts(item, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_item_opts", rc))
    }
}

pub fn set_item_term(menu: MENU, hook: Menu_Hook) -> menu_result!(()) {
    match unsafe { nmenu::set_item_term(menu, hook) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_item_term", rc))
    }
}

pub fn set_item_userptr(item: ITEM, userptr: MenuUserPtr) {
    unsafe { nmenu::set_item_userptr(item, userptr) };
}

pub fn set_item_value(item: ITEM, value: bool) -> menu_result!(()) {
    match unsafe { nmenu::set_item_value(item, value) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_item_value", rc))
    }
}

pub fn set_menu_back(menu: MENU, attr: normal::Attributes) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_back(menu, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_back", rc))
    }
}

pub fn set_menu_fore(menu: MENU, attr: normal::Attributes) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_fore(menu, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_fore", rc))
    }
}

pub fn set_menu_format(menu: Option<MENU>, menu_size: MenuSize) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_format(menu, menu_size.rows, menu_size.columns) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_format", rc))
    }
}

pub fn set_menu_grey(menu: MENU, attr: normal::Attributes) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_grey(menu, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_grey", rc))
    }
}

pub fn set_menu_init(menu: MENU, hook: Menu_Hook) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_init(menu, hook) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_init", rc))
    }
}

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

pub fn set_menu_opts(menu: MENU, opts: MenuOptions) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_opts(menu, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_opts", rc))
    }
}

pub fn set_menu_pad(menu: MENU, pad: char) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_pad(menu, i32::from(pad as u8)) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_pad", rc))
    }
}

pub fn set_menu_pattern(menu: MENU, pattern: &str) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_pattern(menu, c_str_with_nul!(pattern)) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_pattern", rc))
    }
}

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

pub fn set_menu_sub(menu: Option<MENU>, win: Option<WINDOW>) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_sub(menu, win) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_sub", rc))
    }
}

pub fn set_menu_term(menu: MENU, hook: Menu_Hook) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_term(menu, hook) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_term", rc))
    }
}

pub fn set_menu_userptr(menu: MENU, userptr: MenuUserPtr) {
    unsafe { nmenu::set_menu_userptr(menu, userptr) };
}

pub fn set_menu_win(menu: Option<MENU>, win: Option<WINDOW>) -> menu_result!(()) {
    match unsafe { nmenu::set_menu_win(menu, win) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_menu_win", rc))
    }
}

pub fn set_top_row(menu: MENU, row: i32) -> menu_result!(()) {
    assert!(row >= 0, "{}set_top_row() : row={}", MODULE_PATH, row);

    match unsafe { nmenu::set_top_row(menu, row) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("set_top_row", rc))
    }
}

pub fn top_row(menu: MENU) -> i32 {
    unsafe { nmenu::top_row(menu) }
}

pub fn unpost_menu(menu: MENU) -> menu_result!(()) {
    match unsafe { nmenu::unpost_menu(menu) } {
        E_OK => Ok(()),
        rc   => Err(menu_function_error_with_rc!("unpost_menu", rc))
    }
}
