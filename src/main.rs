// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![feature(box_syntax)]
// In an ideal world there would be a stable well documented set of crates containing a specific
// version of the Rust compiler along with its sources and debug information. We'd then just get
// those from crate.io and merely go on our way as just another Rust application. Rust compiler
// upgrades will be non events for Mirai until it is ready to jump to another release and old
// versions of Mirai will continue to work just as before.
//
// In the current world, however, we have to use the following hacky feature to get access to a
// private and not very stable set of APIs from whatever compiler is in the path when we run Mirai.
// While pretty bad, it is a lot less bad than having to write our own compiler, so here goes.
#![feature(rustc_private)]

extern crate mirai;
extern crate rustc_driver;

use mirai::callbacks;
use mirai::utils;
use std::path::Path;

#[cfg_attr(tarpaulin, skip)]
fn main() {
    rustc_driver::run(|| {
        rustc_driver::init_rustc_env_logger();
        let mut command_line_arguments: Vec<_> = std::env::args().collect();

        // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
        // We're invoking the compiler programmatically, so we ignore this.
        if command_line_arguments.len() > 1
            && Path::new(&command_line_arguments[1]).file_stem() == Some("rustc".as_ref())
        {
            // we still want to be able to invoke it normally though
            command_line_arguments.remove(1);
        }

        // Tell compiler where to find the std library and so on.
        // Have to do this here because the standard rustc driver does it and we are replacing it.
        command_line_arguments.push(String::from("--sysroot"));
        command_line_arguments.push(utils::find_sysroot());
        // todo figure out how and where summaries will be written to
        rustc_driver::run_compiler(
            &command_line_arguments,
            box callbacks::MiraiCallbacks::default(),
            None, // use default file loader
            None, // emit output to default destination
        )
    });
}
