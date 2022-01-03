/*
    build.rs

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

#![allow(unused_imports)]
#![allow(dead_code)]

extern crate bindgen;

use std::{env, path::{Path, PathBuf}, process::Command};

const NCURSES_VERSION: &'static str = "v6.1";

#[derive(Debug)]
pub struct Fix753 { }

// Workaround for rust-bindgen#753
impl bindgen::callbacks::ParseCallbacks for Fix753 {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        Some(original_item_name.trim_start_matches("Fix753_").to_owned())
    }
}

#[cfg(feature = "docs-rs")]
fn main() { } // Skip the build script when the doc is building.

#[cfg(not(feature = "docs-rs"))]
fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR")
        .expect("Environment variable 'OUT_DIR' is undefined."));

    // out upstream working directory for NCurses.
    let clone_path = Path::new(&out_path).join("ncursesw-upstream");

    // if we haven't got a .git directory then clone NCurses from it's repo and panic if we can't
    // get it for any reason.
    if !clone_path.join(".git").exists() {
        let status = Command::new("git")
            .args(&["clone",
                    "--branch",
                    NCURSES_VERSION,
                    "--depth",
                    "1",
                    "https://github.com/mirror/ncurses.git",
                    clone_path.to_str().unwrap()])
            .status().unwrap();

        if !status.success() {
            panic!("git clone of ncurses was not successful!");
        }
    }

    // if we don't already have a makefile i.e. we haven't already configured NCurses then
    // configure with the features required and panic if we can't do it for any reason.
    if !clone_path.join("Makefile").exists() {
        let status = Command::new("./configure")
            .current_dir(clone_path.clone())
            .args(&["--prefix=".to_owned() + &clone_path.to_str().unwrap(),
                    "--without-ada".to_owned(),
                    "--without-cxx-binding".to_owned(),
                    "--without-progs".to_owned(),
                    "--without-tack".to_owned(),
                    "--without-tests".to_owned(),
                    "--enable-sp-funcs".to_owned(),
                    "--enable-widec".to_owned(),
                    "--enable-ext-colors".to_owned(),
                    "--enable-ext-mouse".to_owned(),
                    "--enable-ext-putwin".to_owned(),
                    "CPPFLAGS=-P".to_owned()])
            .status().unwrap();

        if !status.success() {
            panic!("configure of ncurses was not successful!");
        }
    }

    // make NCurses and panic if can't do it for any reason.
    let status = Command::new("make")
        .current_dir(clone_path.clone())
        .status().unwrap();

    if !status.success() {
        panic!("make of ncurses was not successful!");
    }

    // say to the compiler where our native libraries are.
    println!("cargo:rustc-link-search=native={}/lib", clone_path.display());

    // say to the compiler which libraries we won't to link to.
    println!("cargo:rustc-link-lib=formw");
    println!("cargo:rustc-link-lib=menuw");
    println!("cargo:rustc-link-lib=panelw");
    println!("cargo:rustc-link-lib=ncursesw");

    // build the crates bindings using bindgen and panic if we can't do it for any reason.
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")                    // 'C' header file
        // NCurses core functions
        .blocklist_function("getcchar")         // blacklisted to implement our own function
        .blocklist_function("ripoffline")       // blacklisted to implement our own function
        .blocklist_function("ripoffline_sp")    // blacklisted to implement our own function
        // NCurses menu types.
        .blocklist_type("ITEM")                 // blacklisted to implement our own type
        .blocklist_type("MENU")                 // blacklisted to implement our own type
        //
        .parse_callbacks(Box::new(Fix753 { }))  // parse output to deal with rust-bindgen#753
        .generate()                             // generate the binding
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
