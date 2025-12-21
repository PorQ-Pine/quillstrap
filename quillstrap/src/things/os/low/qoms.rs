use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct Qoms;

impl SetupThing for Qoms {
    fn name(&self) -> &'static str {
        "qoms"
    }

    fn path(&self) -> &'static str {
        "os/low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["rootfs_sysroot"]
    }

    fn git(&self) -> &'static str {
        "qoms"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("out/qoms")
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        dir_change("qoms");
        run_command("cargo clean", _options.config.command_output).unwrap();
        dir_change("../");
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var("PKG_CONFIG_SYSROOT_DIR", "../../rootfs_sysroot/sysroot");
        /*
        set_var(
            "PKG_CONFIG_PATH",
            "../../rootfs_sysroot/sysroot/usr/lib/aarch64-linux-gnu/pkgconfig",
        );
        */
        set_var("RUSTFLAGS", "-L ../../rootfs_sysroot/sysroot/usr/lib64");

        dir_change("qoms");
        run_command(
            "cargo zigbuild --release --target aarch64-unknown-linux-gnu",
            _options.config.command_output,
        )
        .unwrap();
        dir_change("../");

        dir_change("qoms_coms_bin");
        run_command(
            "cargo zigbuild --release --target aarch64-unknown-linux-gnu",
            _options.config.command_output,
        )
        .unwrap();
        dir_change("../");

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("RUSTFLAGS", "");

        mkdir_p("out");
        copy_file(
            "qoms/target/aarch64-unknown-linux-gnu/release/qoms",
            "out/qoms",
        )
        .unwrap();
            copy_file(
            "qoms_coms_bin/target/aarch64-unknown-linux-gnu/release/send_qoms",
            "out/send_qoms",
        )
        .unwrap();

        copy_dir_content("scripts/", "out/");
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let port = _options.config.rootfs_options.deploy_ssh_port;
        ssh_execute("systemctl stop qoms", port, _options);
        ssh_execute("killall -9 qoms", port, _options);
        ssh_execute("rm -rf /opt/qoms/*", port, _options);
        ssh_send("out/*", "/opt/qoms/", port, _options);
        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        let port = _options.config.rootfs_options.deploy_ssh_port;
        ssh_execute("systemctl stop qoms", port, _options);
        ssh_execute("killall -9 qoms", port, _options);
        ssh_execute("/opt/qoms/qoms", port, _options);
        Ok(())
    }
}
