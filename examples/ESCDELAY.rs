#![allow(non_snake_case)]

extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    addstr(&format!("escape delay is {:?}", ESCDELAY()?))?;

    addstr("\n\nhit <return> to continue ")?;
    getch()?;


    delwin(h)?;
    endwin()?;

    Ok(())
}