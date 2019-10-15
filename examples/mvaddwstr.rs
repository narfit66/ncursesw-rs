extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    use_default_colors()?;

    let wide_str = WideString::from_str("Testing..Testing..1..2..3..");

    let add_length = wide_str.len() as i32;
    let mut origin = Origin { y: LINES() / 2, x: (COLS() / 2) - (add_length / 2) };

    mvaddwstr(origin, &wide_str)?;

    origin.y += 3;
    origin.x = (COLS() / 2) - 12;

    r#move(origin)?;
    addstr("hit <return> to continue ")?;

    refresh()?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}