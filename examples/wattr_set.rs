extern crate ncursesw;

use std::error::Error;
use ncursesw::*;
use ncursesw::extend::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    if has_colors() {
        start_color()?;

        use_default_colors()?;

        let yellow = Color::Dark(BaseColor::Yellow);
        let blue = Color::Dark(BaseColor::Blue);

        let color_pair0 = ColorPair::default();
        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;

        let attrs0 = Attributes::default() | Attribute::Dim;
        let attrs1 = Attributes::default() | Attribute::Bold;

        let complex_char = ComplexChar::from_char('A', &attrs0, &color_pair0)?;

        wattr_set(win, attrs1, color_pair1)?;
        wadd_wch(win, complex_char)?;

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

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
