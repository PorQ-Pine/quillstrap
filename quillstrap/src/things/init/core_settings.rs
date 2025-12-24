use crate::prelude::*;

pub const CORE_SETTINGS_BINARY: &str = "core_settings";
pub const CORE_SETTINGS_SRC_DIR: &str = CORE_SETTINGS_BINARY;

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
        vec!["quill_init", "kernel", "initrd", "rootfs"]
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
        run_command("cargo clean", _options.config.command_output)
            .expect("Failed to clean core-settings");
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let cur_dir = dir_current();
        mkdir_p("out/");

        dir_change(&CORE_SETTINGS_SRC_DIR);

        let mut features: Vec<&str> = vec![];

        if _options.config.unrestricted {
            features.push("free_roam");
        }
        if _options.config.unsecure_debug {
            features.push("debug");
        }

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

        if _options.args.quill_init_options.qi_ssh_build {
            run_command(
                &format!(
                    "cargo zigbuild --target aarch64-unknown-linux-musl --features={}",
                    features.join(",")
                ),
                _options.config.command_output,
            )
            .unwrap();
            copy_file(
                &format!(
                    "target/aarch64-unknown-linux-musl/debug/{}",
                    &CORE_SETTINGS_BINARY
                ),
                &format!("../out/{}", &CORE_SETTINGS_BINARY),
            )
            .unwrap();
        } else {
            run_command(
                &format!(
                    "cargo zigbuild --release --target aarch64-unknown-linux-musl --features={}",
                    features.join(",")
                ),
                _options.config.command_output,
            )
            .unwrap();
            copy_file(
                &format!(
                    "target/aarch64-unknown-linux-musl/release/{}",
                    &CORE_SETTINGS_BINARY
                ),
                &format!("../out/{}", &CORE_SETTINGS_BINARY),
            )
            .unwrap();
        }

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("OPENSSL_INCLUDE_DIR", "");
        set_var("RUSTFLAGS", "");

        dir_change(&cur_dir);
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let ip_str = _options
            .config
            .deploy_ip_addr
            .map(|b| b.to_string())
            .join(".");

        run_command(
            &format!(
                "ssh -p {} root@{} killall {}",
                &_options.config.qinit_options.deploy_ssh_port, &ip_str, &CORE_SETTINGS_BINARY
            ),
            false,
        )
        .unwrap();
        run_shell_command(
            &format!(
                "lftp {}:{} -e 'put out/{} -o /tmp/{}; bye'",
                &ip_str,
                &_options.config.qinit_options.deploy_ftp_port,
                &CORE_SETTINGS_BINARY,
                &CORE_SETTINGS_BINARY,
            ),
            true,
        )
        .unwrap();
        run_shell_command(
            &format!(
                "ssh -t -p {} root@{} 'chmod 755 /tmp/{} && RUST_LOG=debug SLINT_KMS_ROTATION=270 SLINT_BACKEND_LINUXFB=1 /tmp/{}'",
                &_options.config.qinit_options.deploy_ssh_port,
                &ip_str,
                &CORE_SETTINGS_BINARY,
                &CORE_SETTINGS_BINARY,
            ),
            true,
        )
        .unwrap();

        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
