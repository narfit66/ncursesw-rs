/*
    src/ncursescolortype/funcs.rs

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

use std::{fmt::{Display, Formatter}, sync::atomic::{AtomicBool, Ordering}};

lazy_static! {
    // Flag internally to the crate if ansi or extended colors are being used,
    // this is used by `attr_get()`, `wattr_get()` and `getcchar()` so that they
    // can return the correct `attributes` and `colorpair` values.
    //
    // The flag is set by `init_extended_pair()`, `init_pair()`, `alloc_pair()`,
    // `find_pair()`, `ColorPair::default()` and `Colors::new()`.
    //
    // This works on the assumption that the calling code will only ever use
    // ansi or extended colors but not both (if both are inter-mixed then a
    // panic will occur.
    static ref EXTENDED_COLORS: AtomicBool = AtomicBool::new(false);
    // Has the crates EXTENDED_COLORS been set.
    static ref EXTENDED_COLORS_ALREADY_SET: AtomicBool = AtomicBool::new(false);
}

/// The color type that ncursesw is running under. by default it will be
/// ColorType::Normal until a ColorPair structure is generated.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum NCursesColorType {
    /// Normal (Ansi) colors, upto 16 colors.
    Normal,
    /// Extended colors, upto 256 colors.
    Extended
}

impl Display for NCursesColorType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            NCursesColorType::Normal   => "Normal",
            NCursesColorType::Extended => "Extended"
        })
    }
}

/// Returns the color type that ncursesw is running under. by default it will be
/// ColorType::Normal until a ColorPair structure is generated.
pub fn ncurses_colortype() -> NCursesColorType {
    if EXTENDED_COLORS.load(Ordering::SeqCst) {
        NCursesColorType::Extended
    } else {
        NCursesColorType::Normal
    }
}

/// Has the crates color type been set.
pub fn ncurses_colortype_set() -> bool {
    EXTENDED_COLORS_ALREADY_SET.load(Ordering::SeqCst)
}

pub(in crate) fn set_ncurses_colortype(colortype: NCursesColorType) {
    let set_colortype = ncurses_colortype();

    if colortype != set_colortype {
        if ncurses_colortype_set() {
            panic!("ncursesw color type already set as {}!", set_colortype);
        } else {
            EXTENDED_COLORS_ALREADY_SET.store(true, Ordering::SeqCst);

            EXTENDED_COLORS.store(match colortype {
                    NCursesColorType::Normal   => false,
                    NCursesColorType::Extended => true
                },
                Ordering::SeqCst
            );
        }
    }
}
