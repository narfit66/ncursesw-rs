#![allow(non_snake_case)]

extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    if has_colors() {
        start_color()?;

        addstr(&format!("terminal supports {} color pairs", COLOR_PAIRS()))?;
    } else {
        addstr("terminal has no color support!!!")?;
    }

    addstr("\n\nhit <return> to continue ")?;
    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
