use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct SlintGallery;

impl SetupThing for SlintGallery {
    fn name(&self) -> &'static str {
        "slint_gallery"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "slint-gallery"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        run_command(
            "cargo clean",
            _options.config.command_output,
        )
        .unwrap();
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p("out");
        let full_path = get_path_of_thing_native(self, _options);

        // Building for host
        /*
        run_command(
            "cargo build --release --features=host",
            _options.config.command_output,
        )
        .unwrap();
        copy_file("target/release/gallery", "out/slint_gallery_host").unwrap();
        */
        
        // Building for rootfs
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var("PKG_CONFIG_SYSROOT_DIR", "../../low/rootfs_sysroot/sysroot");
        set_var("RUSTFLAGS", "-L ../../low/rootfs_sysroot/sysroot/usr/lib64");

        // Important! glibc version specified by zig
        run_command(
            "cargo zigbuild --release --features=pinenote_rootfs --target aarch64-unknown-linux-gnu.2.41",
            _options.config.command_output,
        )
        .unwrap();

        copy_file(
            "target/aarch64-unknown-linux-gnu/release/gallery",
            "out/slint_gallery_rootfs",
        )
        .unwrap();

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("RUSTFLAGS", "");

        // Building for initrd
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var(
            "PKG_CONFIG_PATH",
            &format!("{}../../../init/sysroot/usr/lib/pkgconfig", full_path),
        );
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var(
            "PKG_CONFIG_SYSROOT_DIR",
            &format!("{}../../../init/sysroot/", full_path),
        );
        set_var(
            "OPENSSL_INCLUDE_DIR",
            &format!("{}../../../init/sysroot/usr/include/openssl", full_path),
        );
        set_var(
            "RUSTFLAGS",
            &format!(
                "-C target-feature=-crt-static -L {}../../../init/sysroot/usr/lib/",
                full_path
            ),
        );

        run_command(
            "cargo zigbuild --release --features=pinenote_initrd --target aarch64-unknown-linux-musl",
            _options.config.command_output,
        )
        .unwrap();
        copy_file(
            "target/aarch64-unknown-linux-musl/release/gallery",
            "out/slint_gallery_initrd",
        )
        .unwrap();

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("OPENSSL_INCLUDE_DIR", "");
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
