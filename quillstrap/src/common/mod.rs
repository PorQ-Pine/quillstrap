use crate::prelude::*;
use std::{thread, time};

pub mod cli;
pub mod commands;
pub mod git;
pub mod io;
pub mod serial;
pub mod deploy;
pub mod mounts;
pub mod signing;
pub mod partitions;
pub mod network;
pub mod ssh;

// TODO: make this show on the cli as a animation, then it dissapears
pub fn sleep_millis(ms: u64) {
    let ten_millis = time::Duration::from_millis(ms);
    thread::sleep(ten_millis);
}

// With / at the end
pub fn get_path_of_thing(thing: &TraitWrapper, options: &crate::Options) -> String {
    format!(
        "{}{}/{}{}/",
        options.path_of_repo,
        MAIN_BUILD_DIR,
        thing.path(),
        thing.name()
    )
}

// With / at the end
pub fn get_path_of_thing_native<T: SetupThing>(thing: &T, options: &crate::Options) -> String {
    format!(
        "{}{}/{}{}/",
        options.path_of_repo,
        MAIN_BUILD_DIR,
        thing.path(),
        thing.name()
    )
}
