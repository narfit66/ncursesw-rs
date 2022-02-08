/*
    src/keybinding.rs

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

#![allow(unused_attributes)]
#![allow(deprecated)]
#![allow(clippy::from_over_into)]

use std::convert::TryFrom;
use crate::{NCurseswError, shims::{constants::*, ncurses::wint_t}};

/// Keys returned by NCurses `getch()` and `get_wch()` families of functions.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyBinding {
    /// Break key (unreliable)
    Break,                     
    /// Soft (partial) reset (unreliable)
    SoftReset,                 
    /// Reset or hard reset (unreliable)
    Reset,                     
    /// down-arrow key
    DownArrow,                 
    /// up-arrow key
    UpArrow,                   
    /// left-arrow key
    LeftArrow,                 
    /// right-arrow key
    RightArrow,                
    /// home key
    Home,                      
    /// backspace key
    Backspace,                 
    /// Function keys. Space for 64
    FunctionKey(u8),           
    /// delete-line key
    DeleteLine,                
    /// insert-line key
    InsertLine,                
    /// delete-character key
    DeleteCharacter,           
    /// insert-character key
    InsertCharacter,           
    /// sent by rmir or smir in insert mode
    InsertMode,                
    /// clear-screen or erase key
    Erase,                     
    /// clear-to-end-of-screen key
    ClearToEndOfScreen,        
    /// clear-to-end-of-line key
    ClearToEndOfLine,          
    /// scroll-forward key
    ScrollForward,             
    /// scroll-backward key
    ScrollBackward,            
    /// next-page key
    NextPage,                  
    /// previous-page key
    PreviousPage,              
    /// set-tab key
    SetTab,                    
    /// clear-tab key
    ClearTab,                  
    /// clear-all-tabs key
    ClearAllTabs,              
    /// enter/send key
    Enter,                     
    /// print key
    Print,                     
    /// lower-left key (home down)
    HomeDown,                  
    /// upper left of keypad
    KeyPadUpperLeft,           
    /// upper right of keypad
    KeyPadUpperRight,          
    /// center of keypad
    KeyPadCenter,              
    /// lower left of keypad
    KeyPadLowerLeft,           
    /// lower right of keypad
    KeyPadLowerRight,          
    /// back-tab key
    BackTab,                   
    /// begin key
    Begin,                     
    /// cancel key
    Cancel,                    
    /// close key
    Close,                     
    /// command key
    Command,                   
    /// copy key
    Copy,                      
    /// create key
    Create,                    
    /// end key
    End,                       
    /// exit key
    Exit,                      
    /// find key
    Find,                      
    /// help key
    Help,                      
    /// mark key
    Mark,                      
    /// message key
    Message,                   
    /// move key
    Move,                      
    /// next key
    Next,                      
    /// open key
    Open,                      
    /// options key
    Options,                   
    /// previous key
    Previous,                  
    /// redo key
    Redo,                      
    /// reference key
    Reference,                 
    /// refresh key
    Refresh,                   
    /// replace key
    Replace,                   
    /// restart key
    Restart,                   
    /// resume key
    Resume,                    
    /// save key
    Save,                      
    /// shifted begin key
    ShiftBegin,                
    /// shifted cancel key
    ShiftCancel,               
    /// shifted command key
    ShiftCommand,              
    /// shifted copy key
    ShiftCopy,                 
    /// shifted create key
    ShiftCreate,               
    /// shifted delete-character key
    ShiftDeleteCharacter,      
    /// shifted delete-line key
    ShiftDeleteLine,           
    /// select key
    Select,                    
    /// shifted end key
    ShiftEnd,                  
    /// shifted clear-to-end-of-line key
    ShiftClearToEndOfLine,     
    /// shifted exit key
    ShiftExit,                 
    /// shifted find key
    ShiftFind,                 
    /// shifted help key
    ShiftHelp,                 
    /// shifted home key
    ShiftHome,                 
    /// shifted insert-character key
    ShiftInsertCharacter,      
    /// shifted left-arrow key
    ShiftLeftArrow,            
    /// shifted message key
    ShiftMessage,              
    /// shifted move key
    ShiftMove,                 
    /// shifted next key
    ShiftNext,                 
    /// shifted options key
    ShiftOptions,              
    /// shifted previous key
    ShiftPrevious,             
    /// shifted print key
    ShiftPrint,                
    /// shifted redo key
    ShiftRedo,                 
    /// shifted replace key
    ShiftReplace,              
    /// shifted right-arrow key
    ShiftRightArrow,           
    /// shifted resume key
    ShiftResume,               
    /// shifted save key
    ShiftSave,                 
    /// shifted suspend key
    ShiftSuspend,              
    /// shifted undo key
    ShiftUndo,                 
    /// suspend key
    Suspend,                   
    /// undo key
    Undo,                      
    /// Mouse event has occurred
    MouseEvent,                
    /// Terminal resize event
    ResizeEvent,               
    #[deprecated(since = "0.6.3", note = "this was deprecated as of NCurses API v6.3.20211021")]
    /// We were interrupted by an event
    Event,                     
    /// A user defined key.
    UserDefined(i32)
}

impl TryFrom<wint_t> for KeyBinding {
    type Error = NCurseswError;

    fn try_from(key: wint_t) -> Result<Self, Self::Error> {
        Ok(KeyBinding::from(i32::try_from(key)?))
    }
}

impl From<i32> for KeyBinding {
    fn from(key: i32) -> Self {
        match key {
            KEY_BREAK        => KeyBinding::Break,
            KEY_SRESET       => KeyBinding::SoftReset,
            KEY_RESET        => KeyBinding::Reset,
            KEY_DOWN         => KeyBinding::DownArrow,
            KEY_UP           => KeyBinding::UpArrow,
            KEY_LEFT         => KeyBinding::LeftArrow,
            KEY_RIGHT        => KeyBinding::RightArrow,
            KEY_HOME         => KeyBinding::Home,
            KEY_BACKSPACE    => KeyBinding::Backspace,
            KEY_F0..=KEY_F63 => KeyBinding::FunctionKey((key - KEY_F0) as u8),
            KEY_DL           => KeyBinding::DeleteLine,
            KEY_IL           => KeyBinding::InsertLine,
            KEY_DC           => KeyBinding::DeleteCharacter,
            KEY_IC           => KeyBinding::InsertCharacter,
            KEY_EIC          => KeyBinding::InsertMode,
            KEY_CLEAR        => KeyBinding::Erase,
            KEY_EOS          => KeyBinding::ClearToEndOfScreen,
            KEY_EOL          => KeyBinding::ClearToEndOfLine,
            KEY_SF           => KeyBinding::ScrollForward,
            KEY_SR           => KeyBinding::ScrollBackward,
            KEY_NPAGE        => KeyBinding::NextPage,
            KEY_PPAGE        => KeyBinding::PreviousPage,
            KEY_STAB         => KeyBinding::SetTab,
            KEY_CTAB         => KeyBinding::ClearTab,
            KEY_CATAB        => KeyBinding::ClearAllTabs,
            KEY_ENTER        => KeyBinding::Enter,
            KEY_PRINT        => KeyBinding::Print,
            KEY_LL           => KeyBinding::HomeDown,
            KEY_A1           => KeyBinding::KeyPadUpperLeft,
            KEY_A3           => KeyBinding::KeyPadUpperRight,
            KEY_B2           => KeyBinding::KeyPadCenter,
            KEY_C1           => KeyBinding::KeyPadLowerLeft,
            KEY_C3           => KeyBinding::KeyPadLowerRight,
            KEY_BTAB         => KeyBinding::BackTab,
            KEY_BEG          => KeyBinding::Begin,
            KEY_CANCEL       => KeyBinding::Cancel,
            KEY_CLOSE        => KeyBinding::Close,
            KEY_COMMAND      => KeyBinding::Command,
            KEY_COPY         => KeyBinding::Copy,
            KEY_CREATE       => KeyBinding::Create,
            KEY_END          => KeyBinding::End,
            KEY_EXIT         => KeyBinding::Exit,
            KEY_FIND         => KeyBinding::Find,
            KEY_HELP         => KeyBinding::Help,
            KEY_MARK         => KeyBinding::Mark,
            KEY_MESSAGE      => KeyBinding::Message,
            KEY_MOVE         => KeyBinding::Move,
            KEY_NEXT         => KeyBinding::Next,
            KEY_OPEN         => KeyBinding::Open,
            KEY_OPTIONS      => KeyBinding::Options,
            KEY_PREVIOUS     => KeyBinding::Previous,
            KEY_REDO         => KeyBinding::Redo,
            KEY_REFERENCE    => KeyBinding::Reference,
            KEY_REFRESH      => KeyBinding::Refresh,
            KEY_REPLACE      => KeyBinding::Replace,
            KEY_RESTART      => KeyBinding::Restart,
            KEY_RESUME       => KeyBinding::Resume,
            KEY_SAVE         => KeyBinding::Save,
            KEY_SBEG         => KeyBinding::ShiftBegin,
            KEY_SCANCEL      => KeyBinding::ShiftCancel,
            KEY_SCOMMAND     => KeyBinding::ShiftCommand,
            KEY_SCOPY        => KeyBinding::ShiftCopy,
            KEY_SCREATE      => KeyBinding::ShiftCreate,
            KEY_SDC          => KeyBinding::ShiftDeleteCharacter,
            KEY_SDL          => KeyBinding::ShiftDeleteLine,
            KEY_SELECT       => KeyBinding::Select,
            KEY_SEND         => KeyBinding::ShiftEnd,
            KEY_SEOL         => KeyBinding::ShiftClearToEndOfLine,
            KEY_SEXIT        => KeyBinding::ShiftExit,
            KEY_SFIND        => KeyBinding::ShiftFind,
            KEY_SHELP        => KeyBinding::ShiftHelp,
            KEY_SHOME        => KeyBinding::ShiftHome,
            KEY_SIC          => KeyBinding::ShiftInsertCharacter,
            KEY_SLEFT        => KeyBinding::ShiftLeftArrow,
            KEY_SMESSAGE     => KeyBinding::ShiftMessage,
            KEY_SMOVE        => KeyBinding::ShiftMove,
            KEY_SNEXT        => KeyBinding::ShiftNext,
            KEY_SOPTIONS     => KeyBinding::ShiftOptions,
            KEY_SPREVIOUS    => KeyBinding::ShiftPrevious,
            KEY_SPRINT       => KeyBinding::ShiftPrint,
            KEY_SREDO        => KeyBinding::ShiftRedo,
            KEY_SREPLACE     => KeyBinding::ShiftReplace,
            KEY_SRIGHT       => KeyBinding::ShiftRightArrow,
            KEY_SRSUME       => KeyBinding::ShiftResume,
            KEY_SSAVE        => KeyBinding::ShiftSave,
            KEY_SSUSPEND     => KeyBinding::ShiftSuspend,
            KEY_SUNDO        => KeyBinding::ShiftUndo,
            KEY_SUSPEND      => KeyBinding::Suspend,
            KEY_UNDO         => KeyBinding::Undo,
            KEY_MOUSE        => KeyBinding::MouseEvent,
            KEY_RESIZE       => KeyBinding::ResizeEvent,
            #[deprecated(since = "0.6.3", note = "this was deprecated as of NCurses API v6.3.20211021")]
            KEY_EVENT        => KeyBinding::Event,
            _                => KeyBinding::UserDefined(key)
        }
    }
}

impl Into<i32> for KeyBinding {
    fn into(self) -> i32 {
        match self {
            KeyBinding::Break                 => KEY_BREAK,
            KeyBinding::SoftReset             => KEY_SRESET,
            KeyBinding::Reset                 => KEY_RESET,
            KeyBinding::DownArrow             => KEY_DOWN,
            KeyBinding::UpArrow               => KEY_UP,
            KeyBinding::LeftArrow             => KEY_LEFT,
            KeyBinding::RightArrow            => KEY_RIGHT,
            KeyBinding::Home                  => KEY_HOME,
            KeyBinding::Backspace             => KEY_BACKSPACE,
            KeyBinding::FunctionKey(key)      => {
                assert!(i32::from(key) <= KEY_F63 - KEY_F0);

                KEY_F0 + i32::from(key)
            },
            KeyBinding::DeleteLine            => KEY_DL,
            KeyBinding::InsertLine            => KEY_IL,
            KeyBinding::DeleteCharacter       => KEY_DC,
            KeyBinding::InsertCharacter       => KEY_IC,
            KeyBinding::InsertMode            => KEY_EIC,
            KeyBinding::Erase                 => KEY_CLEAR,
            KeyBinding::ClearToEndOfScreen    => KEY_EOS,
            KeyBinding::ClearToEndOfLine      => KEY_EOL,
            KeyBinding::ScrollForward         => KEY_SF,
            KeyBinding::ScrollBackward        => KEY_SR,
            KeyBinding::NextPage              => KEY_NPAGE,
            KeyBinding::PreviousPage          => KEY_PPAGE,
            KeyBinding::SetTab                => KEY_STAB,
            KeyBinding::ClearTab              => KEY_CTAB,
            KeyBinding::ClearAllTabs          => KEY_CATAB,
            KeyBinding::Enter                 => KEY_ENTER,
            KeyBinding::Print                 => KEY_PRINT,
            KeyBinding::HomeDown              => KEY_LL,
            KeyBinding::KeyPadUpperLeft       => KEY_A1,
            KeyBinding::KeyPadUpperRight      => KEY_A3,
            KeyBinding::KeyPadCenter          => KEY_B2,
            KeyBinding::KeyPadLowerLeft       => KEY_C1,
            KeyBinding::KeyPadLowerRight      => KEY_C3,
            KeyBinding::BackTab               => KEY_BTAB,
            KeyBinding::Begin                 => KEY_BEG,
            KeyBinding::Cancel                => KEY_CANCEL,
            KeyBinding::Close                 => KEY_CLOSE,
            KeyBinding::Command               => KEY_COMMAND,
            KeyBinding::Copy                  => KEY_COPY,
            KeyBinding::Create                => KEY_CREATE,
            KeyBinding::End                   => KEY_END,
            KeyBinding::Exit                  => KEY_EXIT,
            KeyBinding::Find                  => KEY_FIND,
            KeyBinding::Help                  => KEY_HELP,
            KeyBinding::Mark                  => KEY_MARK,
            KeyBinding::Message               => KEY_MESSAGE,
            KeyBinding::Move                  => KEY_MOVE,
            KeyBinding::Next                  => KEY_NEXT,
            KeyBinding::Open                  => KEY_OPEN,
            KeyBinding::Options               => KEY_OPTIONS,
            KeyBinding::Previous              => KEY_PREVIOUS,
            KeyBinding::Redo                  => KEY_REDO,
            KeyBinding::Reference             => KEY_REFERENCE,
            KeyBinding::Refresh               => KEY_REFRESH,
            KeyBinding::Replace               => KEY_REPLACE,
            KeyBinding::Restart               => KEY_RESTART,
            KeyBinding::Resume                => KEY_RESUME,
            KeyBinding::Save                  => KEY_SAVE,
            KeyBinding::ShiftBegin            => KEY_SBEG,
            KeyBinding::ShiftCancel           => KEY_SCANCEL,
            KeyBinding::ShiftCommand          => KEY_SCOMMAND,
            KeyBinding::ShiftCopy             => KEY_SCOPY,
            KeyBinding::ShiftCreate           => KEY_SCREATE,
            KeyBinding::ShiftDeleteCharacter  => KEY_SDC,
            KeyBinding::ShiftDeleteLine       => KEY_SDL,
            KeyBinding::Select                => KEY_SELECT,
            KeyBinding::ShiftEnd              => KEY_SEND,
            KeyBinding::ShiftClearToEndOfLine => KEY_SEOL,
            KeyBinding::ShiftExit             => KEY_SEXIT,
            KeyBinding::ShiftFind             => KEY_SFIND,
            KeyBinding::ShiftHelp             => KEY_SHELP,
            KeyBinding::ShiftHome             => KEY_SHOME,
            KeyBinding::ShiftInsertCharacter  => KEY_SIC,
            KeyBinding::ShiftLeftArrow        => KEY_SLEFT,
            KeyBinding::ShiftMessage          => KEY_SMESSAGE,
            KeyBinding::ShiftMove             => KEY_SMOVE,
            KeyBinding::ShiftNext             => KEY_SNEXT,
            KeyBinding::ShiftOptions          => KEY_SOPTIONS,
            KeyBinding::ShiftPrevious         => KEY_SPREVIOUS,
            KeyBinding::ShiftPrint            => KEY_SPRINT,
            KeyBinding::ShiftRedo             => KEY_SREDO,
            KeyBinding::ShiftReplace          => KEY_SREPLACE,
            KeyBinding::ShiftRightArrow       => KEY_SRIGHT,
            KeyBinding::ShiftResume           => KEY_SRSUME,
            KeyBinding::ShiftSave             => KEY_SSAVE,
            KeyBinding::ShiftSuspend          => KEY_SSUSPEND,
            KeyBinding::ShiftUndo             => KEY_SUNDO,
            KeyBinding::Suspend               => KEY_SUSPEND,
            KeyBinding::Undo                  => KEY_UNDO,
            KeyBinding::MouseEvent            => KEY_MOUSE,
            KeyBinding::ResizeEvent           => KEY_RESIZE,
            #[deprecated(since = "0.6.3", note = "this was deprecated as of NCurses API v6.3.20211021")]
            KeyBinding::Event                 => KEY_EVENT,
            KeyBinding::UserDefined(key)      => key
        }
    }
}

impl std::fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyBinding::Break                 => write!(f, "Break"),
            KeyBinding::SoftReset             => write!(f, "SoftReset"),
            KeyBinding::Reset                 => write!(f, "Reset"),
            KeyBinding::DownArrow             => write!(f, "DownArrow"),
            KeyBinding::UpArrow               => write!(f, "UpArrow"),
            KeyBinding::LeftArrow             => write!(f, "LeftArrow"),
            KeyBinding::RightArrow            => write!(f, "RightArrow"),
            KeyBinding::Home                  => write!(f, "Home"),
            KeyBinding::Backspace             => write!(f, "Backspace"),
            KeyBinding::FunctionKey(key)      => {
                match key {
                    0  => write!(f, "FunctionKey(0)"),
                    1  => write!(f, "FunctionKey(1)"),
                    2  => write!(f, "FunctionKey(2)"),
                    3  => write!(f, "FunctionKey(3)"),
                    4  => write!(f, "FunctionKey(4)"),
                    5  => write!(f, "FunctionKey(5)"),
                    6  => write!(f, "FunctionKey(6)"),
                    7  => write!(f, "FunctionKey(7)"),
                    8  => write!(f, "FunctionKey(8)"),
                    9  => write!(f, "FunctionKey(9)"),
                    10 => write!(f, "FunctionKey(10)"),
                    11 => write!(f, "FunctionKey(11)"),
                    12 => write!(f, "FunctionKey(12)"),
                    13 => write!(f, "FunctionKey(13)"),
                    14 => write!(f, "FunctionKey(14)"),
                    15 => write!(f, "FunctionKey(15)"),
                    16 => write!(f, "FunctionKey(16)"),
                    17 => write!(f, "FunctionKey(17)"),
                    18 => write!(f, "FunctionKey(18)"),
                    19 => write!(f, "FunctionKey(19)"),
                    20 => write!(f, "FunctionKey(20)"),
                    21 => write!(f, "FunctionKey(21)"),
                    22 => write!(f, "FunctionKey(22)"),
                    23 => write!(f, "FunctionKey(23)"),
                    24 => write!(f, "FunctionKey(24)"),
                    25 => write!(f, "FunctionKey(25)"),
                    26 => write!(f, "FunctionKey(26)"),
                    27 => write!(f, "FunctionKey(27)"),
                    28 => write!(f, "FunctionKey(28)"),
                    29 => write!(f, "FunctionKey(29)"),
                    30 => write!(f, "FunctionKey(30)"),
                    31 => write!(f, "FunctionKey(31)"),
                    32 => write!(f, "FunctionKey(32)"),
                    33 => write!(f, "FunctionKey(33)"),
                    34 => write!(f, "FunctionKey(34)"),
                    35 => write!(f, "FunctionKey(35)"),
                    36 => write!(f, "FunctionKey(36)"),
                    37 => write!(f, "FunctionKey(37)"),
                    38 => write!(f, "FunctionKey(38)"),
                    39 => write!(f, "FunctionKey(39)"),
                    40 => write!(f, "FunctionKey(40)"),
                    41 => write!(f, "FunctionKey(41)"),
                    42 => write!(f, "FunctionKey(42)"),
                    43 => write!(f, "FunctionKey(43)"),
                    44 => write!(f, "FunctionKey(44)"),
                    45 => write!(f, "FunctionKey(45)"),
                    46 => write!(f, "FunctionKey(46)"),
                    47 => write!(f, "FunctionKey(47)"),
                    48 => write!(f, "FunctionKey(48)"),
                    49 => write!(f, "FunctionKey(49)"),
                    50 => write!(f, "FunctionKey(50)"),
                    51 => write!(f, "FunctionKey(51)"),
                    52 => write!(f, "FunctionKey(52)"),
                    53 => write!(f, "FunctionKey(53)"),
                    54 => write!(f, "FunctionKey(54)"),
                    55 => write!(f, "FunctionKey(55)"),
                    56 => write!(f, "FunctionKey(56)"),
                    57 => write!(f, "FunctionKey(57)"),
                    58 => write!(f, "FunctionKey(58)"),
                    59 => write!(f, "FunctionKey(59)"),
                    60 => write!(f, "FunctionKey(60)"),
                    61 => write!(f, "FunctionKey(61)"),
                    62 => write!(f, "FunctionKey(62)"),
                    63 => write!(f, "FunctionKey(63)"),
                    64 => write!(f, "FunctionKey(64)"),
                    _  => write!(f, "FunctionKey(invalid)")
                }
            },
            KeyBinding::DeleteLine            => write!(f, "DeleteLine"),
            KeyBinding::InsertLine            => write!(f, "InsertLine"),
            KeyBinding::DeleteCharacter       => write!(f, "DeleteCharacter"),
            KeyBinding::InsertCharacter       => write!(f, "InsertCharacter"),
            KeyBinding::InsertMode            => write!(f, "InsertMode"),
            KeyBinding::Erase                 => write!(f, "Erase"),
            KeyBinding::ClearToEndOfScreen    => write!(f, "ClearToEndOfScreen"),
            KeyBinding::ClearToEndOfLine      => write!(f, "ClearToEndOfLine"),
            KeyBinding::ScrollForward         => write!(f, "ScrollForward"),
            KeyBinding::ScrollBackward        => write!(f, "ScrollBackward"),
            KeyBinding::NextPage              => write!(f, "NextPage"),
            KeyBinding::PreviousPage          => write!(f, "PreviousPage"),
            KeyBinding::SetTab                => write!(f, "SetTab"),
            KeyBinding::ClearTab              => write!(f, "ClearTab"),
            KeyBinding::ClearAllTabs          => write!(f, "ClearAllTabs"),
            KeyBinding::Enter                 => write!(f, "Enter"),
            KeyBinding::Print                 => write!(f, "Print"),
            KeyBinding::HomeDown              => write!(f, "HomeDown"),
            KeyBinding::KeyPadUpperLeft       => write!(f, "KeyPadUpperLeft"),
            KeyBinding::KeyPadUpperRight      => write!(f, "KeyPadUpperRight"),
            KeyBinding::KeyPadCenter          => write!(f, "KeyPadCenter"),
            KeyBinding::KeyPadLowerLeft       => write!(f, "KeyPadLowerLeft"),
            KeyBinding::KeyPadLowerRight      => write!(f, "KeyPadLowerRight"),
            KeyBinding::BackTab               => write!(f, "BackTab"),
            KeyBinding::Begin                 => write!(f, "Begin"),
            KeyBinding::Cancel                => write!(f, "Cancel"),
            KeyBinding::Close                 => write!(f, "Close"),
            KeyBinding::Command               => write!(f, "Command"),
            KeyBinding::Copy                  => write!(f, "Copy"),
            KeyBinding::Create                => write!(f, "Create"),
            KeyBinding::End                   => write!(f, "End"),
            KeyBinding::Exit                  => write!(f, "Exit"),
            KeyBinding::Find                  => write!(f, "Find"),
            KeyBinding::Help                  => write!(f, "Help"),
            KeyBinding::Mark                  => write!(f, "Mark"),
            KeyBinding::Message               => write!(f, "Message"),
            KeyBinding::Move                  => write!(f, "Move"),
            KeyBinding::Next                  => write!(f, "Next"),
            KeyBinding::Open                  => write!(f, "Open"),
            KeyBinding::Options               => write!(f, "Options"),
            KeyBinding::Previous              => write!(f, "Previous"),
            KeyBinding::Redo                  => write!(f, "Redo"),
            KeyBinding::Reference             => write!(f, "Reference"),
            KeyBinding::Refresh               => write!(f, "Refresh"),
            KeyBinding::Replace               => write!(f, "Replace"),
            KeyBinding::Restart               => write!(f, "Restart"),
            KeyBinding::Resume                => write!(f, "Resume"),
            KeyBinding::Save                  => write!(f, "Save"),
            KeyBinding::ShiftBegin            => write!(f, "ShiftBegin"),
            KeyBinding::ShiftCancel           => write!(f, "ShiftCancel"),
            KeyBinding::ShiftCommand          => write!(f, "ShiftCommand"),
            KeyBinding::ShiftCopy             => write!(f, "ShiftCopy"),
            KeyBinding::ShiftCreate           => write!(f, "ShiftCreate"),
            KeyBinding::ShiftDeleteCharacter  => write!(f, "ShiftDeleteCharacter"),
            KeyBinding::ShiftDeleteLine       => write!(f, "ShiftDeleteLine"),
            KeyBinding::Select                => write!(f, "Select"),
            KeyBinding::ShiftEnd              => write!(f, "ShiftEnd"),
            KeyBinding::ShiftClearToEndOfLine => write!(f, "ShiftClearToEndOfLine"),
            KeyBinding::ShiftExit             => write!(f, "ShiftExit"),
            KeyBinding::ShiftFind             => write!(f, "ShiftFind"),
            KeyBinding::ShiftHelp             => write!(f, "ShiftHelp"),
            KeyBinding::ShiftHome             => write!(f, "ShiftHome"),
            KeyBinding::ShiftInsertCharacter  => write!(f, "ShiftInsertCharacter"),
            KeyBinding::ShiftLeftArrow        => write!(f, "ShiftLeftArrow"),
            KeyBinding::ShiftMessage          => write!(f, "ShiftMessage"),
            KeyBinding::ShiftMove             => write!(f, "ShiftMove"),
            KeyBinding::ShiftNext             => write!(f, "ShiftNext"),
            KeyBinding::ShiftOptions          => write!(f, "ShiftOptions"),
            KeyBinding::ShiftPrevious         => write!(f, "ShiftPrevious"),
            KeyBinding::ShiftPrint            => write!(f, "ShiftPrint"),
            KeyBinding::ShiftRedo             => write!(f, "ShiftRedo"),
            KeyBinding::ShiftReplace          => write!(f, "ShiftReplace"),
            KeyBinding::ShiftRightArrow       => write!(f, "ShiftRightArrow"),
            KeyBinding::ShiftResume           => write!(f, "ShiftResume"),
            KeyBinding::ShiftSave             => write!(f, "ShiftSave"),
            KeyBinding::ShiftSuspend          => write!(f, "ShiftSuspend"),
            KeyBinding::ShiftUndo             => write!(f, "ShiftUndo"),
            KeyBinding::Suspend               => write!(f, "Suspend"),
            KeyBinding::Undo                  => write!(f, "Undo"),
            KeyBinding::MouseEvent            => write!(f, "MouseEvent"),
            KeyBinding::ResizeEvent           => write!(f, "ResizeEvent"),
            KeyBinding::Event                 => write!(f, "Event"),
            KeyBinding::UserDefined(key)      => {
                match key {
                    0  => write!(f, "Unknown(0)"),
                    1  => write!(f, "Unknown(1)"),
                    2  => write!(f, "Unknown(2)"),
                    3  => write!(f, "Unknown(3)"),
                    4  => write!(f, "Unknown(4)"),
                    5  => write!(f, "Unknown(5)"),
                    6  => write!(f, "Unknown(6)"),
                    7  => write!(f, "Unknown(7)"),
                    8  => write!(f, "Unknown(8)"),
                    9  => write!(f, "Unknown(9)"),
                    10 => write!(f, "Unknown(10)"),
                    11 => write!(f, "Unknown(11)"),
                    12 => write!(f, "Unknown(12)"),
                    13 => write!(f, "Unknown(13)"),
                    14 => write!(f, "Unknown(14)"),
                    15 => write!(f, "Unknown(15)"),
                    16 => write!(f, "Unknown(16)"),
                    17 => write!(f, "Unknown(17)"),
                    18 => write!(f, "Unknown(18)"),
                    19 => write!(f, "Unknown(19)"),
                    20 => write!(f, "Unknown(20)"),
                    21 => write!(f, "Unknown(21)"),
                    22 => write!(f, "Unknown(22)"),
                    23 => write!(f, "Unknown(23)"),
                    24 => write!(f, "Unknown(24)"),
                    25 => write!(f, "Unknown(25)"),
                    26 => write!(f, "Unknown(26)"),
                    27 => write!(f, "Unknown(27)"),
                    28 => write!(f, "Unknown(28)"),
                    29 => write!(f, "Unknown(29)"),
                    30 => write!(f, "Unknown(30)"),
                    31 => write!(f, "Unknown(31)"),
                    _  => write!(f, "Unknown(>31)")
                }
            }
        }
    }
}