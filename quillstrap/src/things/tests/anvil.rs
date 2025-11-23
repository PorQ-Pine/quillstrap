use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Anvil;

impl SetupThing for Anvil {
    fn name(&self) -> &'static str {
        "anvil"
    }

    fn path(&self) -> &'static str {
        "tests/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "smithay"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    // no one cares
    fn is_built(&self) -> bool {
        true
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        dir_change("anvil");

        let full_path = get_path_of_thing_native(self, _options);
        warn!("full_path: {}", full_path);

        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var(
            "PKG_CONFIG_PATH",
            &format!(
                "{}../../os/low/rootfs_sysroot/sysroot/usr/lib64/pkgconfig/:{}../../os/low/rootfs_sysroot/sysroot/usr/share/pkgconfig",
                full_path, full_path
            ),
        );
        set_var(
            "PKG_CONFIG_SYSROOT_DIR",
            &format!("{}../../os/low/rootfs_sysroot/sysroot", full_path),
        );
        set_var(
            "RUSTFLAGS",
            &format!(
                "-L {}../../os/low/rootfs_sysroot/sysroot/usr/lib64",
                full_path
            ),
        );
        set_var(
            "PKG_CONFIG_LIBDIR",
            &format!(
                "{}../../os/low/rootfs_sysroot/sysroot/usr/lib64/pkgconfig/:{}../../os/low/rootfs_sysroot/sysroot/usr/share/pkgconfig",
                full_path, full_path
            ),
        );

        run_command(
            "cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.41",
            _options.config.command_output,
        )
        .unwrap();

        dir_change("../");
        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("RUSTFLAGS", "");

        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
