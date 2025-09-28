use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Greetd;

impl SetupThing for Greetd {
    fn name(&self) -> &'static str {
        "greetd"
    }

    fn path(&self) -> &'static str {
        "os/low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "greetd"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        dir_change("greetd");
        run_command("cargo clean", _options.config.command_output).unwrap();
        dir_change("../");
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        dir_change("greetd");

        let full_path = format!("{}greetd/", get_path_of_thing_native(self, _options));
        info!("Full path is: {}", full_path);
        
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var("PKG_CONFIG_SYSROOT_DIR", &format!("{}../../rootfs_sysroot/sysroot", full_path));
        /*
        set_var(
            "PKG_CONFIG_PATH",
            "../../rootfs_sysroot/sysroot/usr/lib/aarch64-linux-gnu/pkgconfig",
        );
        */
        set_var("RUSTFLAGS", &format!("-L {}../../rootfs_sysroot/sysroot/usr/lib64", full_path));

        run_command(
            "cargo zigbuild --release --target aarch64-unknown-linux-gnu",
            _options.config.command_output,
        )
        .unwrap();

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("RUSTFLAGS", "");

        dir_change("../");
        mkdir_p("out");
        copy_file(
            "target/aarch64-unknown-linux-gnu/release/greetd",
            "out/greetd",
        )
        .unwrap();
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
