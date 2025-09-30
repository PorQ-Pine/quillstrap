use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Niri;

impl SetupThing for Niri {
    fn name(&self) -> &'static str {
        "niri"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "niri"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
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
            "cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.41",
            _options.config.command_output,
        )
        .unwrap();

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
