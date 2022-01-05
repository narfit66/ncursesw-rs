#define _XOPEN_SOURCE_EXTENDED 1

#include <ctype.h>
#include <locale.h>

#include "%include%/ncurses_dll.h"
#include "%include%/ncurses.h"
#include "%include%/panel.h"
#include "%include%/menu.h"
#include "%include%/form.h"

// Workaround for rust-bindgen#753
#define MARK_FIX_753(req_name, type) const type Fix753_##req_name = req_name;

MARK_FIX_753(A_NORMAL, attr_t);
MARK_FIX_753(A_ATTRIBUTES, attr_t);
MARK_FIX_753(A_CHARTEXT, attr_t);
MARK_FIX_753(A_COLOR, attr_t);
MARK_FIX_753(A_STANDOUT, attr_t);
MARK_FIX_753(A_UNDERLINE, attr_t);
MARK_FIX_753(A_REVERSE, attr_t);
MARK_FIX_753(A_BLINK, attr_t);
MARK_FIX_753(A_DIM, attr_t);
MARK_FIX_753(A_BOLD, attr_t);
MARK_FIX_753(A_ALTCHARSET, attr_t);
MARK_FIX_753(A_INVIS, attr_t);
MARK_FIX_753(A_PROTECT, attr_t);
MARK_FIX_753(A_HORIZONTAL, attr_t);
MARK_FIX_753(A_LEFT, attr_t);
MARK_FIX_753(A_LOW, attr_t);
MARK_FIX_753(A_RIGHT, attr_t);
MARK_FIX_753(A_TOP, attr_t);
MARK_FIX_753(A_VERTICAL, attr_t);
MARK_FIX_753(A_ITALIC, attr_t);
