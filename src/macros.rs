/*
    src/macros.rs

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

#![macro_use]

macro_rules! result { ($t: ty) => { Result<$t, NCurseswError> } }

macro_rules! simple_ncurses_function {
    ($f: ident) => {
        pub fn $f() {
            ncurses::$f()
        }
    }
}

macro_rules! simple_ncurses_function_with_window_returns_bool {
    ($f: ident) => {
        pub fn $f(handle: WINDOW) -> bool {
            unsafe {
                ncurses::$f(handle)
            }
        }
    }
}

macro_rules! basic_ncurses_function {
    ($f: ident, $n: expr) => {
        pub fn $f() -> result!(()) {
            match ncurses::$f() {
                OK => Ok(()),
                rc => Err(ncurses_function_error_with_rc!($n, rc))
            }
        }
    }
}

macro_rules! basic_ncurses_function_with_window {
    ($f: ident, $n: expr) => {
        pub fn $f(handle: WINDOW) -> result!(()) {
            match unsafe { ncurses::$f(handle) } {
                OK => Ok(()),
                rc => Err(ncurses_function_error_with_rc!($n, rc))
            }
        }
    }
}

macro_rules! ncurses_function_error { ($func: expr) => { NCurseswError::NCursesFunction { func: String::from($func), rc: ERR } } }
macro_rules! ncurses_function_error_with_rc { ($func: expr, $rc: expr) => { NCurseswError::NCursesFunction { func: String::from($func), rc: $rc } } }
macro_rules! panels_function_error { ($func: expr) => { NCurseswError::PanelsFunction { func: String::from($func), rc: ERR } } }
macro_rules! panels_function_error_with_rc { ($func: expr, $rc: expr) => { NCurseswError::PanelsFunction { func: String::from($func), rc: $rc } } }
macro_rules! mouse_function_error { ($func: expr) => { NCurseswError::MouseFunction { func: String::from($func), rc: ERR } } }
macro_rules! mouse_function_error_with_rc { ($func: expr, $rc: expr) => { NCurseswError::MouseFunction { func: String::from($func), rc: $rc } } }

macro_rules! wrap_const { ($name: ident : $type: ty) => { pub const $name: $type = bindings::$name as $type; } }

macro_rules! c_str_with_nul { ($name: ident) => { &*($name.to_c_str().as_bytes_with_nul() as *const [u8] as *const [i8]) } }

macro_rules! raw_with_nul_as_slice { ($name: ident) => { $name.clone().raw_with_nul().as_slice() } }

macro_rules! return_optional_mut_ptr {
    ($ptr: ident) => {
        if $ptr.is_null() {
            None
        } else {
            Some($ptr)
        }
    }
}

macro_rules! return_mut_ptr {
    ($ptr: ident) => {
        match $ptr {
            Some(ptr) => ptr,
            None      => std::ptr::null_mut()
        }
    }
}
