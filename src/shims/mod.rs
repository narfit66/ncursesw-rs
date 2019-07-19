/*
    src/shims/mod.rs

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

/// NCurses API as generated by `bindgen`, all functions are `unsafe`
///
/// `ripoffline` is manually defined as `bindgen` does not correctly define callbacks.
pub mod bindings;
/// Defined constatnts.
pub mod constants;
/// Main ncurses functions thin API wrapper with assertions as appropriate
///
/// Most of the functions are `unsafe`.
pub mod ncurses;
/// NCurses mouse functions thin API wrapper with assertions as appropriate
///
/// Most of the functions are `unsafe`e.
pub mod nmouse;
/// NCurses panels functions thin API wrapper with assertions as appropriate
///
/// Most of the functions are `unsafe`e.
pub mod npanels;
/// utility functions thin API wrapper with assertions as appropriate.
pub mod utils;
