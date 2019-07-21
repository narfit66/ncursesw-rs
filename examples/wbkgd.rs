extern crate ncursesw;
extern crate ascii;

use ascii::*;
use std::str::FromStr;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;
use shims::ncurses::ACS_CKBOARD;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    let mut color_pair = ColorPair::default();

    if has_colors() {
        start_color()?;

        let yellow = Color::from_str("yellow")?;
        let blue = Color::from_str("blue")?;

        color_pair = ColorPair::new(1, Colors::new(yellow, blue))?;
    }

    let attrs = Attribute::Bold | color_pair;

    wbkgd(win, ChtypeChar::from_chtype(ACS_CKBOARD()) | attrs)?;

    let ascii_str = AsciiString::from_ascii("hit <return> to continue")?;
    let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;

    let origin = Origin { y: 3, x: 0 };

    mvwaddchstr(win, origin, &chtype_str)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
