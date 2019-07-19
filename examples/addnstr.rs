extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    use_default_colors()?;

    let ascii_str = "Testing..Testing..1..2..3..";

    addnstr(&ascii_str, 18)?;

    r#move(Origin { y: 3, x: 0 })?;
    addstr("hit <return> to continue ")?;

    refresh()?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
