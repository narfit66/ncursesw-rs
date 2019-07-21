extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;
    let wide_str = WideString::from_str("Testing..1..2..3..");

    addwstr(&wide_str)?;

    r#move(Origin { y: 0, x: 0 })?;
    ins_nwstr(&wide_str, 9)?;

    r#move(Origin { y: 3, x: 0 })?;
    addstr("hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
