/*
    src/form/fieldoption.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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

/// Field option.
pub enum FieldOption {
    /// The field is displayed. If this option is off, display of the field is suppressed.
    Visible,
    /// The field is visited during processing. If this option is off, the field will not
    /// be reachable by navigation keys. Please notice that an invisible field appears to
    /// be inactive also.
    Active,
    /// The field contents are displayed as data is entered.
    Public,
    /// The field can be edited.
    Edit,
    /// Words that do not fit on a line are wrapped to the next line. Words are blank-separated.
    Wrap,
    /// The field is cleared whenever a character is entered at the first position.
    Blank,
    /// Skip to the next field when this one fills.
    AutoSkip,
    /// Allow a blank field.
    NullOk,
    /// Validate field only if modified by user.
    PassOk,
    /// Field buffers are fixed to field's original size. Turn this option off to create a dynamic field.
    Static,
    /// Permit dynamic fields to be justified, like static fields.
    DynamicJustify,
    /// Preserve leading whitespace in the field buffer, which is normally discarded.
    NoLeftStrip,
    /// When inserting into a field up to the boundary position, optionally delay the scrolling, so that
    /// the last inserted character remains visible, but advance the cursor to reflect the insertion.
    /// This allows the form library to display the inserted character in one-character fields as well as
    /// allowing the library to maintain consistent state.
    EdgeInsertStay,
    /// The `set_max_field` function checks for this extension, which allows a dynamic field to shrink if
    /// the new limit is smaller than the current field size.
    InputLimit
}
