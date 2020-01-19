/*
    src/include/colorpair.rs

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

use shims::ncurses;
use gen::{ColorType, ColorsType};
use ncursescolortype::*;

macro_rules! extend_colorpair {
    ($extend: expr) => {
        impl Default for ColorPair {
            fn default() -> Self {
                set_ncurses_colortype($extend);

                Self { raw: 0 }
            }
        }

        pub fn alloc_pair(colors: Colors) -> result!(ColorPair) {
            let pair = ncurses::alloc_pair(colors.foreground().number(), colors.background().number());

            if pair < 0 {
                Err(ncurses_function_error_with_rc!("alloc_pair", pair))
            } else {
                set_ncurses_colortype($extend);

                Ok(ColorPair::from(pair))
            }
        }

        pub fn find_pair(colors: Colors) -> Option<ColorPair> {
            let pair = ncurses::find_pair(colors.foreground().number(), colors.background().number());

            if pair < 0 {
                None
            } else {
                set_ncurses_colortype($extend);

                Some(ColorPair::from(pair))
            }
        }

        pub fn alloc_pair_sp(screen: ncurses::SCREEN, colors: Colors) -> result!(ColorPair) {
            let pair = unsafe { ncurses::alloc_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

            if pair < 0 {
                Err(ncurses_function_error_with_rc!("alloc_pair_sp", pair))
            } else {
                set_ncurses_colortype($extend);

                Ok(ColorPair::from(pair))
            }
        }

        pub fn find_pair_sp(screen: ncurses::SCREEN, colors: Colors) -> Option<ColorPair> {
            let pair = unsafe { ncurses::find_pair_sp(screen, colors.foreground().number(), colors.background().number()) };

            if pair < 0 {
                None
            } else {
                set_ncurses_colortype($extend);

                Some(ColorPair::from(pair))
            }
        }
    }
}
