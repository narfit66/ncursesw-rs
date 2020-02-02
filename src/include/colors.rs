/*
    src/include/colors.rs

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

macro_rules! define_colors {
    ($type: ty, $extend: expr) => {
        use crate::ncursescolortype::set_ncurses_colortype;

        /// Foreground and background colors.
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
        pub struct Colors {
            foreground: Color,
            background: Color
        }

        impl ColorsType<Color, $type> for Colors {
            /// Create a new instance of foreground and background colors.
            fn new(foreground: Color, background: Color) -> Self {
                set_ncurses_colortype($extend);

                Self { foreground, background }
            }

            /// Foreground color.
            fn foreground(&self) -> Color {
                self.foreground
            }

            /// Background color.
            fn background(&self) -> Color {
                self.background
            }
        }
    }
}
