extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;
    let wide_str = WideString::from_str("Testing..1..2..3..");

    let mut origin = Origin { y: LINES() / 2, x: (COLS() / 2) - 14 };

    mvaddwstr(origin, &wide_str)?;

    mvins_nwstr(origin, &wide_str, 9)?;

    origin.y += 3;
    origin.x = (COLS() / 2) - 12;

    mvaddstr(origin, "hit <return> to continue ")?;

    refresh()?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
