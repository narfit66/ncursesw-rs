/*
    src/menu/menurequest.rs

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

use std::convert::TryFrom;

use shims::constants;
use menu::NCurseswMenuError;

/// Menu request.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MenuRequest {
    LeftItem,
    RightItem,
    UpItem,
    DownItem,
    ScrollUpLine,
    ScrollDownLine,
    ScrollUpPage,
    ScrollDownPage,
    FirstItem,
    LastItem,
    NextItem,
    PreviousItem,
    ToggleItem,
    ClearPattern,
    BackPattern,
    NextMatch,
    PreviousMatch,
    Navigate(char),
    Mouse
}

impl MenuRequest {
    pub(in crate::menu) fn new(request: i32) -> Self {
        match request {
            constants::REQ_LEFT_ITEM     => MenuRequest::LeftItem,
            constants::REQ_RIGHT_ITEM    => MenuRequest::RightItem,
            constants::REQ_UP_ITEM       => MenuRequest::UpItem,
            constants::REQ_DOWN_ITEM     => MenuRequest::DownItem,
            constants::REQ_SCR_ULINE     => MenuRequest::ScrollUpLine,
            constants::REQ_SCR_DLINE     => MenuRequest::ScrollDownLine,
            constants::REQ_SCR_UPAGE     => MenuRequest::ScrollUpPage,
            constants::REQ_SCR_DPAGE     => MenuRequest::ScrollDownPage,
            constants::REQ_FIRST_ITEM    => MenuRequest::FirstItem,
            constants::REQ_LAST_ITEM     => MenuRequest::LastItem,
            constants::REQ_NEXT_ITEM     => MenuRequest::NextItem,
            constants::REQ_PREV_ITEM     => MenuRequest::PreviousItem,
            constants::REQ_TOGGLE_ITEM   => MenuRequest::ToggleItem,
            constants::REQ_CLEAR_PATTERN => MenuRequest::ClearPattern,
            constants::REQ_BACK_PATTERN  => MenuRequest::BackPattern,
            constants::REQ_NEXT_MATCH    => MenuRequest::NextMatch,
            constants::REQ_PREV_MATCH    => MenuRequest::PreviousMatch,
            constants::KEY_MOUSE         => MenuRequest::Mouse,
            _                            => panic!("MenuRequest::new({}) out of bounds!!!", request)
        }
    }

    pub(in crate::menu) fn value(self) -> menu_result!(i32) {
        Ok(match self {
            MenuRequest::LeftItem       => constants::REQ_LEFT_ITEM,
            MenuRequest::RightItem      => constants::REQ_RIGHT_ITEM,
            MenuRequest::UpItem         => constants::REQ_UP_ITEM,
            MenuRequest::DownItem       => constants::REQ_DOWN_ITEM,
            MenuRequest::ScrollUpLine   => constants::REQ_SCR_ULINE,
            MenuRequest::ScrollDownLine => constants::REQ_SCR_DLINE,
            MenuRequest::ScrollUpPage   => constants::REQ_SCR_UPAGE,
            MenuRequest::ScrollDownPage => constants::REQ_SCR_DPAGE,
            MenuRequest::FirstItem      => constants::REQ_FIRST_ITEM,
            MenuRequest::LastItem       => constants::REQ_LAST_ITEM,
            MenuRequest::NextItem       => constants::REQ_NEXT_ITEM,
            MenuRequest::PreviousItem   => constants::REQ_PREV_ITEM,
            MenuRequest::ToggleItem     => constants::REQ_TOGGLE_ITEM,
            MenuRequest::ClearPattern   => constants::REQ_CLEAR_PATTERN,
            MenuRequest::BackPattern    => constants::REQ_BACK_PATTERN,
            MenuRequest::NextMatch      => constants::REQ_NEXT_MATCH,
            MenuRequest::PreviousMatch  => constants::REQ_PREV_MATCH,
            MenuRequest::Navigate(key)  => i32::try_from(u32::try_from(key)?)?,
            MenuRequest::Mouse          => constants::KEY_MOUSE
        })
    }
}
