/*
    src/include/colorpair.rs

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

use std::sync::atomic::Ordering;

use shims::ncurses;
use constants::ERR;
use gen::{ColorType, ColorsType};
use crate::EXTENDED_COLORS;

macro_rules! extend_colorpair {
    ($extend: expr) => {
        impl Default for ColorPair {
            fn default() -> Self {
                EXTENDED_COLORS.store($extend, Ordering::SeqCst);

                Self { raw: 0 }
            }
        }

        pub fn alloc_pair(colors: Colors) -> result!(ColorPair) {
            match ncurses::alloc_pair(colors.foreground().number(), colors.background().number()) {
                ERR  => Err(ncurses_function_error!("alloc_pair")),
                pair => {
                    EXTENDED_COLORS.store($extend, Ordering::SeqCst);

                    Ok(ColorPair::from(pair))
                }
            }
        }

        pub fn find_pair(colors: Colors) -> Option<ColorPair> {
            match ncurses::find_pair(colors.foreground().number(), colors.background().number()) {
                ERR  => None,
                pair => {
                    EXTENDED_COLORS.store($extend, Ordering::SeqCst);

                    Some(ColorPair::from(pair))
                }
            }
        }
    }
}
