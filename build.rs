/*
    build.rs

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

extern crate bindgen;
extern crate pkg_config;

use std::{env, path::PathBuf};

use pkg_config::Library;

#[derive(Debug)]
pub struct Fix753 { }

// Workaround for rust-bindgen#753
impl bindgen::callbacks::ParseCallbacks for Fix753 {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        Some(original_item_name.trim_start_matches("Fix753_").to_owned())
    }
}

fn find_library(name: &str) -> Option<Library> {
    if let Ok(lib) = pkg_config::probe_library(name) {
        return Some(lib);
    }

    None
}

fn main() {
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");

    find_library("panelw").expect("unable to find 'panelw' library");
    find_library("menuw").expect("unable to find 'menuw' library");
    find_library("formw").expect("unable to find 'formw' library");

    let ncursesw_lib = pkg_config::Config::new()
        .atleast_version("6.1")
        .probe("ncursesw");

    if let Ok(rustc_link_lib) =  env::var("NCURSESW_RUSTC_LINK_LIB") {
        println!("cargo:rustc-link-lib={}", rustc_link_lib);
    } else {
        ncursesw_lib.expect("unable to find 'ncursesw' library");
    }

    if let Ok(rustc_flags) = env::var("NCURSESW_RUSTC_FLAGS") {
        println!("cargo:rustc-flags={}", rustc_flags);
    }

    //

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")                    // 'c' header file
        // NCurses core functions
        .blacklist_function("getcchar")         // blacklisted to implement our own function
        .blacklist_function("ripoffline")       // blacklisted to implement our own function
        // NCurses menu types.
        .blacklist_type("ITEM")                 // blacklisted to implement our own type
        .blacklist_type("MENU")                 // blacklisted to implement our own type
        .blacklist_type("Menu_Hook")            // blacklisted to implement our own type
        // NCurses menu functions.
        .blacklist_function("item_init")        // blacklisted to implement our own function
        .blacklist_function("item_term")        // blacklisted to implement our own function
        .blacklist_function("menu_init")        // blacklisted to implement our own function
        .blacklist_function("menu_term")        // blacklisted to implement our own function
        .blacklist_function("set_item_init")    // blacklisted to implement our own function
        .blacklist_function("set_item_term")    // blacklisted to implement our own function
        .blacklist_function("set_menu_init")    // blacklisted to implement our own function
        .blacklist_function("set_menu_term")    // blacklisted to implement our own function
        //
        .parse_callbacks(Box::new(Fix753 { }))  // parse output to deal with rust-bindgen#753
        .generate()                             // generate the binding
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
