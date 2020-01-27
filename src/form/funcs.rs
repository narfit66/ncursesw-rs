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

/// Form.
pub type FORM = nform::FORM;
/// Form field.
pub type FIELD = nform::FIELD;
/// Form field type.
pub type FIELDTYPE = nform::FIELDTYPE;
/// Form callback function.
pub type Form_Hook = nform::Form_Hook;

/// Returns the current field of the given form.
pub fn current_field(form: FORM) -> form_result!(FIELD) {
    unsafe { nform::current_field(form).ok_or_else(|| form_function_error!("current_field")) }
}

/// Tests whether there is off-screen data ahead in the given form.
pub fn data_ahead(form: FORM) -> bool {
    unsafe { nform::data_ahead(form) }
}

/// Tests whether there is off-screen data behind in the given form.
pub fn data_behind(form: FORM) -> bool {
    unsafe { nform::data_behind(form) }
}

/// Duplicates a field at a new location. Most attributes (including current
/// contents, size, validation type, buffer count, growth threshold, justification,
/// foreground, background, pad character, options, and user pointer) are copied.
/// Field status and the field page bit are not copied.
pub fn dup_field(field: FIELD, origin: Origin) -> form_result!(FIELD) {
    unsafe { nform::dup_field(field, origin.y, origin.x).ok_or_else(|| form_function_error!("dup_field")) }
}

/// Returns the actual size of the field, and its maximum possible size.
/// If the field has no size limit, the return max will be set to 0.
/// A field can be made dynamic by turning off the `FieldOptions::Static`
/// option with `field_opts_off()`.
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

/// Returns the background attribute. The default is `normal::Attributes::Normal`.
pub fn field_back(field: FIELD) -> normal::Attributes {
    unsafe { normal::Attributes::from(nform::field_back(field)) }
}

/// Returns a vector of the contents of the given numbered buffer:
///
/// - The buffer contents always have the same length, and are padded with trailing
///   spaces as needed to ensure this length is the same.
/// - The buffer may contain leading spaces, depending on how it was set.
/// - The buffer contents are set with `set_field_buffer()`, or as a side effect of
///   any editing operations on the corresponding field.
/// - Editing operations are based on the window which displays the field, rather
///   than a string. The window contains only printable characters, and is filled
///   with blanks. If you want the raw data, you must write your own routine that
///   copies the value out of the buffer and removes the leading and trailing spaces.
/// - Because editing operations change the content of the buffer to correspond to
///   the window, you should not rely on using buffers for long-term storage of form data.
pub fn field_buffer(field: FIELD, buffer_number: i32) -> form_result!(Vec<i8>) {
    unsafe { nform::field_buffer(field, buffer_number).ok_or_else(|| form_function_error!("field_buffer")) }
}

/// Returns the count of fields in form.
pub fn field_count(form: FORM) -> form_result!(i32) {
    let rc = unsafe { nform::field_count(form) };

    if rc < 0 {
        Err(NCurseswFormError::UnknownError { func: "field_count".to_string(), errno: rc })
    } else {
        Ok(rc)
    }
}

/// Returns the foreground attribute. The default is `normal::Attributes::Standout`.
pub fn field_fore(field: FIELD) -> normal::Attributes {
    unsafe { normal::Attributes::from(nform::field_fore(field)) }
}

/// Returns the index of the field in the field array of the form it is connected to.
pub fn field_index(field: FIELD) -> form_result!(i32) {
    let rc = unsafe { nform::field_index(field) };

    if rc < 0 {
        Err(NCurseswFormError::UnknownError { func: "field_index".to_string(), errno: rc })
    } else {
        Ok(rc)
    }
}

/// Returns the sizes and other attributes passed in to the field at its creation time.
/// The attributes are: height, width, row of upper-left corner, column of upper-left
/// corner, number off-screen rows, and number of working buffers.
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

/// Returns the current field init hook.
pub fn field_init(form: FORM) -> form_result!(Form_Hook) {
    unsafe { nform::field_init(form).ok_or_else(|| form_function_error_with_rc!("field_init", errno().into())) }
}

/// Returns a field's justification attribute.
pub fn field_just(field: FIELD) -> form_result!(FieldJustification) {
    match unsafe { nform::field_just(field) } {
        NO_JUSTIFICATION => Ok(FieldJustification::None),
        JUSTIFY_LEFT     => Ok(FieldJustification::Left),
        JUSTIFY_CENTER   => Ok(FieldJustification::Centered),
        JUSTIFY_RIGHT    => Ok(FieldJustification::Right),
        rc               => Err(form_function_error_with_rc!("field_just", rc))
    }
}

/// Returns the field's current options.
pub fn field_opts(field: FIELD) -> FieldOptions {
    unsafe { FieldOptions::from(nform::field_opts(field)) }
}

/// Turns off the given options, and leaves others alone.
pub fn field_opts_off(field: FIELD, opts: FieldOptions) -> form_result!(()) {
    match unsafe { nform::field_opts_off(field, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("field_opts_off", rc))
    }
}

/// Turns on the given options, and leaves others alone.
pub fn field_opts_on(field: FIELD, opts: FieldOptions) -> form_result!(()) {
    match unsafe { nform::field_opts_on(field, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("field_opts_on", rc))
    }
}

/// Returns the given form's pad character. The default is a blank.
pub fn field_pad(field: FIELD) -> char {
    unsafe { nform::field_pad(field) as u8 as char }
}

/// Gets the current field status value. The status is `true` whenever the field changes.
pub fn field_status(field: FIELD) -> bool {
    unsafe { nform::field_status(field) }
}

/// Returns the current field term hook.
pub fn field_term(form: FORM) -> form_result!(Form_Hook) {
    unsafe { nform::field_term(form).ok_or_else(|| form_function_error_with_rc!("field_term", errno().into())) }
}

/// Returns the data type validation for fields.
pub fn field_type(field: FIELD) -> form_result!(FIELDTYPE) {
    unsafe { nform::field_type(field).ok_or_else(|| form_function_error_with_rc!("field_type", errno().into())) }
}

/// Returns the fields user pointer.
pub fn field_userptr(field: FIELD) -> form_result!(*mut libc::c_void) {
    unsafe { nform::field_userptr(field).ok_or_else(|| form_function_error!("field_userptr")) }
}

/// Once a form has been posted (displayed), you should funnel input events to
/// it through `form_driver()`.
pub fn form_driver(form: FORM, request: FormRequest) -> form_result!(Option<FormRequest>) {
    match unsafe { nform::form_driver(form, request.value()?) } {
        E_OK => Ok(None),
        rc   => if request == FormRequest::Mouse {
            if rc == E_UNKNOWN_COMMAND {
                Ok(None)
            } else {
                let form_request = FormRequest::new(rc);

                if form_request.is_some() {
                    Ok(FormRequest::new(rc))
                } else {
                    Err(form_function_error_with_rc!("form_driver", rc))
                }
            }
        } else {
            Err(form_function_error_with_rc!("form_driver", rc))
        }
    }
}

/// This extension simplifies the use of the forms library using wide characters.
/// The input is either a key code (a request) or a wide character returned by
/// `get_wch()`. The type must be passed as well, to enable the library to
/// determine whether the parameter is a wide character or a request.
pub fn form_driver_w(form: FORM, request: FormRequest, wch: WideChar) -> form_result!(Option<FormRequest>) {
    match unsafe { nform::form_driver_w(form, request.value()?, wch.into()) } {
        E_OK => Ok(None),
        rc   => if request == FormRequest::Mouse {
            if rc == E_UNKNOWN_COMMAND {
                Ok(None)
            } else {
                let form_request = FormRequest::new(rc);

                if form_request.is_some() {
                    Ok(FormRequest::new(rc))
                } else {
                    Err(form_function_error_with_rc!("form_driver", rc))
                }
            }
        } else {
            Err(form_function_error_with_rc!("form_driver_w", rc))
        }
    }
}

/// Returns a vector of the fields of the given form.
pub fn form_fields(form: FORM) -> form_result!(Vec<FIELD>) {
    unsafe { nform::form_fields(form).ok_or_else(|| form_function_error!("form_fields")) }
}

/// Returns the current form init hook.
pub fn form_init(form: FORM) -> form_result!(Form_Hook) {
    unsafe { nform::form_init(form).ok_or_else(|| form_function_error_with_rc!("form_init", errno().into())) }
}

/// Returns the form's current options.
pub fn form_opts(form: FORM) -> FormOptions {
    unsafe { FormOptions::from(nform::form_opts(form)) }
}

/// Turns off the given options, and leaves others alone.
pub fn form_opts_off(form: FORM, opts: FormOptions) -> form_result!(()) {
    match unsafe { nform::form_opts_off(form, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("form_opts_off", rc))
    }
}

/// Turns on the given options, and leaves others alone.
pub fn form_opts_on(form: FORM, opts: FormOptions) -> form_result!(()) {
    match unsafe { nform::form_opts_on(form, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("form_opts_on", rc))
    }
}

/// Returns the form's current page number.
pub fn form_page(form: FORM) -> form_result!(i32) {
    let rc = unsafe { nform::form_page(form) };

    if rc < 0 {
        Err(NCurseswFormError::UnknownError { func: "form_page".to_string(), errno: rc })
    } else {
        Ok(rc)
    }
}

/// Searches in the name-table for a request with the given name and returns
/// its request code as a `Some`. Otherwise `None` is returned.
pub fn form_request_by_name(name: &str) -> form_result!(Option<FormRequest>) {
    match unsafe { nform::form_request_by_name(c_str_with_nul!(name)) } {
        E_NO_MATCH => Ok(None),
        rc         => {
            let form_request = FormRequest::new(rc);

            if form_request.is_some() {
                Ok(form_request)
            } else {
                Err(form_function_error_with_rc!("form_request_by_name", rc))
            }
        }
    }
}

/// Returns the printable name of a form request code.
pub fn form_request_name(request: FormRequest) -> form_result!(String) {
    nform::form_request_name(request.value()?).ok_or_else(|| form_function_error_with_rc!("form_request_name", errno().into()))
}

/// Return the forms sub-window.
pub fn form_sub(form: FORM) -> form_result!(WINDOW) {
    unsafe { nform::form_sub(form) }.ok_or_else(|| form_function_error!("form_sub"))
}

/// Returns the current form term hook.
pub fn form_term(form: FORM) -> form_result!(Form_Hook) {
    unsafe { nform::form_term(form).ok_or_else(|| form_function_error_with_rc!("form_term", errno().into())) }
}

/// Returns the forms user pointer.
pub fn form_userptr(form: FORM) -> form_result!(*mut libc::c_void) {
    unsafe { nform::form_userptr(form).ok_or_else(|| form_function_error!("form_userptr")) }
}

/// Return the forms main-window.
pub fn form_win(form: FORM) -> form_result!(WINDOW) {
    unsafe { nform::form_win(form).ok_or_else(|| form_function_error!("form_win")) }
}

/// De-allocates storage associated with a field.
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

/// The function `free_form()` disconnects form from its field array and
/// frees the storage allocated for the form.
pub fn free_form(form: FORM) -> form_result!(()) {
    match unsafe { nform::free_form(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("free_form", rc))
    }
}

/// Acts like `dup_field()`, but the new field shares buffers with its parent.
/// Attribute data is separate.
pub fn link_field(field: FIELD, origin: Origin) -> form_result!(FIELD) {
    unsafe { nform::link_field(field, origin.y, origin.x).ok_or_else(|| form_function_error!("link_field")) }
}

pub fn link_fieldtype(type1: FIELDTYPE, type2: FIELDTYPE) -> form_result!(FIELDTYPE) {
    unsafe { nform::link_fieldtype(type1, type2).ok_or_else(|| form_function_error!("link_fieldtype")) }
}

/// Moves the given field (which must be disconnected) to a specified location on the screen.
pub fn move_field(field: FIELD, origin: Origin) -> form_result!(()) {
    match unsafe { nform::move_field(field, origin.y, origin.x) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("move_field", rc))
    }
}

/// Allocates a new field and initializes it from the contents of a `FieldPrameters`
/// type given: height, width, row of upper-left corner, column of upper-left corner,
/// number off-screen rows, and number of additional working buffers.
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

/// Creates a new form connected to specified fields as a vector (the vector
/// must point to an area of contiguous memory representing the fields in order).
pub fn new_form(fields: &mut Vec<FIELD>) -> form_result!(FORM) {
    fields.push(ptr::null_mut());
    fields.shrink_to_fit();

    let form = unsafe { nform::new_form(fields.as_mut_ptr() as *mut FIELD) };

    fields.pop();

    form.ok_or_else(|| form_function_error_with_rc!("new_form", errno().into()))
}

/// The function `new_page()` is a predicate which tests if a given field
/// marks a page beginning on its form.
pub fn new_page(field: FIELD) -> bool {
    unsafe { nform::new_page(field) }
}

/// Restores the cursor to the position required for the forms driver to
/// continue processing requests. This is useful after NCurses routines
/// have been called to do screen-painting in response to a form operation.
pub fn pos_form_cursor(form: FORM) -> form_result!(()) {
    match unsafe { nform::pos_form_cursor(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("pos_form_cursor", rc))
    }
}

/// Displays a form to its associated sub-window. To trigger physical display
/// of the sub-window, use `refresh()` or some equivalent NCurses routine
/// (the implicit `doupdate()` triggered by a NCurses input request will do).
pub fn post_form(form: FORM) -> form_result!(()) {
    match unsafe { nform::post_form(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("post_form", rc))
    }
}

/// Returns the minimum size required for the sub-window of form.
pub fn scale_form(form: FORM) -> form_result!(Size) {
    let mut rows: [i32; 1] = [0];
    let mut cols: [i32; 1] = [0];

    match unsafe { nform::scale_form(form, rows.as_mut_ptr(), cols.as_mut_ptr()) } {
        E_OK => Ok(Size { lines: rows[0], columns: cols[0] }),
        rc   => Err(form_function_error_with_rc!("scale_form", rc))
    }
}

/// Sets the current field of the given form.
pub fn set_current_field(form: FORM, field: FIELD) -> form_result!(()) {
    match unsafe { nform::set_current_field(form, field) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_current_field", rc))
    }
}

/// Sets the background attribute of form. This is the highlight used to
/// display the extent fields in the form.
pub fn set_field_back(field: FIELD, attr: normal::Attributes) -> form_result!(()) {
    match unsafe { nform::set_field_back(field, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_back", rc))
    }
}

/// Sets the associated status flag of field; field_status gets the current value.
/// The status flag is set to a nonzero value whenever the field changes.
pub fn set_field_buffer(field: FIELD, buffer_number: i32, buffer: &[i8]) -> form_result!(()) {
    match unsafe { nform::set_field_buffer(field, buffer_number, buffer) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_buffer", rc))
    }
}

/// Sets the foreground attribute of field. This is the highlight used to
/// display the field contents.
pub fn set_field_fore(field: FIELD, attr: normal::Attributes) -> form_result!(()) {
    match unsafe { nform::set_field_fore(field, attr.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_fore", rc))
    }
}

/// Sets a hook to be called at form-post time and each time the selected
/// field changes (after the change).
pub fn set_field_init(form: FORM, func: Form_Hook) -> form_result!(()) {
    match unsafe { nform::set_field_init(form, func) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_init", rc))
    }
}

/// Sets the justification attribute of a field.
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

/// Sets all the given field's options.
pub fn set_field_opts(field: FIELD, opts: FieldOptions) -> form_result!(()) {
    match unsafe { nform::set_field_opts(field, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_opts", rc))
    }
}

/// Sets the character used to fill the field.
pub fn set_field_pad(field: FIELD, pad: char) -> form_result!(()) {
    match unsafe { nform::set_field_pad(field, i32::from(pad as u8)) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_pad", rc))
    }
}

/// Sets the associated status flag of field.
pub fn set_field_status(field: FIELD, status: bool) -> form_result!(()) {
    match unsafe { nform::set_field_status(field, status) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_status", rc))
    }
}

/// Sets a hook to be called at form-unpost time and each time the selected
/// field changes (before the change).
pub fn set_field_term(form: FORM, func: Form_Hook) -> form_result!(()) {
    match unsafe { nform::set_field_term(form, func) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_term", rc))
    }
}

/// Declares a data type for a given form field.
/// This is the type checked by validation functions.
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

/// Sets the fields user pointer.
pub fn set_field_userptr(field: FIELD, userptr: Option<*mut libc::c_void>) -> form_result!(()) {
    match unsafe { nform::set_field_userptr(field, userptr) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_field_userptr", rc))
    }
}

/// Changes the field pointer array of the given form.
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

/// Sets a hook to be called at form-post time and just after a page change once it is posted.
pub fn set_form_init(form: FORM, func: Form_Hook) -> form_result!(()) {
    match unsafe { nform::set_form_init(form, func) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_init", rc))
    }
}

/// Sets all the given form's options.
pub fn set_form_opts(form: FORM, opts: FormOptions) -> form_result!(()) {
    match unsafe { nform::set_form_opts(form, opts.into()) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_opts", rc))
    }
}

/// Sets the form's page number (goes to page `n` of the form).
pub fn set_form_page(form: FORM, n: i32) -> form_result!(()) {
    match unsafe { nform::set_form_page(form, n) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_page", rc))
    }
}

/// Sets the forms sub-window. if `form` is `None` then `window` is
/// default for all forms, if `window` is `None` the `stdscr()` is used.
pub fn set_form_sub(form: Option<FORM>, window: Option<WINDOW>) -> form_result!(()) {
    match unsafe { nform::set_form_sub(form, window) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_sub", rc))
    }
}

/// Sets a hook to be called at form-unpost time and just before a page change once it is posted.
pub fn set_form_term(form: FORM, func: Form_Hook) -> form_result!(()) {
    match unsafe { nform::set_form_term(form, func) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_term", rc))
    }
}

/// Sets the forms user pointer.
pub fn set_form_userptr(form: FORM, userptr: Option<*mut libc::c_void>) -> form_result!(()) {
    match unsafe { nform::set_form_userptr(form, userptr) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_userptr", rc))
    }
}

/// Sets the forms main-window. if `form` is `None` then `window` is
/// default for all forms, if `window` is `None` the `stdscr()` is used.
pub fn set_form_win(form: Option<FORM>, window: Option<WINDOW>) -> form_result!(()) {
    match unsafe { nform::set_form_win(form, window) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_form_win", rc))
    }
}

/// Sets the maximum size for a dynamic field. An argument of 0 turns off any
/// maximum size threshold for that field.
pub fn set_max_field(field: FIELD, max: i32) -> form_result!(()) {
    match unsafe { nform::set_max_field(field, max) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_max_field", rc))
    }
}

/// Sets or resets a flag marking the given field as the beginning of a new page on its form.
pub fn set_new_page(field: FIELD, new_page_flag: bool) -> form_result!(()) {
    match unsafe { nform::set_new_page(field, new_page_flag) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("set_new_page", rc))
    }
}

/// Removes the focus from the current field of the form.
/// In such state, inquiries via `current_field()` will error.
pub fn unfocus_current_field(form: FORM) -> form_result!(()) {
    match unsafe { nform::unfocus_current_field(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("unfocus_current_field", rc))
    }
}

/// Erases form from its associated sub-window.
pub fn unpost_form(form: FORM) -> form_result!(()) {
    match unsafe { nform::unpost_form(form) } {
        E_OK => Ok(()),
        rc   => Err(form_function_error_with_rc!("unpost_form", rc))
    }
}

// screen `_sp` functions.

/// Screen function of `new_form()`.
pub fn new_form_sp(screen: SCREEN, fields: &mut Vec<FIELD>) -> form_result!(FORM) {
    fields.push(ptr::null_mut());
    fields.shrink_to_fit();

    let form = unsafe { nform::new_form_sp(screen, fields.as_mut_ptr() as *mut FIELD) };

    fields.pop();

    form.ok_or_else(|| form_function_error_with_rc!("new_form_sp", errno().into()))
}
