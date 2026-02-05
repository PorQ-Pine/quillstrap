use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct EwwNiriToolbar;

impl SetupThing for EwwNiriToolbar {
    fn name(&self) -> &'static str {
        "eww_niri_toolbar"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["rootfs_sysroot"]
    }

    fn git(&self) -> &'static str {
        "eww-niri-toolbar"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("target/aarch64-unknown-linux-gnu/release/eww-niri-taskbar")
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let full_path = get_path_of_thing_native(self, _options);
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var(
            "PKG_CONFIG_PATH",
            &format!(
                "{}../../low/rootfs_sysroot/sysroot/usr/lib64/pkgconfig/:{}../../low/rootfs_sysroot/sysroot/usr/share/pkgconfig",
                full_path, full_path
            ),
        );
        set_var(
            "PKG_CONFIG_SYSROOT_DIR",
            &format!("{}../../low/rootfs_sysroot/sysroot", full_path),
        );
        set_var(
            "RUSTFLAGS",
            &format!("-L {}../../low/rootfs_sysroot/sysroot/usr/lib64", full_path),
        );
        set_var(
            "PKG_CONFIG_LIBDIR",
            &format!(
                "{}../../low/rootfs_sysroot/sysroot/usr/lib64/pkgconfig/:{}../../low/rootfs_sysroot/sysroot/usr/share/pkgconfig",
                full_path, full_path
            ),
        );

        run_command(
            "cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.41 --no-default-features",
            _options.config.command_output,
        )
        .unwrap();

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("RUSTFLAGS", "");
        set_var("PKG_CONFIG_LIBDIR", "");

        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let port = _options.config.rootfs_options.deploy_ssh_port;
        ssh_execute("killall -9 eww eww-niri-taskbar", port, _options);
        ssh_execute("rm -rf /usr/bin/eww-niri-taskbar", port, _options);
        ssh_send(
            "target/aarch64-unknown-linux-gnu/release/eww-niri-taskbar",
            "/usr/bin/eww-niri-taskbar",
            port,
            _options,
        );
        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
