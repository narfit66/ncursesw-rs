/*
    wrapper.h

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

#define _XOPEN_SOURCE_EXTENDED 1

#include <ctype.h>
#include <locale.h>

#include <ncurses.h>
#include <panel.h>
#include <menu.h>

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
