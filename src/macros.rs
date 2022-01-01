/*
    src/macros.rs

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

#![macro_use]

macro_rules! result { ($type: ty) => { Result<$type, NCurseswError> } }
macro_rules! panels_result { ($type: ty) => { Result<$type, NCurseswPanelsError> } }
macro_rules! mouse_result { ($type: ty) => { Result<$type, NCurseswMouseError> } }
macro_rules! menu_result { ($type: ty) => { Result<$type, NCurseswMenuError> } }
macro_rules! form_result { ($type: ty) => { Result<$type, NCurseswFormError> } }

macro_rules! ncurses_function_error { ($func: expr) => { NCurseswError::LibraryError { func: String::from($func), rc: None } } }
macro_rules! ncurses_function_error_with_rc { ($func: expr, $rc: expr) => { NCurseswError::LibraryError { func: String::from($func), rc: Some($rc) } } }
macro_rules! ncurses_os_error { ($func: expr) => { NCurseswError::OSError { func: String::from($func), errno: errno::errno() } } }

macro_rules! panels_function_error { ($func: expr) => { NCurseswPanelsError::LibraryError { func: String::from($func), rc: None } } }
macro_rules! panels_function_error_with_rc { ($func: expr, $rc: expr) => { NCurseswPanelsError::LibraryError { func: String::from($func), rc: Some($rc) } } }

macro_rules! mouse_function_error { ($func: expr) => { NCurseswMouseError::LibraryError { func: String::from($func), rc: None } } }
macro_rules! mouse_function_error_with_rc { ($func: expr, $rc: expr) => { NCurseswMouseError::LibraryError { func: String::from($func), rc: Some($rc) } } }

macro_rules! menu_function_error { ($func: expr) => { ncursesw_menu_error_system_error($func) } }
macro_rules! menu_function_error_with_rc { ($func: expr, $rc: expr) => { ncursesw_menu_error_from_rc($func, $rc) } }
macro_rules! menu_function_unknown_error { ($func: expr, $rc: expr) => { NCurseswMenuError::UnknownError { func: String::from($func), errno: $rc } } }

macro_rules! form_function_error { ($func: expr) => { ncursesw_form_error_system_error($func) } }
macro_rules! form_function_error_with_rc { ($func: expr, $rc: expr) => { ncursesw_form_error_from_rc($func, $rc) } }
macro_rules! form_function_unknown_error { ($func: expr, $rc: expr) => { NCurseswFormError::UnknownError { func: String::from($func), errno: $rc } } }

macro_rules! wrap_const { ($name: ident : $type: ty) => { pub const $name: $type = bindings::$name as $type; } }

macro_rules! c_str_with_nul { ($name: ident) => { &*($name.to_c_str()?.as_bytes_with_nul() as *const [u8] as *const [i8]) } }
macro_rules! raw_with_nul_as_slice { ($name: ident) => { $name.clone().raw_with_nul().as_slice() } }

macro_rules! ptr_as_string { ($ptr: ident) => { std::str::from_utf8_unchecked(CStr::from_ptr($ptr).to_bytes()).to_owned() } }
macro_rules! option_str_as_ptr { ($name: ident) => { $name.map_or_else(std::ptr::null, |name| name.as_ptr()) as *const i8 } }

macro_rules! return_mut_ptr { ($ptr: ident) => { $ptr.unwrap_or(std::ptr::null_mut()) } }

macro_rules! option_getter {
    ($func: ident, $attr: ident) => {
        pub fn $func(&self) -> bool {
            (self.raw & constants::$attr) > 0
        }
    };
}

macro_rules! option_setter {
    ($func: ident, $attr: ident) => {
        pub fn $func(&self, enabled: bool) -> Self {
            Self { raw: if enabled {
                self.raw | constants::$attr
            } else {
                self.raw ^ constants::$attr
            }}
        }
    };
}
