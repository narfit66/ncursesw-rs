extern crate ncursesw;
extern crate ascii;

use std::str::FromStr;
use ascii::*;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    use_default_colors()?;

    let mut color_pair = ColorPair::default();

    if has_colors() {
        start_color()?;

        let yellow = Color::from_str("yellow")?;
        let blue = Color::from_str("blue")?;

        color_pair = ColorPair::new(1, Colors::new(yellow, blue))?;
    }

    let attrs = Attribute::Bold | color_pair;

    let ascii_str = AsciiString::from_ascii("Testing..Testing..1..2..3..")?;
    let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;

    let add_length = chtype_str.len() as i32;
    let mut origin = Origin { y: LINES() / 2, x: (COLS() / 2) - (add_length / 2) };

    mvaddchstr(origin, &chtype_str)?;

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
