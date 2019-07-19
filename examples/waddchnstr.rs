extern crate ncursesw;
extern crate ascii;

use std::str::FromStr;
use ascii::*;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

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

    waddchnstr(win, &chtype_str, 18)?;

    wmove(win, Origin { y: 3, x: 0})?;
    waddstr(win, "hit <return> to continue ")?;

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
