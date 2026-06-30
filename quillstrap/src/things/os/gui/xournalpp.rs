use crate::prelude::*;

const SCRIPT: &str = r#"#!/usr/bin/bash

GDK_BACKEND=x11 GDK_DPI_SCALE=0.5 GDK_SCALE=2 xournalpp
"#;

#[derive(Clone, Copy, Default, Debug)]
pub struct Xournalpp;

impl SetupThing for Xournalpp {
    fn name(&self) -> &'static str {
        "xournalpp"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["rootfs_sysroot"]
    }

    fn git(&self) -> &'static str {
        "xournalpp"
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
        path_exists("build/install/bin/xournalpp")
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let full_path = get_path_of_thing_native(self, _options);
        let sysroot_path = format!("{}../../low/rootfs_sysroot/sysroot", full_path);
        warn!("full_path: {}", full_path);
        Rootfs::turn_on_chroot(&format!("{}/", sysroot_path));

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

        mkdir_p("build");

        // AAA
        Rootfs::execute(
            &sysroot_path,
            "git config --global --add safe.directory \"*\"",
            _options.config.command_output,
        );

        Rootfs::execute(
            &sysroot_path,
            "cmake /quillstrap/build_all/os/gui/xournalpp \
                    -B /quillstrap/build_all/os/gui/xournalpp/build \
                    -G Ninja \
                    -DCMAKE_INSTALL_PREFIX=/quillstrap/build_all/os/gui/xournalpp/build/install",
            _options.config.command_output,
        );
        Rootfs::execute(
            &sysroot_path,
            "cmake --build /quillstrap/build_all/os/gui/xournalpp/build",
            _options.config.command_output,
        );
        Rootfs::execute(
            &sysroot_path,
            "cmake --build /quillstrap/build_all/os/gui/xournalpp/build --target install",
            _options.config.command_output,
        );

        let file_path = "build/install/share/applications/com.github.xournalpp.xournalpp.desktop";
        let file_content = read_file_str(file_path.to_string()).unwrap();
        if !file_content.contains("Exec=env GDK_BACKEND=x11 GDK_SCALE=2 xournalpp-wrapper %f") {
            /*
            replace_string_file(
                file_path,
                "Exec=xournalpp-wrapper %f",
                "Exec=env GDK_BACKEND=x11 GDK_SCALE=2 xournalpp-wrapper %f",
            );
            */
            replace_string_file(
                file_path,
                "Exec=xournalpp-wrapper %f",
                "Exec=xournalpp_hacky_fix.sh",
            );
        }

        // For older xournalpp
        if !file_content.contains("Exec=env GDK_BACKEND=x11 GDK_SCALE=2 xournalpp %f") {
            /*
            replace_string_file(
                file_path,
                "Exec=xournalpp %f",
                "Exec=env GDK_BACKEND=x11 GDK_SCALE=2 xournalpp %f",
            );
            */
            replace_string_file(
                file_path,
                "Exec=xournalpp %f",
                "Exec=xournalpp_hacky_fix.sh",
            );
        }

        run_command(
            &format!("umount {}", quillstrap_mount),
            _options.config.command_output,
        )
        .unwrap();

        // Hacky fix
        let path = Path::new("build/install/bin/xournalpp_hacky_fix.sh");
        std::fs::write(path, SCRIPT).unwrap();
        run_command(
            &format!("chmod +x {}", path.to_string_lossy()),
            _options.config.command_output,
        )
        .unwrap();

        umount_recursive(&&format!("{}/", sysroot_path));

        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let port = _options.config.rootfs_options.deploy_ssh_port;
        ssh_execute("killall -9 squeekboard", port, _options);
        ssh_execute("rm -rf /usr/bin/squeekboard", port, _options);
        ssh_send(
            "builddir/src/squeekboard",
            "/usr/bin/squeekboard",
            port,
            _options,
        );
        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
