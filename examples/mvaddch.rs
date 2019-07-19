extern crate ncursesw;
extern crate ascii;

use std::str::FromStr;
use std::error::Error;
use ascii::*;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let mut color_pair = ColorPair::default();

    if has_colors() {
        start_color()?;

        let yellow = Color::from_str("yellow")?;
        let blue = Color::from_str("blue")?;

        color_pair = ColorPair::new(1, Colors::new(yellow, blue))?;
    }

    let mut attrs = Attributes::default() | color_pair;
    attrs.set_bold(true);

    let ascii_char = AsciiChar::A;
    let chtype_char = ChtypeChar::new(ascii_char) | attrs;

    let mut origin = Origin { y: LINES() / 2, x: COLS() / 2 };

    mvaddch(origin, chtype_char)?;

    origin.y += 3;
    origin.x = (COLS() / 2) - 12;

    r#move(origin)?;
    addstr("hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
