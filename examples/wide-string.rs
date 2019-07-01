extern crate ncursesw;

use ncursesw::{initscr, endwin, setlocale, addwstr, WideString, refresh, getch, LcCategory, NCurseswError};

pub fn main() {
    if let Err(e) = main_routine() {
        let _ = endwin();
        println!("{}", e.to_string());
    }
}

pub fn main_routine() -> Result<(), NCurseswError> {
    setlocale(LcCategory::All, "");

    initscr()?;

    let str1 = "\u{41f}\u{440}\u{438}\u{432}\u{435}\u{442} is hello in russian!\n";
    let str2 = "🙈🙊🙉🙈🙊🙉\n";

    let wide_string1 = WideString::from_str(str1);
    let wide_string2 = WideString::from_str(str2);

    addwstr(&wide_string1)?;
    addwstr(&wide_string2)?;

    refresh()?;

    getch()?;

    endwin()?;

    Ok(())
}
