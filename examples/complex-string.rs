extern crate ncursesw;

use ncursesw::{initscr, endwin, setlocale, mvadd_wchstr, WideString, ComplexString, refresh, mvgetch, LcCategory, Origin, NCurseswError};
use ncursesw::normal::{ColorPair, Attribute};

pub fn main() {
    if let Err(e) = main_routine() {
        let _ = endwin();
        println!("{}", e.to_string());
    }
}

pub fn main_routine() -> Result<(), NCurseswError> {
    setlocale(LcCategory::All, "");

    initscr()?;

    let color_pair0 = ColorPair::default();
    let attrs = Attribute::Normal | color_pair0;

    let str1 = "\u{41f}\u{440}\u{438}\u{432}\u{435}\u{442} is hello in russian!";
    let str2 = "🙈🙊🙉🙈🙊🙉";

    let complex_string1 = ComplexString::from_wide_string(&WideString::from_str(str1), &attrs, &color_pair0)?;
    let complex_string2 = ComplexString::from_wide_string(&WideString::from_str(str2), &attrs, &color_pair0)?;

    let mut origin = Origin { y: 1, x: 1 };

    mvadd_wchstr(origin, &complex_string1)?;

    origin.y += 1;

    mvadd_wchstr(origin, &complex_string2)?;

    refresh()?;

    origin.y += 1;

    mvgetch(origin)?;

    endwin()?;

    Ok(())
}
