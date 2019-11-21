/*
    src/shims/nmenu.rs

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

#![allow(clippy::crosspointer_transmute)]
#![allow(non_camel_case_types)]

// See <https://invisible-island.net/ncurses/man/menu.3x.html> for documentation.

use std::{mem, slice};
use libc::c_void;
use std::ffi::{CStr, CString};

use bindings;
use bindings::{MenuHook, chtype};
use cstring::*;
use crate::shims::ncurses::WINDOW;

pub type MENU = *mut bindings::tagMENU;
pub type ITEM = *mut bindings::tagITEM;
pub type MENU_USERPTR = Option<*mut c_void>;

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn current_item(menu: MENU) -> Option<ITEM> {
    assert!(!menu.is_null(), "nmenu::current_item() : menu.is_null()");

    let item = bindings::current_item(menu);

    return_optional_mut_ptr!(item)
}

/// <https://invisible-island.net/ncurses/man/mitem_new.3x.html>
pub unsafe fn free_item(item: ITEM) -> i32 {
    assert!(!item.is_null(), "nmenu::free_item() : item.is_null()");

    // if an item name has been defined then unallocate it.
    let name = bindings::item_name(item) as *mut i8;

    if !name.is_null() {
        let _ = CString::from_raw(name);
    }

    // if an item description has been defined then unallocate it.
    let desc = bindings::item_description(item) as *mut i8;

    if !desc.is_null() {
        let _ = CString::from_raw(desc);
    }

    bindings::free_item(item)
}

/// <https://invisible-island.net/ncurses/man/menu_new.3x.html>
pub unsafe fn free_menu(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "nmenu::free_menu() : menu.is_null()");

    bindings::free_menu(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_items.3x.html>
pub unsafe fn item_count(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "nmenu::item_count() : menu.is_null()");

    bindings::item_count(menu)
}

/// <https://invisible-island.net/ncurses/man/mitem_name.3x.html>
pub unsafe fn item_description(item: ITEM) -> Option<String> {
    assert!(!item.is_null(), "nmenu::item_description() : item.is_null()");

    let ptr = bindings::item_description(item);

    if ptr.is_null() {
        None
    } else {
        Some(ptr_to_string!(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn item_index(item: ITEM) -> i32 {
    assert!(!item.is_null(), "nmenu::item_index() : item.is_null()");

    bindings::item_index(item)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn item_init(menu: MENU) -> Option<MenuHook> {
    assert!(!menu.is_null(), "nmenu::item_init() : menu.is_null()");

    let ptr = bindings::item_init(menu);

    if ptr.is_null() {
        None
    } else {
        Some(mem::transmute(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/mitem_name.3x.html>
pub unsafe fn item_name(item: ITEM) -> Option<String> {
    assert!(!item.is_null(), "nmenu::item_name() : item.is_null()");

    let ptr = bindings::item_name(item);

    if ptr.is_null() {
        None
    } else {
        Some(ptr_to_string!(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/mitem_opts.3x.html>
pub unsafe fn item_opts(item: ITEM) -> i32 {
    assert!(!item.is_null(), "nmenu::item_opts() : item.is_null()");

    bindings::item_opts(item)
}

/// <https://invisible-island.net/ncurses/man/mitem_opts.3x.html>
pub unsafe fn item_opts_off(item: ITEM, opts: i32) -> i32 {
    assert!(!item.is_null(), "nmenu::item_opts_off() : item.is_null()");

    bindings::item_opts_off(item, opts)
}

/// <https://invisible-island.net/ncurses/man/mitem_opts.3x.html>
pub unsafe fn item_opts_on(item: ITEM, opts: i32) -> i32 {
    assert!(!item.is_null(), "nmenu::item_opts_on() : item.is_null()");

    bindings::item_opts_on(item, opts)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn item_term(menu: MENU) -> Option<MenuHook> {
    assert!(!menu.is_null(), "nmenu::item_term() : menu.is_null()");

    let ptr = bindings::item_term(menu);

    if ptr.is_null() {
        None
    } else {
        Some(mem::transmute(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/mitem_userptr.3x.html>
pub unsafe fn item_userptr(item: ITEM) -> MENU_USERPTR {
    assert!(!item.is_null(), "nmenu::item_userptr() : item.is_null()");

    let ptr = bindings::item_userptr(item);

    return_optional_mut_ptr!(ptr)
}

/// <https://invisible-island.net/ncurses/man/mitem_value.3x.html>
pub unsafe fn item_value(item: ITEM) -> bool {
    assert!(!item.is_null(), "nmenu::item_value() : item.is_null()");

    bindings::item_value(item)
}

/// <https://invisible-island.net/ncurses/man/mitem_visible.3x.html>
pub unsafe fn item_visible(item: ITEM) -> bool {
    assert!(!item.is_null(), "nmenu::item_visible() : item.is_null()");

    bindings::item_visible(item)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn menu_back(menu: MENU) -> chtype {
    assert!(!menu.is_null(), "nmenu::menu_back() : menu.is_null()");

    bindings::menu_back(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_driver.3x.html>
pub unsafe fn menu_driver(menu: MENU, c: i32) -> i32 {
    assert!(!menu.is_null(), "nmenu::menu_driver() : menu.is_null()");

    bindings::menu_driver(menu, c)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn menu_fore(menu: MENU) -> chtype {
    assert!(!menu.is_null(), "nmenu::menu_fore() : menu.is_null()");

    bindings::menu_fore(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_format.3x.html>
pub unsafe fn menu_format(menu: MENU, rows: *mut i32, cols: *mut i32) {
    assert!(!menu.is_null(), "nmenu::menu_format() : menu.is_null()");
    assert!(!rows.is_null(), "nmenu::menu_format() : rows.is_null()");
    assert!(!cols.is_null(), "nmenu::menu_format() : cols.is_null()");

    bindings::menu_format(menu, rows, cols)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn menu_grey(menu: MENU) -> chtype {
    assert!(!menu.is_null(), "nmenu::menu_grey() : menu.is_null()");

    bindings::menu_grey(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn menu_init(menu: MENU) -> Option<MenuHook> {
    assert!(!menu.is_null(), "nmenu::menu_init() : menu.is_null()");

    let ptr = bindings::menu_init(menu);

    if ptr.is_null() {
        None
    } else {
        Some(mem::transmute(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/menu_items.3x.html>
pub unsafe fn menu_items(menu: MENU) -> Option<Vec<ITEM>> {
    assert!(!menu.is_null(), "nmenu::menu_items() : menu.is_null()");

    let ptr = bindings::menu_items(menu);

    if ptr.is_null() {
        None
    } else {
        let item_count = item_count(menu);

        if item_count <= 0 {
            None
        } else {
            Some(slice::from_raw_parts(ptr, item_count as usize).to_vec())
        }
    }
}

/// <https://invisible-island.net/ncurses/man/menu_mark.3x.html>
pub unsafe fn menu_mark(menu: MENU) -> Option<String> {
    assert!(!menu.is_null(), "nmenu::menu_mark() : menu.is_null()");

    let ptr = bindings::menu_mark(menu);

    if ptr.is_null() {
        None
    } else {
        Some(FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/menu_opts.3x.html>
pub unsafe fn menu_opts(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "nmenu::menu_opts() : menu.is_null()");

    bindings::menu_opts(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_opts.3x.html>
pub unsafe fn menu_opts_off(menu: MENU, opts: i32) -> i32 {
    assert!(!menu.is_null(), "nmenu::menu_opts_off() : menu.is_null()");

    bindings::menu_opts_off(menu, opts)
}

/// <https://invisible-island.net/ncurses/man/menu_opts.3x.html>
pub unsafe fn menu_opts_on(menu: MENU, opts: i32) -> i32 {
    assert!(!menu.is_null(), "nmenu::menu_opts_on() : menu.is_null()");

    bindings::menu_opts_on(menu, opts)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn menu_pad(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "nmenu::menu_pad() : menu.is_null()");

    bindings::menu_pad(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_pattern.3x.html>
pub unsafe fn menu_pattern(menu: MENU) -> Option<String> {
    assert!(!menu.is_null(), "nmenu::menu_pattern() : menu.is_null()");

    let ptr = bindings::menu_pattern(menu);

    if ptr.is_null() {
        None
    } else {
        Some(FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/menu_requestname.3x.html>
pub unsafe fn menu_request_by_name(name: &[i8]) -> i32 {
    bindings::menu_request_by_name(name.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/menu_requestname.3x.html>
pub unsafe fn menu_request_name(request: i32) -> Option<String> {
    let ptr = bindings::menu_request_name(request);

    if ptr.is_null() {
        None
    } else {
        Some(FromCStr::from_c_str(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/menu_spacing.3x.html>
pub unsafe fn menu_spacing(
    menu:            MENU,
    spc_description: *mut i32,
    spc_rows:        *mut i32,
    spc_columns:     *mut i32
) -> i32 {
    assert!(!menu.is_null(), "nmenu::menu_spacing() : menu.is_null()");
    assert!(!spc_description.is_null(), "nmenu::menu_spacing() : spc_description.is_null()");
    assert!(!spc_rows.is_null(), "nmenu::menu_spacing() : spc_rows.is_null()");
    assert!(!spc_columns.is_null(), "nmenu::menu_spacing() : spc_columns.is_null()");

    bindings::menu_spacing(menu, spc_description, spc_rows, spc_columns)
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn menu_sub(menu: MENU) -> Option<WINDOW> {
    assert!(!menu.is_null(), "nmenu::menu_sub() : menu.is_null()");

    let win = bindings::menu_sub(menu);

    return_optional_mut_ptr!(win)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn menu_term(menu: MENU) -> Option<MenuHook> {
    assert!(!menu.is_null(), "nmenu::menu_term() : menu.is_null()");

    let ptr = bindings::menu_term(menu);

    if ptr.is_null() {
        None
    } else {
        Some(mem::transmute(ptr))
    }
}

/// <https://invisible-island.net/ncurses/man/menu_userptr.3x.html>
pub unsafe fn menu_userptr(menu: MENU) -> MENU_USERPTR {
    assert!(!menu.is_null(), "nmenu::menu_userptr() : menu.is_null()");

    let ptr = bindings::menu_userptr(menu);

    return_optional_mut_ptr!(ptr)
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn menu_win(menu: MENU) -> Option<WINDOW> {
    assert!(!menu.is_null(), "nmenu::menu_win() : menu.is_null()");

    let win = bindings::menu_win(menu);

    return_optional_mut_ptr!(win)
}

/// <https://invisible-island.net/ncurses/man/mitem_new.3x.html>
pub unsafe fn new_item(name: *mut i8, description: *mut i8) -> Option<ITEM> {
    assert!(!name.is_null(), "nmenu::new_item() : name.is_null()");
    assert!(!description.is_null(), "nmenu::new_item() : description.is_null()");

    let item = bindings::new_item(name, description);

    return_optional_mut_ptr!(item)
}

/// <https://invisible-island.net/ncurses/man/menu_new.3x.html>
pub unsafe fn new_menu(items: *mut ITEM) -> Option<MENU> {
    assert!(!items.is_null(), "nmenu::new_menu() : items.is_null()");

    let menu = bindings::new_menu(items);

    return_optional_mut_ptr!(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_cursor.3x.html>
pub unsafe fn pos_menu_cursor(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "nmenu::pos_menu_cursor() : menu.is_null()");

    bindings::pos_menu_cursor(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_post.3x.html>
pub unsafe fn post_menu(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "nmenu::post_menu() : menu.is_null()");

    bindings::post_menu(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn scale_menu(menu: MENU, rows: *mut i32, cols: *mut i32) -> i32 {
    assert!(!menu.is_null(), "nmenu::scale_menu() : menu.is_null()");
    assert!(!rows.is_null(), "nmenu::scale_menu() : rows.is_null()");
    assert!(!cols.is_null(), "nmenu::scale_menu() : cols.is_null()");

    bindings::scale_menu(menu, rows, cols)
}

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn set_current_item(menu: MENU, item: ITEM) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_current_item() : menu.is_null()");
    assert!(!item.is_null(), "nmenu::set_current_item() : item.is_null()");

    bindings::set_current_item(menu, item)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn set_item_init(menu: MENU, hook: MenuHook) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_item_init() : menu.is_null()");

    bindings::set_item_init(menu, hook)
}

/// <https://invisible-island.net/ncurses/man/mitem_opts.3x.html>
pub unsafe fn set_item_opts(item: ITEM, opts: i32) -> i32 {
    assert!(!item.is_null(), "nmenu::set_item_opts() : item.is_null()");

    bindings::set_item_opts(item, opts)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn set_item_term(menu: MENU, hook: MenuHook) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_item_term() : menu.is_null()");

    bindings::set_item_term(menu, hook)
}

/// <https://invisible-island.net/ncurses/man/mitem_userptr.3x.html>
pub unsafe fn set_item_userptr(item: ITEM, userptr: MENU_USERPTR) -> i32 {
    assert!(!item.is_null(), "nmenu::set_item_userptr() : item.is_null()");

    bindings::set_item_userptr(item, return_mut_ptr!(userptr))
}

/// <https://invisible-island.net/ncurses/man/mitem_value.3x.html>
pub unsafe fn set_item_value(item: ITEM, value: bool) -> i32 {
    assert!(!item.is_null(), "nmenu::set_item_value() : item.is_null()");

    bindings::set_item_value(item, value)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn set_menu_back(menu: MENU, attr: chtype) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_back() : menu.is_null()");

    bindings::set_menu_back(menu, attr)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn set_menu_fore(menu: MENU, attr: chtype) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_fore() : menu.is_null()");

    bindings::set_menu_fore(menu, attr)
}

/// <https://invisible-island.net/ncurses/man/menu_format.3x.html>
pub unsafe fn set_menu_format(menu: Option<MENU>, rows: i32, cols: i32) -> i32 {
    if let Some(ptr) = menu {
        assert!(!ptr.is_null(), "nmenu::set_menu_format() : menu.is_null()");
    }

    bindings::set_menu_format(return_mut_ptr!(menu), rows, cols)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn set_menu_grey(menu: MENU, attr: chtype) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_grey() : menu.is_null()");

    bindings::set_menu_grey(menu, attr)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn set_menu_init(menu: MENU, hook: MenuHook) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_init() : menu.is_null()");

    bindings::set_menu_init(menu, hook)
}

/// <https://invisible-island.net/ncurses/man/menu_items.3x.html>
pub unsafe fn set_menu_items(menu: MENU, items: *mut ITEM) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_items() : menu.is_null()");
    assert!(!items.is_null(), "nmenu::set_menu_items() : items.is_null()");

    bindings::set_menu_items(menu, items)
}

/// <https://invisible-island.net/ncurses/man/menu_mark.3x.html>
pub unsafe fn set_menu_mark(menu: MENU, mark: &[i8]) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_mark() : menu.is_null()");

    bindings::set_menu_mark(menu, mark.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/menu_opts.3x.html>
pub unsafe fn set_menu_opts(menu: MENU, opts: i32) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_opts() : menu.is_null()");

    bindings::set_menu_opts(menu, opts)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn set_menu_pad(menu: MENU, opts: i32) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_pad() : menu.is_null()");

    bindings::set_menu_pad(menu, opts)
}

/// <https://invisible-island.net/ncurses/man/menu_pattern.3x.html>
pub unsafe fn set_menu_pattern(menu: MENU, pattern: &[i8]) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_pattern() : menu.is_null()");

    bindings::set_menu_pattern(menu, pattern.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/menu_spacing.3x.html>
pub unsafe fn set_menu_spacing(
    menu:            MENU,
    spc_description: i32,
    spc_rows:        i32,
    spc_columns:     i32
) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_spacing() : menu.is_null()");

    bindings::set_menu_spacing(menu, spc_description, spc_rows, spc_columns)
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn set_menu_sub(menu: MENU, win: Option<WINDOW>) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_sub() : menu.is_null()");

    bindings::set_menu_sub(menu, return_mut_ptr!(win))
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn set_menu_term(menu: MENU, hook: MenuHook) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_term() : menu.is_null()");

    bindings::set_menu_term(menu, hook)
}

/// <https://invisible-island.net/ncurses/man/menu_userptr.3x.html>
pub unsafe fn set_menu_userptr(menu: MENU, userptr: MENU_USERPTR) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_userptr() : menu.is_null()");

    bindings::set_menu_userptr(menu, return_mut_ptr!(userptr))
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn set_menu_win(menu: MENU, win: Option<WINDOW>) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_menu_win() : menu.is_null()");

    bindings::set_menu_win(menu, return_mut_ptr!(win))
}

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn set_top_row(menu: MENU, row: i32) -> i32 {
    assert!(!menu.is_null(), "nmenu::set_top_row() : menu.is_null()");

    bindings::set_top_row(menu, row)
}

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn top_row(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "nmenu::top_row() : menu.is_null()");

    bindings::top_row(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_post.3x.html>
pub unsafe fn unpost_menu(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "nmenu::unpost_menu() : menu.is_null()");

    bindings::unpost_menu(menu)
}
