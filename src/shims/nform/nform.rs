/*
    src/shims/nform/nform.rs

    Copyright (c) 2019-2021 Stephen Whittle  All rights reserved.

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

// See <https://invisible-island.net/ncurses/man/form.3x.html>

use std::{mem, slice, ptr};
use crate::{
    shims::{
        bindings,
        bindings::{
            chtype, wchar_t, va_list, TYPE_ALNUM, TYPE_ALPHA, TYPE_ENUM,
            TYPE_INTEGER, TYPE_NUMERIC, TYPE_REGEXP, TYPE_IPV4
        },
        ncurses::{SCREEN, WINDOW}
    },
    cstring::FromCStr,
    nform::fieldtype::FieldType
};

pub type FIELD = *mut bindings::FIELD;
pub type FIELDTYPE = *mut bindings::FIELDTYPE;
pub type FORM = *mut bindings::FORM;

pub use crate::bindings::Form_Hook;

static MODULE_PATH: &str = "ncursesw::shims::nform::";

/// <https://invisible-island.net/ncurses/man/form_page.3x.html>
pub unsafe fn current_field(form: Option<FORM>) -> Option<FIELD> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}current_field() : form.is_null()", MODULE_PATH);

    bindings::current_field(return_mut_ptr!(form)).as_mut().map(|ptr| ptr as FIELD)
}

/// <https://invisible-island.net/ncurses/man/form_data.3x.html>
pub unsafe fn data_ahead(form: FORM) -> bool {
    assert!(!form.is_null(), "{}data_ahead() : form.is_null()", MODULE_PATH);

    bindings::data_ahead(form)
}

/// <https://invisible-island.net/ncurses/man/form_data.3x.html>
pub unsafe fn data_behind(form: FORM) -> bool {
    assert!(!form.is_null(), "{}data_behind() : form.is_null()", MODULE_PATH);

    bindings::data_behind(form)
}

/// <https://invisible-island.net/ncurses/man/form_field_new.3x.html>
pub unsafe fn dup_field(field: FIELD, toprow: i32, leftcol: i32) -> Option<FIELD> {
    assert!(!field.is_null(), "{}dup_field() : field.is_null()", MODULE_PATH);
    assert!(toprow >= 0, "{}dup_field() : top_row: {}", MODULE_PATH, toprow);
    assert!(leftcol >= 0, "{}dup_field() : leftcol: {}", MODULE_PATH, leftcol);

    bindings::dup_field(field, toprow, leftcol).as_mut().map(|ptr| ptr as FIELD)
}

/// <https://invisible-island.net/ncurses/man/form_field_info.3x.html>
pub unsafe fn dynamic_field_info(field: FIELD, rows: *mut i32, cols: *mut i32, max: *mut i32) -> i32 {
    assert!(!field.is_null(), "{}dynamic_field_info() : field.is_null()", MODULE_PATH);
    assert!(!rows.is_null(), "{}dynamic_field_info() : rows.is_null()", MODULE_PATH);
    assert!(!cols.is_null(), "{}dynamic_field_info() : cols.is_null()", MODULE_PATH);
    assert!(!max.is_null(), "{}dynamic_field_info() : max.is_null()", MODULE_PATH);

    bindings::dynamic_field_info(field, rows, cols, max)
}

/// <https://invisible-island.net/ncurses/man/form_field_validation.3x.html>
pub unsafe fn field_arg(field: Option<FIELD>) -> Option<*mut libc::c_void> {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_arg() : field.is_null()", MODULE_PATH);

    bindings::field_arg(return_mut_ptr!(field)).as_mut().map(|ptr| ptr as *mut libc::c_void)
}

/// <https://invisible-island.net/ncurses/man/form_field_attributes.3x.html>
pub unsafe fn field_back(field: Option<FIELD>) -> chtype {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_back() : field.is_null()", MODULE_PATH);

    bindings::field_back(return_mut_ptr!(field))
}

/// <https://invisible-island.net/ncurses/man/form_field_buffer.3x.html>
pub unsafe fn field_buffer(field: FIELD, buf: i32) -> Option<Vec<i8>> {
    assert!(!field.is_null(), "{}field_buffer() : field.is_null()", MODULE_PATH);
    assert!(buf >= 0, "{}field_buffer() : buf = {}", MODULE_PATH, buf);

    let ptr = bindings::field_buffer(field, buf);

    if !ptr.is_null() {
        let mut buffer = vec!();
        let mut offset = 0;

        let mut byte = ptr::read(ptr);

        while byte != 0 {
            buffer.push(byte);

            offset += 1;

            byte = ptr::read(ptr.offset(offset));
        }

        Some(buffer)
    } else {
        None
    }
}

/// <https://invisible-island.net/ncurses/man/form_field.3x.html>
pub unsafe fn field_count(form: Option<FORM>) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}field_count() : form.is_null()", MODULE_PATH);

    bindings::field_count(return_mut_ptr!(form))
}

/// <https://invisible-island.net/ncurses/man/form_field_attributes.3x.html>
pub unsafe fn field_fore(field: Option<FIELD>) -> chtype {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_fore() : field.is_null()", MODULE_PATH);

    bindings::field_fore(return_mut_ptr!(field))
}

/// <https://invisible-island.net/ncurses/man/form_page.3x.html>
pub unsafe fn field_index(field: FIELD) -> i32 {
    assert!(!field.is_null(), "{}field_index() : field.is_null()", MODULE_PATH);

    bindings::field_index(field)
}

/// <https://invisible-island.net/ncurses/man/form_field_info.3x.html>
pub unsafe fn field_info(field: FIELD, rows: *mut i32, cols: *mut i32, frow: *mut i32, fcol: *mut i32, nrow: *mut i32, nbuf: *mut i32) -> i32 {
    assert!(!field.is_null(), "{}field_info() : field.is_null()", MODULE_PATH);
    assert!(!rows.is_null(), "{}field_info() : rows.is_null()", MODULE_PATH);
    assert!(!cols.is_null(), "{}field_info() : cols.is_null()", MODULE_PATH);
    assert!(!frow.is_null(), "{}field_info() : frow.is_null()", MODULE_PATH);
    assert!(!fcol.is_null(), "{}field_info() : fcol.is_null()", MODULE_PATH);
    assert!(!nrow.is_null(), "{}field_info() : nrow.is_null()", MODULE_PATH);
    assert!(!nbuf.is_null(), "{}field_info() : nbuf.is_null()", MODULE_PATH);

    bindings::field_info(field, rows, cols, frow, fcol, nrow, nbuf)
}

/// <https://invisible-island.net/ncurses/man/form_hook.3x.html>
pub unsafe fn field_init(form: Option<FORM>) -> Option<Form_Hook> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}field_init() : form.is_null()", MODULE_PATH);

    (bindings::field_init(return_mut_ptr!(form)) as Form_Hook).as_mut().map(|ptr| mem::transmute(ptr))
}

/// <https://invisible-island.net/ncurses/man/form_field_just.3x.html>
pub unsafe fn field_just(field: Option<FIELD>) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_just() : field.is_null()", MODULE_PATH);

    bindings::field_just(return_mut_ptr!(field))
}

/// <https://invisible-island.net/ncurses/man/form_field_opts.3x.html>
pub unsafe fn field_opts(field: Option<FIELD>) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_opts() : field.is_null()", MODULE_PATH);

    bindings::field_opts(return_mut_ptr!(field))
}

/// <https://invisible-island.net/ncurses/man/form_field_opts.3x.html>
pub unsafe fn field_opts_off(field: Option<FIELD>, opts: i32) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_opts_off() : field.is_null()", MODULE_PATH);

    bindings::field_opts_off(return_mut_ptr!(field), opts)
}

/// <https://invisible-island.net/ncurses/man/form_field_opts.3x.html>
pub unsafe fn field_opts_on(field: Option<FIELD>, opts: i32) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_opts_on() : field.is_null()", MODULE_PATH);

    bindings::field_opts_on(return_mut_ptr!(field), opts)
}

/// <https://invisible-island.net/ncurses/man/form_field_attributes.3x.html>
pub unsafe fn field_pad(field: Option<FIELD>) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_pad() : field.is_null()", MODULE_PATH);

    bindings::field_pad(return_mut_ptr!(field))
}

/// <https://invisible-island.net/ncurses/man/form_field_buffer.3x.html>
pub unsafe fn field_status(field: Option<FIELD>) -> bool {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_status() : field.is_null()", MODULE_PATH);

    bindings::field_status(return_mut_ptr!(field))
}

/// <https://invisible-island.net/ncurses/man/form_hook.3x.html>
pub unsafe fn field_term(form: Option<FORM>) -> Option<Form_Hook> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}field_term() : form.is_null()", MODULE_PATH);

    (bindings::field_term(return_mut_ptr!(form)) as Form_Hook).as_mut().map(|ptr| mem::transmute(ptr))
}

/// <https://invisible-island.net/ncurses/man/form_field_validation.3x.html>
pub unsafe fn field_type(field: Option<FIELD>) -> Option<FIELDTYPE> {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_type() : field.is_null()", MODULE_PATH);

    bindings::field_type(return_mut_ptr!(field)).as_mut().map(|ptr| ptr as FIELDTYPE)
}

/// <https://invisible-island.net/ncurses/man/form_field_userptr.3x.html>
pub unsafe fn field_userptr(field: Option<FIELD>) -> Option<*mut libc::c_void> {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}field_userptr() : field.is_null()", MODULE_PATH);

    bindings::field_userptr(return_mut_ptr!(field)).as_mut().map(|ptr| ptr as *mut libc::c_void)
}

/// <https://invisible-island.net/ncurses/man/form_driver.3x.html>
pub unsafe fn form_driver(form: FORM, c: i32) -> i32 {
    assert!(!form.is_null(), "{}form_driver() : form.is_null()", MODULE_PATH);

    bindings::form_driver(form, c)
}

/// <https://invisible-island.net/ncurses/man/form_driver.3x.html>
pub unsafe fn form_driver_w(form: FORM, c: i32, wch: wchar_t) -> i32 {
    assert!(!form.is_null(), "{}form_driver_w() : form.is_null()", MODULE_PATH);

    bindings::form_driver_w(form, c, wch)
}

/// <https://invisible-island.net/ncurses/man/form_field_buffer.3x.html>
pub unsafe fn form_fields(form: Option<FORM>) -> Option<Vec<FIELD>> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_fields() : form.is_null()", MODULE_PATH);

    bindings::form_fields(return_mut_ptr!(form))
        .as_mut()
        .map(|ptr| slice::from_raw_parts(ptr, bindings::field_count(return_mut_ptr!(form)) as usize).to_vec())
}

/// <https://invisible-island.net/ncurses/man/form_hook.3x.html>
pub unsafe fn form_init(form: Option<FORM>) -> Option<Form_Hook> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_init() : form.is_null()", MODULE_PATH);

    (bindings::form_init(return_mut_ptr!(form)) as Form_Hook).as_mut().map(|ptr| mem::transmute(ptr))
}

/// <https://invisible-island.net/ncurses/man/form_opts.3x.html>
pub unsafe fn form_opts(form: Option<FORM>) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_opts() : form.is_null()", MODULE_PATH);

    bindings::form_opts(return_mut_ptr!(form))
}

/// <https://invisible-island.net/ncurses/man/form_opts.3x.html>
pub unsafe fn form_opts_off(form: Option<FORM>, opts: i32) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_opts_off() : form.is_null()", MODULE_PATH);

    bindings::form_opts_off(return_mut_ptr!(form), opts)
}

/// <https://invisible-island.net/ncurses/man/form_opts.3x.html>
pub unsafe fn form_opts_on(form: Option<FORM>, opts: i32) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_opts_on() : form.is_null()", MODULE_PATH);

    bindings::form_opts_on(return_mut_ptr!(form), opts)
}

/// <https://invisible-island.net/ncurses/man/form_page.3x.html>
pub unsafe fn form_page(form: Option<FORM>) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_page() : form.is_null()", MODULE_PATH);

    bindings::form_page(return_mut_ptr!(form))
}

/// <https://invisible-island.net/ncurses/man/form_requestname.3x.html>
pub fn form_request_by_name(name: &[i8]) -> i32 {
    unsafe { bindings::form_request_by_name(name.as_ptr()) }
}

/// <https://invisible-island.net/ncurses/man/form_requestname.3x.html>
pub fn form_request_name(request: i32) -> Option<String> {
    unsafe { (bindings::form_request_name(request) as *mut i8).as_mut().map(|ptr| FromCStr::from_c_str(ptr)) }
}

/// <https://invisible-island.net/ncurses/man/form_win.3x.html>
pub unsafe fn form_sub(form: Option<FORM>) -> Option<WINDOW> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_sub() : form.is_null()", MODULE_PATH);

    bindings::form_sub(return_mut_ptr!(form)).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/form_hook.3x.html>
pub unsafe fn form_term(form: Option<FORM>) -> Option<Form_Hook> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_term() : form.is_null()", MODULE_PATH);

    (bindings::form_term(return_mut_ptr!(form)) as Form_Hook).as_mut().map(|ptr| mem::transmute(ptr))
}

/// <https://invisible-island.net/ncurses/man/form_userptr.3x.html>
pub unsafe fn form_userptr(form: Option<FORM>) -> Option<*mut libc::c_void> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_userptr() : form.is_null()", MODULE_PATH);

    bindings::form_userptr(return_mut_ptr!(form)).as_mut().map(|ptr| ptr as *mut libc::c_void)
}

/// <https://invisible-island.net/ncurses/man/form_win.3x.html>
pub unsafe fn form_win(form: Option<FORM>) -> Option<WINDOW> {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}form_win() : form.is_null()", MODULE_PATH);

    bindings::form_win(return_mut_ptr!(form)).as_mut().map(|ptr| ptr as WINDOW)
}

/// <https://invisible-island.net/ncurses/man/form_field_new.3x.html>
pub unsafe fn free_field(field: FIELD) -> i32 {
    assert!(!field.is_null(), "{}free_field() : field.is_null()", MODULE_PATH);

    bindings::free_field(field)
}

/// <https://invisible-island.net/ncurses/man/form_fieldtype.3x.html>
pub unsafe fn free_fieldtype(fieldtype: FIELDTYPE) -> i32 {
    assert!(!fieldtype.is_null(), "{}free_fieldtype() : fieldtype.is_null()", MODULE_PATH);

    bindings::free_fieldtype(fieldtype)
}

/// <https://invisible-island.net/ncurses/man/form_new.3x.html>
pub unsafe fn free_form(form: FORM) -> i32 {
    assert!(!form.is_null(), "{}free_form() : form.is_null()", MODULE_PATH);

    bindings::free_form(form)
}

/// <https://invisible-island.net/ncurses/man/form_field_new.3x.html>
pub unsafe fn link_field(field: FIELD, toprow: i32, leftcol: i32) -> Option<FIELD> {
    assert!(!field.is_null(), "{}link_field() : field.is_null()", MODULE_PATH);
    assert!(toprow >= 0, "{}link_field() : toprow = {}", MODULE_PATH, toprow);
    assert!(leftcol >= 0, "{}link_field() : leftcol = {}", MODULE_PATH, leftcol);

    bindings::link_field(field, toprow, leftcol).as_mut().map(|ptr| ptr as FIELD)
}

/// <https://invisible-island.net/ncurses/man/form_fieldtype.3x.html>
pub unsafe fn link_fieldtype(type1: FIELDTYPE, type2: FIELDTYPE) -> Option<FIELDTYPE> {
    assert!(!type1.is_null(), "{}link_fieldtype() : type1.is_null()", MODULE_PATH);
    assert!(!type2.is_null(), "{}link_fieldtype() : type2.is_null()", MODULE_PATH);

    bindings::link_fieldtype(type1, type2).as_mut().map(|ptr| ptr as FIELDTYPE)
}

/// <https://invisible-island.net/ncurses/man/form_field_buffer.3x.html>
pub unsafe fn move_field(field: FIELD, frow: i32, fcol: i32) -> i32 {
    assert!(!field.is_null(), "{}move_field() : field.is_null()", MODULE_PATH);
    assert!(frow >= 0, "{}move_field() : frow = {}", MODULE_PATH, frow);
    assert!(fcol >= 0, "{}move_field() : fcol = {}", MODULE_PATH, fcol);

    bindings::move_field(field, frow, fcol)
}

/// <https://invisible-island.net/ncurses/man/form_field_new.3x.html>
pub unsafe fn new_field(height: i32, width: i32, toprow: i32, leftcol: i32, offscreen: i32, nbuffers: i32) -> Option<FIELD> {
    assert!(height >= 0, "{}new_field() : height = {}", MODULE_PATH, height);
    assert!(width >= 0, "{}new_field() : width = {}", MODULE_PATH, width);
    assert!(toprow >= 0, "{}new_field() : toprow = {}", MODULE_PATH, toprow);
    assert!(leftcol >= 0, "{}new_field() : leftcol = {}", MODULE_PATH, leftcol);
    assert!(offscreen >= 0, "{}new_field() : offscreen = {}", MODULE_PATH, offscreen);
    assert!(nbuffers >= 0, "{}new_field() : nbuffers = {}", MODULE_PATH, nbuffers);

    bindings::new_field(height, width, toprow, leftcol, offscreen, nbuffers).as_mut().map(|ptr| ptr as FIELD)
}

/// <https://invisible-island.net/ncurses/man/form_fieldtype.3x.html>
pub unsafe fn new_fieldtype(
    field_check: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool,
    char_check:  unsafe extern "C" fn(_: i32, _: *const libc::c_void) -> bool
) -> Option<FIELDTYPE>
{
    bindings::new_fieldtype(Some(field_check), Some(char_check)).as_mut().map(|ptr| ptr as FIELDTYPE)
}

/// <https://invisible-island.net/ncurses/man/new_form.3x.html>
pub unsafe fn new_form(fields: *mut FIELD) -> Option<FORM> {
    assert!(!fields.is_null(), "{}new_form() : fields.is_null()", MODULE_PATH);

    bindings::new_form(fields).as_mut().map(|ptr| ptr as FORM)
}

/// <https://invisible-island.net/ncurses/man/form_new_page.3x.html>
pub unsafe fn new_page(field: Option<FIELD>) -> bool {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}new_page() : field.is_null()", MODULE_PATH);

    bindings::new_page(return_mut_ptr!(field))
}

/// <https://invisible-island.net/ncurses/man/form_cursor.3x.html>
pub unsafe fn pos_form_cursor(form: FORM) -> i32 {
    assert!(!form.is_null(), "{}pos_form_cursor() : form.is_null()", MODULE_PATH);

    bindings::pos_form_cursor(form)
}

/// <https://invisible-island.net/ncurses/man/form_post.3x.html>
pub unsafe fn post_form(form: FORM) -> i32 {
    assert!(!form.is_null(), "{}post_form() : form.is_null()", MODULE_PATH);

    bindings::post_form(form)
}

/// <https://invisible-island.net/ncurses/man/form_win.3x.html>
pub unsafe fn scale_form(form: FORM, rows: *mut i32, columns: *mut i32) -> i32 {
    assert!(!form.is_null(), "{}scale_form() : form.is_null()", MODULE_PATH);
    assert!(!rows.is_null(), "{}scale_form() : rows.is_null()", MODULE_PATH);
    assert!(!columns.is_null(), "{}scale_form() : columns.is_null()", MODULE_PATH);

    bindings::scale_form(form, rows, columns)
}

/// <https://invisible-island.net/ncurses/man/form_page.3x.html>
pub unsafe fn set_current_field(form: FORM, field: FIELD) -> i32 {
    assert!(!form.is_null(), "{}set_current_field() : form.is_null()", MODULE_PATH);
    assert!(!field.is_null(), "{}set_current_field() : field.is_null()", MODULE_PATH);

    bindings::set_current_field(form, field)
}

/// <https://invisible-island.net/ncurses/man/form_field_attributes.3x.html>
pub unsafe fn set_field_back(field: Option<FIELD>, attr: chtype) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_field_back() : field.is_null()", MODULE_PATH);

    bindings::set_field_back(return_mut_ptr!(field), attr)
}

/// <https://invisible-island.net/ncurses/man/form_field_buffer.3x.html>
pub unsafe fn set_field_buffer(field: FIELD, buf: i32, value: &[i8]) -> i32 {
    assert!(!field.is_null(), "{}set_field_buffer() : field.is_null()", MODULE_PATH);
    assert!(buf >= 0, "{}set_field_buffer() : buf = {}", MODULE_PATH, buf);

    bindings::set_field_buffer(field, buf, value.as_ptr())
}

/// <https://invisible-island.net/ncurses/man/form_field_attributes.3x.html>
pub unsafe fn set_field_fore(field: Option<FIELD>, attr: chtype) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_field_fore() : field.is_null()", MODULE_PATH);

    bindings::set_field_fore(return_mut_ptr!(field), attr)
}

/// <https://invisible-island.net/ncurses/man/form_hook.3x.html>
pub unsafe fn set_field_init(form: Option<FORM>, func: Form_Hook) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}set_field_init() : form.is_null()", MODULE_PATH);

    bindings::set_field_init(return_mut_ptr!(form), func)
}

/// <https://invisible-island.net/ncurses/man/form_field_just.3x.html>
pub unsafe fn set_field_just(field: Option<FIELD>, justification: i32) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_field_just() : field.is_null()", MODULE_PATH);
    assert!((0..=3).contains(&justification), "{}set_field_just() : justification = {}", MODULE_PATH, justification);

    bindings::set_field_just(return_mut_ptr!(field), justification)
}

/// <https://invisible-island.net/ncurses/man/form_field_opts.3x.html>
pub unsafe fn set_field_opts(field: Option<FIELD>, opts: i32) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_field_opts() : field.is_null()", MODULE_PATH);

    bindings::set_field_opts(return_mut_ptr!(field), opts)
}

/// <https://invisible-island.net/ncurses/man/form_field_attributes.3x.html>
pub unsafe fn set_field_pad(field: Option<FIELD>, pad: i32) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_field_pad() : field.is_null()", MODULE_PATH);

    bindings::set_field_pad(return_mut_ptr!(field), pad)
}

/// <https://invisible-island.net/ncurses/man/form_field_buffer.3x.html>
pub unsafe fn set_field_status(field: Option<FIELD>, status: bool) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_field_status() : field.is_null()", MODULE_PATH);

    bindings::set_field_status(return_mut_ptr!(field), status)
}

/// <https://invisible-island.net/ncurses/man/form_hook.3x.html>
pub unsafe fn set_field_term(form: Option<FORM>, func: Form_Hook) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}set_field_term() : form.is_null()", MODULE_PATH);

    bindings::set_field_term(return_mut_ptr!(form), func)
}

/// <https://invisible-island.net/ncurses/man/form_field_validation.3x.html>
pub unsafe fn set_field_type(field: Option<FIELD>, fieldtype: FieldType) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_field_type() : field.is_null()", MODULE_PATH);

    match fieldtype {
        FieldType::AlphaNumeric(width)                        => bindings::set_field_type(return_mut_ptr!(field), TYPE_ALNUM, width),
        FieldType::Alpha(width)                               => bindings::set_field_type(return_mut_ptr!(field), TYPE_ALPHA, width),
        FieldType::Enum(value_list, check_case, check_unique) => {
            assert!(!value_list.is_null(), "{}set_field_type() : value_list.is_null()", MODULE_PATH);

            bindings::set_field_type(return_mut_ptr!(field), TYPE_ENUM, value_list, i32::from(check_case), i32::from(check_unique))
        },
        FieldType::Integer(padding, minimum, maximum)         => bindings::set_field_type(return_mut_ptr!(field), TYPE_INTEGER, padding, minimum, maximum),
        FieldType::Numeric(padding, minimum, maximum)         => bindings::set_field_type(return_mut_ptr!(field), TYPE_NUMERIC, padding, minimum, maximum),
        FieldType::RegExp(regexp)                             => {
            assert!(!regexp.is_null(), "{}set_field_type() : regexp.is_null()", MODULE_PATH);

            bindings::set_field_type(return_mut_ptr!(field), TYPE_REGEXP, regexp)
        },
        FieldType::Ipv4                                       => bindings::set_field_type(return_mut_ptr!(field), TYPE_IPV4),
        FieldType::Custom(fieldtype, args)                    => {
            assert!(!fieldtype.is_null(), "{}set_field_type() : fieldtype.is_null()", MODULE_PATH);

            bindings::set_field_type(return_mut_ptr!(field), fieldtype, args)
        }
    }
}

/// <https://invisible-island.net/ncurses/man/form_fieldtype.3x.html>
pub unsafe fn set_fieldtype_arg(
    fieldtype: FIELDTYPE,
    make_arg: unsafe extern "C" fn(_: *mut va_list) -> *mut libc::c_void,
    copy_arg: Option<unsafe extern "C" fn(_: *const libc::c_void) -> *mut libc::c_void>,
    free_arg: Option<unsafe extern "C" fn(_: *mut libc::c_void)>
) -> i32
{
    assert!(!fieldtype.is_null(), "{}set_fieldtype_arg() : fieldtype.is_null()", MODULE_PATH);

    bindings::set_fieldtype_arg(fieldtype, Some(make_arg), copy_arg, free_arg)
}

/// <https://invisible-island.net/ncurses/man/form_fieldtype.3x.html>
pub unsafe fn set_fieldtype_choice(
    fieldtype: FIELDTYPE,
    next_choice: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool,
    prev_choice: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool
) -> i32
{
    assert!(!fieldtype.is_null(), "{}set_fieldtype_choice() : fieldtype.is_null()", MODULE_PATH);

    bindings::set_fieldtype_choice(fieldtype, Some(next_choice), Some(prev_choice))
}

/// <https://invisible-island.net/ncurses/man/form_field_userptr.3x.html>
pub unsafe fn set_field_userptr(field: Option<FIELD>, userptr: Option<*mut libc::c_void>) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_field_userptr() : field.is_null()", MODULE_PATH);
    assert!(userptr.map_or_else(|| true, |userptr| !userptr.is_null()), "{}set_field_userptr() : userptr.is_null()", MODULE_PATH);

    bindings::set_field_userptr(return_mut_ptr!(field), return_mut_ptr!(userptr))
}

/// <https://invisible-island.net/ncurses/man/form_field_buffer.3x.html>
pub unsafe fn set_form_fields(form: FORM, fields: *mut FIELD) -> i32 {
    assert!(!form.is_null(), "{}set_form_fields() : form.is_null()", MODULE_PATH);
    assert!(!fields.is_null(), "{}set_form_fields() : fields.is_null()", MODULE_PATH);

    bindings::set_form_fields(form, fields)
}

/// <https://invisible-island.net/ncurses/man/form_hook.3x.html>
pub unsafe fn set_form_init(form: Option<FORM>, func: Form_Hook) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}set_form_init() : form.is_null()", MODULE_PATH);

    bindings::set_form_init(return_mut_ptr!(form), func)
}

/// <https://invisible-island.net/ncurses/man/form_opts.3x.html>
pub unsafe fn set_form_opts(form: Option<FORM>, opts: i32) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}set_form_opts() : form.is_null()", MODULE_PATH);

    bindings::set_form_opts(return_mut_ptr!(form), opts)
}

/// <https://invisible-island.net/ncurses/man/form_page.3x.html>
pub unsafe fn set_form_page(form: FORM, n: i32) -> i32 {
    assert!(!form.is_null(), "{}set_form_page() : form.is_null()", MODULE_PATH);
    assert!(n >= 0, "{}set_form_page() : n = {}", MODULE_PATH, n);

    bindings::set_form_page(form, n)
}

/// <https://invisible-island.net/ncurses/man/form_win.3x.html>
pub unsafe fn set_form_sub(form: Option<FORM>, sub: Option<WINDOW>) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}set_form_sub() : form.is_null()", MODULE_PATH);
    assert!(sub.map_or_else(|| true, |sub| !sub.is_null()), "{}set_form_sub() : sub.is_null()", MODULE_PATH);

    bindings::set_form_sub(return_mut_ptr!(form), return_mut_ptr!(sub))
}

/// <https://invisible-island.net/ncurses/man/form_hook.3x.html>
pub unsafe fn set_form_term(form: Option<FORM>, func: Form_Hook) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}set_form_term() : form.is_null()", MODULE_PATH);

    bindings::set_form_init(return_mut_ptr!(form), func)
}

/// <https://invisible-island.net/ncurses/man/form_userptr.3x.html>
pub unsafe fn set_form_userptr(form: Option<FORM>, userptr: Option<*mut libc::c_void>) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}set_form_userptr() : form.is_null()", MODULE_PATH);
    assert!(userptr.map_or_else(|| true, |userptr| !userptr.is_null()), "{}set_form_userptr() : userptr.is_null()", MODULE_PATH);

    bindings::set_form_userptr(return_mut_ptr!(form), return_mut_ptr!(userptr))
}

/// <https://invisible-island.net/ncurses/man/form_win.3x.html>
pub unsafe fn set_form_win(form: Option<FORM>, win: Option<WINDOW>) -> i32 {
    assert!(form.map_or_else(|| true, |form| !form.is_null()), "{}set_form_win() : form.is_null()", MODULE_PATH);
    assert!(win.map_or_else(|| true, |win| !win.is_null()), "{}set_form_win() : win.is_null()", MODULE_PATH);

    bindings::set_form_win(return_mut_ptr!(form), return_mut_ptr!(win))
}

/// <https://invisible-island.net/ncurses/man/form_field_buffer.3x.html>
pub unsafe fn set_max_field(field: FIELD, max: i32) -> i32 {
    assert!(!field.is_null(), "{}set_max_field() : field.is_null()", MODULE_PATH);
    assert!(max >= 0, "{}set_max_field() : max = {}", MODULE_PATH, max);

    bindings::set_max_field(field, max)
}

/// <https://invisible-island.net/ncurses/man/form_new_page.3x.html>
pub unsafe fn set_new_page(field: Option<FIELD>, new_page_flag: bool) -> i32 {
    assert!(field.map_or_else(|| true, |field| !field.is_null()), "{}set_new_page() : field.is_null()", MODULE_PATH);

    bindings::set_new_page(return_mut_ptr!(field), new_page_flag)
}

/// <https://invisible-island.net/ncurses/man/form_page.3x.html>
pub unsafe fn unfocus_current_field(form: FORM) -> i32 {
    assert!(!form.is_null(), "{}unfocus_current_field() : form.is_null()", MODULE_PATH);

    bindings::unfocus_current_field(form)
}

/// <https://invisible-island.net/ncurses/man/form_post.3x.html>
pub unsafe fn unpost_form(form: FORM) -> i32 {
    assert!(!form.is_null(), "{}unpost_form() : form.is_null()", MODULE_PATH);

    bindings::unpost_form(form)
}

/// <https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html>
pub unsafe fn new_form_sp(sp: SCREEN, fields: *mut FIELD) -> Option<FORM> {
    assert!(!sp.is_null(), "{}new_form_sp() : sp.is_null()", MODULE_PATH);
    assert!(!fields.is_null(), "{}new_form_sp() : fields.is_null()", MODULE_PATH);

    bindings::new_form_sp(sp, fields).as_mut().map(|ptr| ptr as FORM)
}
