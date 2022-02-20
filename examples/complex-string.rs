/*
    examples/complex-string.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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

use gettextrs::*;
use ncursesw::{*, normal::*};

pub fn main() {
    if let Err(source) = main_routine() {
        println!("{}", source.to_string());
    }
}

pub fn main_routine() -> Result<(), NCurseswError> {
    setlocale(LocaleCategory::LcAll, "");

    initscr()?;

    let color_pair0 = ColorPair::default();
    let attrs = Attribute::Normal | color_pair0;

    let str1 = "\u{41f}\u{440}\u{438}\u{432}\u{435}\u{442} is hello in russian!";
    let str2 = "🙈🙊🙉🙈🙊🙉";

    let complex_string1 = ComplexString::from_wide_string(WideString::from(str1).as_ref(), attrs.as_ref(), color_pair0.as_ref())?;
    let complex_string2 = ComplexString::from_wide_string(WideString::from(str2).as_ref(), attrs.as_ref(), color_pair0.as_ref())?;

    let mut origin = Origin { y: 1, x: 1 };

    mvadd_wchstr(origin, complex_string1.as_ref())?;

    origin.y += 1;

    mvadd_wchstr(origin, complex_string2.as_ref())?;

    refresh()?;

    origin.y += 1;

    mvgetch(origin)?;

    endwin()
}
