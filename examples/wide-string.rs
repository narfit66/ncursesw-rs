/*
    examples/wide-string.rs

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

extern crate gettextrs;
extern crate ncursesw;

use gettextrs::*;
use ncursesw::*;

pub fn main() {
    if let Err(source) = main_routine() {
        println!("{}", source.to_string());
    }
}

pub fn main_routine() -> Result<(), NCurseswError> {
    setlocale(LocaleCategory::LcAll, "");

    initscr()?;

    let str1 = "\u{41f}\u{440}\u{438}\u{432}\u{435}\u{442} is hello in russian!\n";
    let str2 = "🙈🙊🙉🙈🙊🙉\n";

    let wide_string1 = &WideString::from_str(str1);
    let wide_string2 = &WideString::from_str(str2);

    addwstr(wide_string1)?;
    addwstr(wide_string2)?;

    refresh()?;

    getch()?;

    endwin()
}
