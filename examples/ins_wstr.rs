extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;
    let mut wide_str = WideString::from_str("Testing..1..2..3..");

    addwstr(&wide_str)?;

    wide_str = WideString::from_str("Testing..");

    r#move(Origin { y: 0, x: 0 })?;
    ins_wstr(&wide_str)?;

    r#move(Origin { y: 3, x: 0 })?;
    addstr("hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
