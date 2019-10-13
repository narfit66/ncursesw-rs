extern crate ncursesw;
extern crate ascii;

use ascii::*;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    if has_colors() {
        start_color()?;

        use_default_colors()?;

        let yellow = Color::Dark(BaseColor::Yellow);
        let blue = Color::Dark(BaseColor::Blue);

        let color_pair0 = ColorPair::default();
        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;

        let attrs0 = Attribute::Dim | color_pair0;
        let attrs1 = Attribute::Bold | color_pair1;

        let ascii_char = AsciiChar::A;
        let chtype_char = ChtypeChar::new(ascii_char);

        wattr_set(win, attrs1, color_pair1)?;
        waddch(win, chtype_char | attrs0)?;

        match wattr_get(win)? {
            AttributesColorPairSet::Normal(s)   => {
                waddstr(win, "\n\nNormal attributes and color pair...\n\n")?;
                waddstr(win, &format!("attributes.is_bold={}\n", s.attributes().is_bold()))?;
                waddstr(win, &format!("attributes.is_dim={}\n", s.attributes().is_dim()))?;
                waddstr(win, &format!("attributes.color_pair={:?}", s.color_pair()))?;
            },
            AttributesColorPairSet::Extended(s) => {
                waddstr(win, "\n\nExtended attributes and color pair...\n\n")?;
                waddstr(win, &format!("attributes.is_bold={}\n", s.attributes().is_bold()))?;
                waddstr(win, &format!("attributes.is_dim={}\n", s.attributes().is_dim()))?;
                waddstr(win, &format!("attributes.color_pair={:?}", s.color_pair()))?;
            }
        }
    } else {
        waddstr(win, "terminal has no color support!!!")?;
    }

    waddstr(win, "\n\nhit <return> to continue ")?;
    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
