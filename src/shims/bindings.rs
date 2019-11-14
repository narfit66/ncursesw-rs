/*
    src/shims/bindings.rs

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

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use std::os::raw::{c_short, c_int};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

type _MENU = *mut tagMENU;

pub type RipoffInit = extern "C" fn(*mut WINDOW, i32) -> i32;
pub type MenuHook = extern "C" fn(_MENU);

// ncurses core functions.
extern "C" {
    pub fn getcchar(_: *const cchar_t, _: *mut wchar_t, _: *mut attr_t, _: *mut c_short, _: *mut c_int) -> c_int;
    pub fn ripoffline(_: c_int, _: RipoffInit) -> c_int;
}

// ncurses menu functions.
extern "C" {
    pub fn item_init(_: _MENU) -> MenuHook;
    pub fn item_term(_: _MENU) -> MenuHook;
    pub fn menu_init(_: _MENU) -> MenuHook;
    pub fn menu_term(_: _MENU) -> MenuHook;
    pub fn set_item_init(_: _MENU, _: MenuHook) -> c_int;
    pub fn set_item_term(_: _MENU, _: MenuHook) -> c_int;
    pub fn set_menu_init(_: _MENU, _: MenuHook) -> c_int;
    pub fn set_menu_term(_: _MENU, _: MenuHook) -> c_int;
}
