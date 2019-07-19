extern crate ncursesw;

use std::error::Error;
use ncursesw::*;
use shims::ncurses::{
    ACS_VLINE, ACS_HLINE, ACS_ULCORNER,
    ACS_URCORNER, ACS_LLCORNER, ACS_LRCORNER
};

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let ls = ChtypeChar::from_chtype(ACS_VLINE());
    let rs = ChtypeChar::from_chtype(ACS_VLINE());
    let ts = ChtypeChar::from_chtype(ACS_HLINE());
    let bs = ChtypeChar::from_chtype(ACS_HLINE());
    let tl = ChtypeChar::from_chtype(ACS_ULCORNER());
    let tr = ChtypeChar::from_chtype(ACS_URCORNER());
    let bl = ChtypeChar::from_chtype(ACS_LLCORNER());
    let br = ChtypeChar::from_chtype(ACS_LRCORNER());

    border(ls, rs, ts, bs, tl, tr, bl, br)?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
