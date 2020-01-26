/*
    src/lib.rs

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

#![allow(non_camel_case_types)]

extern crate libc;
extern crate custom_error;
extern crate ascii;
extern crate semver;
extern crate errno;
#[macro_use]
extern crate lazy_static;

mod macros;

/// Extended color's, color pairs and attributes module
pub mod extend;
/// Compiled in `ncursesw` features
pub mod features;
/// NCurses Form module
pub mod form;
/// Traits used by `ncursesw`
mod gen;
/// Normal color's, color pairs and attributes module
pub mod normal;
/// NCurses Menu module
pub mod menu;
/// NCurses Mouse module
pub mod mouse;
/// NCurses Panels module
pub mod panels;
/// NCurses API shims module
pub mod shims;

mod chtypet;
mod complex;
mod wide;

mod attributescolorpairset;
mod basecolor;
mod characterresult;
mod cstring;
mod changed;
mod cursortype;
mod funcs;
mod justification;
mod keybinding;
mod lccategory;
mod legacy;
mod ncurses;
mod ncursescolortype;
mod ncurseswerror;
mod orientation;
mod origin;
mod region;
mod size;
mod softlabeltype;

pub use chtypet::*;
pub use complex::*;
pub use wide::*;

pub use attributescolorpairset::*;
pub use basecolor::*;
pub use characterresult::*;
pub use changed::*;
pub use cursortype::*;
pub use funcs::*;
pub use gen::*;
pub use justification::*;
pub use keybinding::*;
pub use lccategory::*;
pub use legacy::*;
pub use ncurses::*;
pub use ncursescolortype::*;
pub use ncurseswerror::*;
pub use origin::*;
pub use orientation::*;
pub use region::*;
use shims::*;
pub use size::*;
pub use softlabeltype::*;

/// NCurses window raw pointer.
pub type WINDOW = shims::ncurses::WINDOW;
/// NCurses screen raw pointer.
pub type SCREEN = shims::ncurses::SCREEN;
/// Ripoff line callback function signature.
pub type RipoffInit = shims::bindings::RipoffInit;
/// Raw attribute type value.
pub type attr_t = shims::ncurses::attr_t;
