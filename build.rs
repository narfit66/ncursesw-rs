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

use std::{env, fs::{File, remove_file}, io::Write, path::{Path, PathBuf}, process::Command};

const NCURSES_VERSION: &str = "v6.3";

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
// compile NCurses locally to the crate with all the features required so we can statically link it.
// this will enable the crate to be independent of any other NCurses installation with a different
// ABI or feature set installed by the distribution/operating system.
fn main() {
    // the cargo manifest directory.
    let cargo_manifest_dir = &PathBuf::from(env::var("CARGO_MANIFEST_DIR")
        .expect("environment variable 'CARGO_MANIFEST_DIR' is undefined."));

    // the output directory.
    let out_dir = &PathBuf::from(env::var("OUT_DIR")
        .expect("environment variable 'OUT_DIR' is undefined."));

    // our upstream working directory for NCurses.
    let upstream_path = &Path::new(out_dir).join("ncursesw-upstream");

    // our upstream installation directory for NCurses.
    let install_path = &Path::new(out_dir).join("local");

    // if we haven't got a .git directory then clone NCurses from it's repo and panic if it can't
    // be done for any reason (this will create the directory pointed to by 'upstream_path').
    if !upstream_path.join(".git").exists() {
        let status = Command::new("git")
            .args(&["clone",
                    "--branch",
                    NCURSES_VERSION,
                    "--depth",
                    "1",
                    "https://github.com/mirror/ncurses.git",
                    &format!("{}", upstream_path.display())])
            .status()
            .expect("git clone command failed!");

        if !status.success() {
            panic!("git clone of ncurses was not successful!");
        }
    }

    // if we don't already have a makefile i.e. we haven't already configured NCurses then
    // configure with the features required and panic if it can't be done for any reason.
    if !upstream_path.join("Makefile").exists() {
        let status = Command::new("./configure")
            .current_dir(upstream_path)
            .args(&[&format!("--prefix={}", install_path.display()),
                    "--without-ada",
                    "--without-cxx-binding",
                    "--with-default-terminfo-dir=/usr/share/terminfo",
                    "--disable-db-install",
                    "--without-manpages",
                    "--without-progs",
                    "--without-tack",
                    "--without-tests",
                    "--enable-sp-funcs",
                    "--enable-widec",
                    "--enable-ext-colors",
                    "--enable-ext-mouse",
                    "--enable-ext-putwin",
                    "CPPFLAGS=-P"])
            .status()
            .expect("configure command failed!");

        if !status.success() {
            panic!("configure of ncurses was not successful!");
        }
    }

    // make NCurses and panic if it can't be done for any reason.
    let status = Command::new("make")
        .current_dir(upstream_path)
        .status()
        .expect("make command failed!");

    if !status.success() {
        panic!("make of ncurses was not successful!");
    }

    // if the install_path directory doesn't exist then that means we haven't done a 'make install'
    // so do the install and panic if it can't be done for any reason.
    if !install_path.exists() {
        let status = Command::new("make")
            .current_dir(upstream_path)
            .arg("install")
            .status()
            .expect("make install command failed!");

        if !status.success() {
            panic!("make install of ncurses was not successful!");
        }
    }

    // say to the compiler where our native libraries are.
    println!("cargo:rustc-link-search=native={}/lib", install_path.display());

    // say to the compiler which libraries we won't to link to.
    println!("cargo:rustc-link-lib=formw");
    println!("cargo:rustc-link-lib=menuw");
    println!("cargo:rustc-link-lib=panelw");
    println!("cargo:rustc-link-lib=ncursesw");

    // our 'C' wrapper file name for bindgen to process.
    let wrapper_fname = "wrapper.h";
    let wrapper_fname_path = &cargo_manifest_dir.join(wrapper_fname);

    // define our wrapper file contents, this is derived from our asset file replacing all occurances
    // of %include% with the full path that the installed include files will be located in.
    let wrapper_contents = {
        include_str!("assets/wrapper.h")
    }.replace("%include%", install_path.join("include").join("ncursesw").to_str().expect("unable to build wrapper contents!"));

    // create our wrapper file...
    let mut wrapper_file = File::create(wrapper_fname_path)
        .expect("unable to create wrapper file!");

    // and write it's contents.
    wrapper_file.write_all(wrapper_contents.as_bytes())
        .expect("unable to write wrapper file!");

    // sync the file i.e. make sure the contents have been written to disk.
    wrapper_file.sync_all()
        .expect("unable to sync wrapper file to disk!");

    // build the crates bindings using bindgen and panic if we can't do it for any reason.
    let bindings = bindgen::Builder::default()
        .header(wrapper_fname)                  // 'C' header file
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
        .expect("unable to generate bindings!");

    // write our bindings file and panic if we can't do it for any reason.
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("unable to write bindings!");

    // clean up after ourselfs by removing our processed `wrapper.h` file and panic if we can't do
    // it for any reason.
    remove_file(wrapper_fname_path)
        .expect("unable to remove wrapper file!");
}
