/*
    src/form/funcs.rs

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

use std::ptr;

use errno::errno;

use normal;

use cstring::*;
use shims::{
    nform, nform::FieldType, ncurses::{SCREEN, WINDOW}, bindings::va_list,
    constants::{
        E_OK, E_UNKNOWN_COMMAND, E_NO_MATCH, NO_JUSTIFICATION, JUSTIFY_LEFT,
        JUSTIFY_CENTER, JUSTIFY_RIGHT
    }
};
use form::{
    FormOptions, FormRequest, FieldInfo, FieldJustification, FieldOptions, FieldParameters,
    ncurseswformerror::{
        NCurseswFormError, ncursesw_form_error_system_error, ncursesw_form_error_from_rc
    }
};
use wide::WideChar;
use crate::{Origin, Size};

pub type FORM = nform::FORM;
pub type FIELD = nform::FIELD;
pub type FIELDTYPE = nform::FIELDTYPE;
pub type Form_Hook = nform::Form_Hook;

pub fn current_field(form: FORM) -> form_result!(FIELD) {
    unsafe { nform::current_field(form).ok_or_else(|| form_function_error!("current_field")) }
}

pub fn data_ahead(form: FORM) -> bool {
    unsafe { nform::data_ahead(form) }
}

pub fn data_behind(form: FORM) -> bool {
    unsafe { nform::data_behind(form) }
}

pub fn dup_field(field: FIELD, origin: Origin) -> form_result!(FIELD) {
    unsafe { nform::dup_field(field, origin.y, origin.x).ok_or_else(|| form_function_error!("dup_field")) }
}

pub fn dynamic_field_info(field: FIELD) -> form_result!(FieldInfo) {
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];
    let mut max: [i32; 1] = [0];

    match unsafe { nform::dynamic_field_info(field, rows.as_mut_ptr(), cols.as_mut_ptr(), max.as_mut_ptr()) } {
        E_OK => Ok(FieldInfo::new(Size { lines: rows[0], columns: cols[0] }, max[0])),
        rc   => Err(form_function_error_with_rc!("dynamic_field_info", rc))
    }
}

pub fn field_arg(field: FIELD) -> form_result!(*mut libc::c_void) {
    unsafe { nform::field_arg(field).ok_or_else(|| form_function_error!("field_arg")) }
}

pub fn field_back(field: FIELD) -> normal::Attributes {
    unsafe { normal::Attributes::from(nform::field_back(field)) }
}

pub fn field_buffer(field: FIELD, buffer_number: i32) -> form_result!(Vec<i8>) {
    unsafe { nform::field_buffer(field, buffer_number).ok_or_else(|| form_function_error!("field_buffer")) }
}

pub fn field_count(form: FORM) -> form_result!(i32) {
    let rc = unsafe { nform::field_count(form) };

    if rc < 0 {
        Err(NCurseswFormError::UnknownError { func: "field_count".to_string(), errno: rc })
    } else {
        Ok(rc)
    }
}

pub fn field_fore(field: FIELD) -> normal::Attributes {
    unsafe { normal::Attributes::from(nform::field_fore(field)) }
}

pub fn field_index(field: FIELD) -> form_result!(i32) {
    let rc = unsafe { nform::field_index(field) };

    if rc < 0 {
        Err(NCurseswFormError::UnknownError { func: "field_index".to_string(), errno: rc })
    } else {
        Ok(rc)
    }
}

pub fn field_info(field: FIELD) -> form_result!(FieldParameters) {
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];
    let mut frow: [i32; 1] = [0];
    let mut fcol: [i32; 1] = [0];
    let mut ncol: [i32; 1] = [0];
    let mut nbuf: [i32; 1] = [0];

    match unsafe { nform::field_info(field, rows.as_mut_ptr(), cols.as_mut_ptr(), frow.as_mut_ptr(), fcol.as_mut_ptr(), ncol.as_mut_ptr(), nbuf.as_mut_ptr()) } {
        E_OK => Ok(FieldParameters::new(Size { lines: rows[0], columns: cols[0] }, Origin { y: frow[0], x: fcol[0] }, ncol[0], nbuf[0])),
        rc   => Err(form_function_error_with_rc!("field_info", rc))

    }
}

pub fn field_init(form: FORM) -> form_result!(Form_Hook) {
    unsafe { nform::field_init(form).ok_or_else(|| form_function_error_with_rc!("field_init", errno().into())) }
}

pub fn field_just(field: FIELD) -> form_result!(FieldJustification) {
    match unsafe { nform::field_just(field) } {
        NO_JUSTIFICATION => Ok(FieldJustification::None),
        JUSTIFY_LEFT     => Ok(FieldJustification::Left),
        JUSTIFY_CENTER   => Ok(FieldJustification::Centered),
        JUSTIFY_RIGHT    => Ok(FieldJustification::Right),
        rc               => Err(form_function_error_with_rc!("field_just", rc))
    }
}

pub fn field_opts(field: FIELD) -> FieldOptions {
    unsafe { FieldOptions::from(nform::field_opts(field)) }
}

pub fn field_opts_off(field: FIELD, opts: FieldOptions) -> form_result!(()) {
    match unsafe { nform::field_opts_off(field, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("field_opts_off", rc))
    }
}

pub fn field_opts_on(field: FIELD, opts: FieldOptions) -> form_result!(()) {
    match unsafe { nform::field_opts_on(field, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("field_opts_on", rc))
    }
}

pub fn field_pad(field: FIELD) -> char {
    unsafe { nform::field_pad(field) as u8 as char }
}

pub fn field_status(field: FIELD) -> bool {
    unsafe { nform::field_status(field) }
}

pub fn field_term(form: FORM) -> form_result!(Form_Hook) {
    unsafe { nform::field_term(form).ok_or_else(|| form_function_error_with_rc!("field_term", errno().into())) }
}

pub fn field_type(field: FIELD) -> form_result!(FIELDTYPE) {
    unsafe { nform::field_type(field).ok_or_else(|| form_function_error_with_rc!("field_type", errno().into())) }
}

pub fn field_userptr(field: FIELD) -> form_result!(*mut libc::c_void) {
    unsafe { nform::field_userptr(field).ok_or_else(|| form_function_error!("field_userptr")) }
}

pub fn form_driver(form: FORM, request: FormRequest) -> form_result!(Option<FormRequest>) {
    match unsafe { nform::form_driver(form, request.value()?) } {
        E_OK => Ok(None),
        rc   => if request == FormRequest::Mouse {
            if rc == E_UNKNOWN_COMMAND {
                Ok(None)
            } else {
                Ok(Some(FormRequest::new(rc)))
            }
        } else {
            Err(form_function_error_with_rc!("form_driver", rc))
        }
    }
}

pub fn form_driver_w(form: FORM, request: FormRequest, wch: WideChar) -> form_result!(Option<FormRequest>) {
    match unsafe { nform::form_driver_w(form, request.value()?, wch.into()) } {
        E_OK => Ok(None),
        rc   => if request == FormRequest::Mouse {
            if rc == E_UNKNOWN_COMMAND {
                Ok(None)
            } else {
                Ok(Some(FormRequest::new(rc)))
            }
        } else {
            Err(form_function_error_with_rc!("form_driver_w", rc))
        }
    }
}

pub fn form_fields(form: FORM) -> form_result!(Vec<FIELD>) {
    unsafe { nform::form_fields(form).ok_or_else(|| form_function_error!("menu_fields")) }
}

pub fn form_init(form: FORM) -> form_result!(Form_Hook) {
    unsafe { nform::form_init(form).ok_or_else(|| form_function_error_with_rc!("form_init", errno().into())) }
}

pub fn form_opts(form: FORM) -> FormOptions {
    unsafe { FormOptions::from(nform::form_opts(form)) }
}

pub fn form_opts_off(form: FORM, opts: FormOptions) -> form_result!(()) {
    match unsafe { nform::form_opts_off(form, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("form_opts_off", rc))
    }
}

pub fn form_opts_on(form: FORM, opts: FormOptions) -> form_result!(()) {
    match unsafe { nform::form_opts_on(form, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("form_opts_on", rc))
    }
}

pub fn form_page(form: FORM) -> form_result!(i32) {
    let rc = unsafe { nform::form_page(form) };

    if rc < 0 {
        Err(NCurseswFormError::UnknownError { func: "form_page".to_string(), errno: rc })
    } else {
        Ok(rc)
    }
}

pub fn form_request_by_name(name: &str) -> form_result!(bool) {
    match unsafe { nform::form_request_by_name(c_str_with_nul!(name)) } {
        E_OK       => Ok(true),
        E_NO_MATCH => Ok(false),
        rc         => Err(form_function_error_with_rc!("form_request_by_name", rc))
    }
}

pub fn form_request_name(request: FormRequest) -> form_result!(String) {
    nform::form_request_name(request.value()?).ok_or_else(|| form_function_error_with_rc!("form_request_name", errno().into()))
}

pub fn form_sub(form: FORM) -> form_result!(WINDOW) {
    unsafe { nform::form_sub(form) }.ok_or_else(|| form_function_error!("form_sub"))
}

pub fn form_term(form: FORM) -> form_result!(Form_Hook) {
    unsafe { nform::form_term(form).ok_or_else(|| form_function_error_with_rc!("form_term", errno().into())) }
}

pub fn form_userptr(form: FORM) -> form_result!(*mut libc::c_void) {
    unsafe { nform::form_userptr(form).ok_or_else(|| form_function_error!("form_userptr")) }
}

pub fn form_win(form: FORM) -> form_result!(WINDOW) {
    unsafe { nform::form_win(form).ok_or_else(|| form_function_error!("form_win")) }
}

pub fn free_field(field: FIELD) -> form_result!(()) {
    match unsafe { nform::free_field(field) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("free_field", rc))
    }
}

pub fn free_fieldtype(fieldtype: FIELDTYPE) -> form_result!(()) {
    match unsafe { nform::free_fieldtype(fieldtype) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("free_fieldtype", rc))
    }
}

pub fn free_form(form: FORM) -> form_result!(()) {
    match unsafe { nform::free_form(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("free_form", rc))
    }
}

pub fn link_field(field: FIELD, origin: Origin) -> form_result!(FIELD) {
    unsafe { nform::link_field(field, origin.y, origin.x).ok_or_else(|| form_function_error!("link_field")) }
}

pub fn link_fieldtype(type1: FIELDTYPE, type2: FIELDTYPE) -> form_result!(FIELDTYPE) {
    unsafe { nform::link_fieldtype(type1, type2).ok_or_else(|| form_function_error!("link_fieldtype")) }
}

pub fn move_field(field: FIELD, origin: Origin) -> form_result!(()) {
    match unsafe { nform::move_field(field, origin.y, origin.x) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("move_field", rc))
    }
}

pub fn new_field(parameters: FieldParameters) -> form_result!(FIELD) {
    unsafe { nform::new_field(
        parameters.size().lines,
        parameters.size().columns,
        parameters.origin().y,
        parameters.origin().x,
        parameters.offscreen(),
        parameters.nbuffers()
    ).ok_or_else(|| form_function_error!("new_field")) }
}

pub fn new_fieldtype(
    field_check: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool,
    char_check:  unsafe extern "C" fn(_: i32, _: *const libc::c_void) -> bool
) -> form_result!(FIELDTYPE)
{
    unsafe { nform::new_fieldtype(field_check, char_check).ok_or_else(|| form_function_error!("new_fieldtype")) }
}

pub fn new_form(fields: &mut Vec<FIELD>) -> form_result!(FORM) {
    fields.push(ptr::null_mut());
    fields.shrink_to_fit();

    let form = unsafe { nform::new_form(fields.as_mut_ptr() as *mut FIELD) };

    fields.pop();

    form.ok_or_else(|| form_function_error_with_rc!("new_form", errno().into()))
}

pub fn new_page(field: FIELD) -> bool {
    unsafe { nform::new_page(field) }
}

pub fn pos_form_cursor(form: FORM) -> form_result!(()) {
    match unsafe { nform::pos_form_cursor(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("pos_form_cursor", rc))
    }
}

pub fn post_form(form: FORM) -> form_result!(()) {
    match unsafe { nform::post_form(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("post_form", rc))
    }
}

pub fn scale_form(form: FORM) -> form_result!(Size) {
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];

    match unsafe { nform::scale_form(form, rows.as_mut_ptr(), cols.as_mut_ptr()) } {
        E_OK => Ok(Size { lines: rows[0], columns: cols[0] }),
        rc   => Err(form_function_error_with_rc!("scale_form", rc))
    }
}

pub fn set_current_field(form: FORM, field: FIELD) -> form_result!(()) {
    match unsafe { nform::set_current_field(form, field) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_current_field", rc))
    }
}

pub fn set_field_back(field: FIELD, attr: normal::Attributes) -> form_result!(()) {
    match unsafe { nform::set_field_back(field, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_back", rc))
    }
}

pub fn set_field_buffer(field: FIELD, buffer_number: i32, buffer: &[i8]) -> form_result!(()) {
    match unsafe { nform::set_field_buffer(field, buffer_number, buffer) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_buffer", rc))
    }
}

pub fn set_field_fore(field: FIELD, attr: normal::Attributes) -> form_result!(()) {
    match unsafe { nform::set_field_fore(field, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_fore", rc))
    }
}

pub fn set_field_init(form: FORM, func: Form_Hook) -> form_result!(()) {
    match unsafe { nform::set_field_init(form, func) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_init", rc))
    }
}

pub fn set_field_just(field: FIELD, justification: FieldJustification) -> form_result!(()) {
    match unsafe { nform::set_field_just(field, match justification {
        FieldJustification::None     => NO_JUSTIFICATION,
        FieldJustification::Left     => JUSTIFY_LEFT,
        FieldJustification::Centered => JUSTIFY_CENTER,
        FieldJustification::Right    => JUSTIFY_RIGHT
    }) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_just", rc))
    }
}

pub fn set_field_opts(field: FIELD, opts: FieldOptions) -> form_result!(()) {
    match unsafe { nform::set_field_opts(field, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_opts", rc))
    }
}

pub fn set_field_pad(field: FIELD, pad: char) -> form_result!(()) {
    match unsafe { nform::set_field_pad(field, i32::from(pad as u8)) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_pad", rc))
    }
}

pub fn set_field_status(field: FIELD, status: bool) -> form_result!(()) {
    match unsafe { nform::set_field_status(field, status) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_status", rc))
    }
}

pub fn set_field_term(form: FORM, func: Form_Hook) -> form_result!(()) {
    match unsafe { nform::set_field_term(form, func) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_term", rc))
    }
}

pub fn set_field_type(field: FIELD, fieldtype: FieldType) -> form_result!(()) {
    match unsafe { nform::set_field_type(field, fieldtype) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_type", rc))
    }
}

pub fn set_fieldtype_arg(
    fieldtype: FIELDTYPE,
    make_arg:  unsafe extern "C" fn(_: *mut va_list) -> *mut libc::c_void,
    copy_arg:  Option<unsafe extern "C" fn(_: *const libc::c_void) -> *mut libc::c_void>,
    free_arg:  Option<unsafe extern "C" fn(_: *mut libc::c_void)>
) -> form_result!(())
{
    match unsafe { nform::set_fieldtype_arg(fieldtype, make_arg, copy_arg, free_arg) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_fieldtype_arg", rc))
    }
}

pub fn set_fieldtype_choice(
    fieldtype: FIELDTYPE,
    next_choice: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool,
    prev_choice: unsafe extern "C" fn(_: FIELD, _: *const libc::c_void) -> bool
) -> form_result!(())
{
    match unsafe { nform::set_fieldtype_choice(fieldtype, next_choice, prev_choice) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_fieldtype_choice", rc))
    }
}

pub fn set_field_userptr(field: FIELD, userptr: Option<*mut libc::c_void>) -> form_result!(()) {
    match unsafe { nform::set_field_userptr(field, userptr) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_userptr", rc))
    }
}

pub fn set_form_fields(form: FORM, fields: &mut Vec<FIELD>) -> form_result!(()) {
    fields.push(ptr::null_mut());
    fields.shrink_to_fit();

    let rc = unsafe { nform::set_form_fields(form, fields.as_ptr() as *mut FIELD) };

    fields.pop();

    match rc {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_fields", rc))
    }
}

pub fn set_form_init(form: FORM, func: Form_Hook) -> form_result!(()) {
    match unsafe { nform::set_form_init(form, func) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_init", rc))
    }
}

pub fn set_form_opts(form: FORM, opts: FormOptions) -> form_result!(()) {
    match unsafe { nform::set_form_opts(form, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_opts", rc))
    }
}

pub fn set_form_page(form: FORM, n: i32) -> form_result!(()) {
    match unsafe { nform::set_form_page(form, n) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_page", rc))
    }
}

pub fn set_form_sub(form: Option<FORM>, sub: Option<WINDOW>) -> form_result!(()) {
    match unsafe { nform::set_form_sub(form, sub) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_sub", rc))
    }
}

pub fn set_form_term(form: FORM, func: Form_Hook) -> form_result!(()) {
    match unsafe { nform::set_form_term(form, func) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_term", rc))
    }
}

pub fn set_form_userptr(form: FORM, userptr: Option<*mut libc::c_void>) -> form_result!(()) {
    match unsafe { nform::set_form_userptr(form, userptr) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_userptr", rc))
    }
}

pub fn set_form_win(form: Option<FORM>, win: Option<WINDOW>) -> form_result!(()) {
    match unsafe { nform::set_form_win(form, win) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_win", rc))
    }
}

pub fn set_max_field(field: FIELD, max: i32) -> form_result!(()) {
    match unsafe { nform::set_max_field(field, max) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_max_field", rc))
    }
}

pub fn set_new_page(field: FIELD, new_page_flag: bool) -> form_result!(()) {
    match unsafe { nform::set_new_page(field, new_page_flag) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_new_page", rc))
    }
}

pub fn unfocus_current_field(form: FORM) -> form_result!(()) {
    match unsafe { nform::unfocus_current_field(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("unfocus_current_field", rc))
    }
}

pub fn unpost_form(form: FORM) -> form_result!(()) {
    match unsafe { nform::unpost_form(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("unpost_form", rc))
    }
}

pub fn new_form_sp(screen: SCREEN, fields: &mut Vec<FIELD>) -> form_result!(FORM) {
    fields.push(ptr::null_mut());
    fields.shrink_to_fit();

    let form = unsafe { nform::new_form_sp(screen, fields.as_mut_ptr() as *mut FIELD) };

    fields.pop();

    form.ok_or_else(|| form_function_error_with_rc!("new_form_sp", errno().into()))
}
