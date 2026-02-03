use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct CosmicWanderer;

impl SetupThing for CosmicWanderer {
    fn name(&self) -> &'static str {
        "cosmic_wanderer"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["rootfs_sysroot"]
    }

    fn git(&self) -> &'static str {
        "Cosmic-Wanderer"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("target/aarch64-unknown-linux-gnu/release/cosmic-wanderer")
            && path_exists(
                "cosmic-wanderer-opener/target/aarch64-unknown-linux-gnu/release/cosmic-wanderer-opener",
            )
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        run_command("cargo clean", _options.config.command_output).unwrap();
        dir_change("cosmic-wanderer-opener");
        run_command("cargo clean", _options.config.command_output).unwrap();
        dir_change("../");
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
        // Optimisation here
        set_var(
            "RUSTFLAGS",
            &format!(
                "-L {}../../low/rootfs_sysroot/sysroot/usr/lib64 -C target-cpu=cortex-a55 -C target-feature=+neon",
                full_path
            ),
        );
        set_var(
            "PKG_CONFIG_LIBDIR",
            &format!(
                "{}../../low/rootfs_sysroot/sysroot/usr/lib64/pkgconfig/:{}../../low/rootfs_sysroot/sysroot/usr/share/pkgconfig",
                full_path, full_path
            ),
        );
        // Slint qmake
        /*
        set_var(
            "QMAKE",
            &format!(
                "{}../../low/rootfs_sysroot/sysroot/usr/bin/qmake",
                full_path
            ),
        );
        */
        set_var(
            "QT_LIBRARY_PATH",
            &format!(
                "{}../../low/rootfs_sysroot/sysroot/usr/lib64/qt5",
                full_path
            ),
        );
        set_var(
            "QT_INCLUDE_PATH",
            &format!(
                "{}../../low/rootfs_sysroot/sysroot/usr/include/qt5",
                full_path
            ),
        );
        let sysroot_base = format!("{}../../low/rootfs_sysroot/sysroot", full_path);
        let include_path = format!("{}/usr/include", sysroot_base);
        let include_flag = format!("-isystem {}", include_path);
        set_var("CFLAGS", &include_flag);
        set_var("CXXFLAGS", &include_flag);

        run_command(
            "cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.41 --no-default-features --features quill_defaults",
            _options.config.command_output,
        )
        .unwrap();
        dir_change("cosmic-wanderer-opener");
        run_command(
            "cargo zigbuild --release --target aarch64-unknown-linux-gnu.2.41",
            _options.config.command_output,
        )
        .unwrap();
        dir_change("../");

        // set_var("QMAKE", "");
        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_PATH", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("RUSTFLAGS", "");
        set_var("PKG_CONFIG_LIBDIR", "");

        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let port = _options.config.rootfs_options.deploy_ssh_port;
        ssh_execute("killall -9 cosmic-wanderer", port, _options);
        ssh_execute("rm -rf /usr/bin/cosmic-wanderer", port, _options);
        ssh_execute("rm -rf /usr/bin/cosmic-wanderer-opener", port, _options);
        ssh_send(
            "target/aarch64-unknown-linux-gnu/release/cosmic-wanderer",
            "/usr/bin/cosmic-wanderer",
            port,
            _options,
        );
        ssh_send(
            "cosmic-wanderer-opener/target/aarch64-unknown-linux-gnu/release/cosmic-wanderer-opener",
            "/usr/bin/cosmic-wanderer-opener",
            port,
            _options,
        );
        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
