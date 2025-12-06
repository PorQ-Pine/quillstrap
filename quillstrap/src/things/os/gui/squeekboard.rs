use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct Squeekboard;

impl SetupThing for Squeekboard {
    fn name(&self) -> &'static str {
        "squeekboard"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "squeekboard"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("builddir/src/squeekboard")
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let full_path = get_path_of_thing_native(self, _options);
        let sysroot_path = format!("{}../../low/rootfs_sysroot/sysroot", full_path);
        warn!("full_path: {}", full_path);

        /*
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
        */

        let quillstrap_mount = &format!("{}/quillstrap", sysroot_path);
        if !path_exists(quillstrap_mount) || !is_mount_point(quillstrap_mount) {
            mkdir_p(quillstrap_mount);
            run_command(
                &format!(
                    "mount --bind {} {}",
                    _options.path_of_repo, quillstrap_mount
                ),
                _options.config.command_output,
            )
            .unwrap();
        }

        // meson setup --wipe
        // -Dstrip=true -Ddebug=false doesn't work
        // TODO: it takes 20MB now, should only 8,9, idk
        Rootfs::execute(
            &sysroot_path,
            "meson setup /quillstrap/build_all/os/gui/squeekboard /quillstrap/build_all/os/gui/squeekboard/builddir --buildtype=release",
            _options.config.command_output,
        );
        Rootfs::execute(
            &sysroot_path,
            "ninja -C /quillstrap/build_all/os/gui/squeekboard/builddir",
            _options.config.command_output,
        );

        run_command(
            &format!("umount {}", quillstrap_mount),
            _options.config.command_output,
        )
        .unwrap();

        /*
        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("RUSTFLAGS", "");
        */
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let port = _options.config.rootfs_options.deploy_ssh_port;
        ssh_execute("killall -9 squeekboard", port, _options);
        ssh_execute("rm -rf /usr/bin/squeekboard", port, _options);
        ssh_send("builddir/src/squeekboard", "/usr/bin/squeekboard", port, _options);
        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
