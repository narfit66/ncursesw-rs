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

    let add_length = chtype_str.len() as i32;
    let win_size = getmaxyx(win)?;

    let mut origin = Origin { y: (win_size.lines - 1) / 2, x: ((win_size.columns - 1) / 2) - (add_length / 2) };

    let default_border = ChtypeChar::from_chtype(0);
    wborder(win, default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    mvwaddchstr(win, origin, &chtype_str)?;

    origin.y += 3;
    origin.x = ((win_size.columns - 1) / 2) - 12;

    wmove(win, origin)?;
    waddstr(win, "hit <return> to continue ")?;

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
