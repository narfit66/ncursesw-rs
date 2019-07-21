extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;
    let s = "Testing..1..2..3..";

    addstr(&s)?;

    r#move(Origin { y: 0, x: 0 })?;
    insnstr(&s, 9)?;

    r#move(Origin { y: 3, x: 0 })?;
    addstr("hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
