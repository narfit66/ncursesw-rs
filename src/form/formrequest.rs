/*
    src/menu/formrequest.rs

    Copyright (c) 2019, 2020 Stephen Whittle  All rights reserved.

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
use crate::{shims::constants, form::NCurseswFormError};

/// Form request.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FormRequest {
    NextPage,
    PreviousPage,
    FirstPage,
    LastPage,
    NextField,
    PreviousField,
    FirstField,
    LastField,
    SortedNextField,
    SortedPreviousField,
    SortedFirstField,
    SortedLastField,
    LeftField,
    RightField,
    UpField,
    DownField,
    NextCharacter,
    PreviousCharacter,
    NextLine,
    PreviousLine,
    NextWord,
    PreviousWord,
    BeginField,
    EndField,
    BeginLine,
    EndLine,
    LeftCharacter,
    RightCharacter,
    UpCharacter,
    DownCharacter,
    Newline,
    InsertCharacter,
    InsertLine,
    DeleteCharacter,
    DeletePreviousCharacter,
    DeleteLine,
    DeleteWord,
    ClearToEndOfLine,
    ClearToEndOfField,
    ClearField,
    OverlayMode,
    InsertMode,
    ScrollForwardLine,
    ScrollBackwardLine,
    ScrollForwardPage,
    ScrollBackwardPage,
    ScrollForwardHalfPage,
    ScrollBackwardHalfPage,
    ScrollForwardCharacter,
    ScrollBackwardCharacter,
    ScrollForwardHorizontalLine,
    ScrollBackwardHorizontalLine,
    ScrollForwardHalfHorizontalLine,
    ScrollBackwardHalfHorizontalLine,
    Validate,
    DisplayNextField,
    DisplayPreviousField,
    Navigate(char),
    Mouse
}

impl FormRequest {
    pub(in crate::form) fn new(request: i32) -> Option<Self> {
        match request {
            constants::REQ_NEXT_PAGE    => Some(FormRequest::NextPage),
            constants::REQ_PREV_PAGE    => Some(FormRequest::PreviousPage),
            constants::REQ_FIRST_PAGE   => Some(FormRequest::FirstPage),
            constants::REQ_LAST_PAGE    => Some(FormRequest::LastPage),
            constants::REQ_NEXT_FIELD   => Some(FormRequest::NextField),
            constants::REQ_PREV_FIELD   => Some(FormRequest::PreviousField),
            constants::REQ_FIRST_FIELD  => Some(FormRequest::FirstField),
            constants::REQ_LAST_FIELD   => Some(FormRequest::LastField),
            constants::REQ_SNEXT_FIELD  => Some(FormRequest::SortedNextField),
            constants::REQ_SPREV_FIELD  => Some(FormRequest::SortedPreviousField),
            constants::REQ_SFIRST_FIELD => Some(FormRequest::SortedFirstField),
            constants::REQ_SLAST_FIELD  => Some(FormRequest::SortedLastField),
            constants::REQ_LEFT_FIELD   => Some(FormRequest::LeftField),
            constants::REQ_RIGHT_FIELD  => Some(FormRequest::RightField),
            constants::REQ_UP_FIELD     => Some(FormRequest::UpField),
            constants::REQ_DOWN_FIELD   => Some(FormRequest::DownField),
            constants::REQ_NEXT_CHAR    => Some(FormRequest::NextCharacter),
            constants::REQ_PREV_CHAR    => Some(FormRequest::PreviousCharacter),
            constants::REQ_NEXT_LINE    => Some(FormRequest::NextLine),
            constants::REQ_PREV_LINE    => Some(FormRequest::PreviousLine),
            constants::REQ_NEXT_WORD    => Some(FormRequest::NextWord),
            constants::REQ_PREV_WORD    => Some(FormRequest::PreviousWord),
            constants::REQ_BEG_FIELD    => Some(FormRequest::BeginField),
            constants::REQ_END_FIELD    => Some(FormRequest::EndField),
            constants::REQ_BEG_LINE     => Some(FormRequest::BeginLine),
            constants::REQ_END_LINE     => Some(FormRequest::EndLine),
            constants::REQ_LEFT_CHAR    => Some(FormRequest::LeftCharacter),
            constants::REQ_RIGHT_CHAR   => Some(FormRequest::RightCharacter),
            constants::REQ_UP_CHAR      => Some(FormRequest::UpCharacter),
            constants::REQ_DOWN_CHAR    => Some(FormRequest::DownCharacter),
            constants::REQ_NEW_LINE     => Some(FormRequest::Newline),
            constants::REQ_INS_CHAR     => Some(FormRequest::InsertCharacter),
            constants::REQ_INS_LINE     => Some(FormRequest::InsertLine),
            constants::REQ_DEL_CHAR     => Some(FormRequest::DeleteCharacter),
            constants::REQ_DEL_PREV     => Some(FormRequest::DeletePreviousCharacter),
            constants::REQ_DEL_LINE     => Some(FormRequest::DeleteLine),
            constants::REQ_DEL_WORD     => Some(FormRequest::DeleteWord),
            constants::REQ_CLR_EOL      => Some(FormRequest::ClearToEndOfLine),
            constants::REQ_CLR_EOF      => Some(FormRequest::ClearToEndOfField),
            constants::REQ_CLR_FIELD    => Some(FormRequest::ClearField),
            constants::REQ_OVL_MODE     => Some(FormRequest::OverlayMode),
            constants::REQ_INS_MODE     => Some(FormRequest::InsertMode),
            constants::REQ_SCR_FLINE    => Some(FormRequest::ScrollForwardLine),
            constants::REQ_SCR_BLINE    => Some(FormRequest::ScrollBackwardLine),
            constants::REQ_SCR_FPAGE    => Some(FormRequest::ScrollForwardPage),
            constants::REQ_SCR_BPAGE    => Some(FormRequest::ScrollBackwardPage),
            constants::REQ_SCR_FHPAGE   => Some(FormRequest::ScrollForwardHalfPage),
            constants::REQ_SCR_BHPAGE   => Some(FormRequest::ScrollBackwardHalfPage),
            constants::REQ_SCR_FCHAR    => Some(FormRequest::ScrollForwardCharacter),
            constants::REQ_SCR_BCHAR    => Some(FormRequest::ScrollBackwardCharacter),
            constants::REQ_SCR_HFLINE   => Some(FormRequest::ScrollForwardHorizontalLine),
            constants::REQ_SCR_HBLINE   => Some(FormRequest::ScrollBackwardHorizontalLine),
            constants::REQ_SCR_HFHALF   => Some(FormRequest::ScrollForwardHalfHorizontalLine),
            constants::REQ_SCR_HBHALF   => Some(FormRequest::ScrollBackwardHalfHorizontalLine),
            constants::REQ_VALIDATION   => Some(FormRequest::Validate),
            constants::REQ_NEXT_CHOICE  => Some(FormRequest::DisplayNextField),
            constants::REQ_PREV_CHOICE  => Some(FormRequest::DisplayPreviousField),
            constants::KEY_MOUSE        => Some(FormRequest::Mouse),
            _                           => None
        }
    }

    pub(in crate::form) fn value(self) -> form_result!(i32) {
        Ok(match self {
            FormRequest::NextPage                         => constants::REQ_NEXT_PAGE,
            FormRequest::PreviousPage                     => constants::REQ_PREV_PAGE,
            FormRequest::FirstPage                        => constants::REQ_FIRST_PAGE,
            FormRequest::LastPage                         => constants::REQ_LAST_PAGE,
            FormRequest::NextField                        => constants::REQ_NEXT_FIELD,
            FormRequest::PreviousField                    => constants::REQ_PREV_FIELD,
            FormRequest::FirstField                       => constants::REQ_FIRST_FIELD,
            FormRequest::LastField                        => constants::REQ_LAST_FIELD,
            FormRequest::SortedNextField                  => constants::REQ_SNEXT_FIELD,
            FormRequest::SortedPreviousField              => constants::REQ_SPREV_FIELD,
            FormRequest::SortedFirstField                 => constants::REQ_SFIRST_FIELD,
            FormRequest::SortedLastField                  => constants::REQ_SLAST_FIELD,
            FormRequest::LeftField                        => constants::REQ_LEFT_FIELD,
            FormRequest::RightField                       => constants::REQ_RIGHT_FIELD,
            FormRequest::UpField                          => constants::REQ_UP_FIELD,
            FormRequest::DownField                        => constants::REQ_DOWN_FIELD,
            FormRequest::NextCharacter                    => constants::REQ_NEXT_CHAR,
            FormRequest::PreviousCharacter                => constants::REQ_PREV_CHAR,
            FormRequest::NextLine                         => constants::REQ_NEXT_LINE,
            FormRequest::PreviousLine                     => constants::REQ_PREV_LINE,
            FormRequest::NextWord                         => constants::REQ_NEXT_WORD,
            FormRequest::PreviousWord                     => constants::REQ_PREV_WORD,
            FormRequest::BeginField                       => constants::REQ_BEG_FIELD,
            FormRequest::EndField                         => constants::REQ_END_FIELD,
            FormRequest::BeginLine                        => constants::REQ_BEG_LINE,
            FormRequest::EndLine                          => constants::REQ_END_LINE,
            FormRequest::LeftCharacter                    => constants::REQ_LEFT_CHAR,
            FormRequest::RightCharacter                   => constants::REQ_RIGHT_CHAR,
            FormRequest::UpCharacter                      => constants::REQ_UP_CHAR,
            FormRequest::DownCharacter                    => constants::REQ_DOWN_CHAR,
            FormRequest::Newline                          => constants::REQ_NEW_LINE,
            FormRequest::InsertCharacter                  => constants::REQ_INS_CHAR,
            FormRequest::InsertLine                       => constants::REQ_INS_LINE,
            FormRequest::DeleteCharacter                  => constants::REQ_DEL_CHAR,
            FormRequest::DeletePreviousCharacter          => constants::REQ_DEL_PREV,
            FormRequest::DeleteLine                       => constants::REQ_DEL_LINE,
            FormRequest::DeleteWord                       => constants::REQ_DEL_WORD,
            FormRequest::ClearToEndOfLine                 => constants::REQ_CLR_EOL,
            FormRequest::ClearToEndOfField                => constants::REQ_CLR_EOF,
            FormRequest::ClearField                       => constants::REQ_CLR_FIELD,
            FormRequest::OverlayMode                      => constants::REQ_OVL_MODE,
            FormRequest::InsertMode                       => constants::REQ_INS_MODE,
            FormRequest::ScrollForwardLine                => constants::REQ_SCR_FLINE,
            FormRequest::ScrollBackwardLine               => constants::REQ_SCR_BLINE,
            FormRequest::ScrollForwardPage                => constants::REQ_SCR_FPAGE,
            FormRequest::ScrollBackwardPage               => constants::REQ_SCR_BPAGE,
            FormRequest::ScrollForwardHalfPage            => constants::REQ_SCR_FHPAGE,
            FormRequest::ScrollBackwardHalfPage           => constants::REQ_SCR_BHPAGE,
            FormRequest::ScrollForwardCharacter           => constants::REQ_SCR_FCHAR,
            FormRequest::ScrollBackwardCharacter          => constants::REQ_SCR_BCHAR,
            FormRequest::ScrollForwardHorizontalLine      => constants::REQ_SCR_HFLINE,
            FormRequest::ScrollBackwardHorizontalLine     => constants::REQ_SCR_HBLINE,
            FormRequest::ScrollForwardHalfHorizontalLine  => constants::REQ_SCR_HFHALF,
            FormRequest::ScrollBackwardHalfHorizontalLine => constants::REQ_SCR_HBHALF,
            FormRequest::Validate                         => constants::REQ_VALIDATION,
            FormRequest::DisplayNextField                 => constants::REQ_NEXT_CHOICE,
            FormRequest::DisplayPreviousField             => constants::REQ_PREV_CHOICE,
            FormRequest::Navigate(key)                    => i32::try_from(u32::try_from(key)?)?,
            FormRequest::Mouse                            => constants::KEY_MOUSE
        })
    }
}
