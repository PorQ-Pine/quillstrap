use crate::prelude::*;

pub fn target_check(target: &str) {
    let output = run_command_get_output("rustup target list");
    for line in output.split("\n") {
        if line.contains(target) {
            if !line.contains("installed") {
                warn!("Fallback: target {} is not installed, installing", target);
                run_command(&format!("rustup target add {}", target), true).ok();
            }
        }
    }
}

pub fn manage_fallback(options: &Options) {
    // For things that break, no time to proper debug
    if !options.args.build.is_empty() {
        target_check("aarch64-unknown-linux-musl");
        target_check("aarch64-unknown-linux-gnu");
    }
}
