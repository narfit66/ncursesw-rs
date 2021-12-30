/*
    src/include/attributescolorpair.rs

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

/// A pair of `Attributes` and `ColorPair`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct AttributesColorPair {
    attributes: Attributes,
    color_pair: ColorPair
}

impl AttributesColorPair {
    pub fn new(attributes: Attributes, color_pair: ColorPair) -> Self {
        assert!(attributes.screen() == color_pair.screen(), "AttributesColorPair::new() : attributes.screen() != color_pair.screen()");

        Self { attributes, color_pair }
    }

    /// Return the attribute of the pair.
    pub fn attributes(&self) -> Attributes {
        self.attributes
    }

    /// Return the color pair of the pair.
    pub fn color_pair(&self) -> ColorPair {
        self.color_pair
    }
}
