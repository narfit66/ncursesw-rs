#![allow(non_snake_case)]

extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    addstr(&format!("tabsize is {} columns", TABSIZE()))?;

    addstr("\n\nhit <return> to continue ")?;
    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
