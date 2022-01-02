/*
    src/shims/constants.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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

use crate::shims::bindings;

type attr_t = bindings::attr_t;

// Success/Failure.
wrap_const!(ERR: i32);
wrap_const!(OK: i32);
wrap_const!(TRUE: i32);
wrap_const!(FALSE: i32);

// Colors.
wrap_const!(COLOR_BLACK: i32);
wrap_const!(COLOR_RED: i32);
wrap_const!(COLOR_GREEN: i32);
wrap_const!(COLOR_YELLOW: i32);
wrap_const!(COLOR_BLUE: i32);
wrap_const!(COLOR_MAGENTA: i32);
wrap_const!(COLOR_CYAN: i32);
wrap_const!(COLOR_WHITE: i32);

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

macro_rules! wrap_key_const{ ($key: ident, $n: expr) => { pub const $key: i32 = KEY_F0 + $n; } }

// Keys.
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

// Attributes.
wrap_const!(A_NORMAL: attr_t);
pub const A_ATTRIBUTES: attr_t = (!0 as attr_t) << 8;
//wrap_const!(A_ATTRIBUTES: attr_t);
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
pub const A_ITALIC: attr_t = 1 << 31;
//wrap_const!(A_ITALIC: attr_t);

// Menu/Form Errors.
wrap_const!(E_BAD_ARGUMENT: i32);
wrap_const!(E_BAD_STATE: i32);
wrap_const!(E_CONNECTED: i32);
wrap_const!(E_CURRENT: i32);
wrap_const!(E_INVALID_FIELD: i32);
wrap_const!(E_NOT_CONNECTED: i32);
wrap_const!(E_NOT_POSTED: i32);
wrap_const!(E_NOT_SELECTABLE: i32);
wrap_const!(E_NO_MATCH: i32);
wrap_const!(E_NO_ROOM: i32);
wrap_const!(E_OK: i32);
wrap_const!(E_POSTED: i32);
wrap_const!(E_REQUEST_DENIED: i32);
wrap_const!(E_SYSTEM_ERROR: i32);
wrap_const!(E_UNKNOWN_COMMAND: i32);

wrap_const!(O_SELECTABLE: i32);

wrap_const!(O_ONEVALUE: i32);
wrap_const!(O_SHOWDESC: i32);
wrap_const!(O_ROWMAJOR: i32);
wrap_const!(O_IGNORECASE: i32);
wrap_const!(O_SHOWMATCH: i32);
wrap_const!(O_NONCYCLIC: i32);
wrap_const!(O_MOUSE_MENU: i32);

wrap_const!(REQ_LEFT_ITEM: i32);
wrap_const!(REQ_RIGHT_ITEM: i32);
wrap_const!(REQ_UP_ITEM: i32);
wrap_const!(REQ_DOWN_ITEM: i32);
wrap_const!(REQ_SCR_ULINE: i32);
wrap_const!(REQ_SCR_DLINE: i32);
wrap_const!(REQ_SCR_DPAGE: i32);
wrap_const!(REQ_SCR_UPAGE: i32);
wrap_const!(REQ_FIRST_ITEM: i32);
wrap_const!(REQ_LAST_ITEM: i32);
wrap_const!(REQ_NEXT_ITEM: i32);
wrap_const!(REQ_PREV_ITEM: i32);
wrap_const!(REQ_TOGGLE_ITEM: i32);
wrap_const!(REQ_CLEAR_PATTERN: i32);
wrap_const!(REQ_BACK_PATTERN: i32);
wrap_const!(REQ_NEXT_MATCH: i32);
wrap_const!(REQ_PREV_MATCH: i32);

wrap_const!(MIN_MENU_COMMAND: i32);
wrap_const!(MAX_MENU_COMMAND: i32);


// Form.

/* field justification */
wrap_const!(NO_JUSTIFICATION: i32);
wrap_const!(JUSTIFY_LEFT: i32);
wrap_const!(JUSTIFY_CENTER: i32);
wrap_const!(JUSTIFY_RIGHT: i32);

/* field options */
wrap_const!(O_VISIBLE: i32);
wrap_const!(O_ACTIVE: i32);
wrap_const!(O_PUBLIC: i32);
wrap_const!(O_EDIT: i32);
wrap_const!(O_WRAP: i32);
wrap_const!(O_BLANK: i32);
wrap_const!(O_AUTOSKIP: i32);
wrap_const!(O_NULLOK: i32);
wrap_const!(O_PASSOK: i32);
wrap_const!(O_STATIC: i32);
wrap_const!(O_DYNAMIC_JUSTIFY: i32);
wrap_const!(O_NO_LEFT_STRIP: i32);

/* form options */
wrap_const!(O_NL_OVERLOAD: i32);
wrap_const!(O_BS_OVERLOAD: i32);

/* form driver commands */
wrap_const!(REQ_NEXT_PAGE: i32);	// move to next page
wrap_const!(REQ_PREV_PAGE: i32);	// move to previous page
wrap_const!(REQ_FIRST_PAGE: i32);	// move to first page
wrap_const!(REQ_LAST_PAGE: i32);	// move to last page

wrap_const!(REQ_NEXT_FIELD: i32);	// move to next field
wrap_const!(REQ_PREV_FIELD: i32);	// move to previous field
wrap_const!(REQ_FIRST_FIELD: i32);	// move to first field
wrap_const!(REQ_LAST_FIELD: i32);	// move to last field
wrap_const!(REQ_SNEXT_FIELD: i32);	// move to sorted next field
wrap_const!(REQ_SPREV_FIELD: i32);	// move to sorted prev field
wrap_const!(REQ_SFIRST_FIELD: i32); // move to sorted first field
wrap_const!(REQ_SLAST_FIELD: i32);	// move to sorted last field
wrap_const!(REQ_LEFT_FIELD: i32);	// move to left to field
wrap_const!(REQ_RIGHT_FIELD: i32);  // move to right to field
wrap_const!(REQ_UP_FIELD: i32);     // move to up to field
wrap_const!(REQ_DOWN_FIELD: i32);   // move to down to field

wrap_const!(REQ_NEXT_CHAR: i32);	// move to next char in field
wrap_const!(REQ_PREV_CHAR: i32);	// move to prev char in field
wrap_const!(REQ_NEXT_LINE: i32);	// move to next line in field
wrap_const!(REQ_PREV_LINE: i32);	// move to prev line in field
wrap_const!(REQ_NEXT_WORD: i32);	// move to next word in field
wrap_const!(REQ_PREV_WORD: i32);	// move to prev word in field
wrap_const!(REQ_BEG_FIELD: i32);	// move to first char in field
wrap_const!(REQ_END_FIELD: i32);	// move after last char in field
wrap_const!(REQ_BEG_LINE: i32);     // move to beginning of line
wrap_const!(REQ_END_LINE: i32);     // move after last char in line
wrap_const!(REQ_LEFT_CHAR: i32);	// move left in field
wrap_const!(REQ_RIGHT_CHAR: i32);	// move right in field
wrap_const!(REQ_UP_CHAR: i32);      // move up in field
wrap_const!(REQ_DOWN_CHAR: i32);	// move down in field

wrap_const!(REQ_NEW_LINE: i32);     // insert/overlay new line
wrap_const!(REQ_INS_CHAR: i32);     // insert blank char at cursor
wrap_const!(REQ_INS_LINE: i32);     // insert blank line at cursor
wrap_const!(REQ_DEL_CHAR: i32);     // delete char at cursor
wrap_const!(REQ_DEL_PREV: i32);     // delete char before cursor
wrap_const!(REQ_DEL_LINE: i32);     // delete line at cursor
wrap_const!(REQ_DEL_WORD: i32);     // delete word at cursor
wrap_const!(REQ_CLR_EOL: i32);      // clear to end of line
wrap_const!(REQ_CLR_EOF: i32);      // clear to end of field
wrap_const!(REQ_CLR_FIELD: i32);	// clear entire field
wrap_const!(REQ_OVL_MODE: i32);     // begin overlay mode
wrap_const!(REQ_INS_MODE: i32);     // begin insert mode
wrap_const!(REQ_SCR_FLINE: i32);	// scroll field forward a line
wrap_const!(REQ_SCR_BLINE: i32);	// scroll field backward a line
wrap_const!(REQ_SCR_FPAGE: i32);	// scroll field forward a page
wrap_const!(REQ_SCR_BPAGE: i32);	// scroll field backward a page
wrap_const!(REQ_SCR_FHPAGE: i32);	// scroll field forward	 half page
wrap_const!(REQ_SCR_BHPAGE: i32);	// scroll field backward half page
wrap_const!(REQ_SCR_FCHAR: i32);	// horizontal scroll char
wrap_const!(REQ_SCR_BCHAR: i32);	// horizontal scroll char
wrap_const!(REQ_SCR_HFLINE: i32);	// horizontal scroll line
wrap_const!(REQ_SCR_HBLINE: i32);   // horizontal scroll line
wrap_const!(REQ_SCR_HFHALF: i32);   // horizontal scroll half line
wrap_const!(REQ_SCR_HBHALF: i32);   // horizontal scroll half line

wrap_const!(REQ_VALIDATION: i32);	// validate field
wrap_const!(REQ_NEXT_CHOICE: i32);	// display next field choice
wrap_const!(REQ_PREV_CHOICE: i32);	// display prev field choice

wrap_const!(MIN_FORM_COMMAND: i32);	// used by form_driver
wrap_const!(MAX_FORM_COMMAND: i32);	// used by form_driver
