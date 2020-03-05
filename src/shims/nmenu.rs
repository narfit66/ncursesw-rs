/*
    src/shims/nmenu.rs

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

#![allow(clippy::crosspointer_transmute)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

// See <https://invisible-island.net/ncurses/man/menu.3x.html> for documentation.

use std::{mem, slice, ffi::CStr};

use crate::{
    shims::{bindings, bindings::{Menu_Hook, chtype}, ncurses::{SCREEN, WINDOW}},
    cstring::FromCStr
};

pub type MENU = *mut bindings::MENU;
pub type ITEM = *mut bindings::ITEM;
pub type MENU_USERPTR = *mut libc::c_void;

static MODULE_PATH: &str = "ncursesw::shims::nmenu::";

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn current_item(menu: MENU) -> Option<ITEM> {
    assert!(!menu.is_null(), "{}current_item() : menu.is_null()", MODULE_PATH);

    bindings::current_item(menu).as_mut().map(|ptr| ptr as ITEM)
}

/// <https://invisible-island.net/ncurses/man/mitem_new.3x.html>
pub unsafe fn free_item(item: ITEM) -> i32 {
    assert!(!item.is_null(), "{}free_item() : item.is_null()", MODULE_PATH);

    bindings::free_item(item)
}

/// <https://invisible-island.net/ncurses/man/menu_new.3x.html>
pub unsafe fn free_menu(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "{}free_menu() : menu.is_null()", MODULE_PATH);

    bindings::free_menu(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_items.3x.html>
pub unsafe fn item_count(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "{}item_count() : menu.is_null()", MODULE_PATH);

    bindings::item_count(menu)
}

/// <https://invisible-island.net/ncurses/man/mitem_name.3x.html>
pub unsafe fn item_description(item: ITEM) -> Option<String> {
    assert!(!item.is_null(), "{}item_description() : item.is_null()", MODULE_PATH);

    (bindings::item_description(item) as *mut i8).as_mut().map(|ptr| ptr_as_string!(ptr))
}

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn item_index(item: ITEM) -> i32 {
    assert!(!item.is_null(), "{}item_index() : item.is_null()", MODULE_PATH);

    bindings::item_index(item)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn item_init(menu: Option<MENU>) -> Option<Menu_Hook> {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}item_init() : menu.is_null()", MODULE_PATH);

    bindings::item_init(return_mut_ptr!(menu)).as_mut().map(|ptr| mem::transmute(ptr))
}

/// <https://invisible-island.net/ncurses/man/mitem_name.3x.html>
pub unsafe fn item_name(item: ITEM) -> Option<String> {
    assert!(!item.is_null(), "{}item_name() : item.is_null()", MODULE_PATH);

    (bindings::item_name(item) as *mut i8).as_mut().map(|ptr| ptr_as_string!(ptr))
}

/// <https://invisible-island.net/ncurses/man/mitem_opts.3x.html>
pub unsafe fn item_opts(item: Option<ITEM>) -> i32 {
    assert!(item.map_or_else(|| true, |item| !item.is_null()), "{}item_opts() : item.is_null()", MODULE_PATH);

    bindings::item_opts(return_mut_ptr!(item))
}

/// <https://invisible-island.net/ncurses/man/mitem_opts.3x.html>
pub unsafe fn item_opts_off(item: Option<ITEM>, opts: i32) -> i32 {
    assert!(item.map_or_else(|| true, |item| !item.is_null()), "{}item_opts_off() : item.is_null()", MODULE_PATH);

    bindings::item_opts_off(return_mut_ptr!(item), opts)
}

/// <https://invisible-island.net/ncurses/man/mitem_opts.3x.html>
pub unsafe fn item_opts_on(item: Option<ITEM>, opts: i32) -> i32 {
    assert!(item.map_or_else(|| true, |item| !item.is_null()), "{}item_opts_on() : item.is_null()", MODULE_PATH);

    bindings::item_opts_on(return_mut_ptr!(item), opts)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn item_term(menu: Option<MENU>) -> Option<Menu_Hook> {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}item_term() : menu.is_null()", MODULE_PATH);

    bindings::item_term(return_mut_ptr!(menu)).as_mut().map(|ptr| mem::transmute(ptr))
}

/// <https://invisible-island.net/ncurses/man/mitem_userptr.3x.html>
pub unsafe fn item_userptr(item: Option<ITEM>) -> Option<MENU_USERPTR> {
    assert!(item.map_or_else(|| true, |item| !item.is_null()), "{}item_userptr() : item.is_null()", MODULE_PATH);

    bindings::item_userptr(return_mut_ptr!(item)).as_mut().map(|ptr| ptr as MENU_USERPTR)
}

/// <https://invisible-island.net/ncurses/man/mitem_value.3x.html>
pub unsafe fn item_value(item: ITEM) -> bool {
    assert!(!item.is_null(), "{}item_value() : item.is_null()", MODULE_PATH);

    bindings::item_value(item)
}

/// <https://invisible-island.net/ncurses/man/mitem_visible.3x.html>
pub unsafe fn item_visible(item: ITEM) -> bool {
    assert!(!item.is_null(), "{}item_visible() : item.is_null()", MODULE_PATH);

    bindings::item_visible(item)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn menu_back(menu: Option<MENU>) -> chtype {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_back() : menu.is_null()", MODULE_PATH);

    bindings::menu_back(return_mut_ptr!(menu))
}

/// <https://invisible-island.net/ncurses/man/menu_driver.3x.html>
pub unsafe fn menu_driver(menu: MENU, c: i32) -> i32 {
    assert!(!menu.is_null(), "{}menu_driver() : menu.is_null()", MODULE_PATH);

    bindings::menu_driver(menu, c)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn menu_fore(menu: Option<MENU>) -> chtype {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_fore() : menu.is_null()", MODULE_PATH);

    bindings::menu_fore(return_mut_ptr!(menu))
}

/// <https://invisible-island.net/ncurses/man/menu_format.3x.html>
pub unsafe fn menu_format(menu: Option<MENU>, rows: *mut i32, cols: *mut i32) {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_format() : menu.is_null()", MODULE_PATH);
    assert!(!rows.is_null(), "{}menu_format() : rows.is_null()", MODULE_PATH);
    assert!(!cols.is_null(), "{}menu_format() : cols.is_null()", MODULE_PATH);

    bindings::menu_format(return_mut_ptr!(menu), rows, cols)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn menu_grey(menu: Option<MENU>) -> chtype {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_grey() : menu.is_null()", MODULE_PATH);

    bindings::menu_grey(return_mut_ptr!(menu))
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn menu_init(menu: Option<MENU>) -> Option<Menu_Hook> {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_init() : menu.is_null()", MODULE_PATH);

    bindings::menu_init(return_mut_ptr!(menu)).as_mut().map(|ptr| mem::transmute(ptr))
}

/// <https://invisible-island.net/ncurses/man/menu_items.3x.html>
pub unsafe fn menu_items(menu: MENU) -> Option<Vec<ITEM>> {
    assert!(!menu.is_null(), "{}menu_items() : menu.is_null()", MODULE_PATH);

    bindings::menu_items(menu)
        .as_mut()
        .map(|ptr| slice::from_raw_parts(ptr, bindings::item_count(menu) as usize).to_vec())
}

/// <https://invisible-island.net/ncurses/man/menu_mark.3x.html>
pub unsafe fn menu_mark(menu: Option<MENU>) -> Option<String> {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_mark() : menu.is_null()", MODULE_PATH);

    (bindings::menu_mark(return_mut_ptr!(menu)) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
}

/// <https://invisible-island.net/ncurses/man/menu_opts.3x.html>
pub unsafe fn menu_opts(menu: Option<MENU>) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_opts() : menu.is_null()", MODULE_PATH);

    bindings::menu_opts(return_mut_ptr!(menu))
}

/// <https://invisible-island.net/ncurses/man/menu_opts.3x.html>
pub unsafe fn menu_opts_off(menu: Option<MENU>, opts: i32) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_opts_off() : menu.is_null()", MODULE_PATH);

    bindings::menu_opts_off(return_mut_ptr!(menu), opts)
}

/// <https://invisible-island.net/ncurses/man/menu_opts.3x.html>
pub unsafe fn menu_opts_on(menu: Option<MENU>, opts: i32) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_opts_on() : menu.is_null()", MODULE_PATH);

    bindings::menu_opts_on(return_mut_ptr!(menu), opts)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn menu_pad(menu: Option<MENU>) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_pad() : menu.is_null()", MODULE_PATH);

    bindings::menu_pad(return_mut_ptr!(menu))
}

/// <https://invisible-island.net/ncurses/man/menu_pattern.3x.html>
pub unsafe fn menu_pattern(menu: MENU) -> Option<String> {
    assert!(!menu.is_null(), "{}menu_pattern() : menu.is_null()", MODULE_PATH);

    (bindings::menu_pattern(menu) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr))
}

/// <https://invisible-island.net/ncurses/man/menu_requestname.3x.html>
pub fn menu_request_by_name(name: &[i8]) -> i32 {
    unsafe { bindings::menu_request_by_name(name.as_ptr()) }
}

/// <https://invisible-island.net/ncurses/man/menu_requestname.3x.html>
pub fn menu_request_name(request: i32) -> Option<String> {
    unsafe { (bindings::menu_request_name(request) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr)) }
}

/// <https://invisible-island.net/ncurses/man/menu_spacing.3x.html>
pub unsafe fn menu_spacing(
    menu:            Option<MENU>,
    spc_description: *mut i32,
    spc_rows:        *mut i32,
    spc_columns:     *mut i32
) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_spacing() : menu.is_null()", MODULE_PATH);
    assert!(!spc_description.is_null(), "{}menu_spacing() : spc_description.is_null()", MODULE_PATH);
    assert!(!spc_rows.is_null(), "{}menu_spacing() : spc_rows.is_null()", MODULE_PATH);
    assert!(!spc_columns.is_null(), "{}menu_spacing() : spc_columns.is_null()", MODULE_PATH);

    bindings::menu_spacing(return_mut_ptr!(menu), spc_description, spc_rows, spc_columns)
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn menu_sub(menu: Option<MENU>) -> Option<WINDOW> {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_sub() : menu.is_null()", MODULE_PATH);

    bindings::menu_sub(return_mut_ptr!(menu)).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn menu_term(menu: Option<MENU>) -> Option<Menu_Hook> {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_term() : menu.is_null()", MODULE_PATH);

    bindings::menu_term(return_mut_ptr!(menu)).as_mut().map(|ptr| mem::transmute(ptr))
}

/// <https://invisible-island.net/ncurses/man/menu_userptr.3x.html>
pub unsafe fn menu_userptr(menu: Option<MENU>) -> Option<MENU_USERPTR> {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_userptr() : menu.is_null()", MODULE_PATH);

    bindings::menu_userptr(return_mut_ptr!(menu)).as_mut().map(|ptr| ptr as MENU_USERPTR)
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn menu_win(menu: Option<MENU>) -> Option<WINDOW> {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}menu_win() : menu.is_null()", MODULE_PATH);

    bindings::menu_win(return_mut_ptr!(menu)).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/mitem_new.3x.html>
pub unsafe fn new_item(name: *mut i8, description: *mut i8) -> Option<ITEM> {
    assert!(!name.is_null(), "{}new_item() : name.is_null()", MODULE_PATH);
    assert!(!description.is_null(), "{}new_item() : description.is_null()", MODULE_PATH);

    bindings::new_item(name, description).as_mut().map(|ptr| ptr as ITEM)
}

/// <https://invisible-island.net/ncurses/man/menu_new.3x.html>
pub unsafe fn new_menu(item_handles: *mut ITEM) -> Option<MENU> {
    assert!(!item_handles.is_null(), "{}new_menu() : item_handles.is_null()", MODULE_PATH);

    bindings::new_menu(item_handles).as_mut().map(|ptr| ptr as MENU)
}

/// <https://invisible-island.net/ncurses/man/menu_cursor.3x.html>
pub unsafe fn pos_menu_cursor(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "{}pos_menu_cursor() : menu.is_null()", MODULE_PATH);

    bindings::pos_menu_cursor(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_post.3x.html>
pub unsafe fn post_menu(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "{}post_menu() : menu.is_null()", MODULE_PATH);

    bindings::post_menu(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn scale_menu(menu: MENU, rows: *mut i32, cols: *mut i32) -> i32 {
    assert!(!menu.is_null(), "{}scale_menu() : menu.is_null()", MODULE_PATH);
    assert!(!rows.is_null(), "{}scale_menu() : rows.is_null()", MODULE_PATH);
    assert!(!cols.is_null(), "{}scale_menu() : cols.is_null()", MODULE_PATH);

    bindings::scale_menu(menu, rows, cols)
}

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn set_current_item(menu: MENU, item: ITEM) -> i32 {
    assert!(!menu.is_null(), "{}set_current_item() : menu.is_null()", MODULE_PATH);
    assert!(!item.is_null(), "{}set_current_item() : item.is_null()", MODULE_PATH);

    bindings::set_current_item(menu, item)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn set_item_init(menu: Option<MENU>, hook: Menu_Hook) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_item_init() : menu.is_null()", MODULE_PATH);

    bindings::set_item_init(return_mut_ptr!(menu), hook)
}

/// <https://invisible-island.net/ncurses/man/mitem_opts.3x.html>
pub unsafe fn set_item_opts(item: Option<ITEM>, opts: i32) -> i32 {
    assert!(item.map_or_else(|| true, |item| !item.is_null()), "{}set_item_opts() : item.is_null()", MODULE_PATH);

    bindings::set_item_opts(return_mut_ptr!(item), opts)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn set_item_term(menu: Option<MENU>, hook: Menu_Hook) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_item_term() : menu.is_null()", MODULE_PATH);

    bindings::set_item_term(return_mut_ptr!(menu), hook)
}

/// <https://invisible-island.net/ncurses/man/mitem_userptr.3x.html>
pub unsafe fn set_item_userptr(item: Option<ITEM>, userptr: Option<MENU_USERPTR>) -> i32 {
    assert!(item.map_or_else(|| true, |item| !item.is_null()), "{}set_item_userptr() : item.is_null()", MODULE_PATH);
    assert!(userptr.map_or_else(|| true, |userptr| !userptr.is_null()), "{}set_item_userptr() : userptr.is_null()", MODULE_PATH);

    bindings::set_item_userptr(return_mut_ptr!(item), return_mut_ptr!(userptr))
}

/// <https://invisible-island.net/ncurses/man/mitem_value.3x.html>
pub unsafe fn set_item_value(item: ITEM, value: bool) -> i32 {
    assert!(!item.is_null(), "{}set_item_value() : item.is_null()", MODULE_PATH);

    bindings::set_item_value(item, value)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn set_menu_back(menu: Option<MENU>, attr: chtype) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_back() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_back(return_mut_ptr!(menu), attr)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn set_menu_fore(menu: Option<MENU>, attr: chtype) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_fore() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_fore(return_mut_ptr!(menu), attr)
}

/// <https://invisible-island.net/ncurses/man/menu_format.3x.html>
pub unsafe fn set_menu_format(menu: Option<MENU>, rows: i32, cols: i32) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_format() : menu.is_null()", MODULE_PATH);
    assert!(rows >= 0, "{}set_menu_format() : rows = {}", MODULE_PATH, rows);
    assert!(cols >= 0, "{}set_menu_format() : cols = {}", MODULE_PATH, cols);

    bindings::set_menu_format(return_mut_ptr!(menu), rows, cols)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn set_menu_grey(menu: Option<MENU>, attr: chtype) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_grey() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_grey(return_mut_ptr!(menu), attr)
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn set_menu_init(menu: Option<MENU>, hook: Menu_Hook) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_init() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_init(return_mut_ptr!(menu), hook)
}

/// <https://invisible-island.net/ncurses/man/menu_items.3x.html>
pub unsafe fn set_menu_items(menu: MENU, item_handles: *mut ITEM) -> i32 {
    assert!(!menu.is_null(), "{}set_menu_items() : menu.is_null()", MODULE_PATH);
    assert!(!item_handles.is_null(), "{}set_menu_items() : item_handles.is_null()", MODULE_PATH);

    bindings::set_menu_items(menu, item_handles)
}

/// <https://invisible-island.net/ncurses/man/menu_mark.3x.html>
pub unsafe fn set_menu_mark(menu: Option<MENU>, mark: &[i8]) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_mark() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_mark(return_mut_ptr!(menu), mark.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/menu_opts.3x.html>
pub unsafe fn set_menu_opts(menu: Option<MENU>, opts: i32) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_opts() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_opts(return_mut_ptr!(menu), opts)
}

/// <https://invisible-island.net/ncurses/man/menu_attributes.3x.html>
pub unsafe fn set_menu_pad(menu: Option<MENU>, opts: i32) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_pad() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_pad(return_mut_ptr!(menu), opts)
}

/// <https://invisible-island.net/ncurses/man/menu_pattern.3x.html>
pub unsafe fn set_menu_pattern(menu: MENU, pattern: &[i8]) -> i32 {
    assert!(!menu.is_null(), "{}set_menu_pattern() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_pattern(menu, pattern.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/menu_spacing.3x.html>
pub unsafe fn set_menu_spacing(
    menu:            Option<MENU>,
    spc_description: i32,
    spc_rows:        i32,
    spc_columns:     i32
) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_spacing() : menu.is_null()", MODULE_PATH);
    assert!(spc_description >= 0, "{}set_menu_spacing() : spc_description = {}", MODULE_PATH, spc_description);
    assert!(spc_rows >= 0, "{}set_menu_spacing() : spc_rows = {}", MODULE_PATH, spc_rows);
    assert!(spc_columns >= 0, "{}set_menu_spacing() : spc_columns = {}", MODULE_PATH, spc_columns);

    bindings::set_menu_spacing(return_mut_ptr!(menu), spc_description, spc_rows, spc_columns)
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn set_menu_sub(menu: Option<MENU>, sub: Option<WINDOW>) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_sub() : menu.is_null()", MODULE_PATH);
    assert!(sub.map_or_else(|| true, |sub| !sub.is_null()), "{}set_menu_sub() : sub.is_null()", MODULE_PATH);

    bindings::set_menu_sub(return_mut_ptr!(menu), return_mut_ptr!(sub))
}

/// <https://invisible-island.net/ncurses/man/menu_hook.3x.html>
pub unsafe fn set_menu_term(menu: Option<MENU>, hook: Menu_Hook) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_term() : menu.is_null()", MODULE_PATH);

    bindings::set_menu_term(return_mut_ptr!(menu), hook)
}

/// <https://invisible-island.net/ncurses/man/menu_userptr.3x.html>
pub unsafe fn set_menu_userptr(menu: Option<MENU>, userptr: Option<MENU_USERPTR>) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_userptr() : menu.is_null()", MODULE_PATH);
    assert!(userptr.map_or_else(|| true, |userptr| !userptr.is_null()), "{}set_menu_userptr() : userptr.is_null()", MODULE_PATH);

    bindings::set_menu_userptr(return_mut_ptr!(menu), return_mut_ptr!(userptr))
}

/// <https://invisible-island.net/ncurses/man/menu_win.3x.html>
pub unsafe fn set_menu_win(menu: Option<MENU>, win: Option<WINDOW>) -> i32 {
    assert!(menu.map_or_else(|| true, |menu| !menu.is_null()), "{}set_menu_win() : menu.is_null()", MODULE_PATH);
    assert!(win.map_or_else(|| true, |win| !win.is_null()), "{}set_menu_win() : win.is_null()", MODULE_PATH);

    bindings::set_menu_win(return_mut_ptr!(menu), return_mut_ptr!(win))
}

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn set_top_row(menu: MENU, row: i32) -> i32 {
    assert!(!menu.is_null(), "{}set_top_row() : menu.is_null()", MODULE_PATH);
    assert!(row >= 0, "{}set_top_row() : row = {}", MODULE_PATH, row);

    bindings::set_top_row(menu, row)
}

/// <https://invisible-island.net/ncurses/man/mitem_current.3x.html>
pub unsafe fn top_row(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "{}top_row() : menu.is_null()", MODULE_PATH);

    bindings::top_row(menu)
}

/// <https://invisible-island.net/ncurses/man/menu_post.3x.html>
pub unsafe fn unpost_menu(menu: MENU) -> i32 {
    assert!(!menu.is_null(), "{}unpost_menu() : menu.is_null()", MODULE_PATH);

    bindings::unpost_menu(menu)
}

/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn new_menu_sp(sp: SCREEN, item_handles: *mut ITEM) -> Option<MENU> {
    assert!(!sp.is_null(), "{}new_menu_sp() : sp.is_null()", MODULE_PATH);
    assert!(!item_handles.is_null(), "{}new_menu_sp() : item_handles.is_null()", MODULE_PATH);

    bindings::new_menu_sp(sp, item_handles).as_mut().map(|ptr| ptr as MENU)
}
