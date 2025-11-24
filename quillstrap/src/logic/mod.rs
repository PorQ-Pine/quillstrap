use crate::{logic::{auto_mode::auto_main, manual_mode::manual_main}, prelude::*};

pub mod auto_mode;
pub mod manual_mode;
pub mod d2_graph;

pub fn choose_mode(options: Options) {
    if options.args.manual_mode {
        manual_main(options);
    } else {
        auto_main(options);
    }
}

pub const MAIN_BUILD_DIR: &str = "build_all";
// We assume we runned simple cargo run, nothing fancy
pub fn things_setup() {
    dir_change("../");
    mkdir_p(MAIN_BUILD_DIR);
    dir_change(MAIN_BUILD_DIR);
}
