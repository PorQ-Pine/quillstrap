use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct WriteApp;

impl SetupThing for WriteApp {
    fn name(&self) -> &'static str {
        "write_app"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["rootfs_sysroot"]
    }

    fn git(&self) -> &'static str {
        "Write"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        remove_dir_all("build").ok();
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("syncscribble/Release/Write")
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let full_path = get_path_of_thing_native(self, _options);
        let sysroot_path = format!("{}../../low/rootfs_sysroot/sysroot", full_path);
        warn!("full_path: {}", full_path);

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

        // Install deps
        Rootfs::execute(
            &sysroot_path,
            "dnf install -y gcc-c++ SDL2-devel",
            _options.config.command_output,
        );

        Rootfs::execute(
            &sysroot_path,
            &format!(r#"env USE_SYSTEM_SDL=1 make -C /quillstrap/build_all/os/gui/write_app/syncscribble -j{}"#, get_cores()),
            _options.config.command_output,
        );

        Rootfs::execute(
            &sysroot_path,
            &format!("strip /quillstrap/build_all/os/gui/write_app/syncscribble/Release/Write"),
            _options.config.command_output,
        );

        run_command(
            &format!("umount {}", quillstrap_mount),
            _options.config.command_output,
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
