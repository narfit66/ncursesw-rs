/*
    src/gen/attributestype.rs

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

use gen::{AttributesGeneric, ColorAttributeTypes};

pub trait AttributesType<T>: AttributesGeneric
    where T: ColorAttributeTypes
{
    fn is_normal(&self) -> bool;
    fn set_normal(&mut self);

    fn is_char_text(&self) -> bool;
    fn set_char_text(&mut self, _: bool);

    fn is_standout(&self) -> bool;
    fn set_standout(&mut self, _: bool);

    fn is_underline(&self) -> bool;
    fn set_underline(&mut self, _: bool);

    fn is_reverse(&self) -> bool;
    fn set_reverse(&mut self, _: bool);

    fn is_blink(&self) -> bool;
    fn set_blink(&mut self, _: bool);

    fn is_dim(&self) -> bool;
    fn set_dim(&mut self, _: bool);

    fn is_bold(&self) -> bool;
    fn set_bold(&mut self, _: bool);

    fn is_alternate_char_set(&self) -> bool;
    fn set_alternative_char_set(&mut self, _: bool);

    fn is_invisible(&self) -> bool;
    fn set_invisible(&mut self, _: bool);

    fn is_protected(&self) -> bool;
    fn set_protected(&mut self, _: bool);

    fn is_horizontal(&self) -> bool;
    fn set_horizontal(&mut self, _: bool);

    fn is_left(&self) -> bool;
    fn set_left(&mut self, _: bool);

    fn is_low(&self) -> bool;
    fn set_low(&mut self, _: bool);

    fn is_right(&self) -> bool;
    fn set_right(&mut self, _: bool);

    fn is_top(&self) -> bool;
    fn set_top(&mut self, _: bool);

    fn is_vertical(&self) -> bool;
    fn set_vertical(&mut self, _: bool);

    fn is_italic(&self) -> bool;
    fn set_italic(&mut self, _: bool);
}
