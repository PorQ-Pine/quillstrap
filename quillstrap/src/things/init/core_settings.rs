use crate::prelude::*;

pub const CORE_SETTINGS_BINARY: &str = "core_settings";

#[derive(Clone, Copy, Default, Debug)]
pub struct CoreSettings;

impl SetupThing for CoreSettings {
    fn name(&self) -> &'static str {
        "core_settings"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "core-settings"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &_options);
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists(&format!("out/{}", &CORE_SETTINGS_BINARY))
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        run_command("cargo clean", _options.config.command_output).expect("Failed to clean core-settings");
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p("out/");

        let full_path = get_path_of_thing_native(self, _options);

        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var(
            "PKG_CONFIG_PATH",
            &format!("{}../sysroot/usr/lib/pkgconfig", &full_path),
        );
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var(
            "PKG_CONFIG_SYSROOT_DIR",
            &format!("{}../sysroot", &full_path),
        );
        set_var(
            "OPENSSL_INCLUDE_DIR",
            &format!("{}../sysroot/usr/include/openssl", &full_path),
        );
        set_var(
            "RUSTFLAGS",
            &format!(
                "-C target-feature=-crt-static -L {}../sysroot/usr/lib/",
                &full_path
            ),
        );

        run_command(
            &format!(
                "cargo zigbuild --release --target aarch64-unknown-linux-musl"
            ),
            _options.config.command_output,
        )
        .unwrap();
        copy_file(
            &format!(
                "{}target/aarch64-unknown-linux-musl/release/{}", &full_path, &CORE_SETTINGS_BINARY
            ),
            &format!("out/{}", &CORE_SETTINGS_BINARY),
        )
        .unwrap();

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("OPENSSL_INCLUDE_DIR", "");
        set_var("RUSTFLAGS", "");

        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
