#![allow(non_snake_case)]

extern crate ncursesw;
extern crate ascii;

use ascii::*;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let color_pair = ColorPair::default();
    let attrs = Attributes::default() | color_pair;

    let ascii_charA = AsciiChar::A;
    let ascii_charB = AsciiChar::B;

    let chtype_charA = ChtypeChar::new(ascii_charA) | attrs;
    let chtype_charB = ChtypeChar::new(ascii_charB) | attrs;

    let mut origin = Origin { y: LINES() / 2, x: COLS() / 2 };

    mvaddch(origin, chtype_charB)?;

    mvinsch(origin, chtype_charA)?;

    origin.y += 3;
    origin.x = (COLS() / 2) - 12;

    mvaddstr(origin, "hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
