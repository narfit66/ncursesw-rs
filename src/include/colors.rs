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
    ($type: ty) => {
        use crate::shims::ncurses::SCREEN;

        /// Foreground and background colors.
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
        pub struct Colors {
            foreground: Color,
            background: Color
        }

        impl Colors {
            /// Create a new instance of foreground and background colors.
            pub fn new(foreground: Color, background: Color) -> Self {
                assert!(foreground.screen() == background.screen(), "Colors::new() : foreground.screen() != background.screen()");

                Self { foreground, background }
            }
        }

        impl ColorsType<Color, $type> for Colors {
            /// Return the associated screen.
            fn screen(&self) -> Option<SCREEN> {
                self.foreground.screen()
            }

            /// Returns the foreground color.
            fn foreground(&self) -> Color {
                self.foreground
            }

            /// Returns the background color.
            fn background(&self) -> Color {
                self.background
            }
        }
    }
}
