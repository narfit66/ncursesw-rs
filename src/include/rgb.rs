/*
    src/include/rgb.rs

    Copyright (c) 2019-2021 Stephen Whittle  All rights reserved.

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

macro_rules! define_rgb {
    ($type: ty) => {
        /// The (R)ed, (G)reen and (B)lue content of a color.
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
        pub struct RGB {
            red:   $type,
            green: $type,
            blue:  $type
        }

        impl RGB {
            /// Create a new rgb instance.
            pub fn new(red: $type, green: $type, blue: $type) -> Self {
                assert!((0..=1000).contains(&red));
                assert!((0..=1000).contains(&green));
                assert!((0..=1000).contains(&blue));

                Self { red, green, blue }
            }

            /// Return the red rgb content.
            pub fn red(&self) -> $type {
                self.red
            }

            /// Return the green rgb content.
            pub fn green(&self) -> $type {
                self.green
            }

            /// Return the blue rgb content.
            pub fn blue(&self) -> $type {
                self.blue
            }
        }
    }
}
