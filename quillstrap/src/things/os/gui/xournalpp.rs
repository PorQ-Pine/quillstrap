use crate::prelude::*;

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
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("build/install/bin/xournalpp")
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
            "dnf install -y gcc-c++ cmake gtk3-devel libxml2-devel portaudio-devel libsndfile-devel \
            poppler-glib-devel texlive-scheme-basic texlive-dvipng gettext libzip-devel \
            librsvg2-devel lua-devel gtksourceview4-devel help2man qpdf-devel git zlib zlib-devel zlib-ng",
            _options.config.command_output,
        );

        // remove_dir_all("build").ok();
        // remove_dir_all("install").ok();

        mkdir_p("build");
        mkdir_p("install");

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
            replace_string_file(
                file_path,
                "Exec=xournalpp-wrapper %f",
                "Exec=env GDK_BACKEND=x11 GDK_SCALE=2 xournalpp-wrapper %f",
            );
        }

        run_command(
            &format!("umount {}", quillstrap_mount),
            _options.config.command_output,
        )
        .unwrap();

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
