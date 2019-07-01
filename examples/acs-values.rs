extern crate ncursesw;

use ncursesw::{initscr, endwin, addstr, refresh, getch, NCurseswError};
use ncursesw::shims::ncurses::*;

pub fn main() {
    if let Err(e) = main_routine() {
        let _ = endwin();
        println!("{}", e.to_string());
    }
}

pub fn main_routine() -> Result<(), NCurseswError> {
    let _ = initscr()?;

    addstr("VT100 symbols\n\n")?;

    addstr(&format!("upper left corner:    {:b}\n", ACS_ULCORNER()))?;
    addstr(&format!("lower left corner:    {:b}\n", ACS_LLCORNER()))?;
    addstr(&format!("upper right corner:   {:b}\n", ACS_URCORNER()))?;
    addstr(&format!("lower left corner:    {:b}\n", ACS_LRCORNER()))?;
    addstr(&format!("tee pointing right:   {:b}\n", ACS_LTEE()))?;
    addstr(&format!("tee pointing left:    {:b}\n", ACS_RTEE()))?;
    addstr(&format!("tee pointing up:      {:b}\n", ACS_BTEE()))?;
    addstr(&format!("tee pointing down:    {:b}\n", ACS_TTEE()))?;
    addstr(&format!("horizontal line:      {:b}\n", ACS_HLINE()))?;
    addstr(&format!("vertical line:        {:b}\n", ACS_VLINE()))?;
    addstr(&format!("crossover:            {:b}\n", ACS_PLUS()))?;
    addstr(&format!("scan line 1:          {:b}\n", ACS_S1()))?;
    addstr(&format!("scan line 9:          {:b}\n", ACS_S9()))?;
    addstr(&format!("diamond:              {:b}\n", ACS_DIAMOND()))?;
    addstr(&format!("checker board:        {:b}\n", ACS_CKBOARD()))?;
    addstr(&format!("degree symbol:        {:b}\n", ACS_DEGREE()))?;
    addstr(&format!("plus/minus:           {:b}\n", ACS_PLMINUS()))?;
    addstr(&format!("bullet:               {:b}\n", ACS_BULLET()))?;

    addstr("\nTeletype 5410v1 symbols\n\n")?;

    addstr(&format!("arrow pointing left:  {:b}\n", ACS_LARROW()))?;
    addstr(&format!("arrow pointing right: {:b}\n", ACS_RARROW()))?;
    addstr(&format!("arrow pointing down:  {:b}\n", ACS_DARROW()))?;
    addstr(&format!("arrow pointing up:    {:b}\n", ACS_UARROW()))?;
    addstr(&format!("board of squares:     {:b}\n", ACS_BOARD()))?;
    addstr(&format!("lantern symbol:       {:b}\n", ACS_LANTERN()))?;
    addstr(&format!("solid square block:   {:b}\n", ACS_BLOCK()))?;

    addstr("\nundocumented\n\n")?;

    addstr(&format!("scan line 3:          {:b}\n", ACS_S3()))?;
    addstr(&format!("scan line 7:          {:b}\n", ACS_S7()))?;
    addstr(&format!("less/equal:           {:b}\n", ACS_LEQUAL()))?;
    addstr(&format!("greater/equal:        {:b}\n", ACS_GEQUAL()))?;
    addstr(&format!("PI:                   {:b}\n", ACS_PI()))?;
    addstr(&format!("not equal:            {:b}\n", ACS_NEQUAL()))?;
    addstr(&format!("UK pound sign:        {:b}\n", ACS_STERLING()))?;

    refresh()?;

    getch()?;

    endwin()?;

    Ok(())
}
