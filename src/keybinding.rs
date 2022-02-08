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
        let binding = match self {
            KeyBinding::Break                 => String::from("Break"),
            KeyBinding::SoftReset             => String::from("SoftReset"),
            KeyBinding::Reset                 => String::from("Reset"),
            KeyBinding::DownArrow             => String::from("DownArrow"),
            KeyBinding::UpArrow               => String::from("UpArrow"),
            KeyBinding::LeftArrow             => String::from("LeftArrow"),
            KeyBinding::RightArrow            => String::from("RightArrow"),
            KeyBinding::Home                  => String::from("Home"),
            KeyBinding::Backspace             => String::from("Backspace"),
            KeyBinding::FunctionKey(key)      => format!("FunctionKey({})", key),
            KeyBinding::DeleteLine            => String::from("DeleteLine"),
            KeyBinding::InsertLine            => String::from("InsertLine"),
            KeyBinding::DeleteCharacter       => String::from("DeleteCharacter"),
            KeyBinding::InsertCharacter       => String::from("InsertCharacter"),
            KeyBinding::InsertMode            => String::from("InsertMode"),
            KeyBinding::Erase                 => String::from("Erase"),
            KeyBinding::ClearToEndOfScreen    => String::from("ClearToEndOfScreen"),
            KeyBinding::ClearToEndOfLine      => String::from("ClearToEndOfLine"),
            KeyBinding::ScrollForward         => String::from("ScrollForward"),
            KeyBinding::ScrollBackward        => String::from("ScrollBackward"),
            KeyBinding::NextPage              => String::from("NextPage"),
            KeyBinding::PreviousPage          => String::from("PreviousPage"),
            KeyBinding::SetTab                => String::from("SetTab"),
            KeyBinding::ClearTab              => String::from("ClearTab"),
            KeyBinding::ClearAllTabs          => String::from("ClearAllTabs"),
            KeyBinding::Enter                 => String::from("Enter"),
            KeyBinding::Print                 => String::from("Print"),
            KeyBinding::HomeDown              => String::from("HomeDown"),
            KeyBinding::KeyPadUpperLeft       => String::from("KeyPadUpperLeft"),
            KeyBinding::KeyPadUpperRight      => String::from("KeyPadUpperRight"),
            KeyBinding::KeyPadCenter          => String::from("KeyPadCenter"),
            KeyBinding::KeyPadLowerLeft       => String::from("KeyPadLowerLeft"),
            KeyBinding::KeyPadLowerRight      => String::from("KeyPadLowerRight"),
            KeyBinding::BackTab               => String::from("BackTab"),
            KeyBinding::Begin                 => String::from("Begin"),
            KeyBinding::Cancel                => String::from("Cancel"),
            KeyBinding::Close                 => String::from("Close"),
            KeyBinding::Command               => String::from("Command"),
            KeyBinding::Copy                  => String::from("Copy"),
            KeyBinding::Create                => String::from("Create"),
            KeyBinding::End                   => String::from("End"),
            KeyBinding::Exit                  => String::from("Exit"),
            KeyBinding::Find                  => String::from("Find"),
            KeyBinding::Help                  => String::from("Help"),
            KeyBinding::Mark                  => String::from("Mark"),
            KeyBinding::Message               => String::from("Message"),
            KeyBinding::Move                  => String::from("Move"),
            KeyBinding::Next                  => String::from("Next"),
            KeyBinding::Open                  => String::from("Open"),
            KeyBinding::Options               => String::from("Options"),
            KeyBinding::Previous              => String::from("Previous"),
            KeyBinding::Redo                  => String::from("Redo"),
            KeyBinding::Reference             => String::from("Reference"),
            KeyBinding::Refresh               => String::from("Refresh"),
            KeyBinding::Replace               => String::from("Replace"),
            KeyBinding::Restart               => String::from("Restart"),
            KeyBinding::Resume                => String::from("Resume"),
            KeyBinding::Save                  => String::from("Save"),
            KeyBinding::ShiftBegin            => String::from("ShiftBegin"),
            KeyBinding::ShiftCancel           => String::from("ShiftCancel"),
            KeyBinding::ShiftCommand          => String::from("ShiftCommand"),
            KeyBinding::ShiftCopy             => String::from("ShiftCopy"),
            KeyBinding::ShiftCreate           => String::from("ShiftCreate"),
            KeyBinding::ShiftDeleteCharacter  => String::from("ShiftDeleteCharacter"),
            KeyBinding::ShiftDeleteLine       => String::from("ShiftDeleteLine"),
            KeyBinding::Select                => String::from("Select"),
            KeyBinding::ShiftEnd              => String::from("ShiftEnd"),
            KeyBinding::ShiftClearToEndOfLine => String::from("ShiftClearToEndOfLine"),
            KeyBinding::ShiftExit             => String::from("ShiftExit"),
            KeyBinding::ShiftFind             => String::from("ShiftFind"),
            KeyBinding::ShiftHelp             => String::from("ShiftHelp"),
            KeyBinding::ShiftHome             => String::from("ShiftHome"),
            KeyBinding::ShiftInsertCharacter  => String::from("ShiftInsertCharacter"),
            KeyBinding::ShiftLeftArrow        => String::from("ShiftLeftArrow"),
            KeyBinding::ShiftMessage          => String::from("ShiftMessage"),
            KeyBinding::ShiftMove             => String::from("ShiftMove"),
            KeyBinding::ShiftNext             => String::from("ShiftNext"),
            KeyBinding::ShiftOptions          => String::from("ShiftOptions"),
            KeyBinding::ShiftPrevious         => String::from("ShiftPrevious"),
            KeyBinding::ShiftPrint            => String::from("ShiftPrint"),
            KeyBinding::ShiftRedo             => String::from("ShiftRedo"),
            KeyBinding::ShiftReplace          => String::from("ShiftReplace"),
            KeyBinding::ShiftRightArrow       => String::from("ShiftRightArrow"),
            KeyBinding::ShiftResume           => String::from("ShiftResume"),
            KeyBinding::ShiftSave             => String::from("ShiftSave"),
            KeyBinding::ShiftSuspend          => String::from("ShiftSuspend"),
            KeyBinding::ShiftUndo             => String::from("ShiftUndo"),
            KeyBinding::Suspend               => String::from("Suspend"),
            KeyBinding::Undo                  => String::from("Undo"),
            KeyBinding::MouseEvent            => String::from("MouseEvent"),
            KeyBinding::ResizeEvent           => String::from("ResizeEvent"),
            KeyBinding::Event                 => String::from("Event"),
            KeyBinding::UserDefined(key)      => format!("UserDefined({})", key)
        };

        write!(f, "{}", binding)
    }
}