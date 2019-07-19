extern crate ncursesw;
extern crate ascii;

use std::str::FromStr;
use std::error::Error;
use ascii::*;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

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

    let win_size = getmaxyx(win)?;
    let mut origin = Origin { y: (win_size.lines - 1) / 2, x: (win_size.columns - 1) / 2 };

    let default_border = ChtypeChar::from_chtype(0);
    wborder(win, default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    mvwaddch(win, origin, chtype_char)?;

    origin.y += 3;
    origin.x = ((win_size.columns - 1) / 2) - 12;

    wmove(win, origin)?;
    waddstr(win, "hit <return> to continue ")?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
