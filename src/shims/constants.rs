/*
    src/shims/constants.rs

    Copyright (c) 2019 Stephen Whittle  All rights reserved.

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom
    the Software is furnished to do so, subject to the following conditions:
    The above copyright notice and this permission notice shall be included
    in all copies or substantial portions of the Software.
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
    THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
    IN THE SOFTWARE.
*/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![warn(missing_debug_implementations)]

use bindings;

type attr_t = bindings::attr_t;

/* Success/Failure. */
wrap_const!(ERR: i32);
wrap_const!(OK: i32);
wrap_const!(TRUE: i32);
wrap_const!(FALSE: i32);

/* Colors */
wrap_const!(COLOR_BLACK: i16);
wrap_const!(COLOR_RED: i16);
wrap_const!(COLOR_GREEN: i16);
wrap_const!(COLOR_YELLOW: i16);
wrap_const!(COLOR_BLUE: i16);
wrap_const!(COLOR_MAGENTA: i16);
wrap_const!(COLOR_CYAN: i16);
wrap_const!(COLOR_WHITE: i16);

/*
 * This value is used in the firstchar and lastchar fields to mark
 * unchanged lines
 */
wrap_const!(_NOCHANGE: i32);

/*
 * This value is used in the oldindex field to mark lines created by insertions
 * and scrolls.
 */
wrap_const!(_NEWINDEX: i32);

macro_rules! wrap_key_const{ ($key:ident, $n:expr) => { pub const $key: i32 = KEY_F0 + $n; } }

/* Keys */
wrap_const!(KEY_CODE_YES: i32);            // A wchar_t contains a key code
wrap_const!(KEY_MIN: i32);                 // Minimum curses key
wrap_const!(KEY_BREAK: i32);               // Break key(unreliable)
wrap_const!(KEY_SRESET: i32);              // Soft(partial) reset(unreliable)
wrap_const!(KEY_RESET: i32);               // Reset or hard reset(unreliable)
wrap_const!(KEY_DOWN: i32);                // down-arrow key
wrap_const!(KEY_UP: i32);                  // up-arrow key
wrap_const!(KEY_LEFT: i32);                // left-arrow key
wrap_const!(KEY_RIGHT: i32);               // right-arrow key
wrap_const!(KEY_HOME: i32);                // home key
wrap_const!(KEY_BACKSPACE: i32);           // backspace key
wrap_const!(KEY_F0: i32);                  // Function keys. Space for 64
wrap_key_const!(KEY_F1, 1);
wrap_key_const!(KEY_F2, 2);
wrap_key_const!(KEY_F3, 3);
wrap_key_const!(KEY_F4, 4);
wrap_key_const!(KEY_F5, 5);
wrap_key_const!(KEY_F6, 6);
wrap_key_const!(KEY_F7, 7);
wrap_key_const!(KEY_F8, 8);
wrap_key_const!(KEY_F9, 9);
wrap_key_const!(KEY_F10, 10);
wrap_key_const!(KEY_F11, 11);
wrap_key_const!(KEY_F12, 12);
wrap_key_const!(KEY_F13, 13);
wrap_key_const!(KEY_F14, 14);
wrap_key_const!(KEY_F15, 15);
wrap_key_const!(KEY_F16, 16);
wrap_key_const!(KEY_F17, 17);
wrap_key_const!(KEY_F18, 18);
wrap_key_const!(KEY_F19, 19);
wrap_key_const!(KEY_F20, 20);
wrap_key_const!(KEY_F21, 21);
wrap_key_const!(KEY_F22, 22);
wrap_key_const!(KEY_F23, 23);
wrap_key_const!(KEY_F24, 24);
wrap_key_const!(KEY_F25, 25);
wrap_key_const!(KEY_F26, 26);
wrap_key_const!(KEY_F27, 27);
wrap_key_const!(KEY_F28, 28);
wrap_key_const!(KEY_F29, 29);
wrap_key_const!(KEY_F30, 30);
wrap_key_const!(KEY_F31, 31);
wrap_key_const!(KEY_F32, 32);
wrap_key_const!(KEY_F33, 33);
wrap_key_const!(KEY_F34, 34);
wrap_key_const!(KEY_F35, 35);
wrap_key_const!(KEY_F36, 36);
wrap_key_const!(KEY_F37, 37);
wrap_key_const!(KEY_F38, 38);
wrap_key_const!(KEY_F39, 39);
wrap_key_const!(KEY_F40, 40);
wrap_key_const!(KEY_F41, 41);
wrap_key_const!(KEY_F42, 42);
wrap_key_const!(KEY_F43, 43);
wrap_key_const!(KEY_F44, 44);
wrap_key_const!(KEY_F45, 45);
wrap_key_const!(KEY_F46, 46);
wrap_key_const!(KEY_F47, 47);
wrap_key_const!(KEY_F48, 48);
wrap_key_const!(KEY_F49, 49);
wrap_key_const!(KEY_F50, 50);
wrap_key_const!(KEY_F51, 51);
wrap_key_const!(KEY_F52, 52);
wrap_key_const!(KEY_F53, 53);
wrap_key_const!(KEY_F54, 54);
wrap_key_const!(KEY_F55, 55);
wrap_key_const!(KEY_F56, 56);
wrap_key_const!(KEY_F57, 57);
wrap_key_const!(KEY_F58, 58);
wrap_key_const!(KEY_F59, 59);
wrap_key_const!(KEY_F60, 60);
wrap_key_const!(KEY_F61, 61);
wrap_key_const!(KEY_F62, 62);
wrap_key_const!(KEY_F63, 63);
wrap_const!(KEY_DL: i32);                  // delete-line key
wrap_const!(KEY_IL: i32);                  // insert-line key
wrap_const!(KEY_DC: i32);                  // delete-character key
wrap_const!(KEY_IC: i32);                  // insert-character key
wrap_const!(KEY_EIC: i32);                 // sent by rmir or smir in insert mode
wrap_const!(KEY_CLEAR: i32);               // clear-screen or erase key
wrap_const!(KEY_EOS: i32);                 // clear-to-end-of-screen key
wrap_const!(KEY_EOL: i32);                 // clear-to-end-of-line key
wrap_const!(KEY_SF: i32);                  // scroll-forward key
wrap_const!(KEY_SR: i32);                  // scroll-backward key
wrap_const!(KEY_NPAGE: i32);               // next-page key
wrap_const!(KEY_PPAGE: i32);               // previous-page key
wrap_const!(KEY_STAB: i32);                // set-tab key
wrap_const!(KEY_CTAB: i32);                // clear-tab key
wrap_const!(KEY_CATAB: i32);               // clear-all-tabs key
wrap_const!(KEY_ENTER: i32);               // enter/send key
wrap_const!(KEY_PRINT: i32);               // print key
wrap_const!(KEY_LL: i32);                  // lower-left key(home down)
wrap_const!(KEY_A1: i32);                  // upper left of keypad
wrap_const!(KEY_A3: i32);                  // upper right of keypad
wrap_const!(KEY_B2: i32);                  // center of keypad
wrap_const!(KEY_C1: i32);                  // lower left of keypad
wrap_const!(KEY_C3: i32);                  // lower right of keypad
wrap_const!(KEY_BTAB: i32);                // back-tab key
wrap_const!(KEY_BEG: i32);                 // begin key
wrap_const!(KEY_CANCEL: i32);              // cancel key
wrap_const!(KEY_CLOSE: i32);               // close key
wrap_const!(KEY_COMMAND: i32);             // command key
wrap_const!(KEY_COPY: i32);                // copy key
wrap_const!(KEY_CREATE: i32);              // create key
wrap_const!(KEY_END: i32);                 // end key
wrap_const!(KEY_EXIT: i32);                // exit key
wrap_const!(KEY_FIND: i32);                // find key
wrap_const!(KEY_HELP: i32);                // help key
wrap_const!(KEY_MARK: i32);                // mark key
wrap_const!(KEY_MESSAGE: i32);             // message key
wrap_const!(KEY_MOVE: i32);                // move key
wrap_const!(KEY_NEXT: i32);                // next key
wrap_const!(KEY_OPEN: i32);                // open key
wrap_const!(KEY_OPTIONS: i32);             // options key
wrap_const!(KEY_PREVIOUS: i32);            // previous key
wrap_const!(KEY_REDO: i32);                // redo key
wrap_const!(KEY_REFERENCE: i32);           // reference key
wrap_const!(KEY_REFRESH: i32);             // refresh key
wrap_const!(KEY_REPLACE: i32);             // replace key
wrap_const!(KEY_RESTART: i32);             // restart key
wrap_const!(KEY_RESUME: i32);              // resume key
wrap_const!(KEY_SAVE: i32);                // save key
wrap_const!(KEY_SBEG: i32);                // shifted begin key
wrap_const!(KEY_SCANCEL: i32);             // shifted cancel key
wrap_const!(KEY_SCOMMAND: i32);            // shifted command key
wrap_const!(KEY_SCOPY: i32);               // shifted copy key
wrap_const!(KEY_SCREATE: i32);             // shifted create key
wrap_const!(KEY_SDC: i32);                 // shifted delete-character key
wrap_const!(KEY_SDL: i32);                 // shifted delete-line key
wrap_const!(KEY_SELECT: i32);              // select key
wrap_const!(KEY_SEND: i32);                // shifted end key
wrap_const!(KEY_SEOL: i32);                // shifted clear-to-end-of-line key
wrap_const!(KEY_SEXIT: i32);               // shifted exit key
wrap_const!(KEY_SFIND: i32);               // shifted find key
wrap_const!(KEY_SHELP: i32);               // shifted help key
wrap_const!(KEY_SHOME: i32);               // shifted home key
wrap_const!(KEY_SIC: i32);                 // shifted insert-character key
wrap_const!(KEY_SLEFT: i32);               // shifted left-arrow key
wrap_const!(KEY_SMESSAGE: i32);            // shifted message key
wrap_const!(KEY_SMOVE: i32);               // shifted move key
wrap_const!(KEY_SNEXT: i32);               // shifted next key
wrap_const!(KEY_SOPTIONS: i32);            // shifted options key
wrap_const!(KEY_SPREVIOUS: i32);           // shifted previous key
wrap_const!(KEY_SPRINT: i32);              // shifted print key
wrap_const!(KEY_SREDO: i32);               // shifted redo key
wrap_const!(KEY_SREPLACE: i32);            // shifted replace key
wrap_const!(KEY_SRIGHT: i32);              // shifted right-arrow key
wrap_const!(KEY_SRSUME: i32);              // shifted resume key
wrap_const!(KEY_SSAVE: i32);               // shifted save key
wrap_const!(KEY_SSUSPEND: i32);            // shifted suspend key
wrap_const!(KEY_SUNDO: i32);               // shifted undo key
wrap_const!(KEY_SUSPEND: i32);             // suspend key
wrap_const!(KEY_UNDO: i32);                // undo key
wrap_const!(KEY_MOUSE: i32);               // Mouse event has occurred
wrap_const!(KEY_RESIZE: i32);              // Terminal resize event
wrap_const!(KEY_EVENT: i32);               // We were interrupted by an event
wrap_const!(KEY_MAX: i32);                 // Maximum key value is 0633

wrap_const!(A_NORMAL: attr_t);
pub const A_ATTRIBUTES: attr_t = ((!0 as attr_t) << 8);
wrap_const!(A_CHARTEXT: attr_t);
wrap_const!(A_COLOR: attr_t);
wrap_const!(A_STANDOUT: attr_t);
wrap_const!(A_UNDERLINE: attr_t);
wrap_const!(A_REVERSE: attr_t);
wrap_const!(A_BLINK: attr_t);
wrap_const!(A_DIM: attr_t);
wrap_const!(A_BOLD: attr_t);
wrap_const!(A_ALTCHARSET: attr_t);
wrap_const!(A_INVIS: attr_t);
wrap_const!(A_PROTECT: attr_t);
wrap_const!(A_HORIZONTAL: attr_t);
wrap_const!(A_LEFT: attr_t);
wrap_const!(A_LOW: attr_t);
wrap_const!(A_RIGHT: attr_t);
wrap_const!(A_TOP: attr_t);
wrap_const!(A_VERTICAL: attr_t);
pub const A_ITALIC: attr_t = ((1 as attr_t) << 31);

wrap_const!(NCURSES_MOUSE_VERSION: i32);

const MASK_SHIFT: i32     = 7 - NCURSES_MOUSE_VERSION;
const MODIFIER_SHIFT: i32 = 4 + NCURSES_MOUSE_VERSION;

/* Mouse Support */
macro_rules! ncurses_mouse_mask( ($b: expr, $m: expr) => ($m << (($b - 1) * MASK_SHIFT)); );

wrap_const!(NCURSES_BUTTON_RELEASED: i32);
wrap_const!(NCURSES_BUTTON_PRESSED: i32);
wrap_const!(NCURSES_BUTTON_CLICKED: i32);
wrap_const!(NCURSES_DOUBLE_CLICKED: i32);
wrap_const!(NCURSES_TRIPLE_CLICKED: i32);
wrap_const!(NCURSES_RESERVED_EVENT: i32);

/* event masks */
pub const BUTTON1_RELEASED: i32       = ncurses_mouse_mask!(1, NCURSES_BUTTON_RELEASED);
pub const BUTTON1_PRESSED: i32        = ncurses_mouse_mask!(1, NCURSES_BUTTON_PRESSED);
pub const BUTTON1_CLICKED: i32        = ncurses_mouse_mask!(1, NCURSES_BUTTON_CLICKED);
pub const BUTTON1_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(1, NCURSES_DOUBLE_CLICKED);
pub const BUTTON1_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(1, NCURSES_TRIPLE_CLICKED);

pub const BUTTON2_RELEASED: i32       = ncurses_mouse_mask!(2, NCURSES_BUTTON_RELEASED);
pub const BUTTON2_PRESSED: i32        = ncurses_mouse_mask!(2, NCURSES_BUTTON_PRESSED);
pub const BUTTON2_CLICKED: i32        = ncurses_mouse_mask!(2, NCURSES_BUTTON_CLICKED);
pub const BUTTON2_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(2, NCURSES_DOUBLE_CLICKED);
pub const BUTTON2_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(2, NCURSES_TRIPLE_CLICKED);

pub const BUTTON3_RELEASED: i32       = ncurses_mouse_mask!(3, NCURSES_BUTTON_RELEASED);
pub const BUTTON3_PRESSED: i32        = ncurses_mouse_mask!(3, NCURSES_BUTTON_PRESSED);
pub const BUTTON3_CLICKED: i32        = ncurses_mouse_mask!(3, NCURSES_BUTTON_CLICKED);
pub const BUTTON3_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(3, NCURSES_DOUBLE_CLICKED);
pub const BUTTON3_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(3, NCURSES_TRIPLE_CLICKED);

pub const BUTTON4_RELEASED: i32       = ncurses_mouse_mask!(4, NCURSES_BUTTON_RELEASED);
pub const BUTTON4_PRESSED: i32        = ncurses_mouse_mask!(4, NCURSES_BUTTON_PRESSED);
pub const BUTTON4_CLICKED: i32        = ncurses_mouse_mask!(4, NCURSES_BUTTON_CLICKED);
pub const BUTTON4_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(4, NCURSES_DOUBLE_CLICKED);
pub const BUTTON4_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(4, NCURSES_TRIPLE_CLICKED);

pub const BUTTON5_RELEASED: i32       = ncurses_mouse_mask!(5, NCURSES_BUTTON_RELEASED);
pub const BUTTON5_PRESSED: i32        = ncurses_mouse_mask!(5, NCURSES_BUTTON_PRESSED);
pub const BUTTON5_CLICKED: i32        = ncurses_mouse_mask!(5, NCURSES_BUTTON_CLICKED);
pub const BUTTON5_DOUBLE_CLICKED: i32 = ncurses_mouse_mask!(5, NCURSES_DOUBLE_CLICKED);
pub const BUTTON5_TRIPLE_CLICKED: i32 = ncurses_mouse_mask!(5, NCURSES_TRIPLE_CLICKED);

pub const BUTTON_CTRL: i32            = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x001);
pub const BUTTON_SHIFT: i32           = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x002);
pub const BUTTON_ALT: i32             = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x004);
pub const REPORT_MOUSE_POSITION: i32  = ncurses_mouse_mask!(MODIFIER_SHIFT, 0x008);

pub const ALL_MOUSE_EVENTS: i32       = REPORT_MOUSE_POSITION - 1;
