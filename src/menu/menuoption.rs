/*
    src/menu/menuoption.rs

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

/// Menu option.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MenuOption {
    /// Only one item can be selected for this menu.
    OneValue,
    /// Display the item descriptions when the menu is posted.
    ShowDescription,
    /// Display the menu in row-major order.
    RowMajor,
    /// Ignore the case when pattern-matching.
    IgnoreCase,
    /// Move the cursor to within the item name while pattern-matching.
    ShowMatch,
    /// Don't wrap around next-item and previous-item, requests to the
    /// other end of the menu.
    NonCyclic,
    /// If user clicks with the mouse and it does not fall on the
    /// currently active menu, push KEY_MOUSE and the MEVENT data
    /// back on the queue to allow processing in another part of
    /// the calling program.
    MouseMenu
}
