use std::io::Write;

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Sysroot;

impl Sysroot {
    pub fn execute_sysroot_command(command: &str, _options: &crate::Options) {
        let cur_dir = dir_current();
        let thing = get_thing_by_name("sysroot", &_options.things);
        let path = &get_path_of_thing(&thing, _options);
        // info!("Sysroot path: {}", path);
        AlpineChrootInstall::turn_on_chroot(_options, path);
        dir_change(path);
        if !path_exists("quillstrap") || !is_mount_point("quillstrap") {
            mkdir_p("quillstrap");
            run_command(
                &format!("mount --bind {} quillstrap/", _options.path_of_repo),
                _options.config.command_output,
            )
            .unwrap();
        }

        run_shell_command(
            &format!("./enter-chroot bash -c \"source ~/.bashrc; {}\"", command),
            _options.config.command_output,
        )
        .unwrap();
        dir_change(&cur_dir);
    }

    pub fn execute_sysroot_command_dir(
        command: &str,
        absolute_path: &str,
        _options: &crate::Options,
    ) {
        // info!("Absolute path is: {}", absolute_path);
        let better_path = format!(
            "/quillstrap/{}{}",
            MAIN_BUILD_DIR,
            absolute_path.split(MAIN_BUILD_DIR).last().unwrap()
        );
        info!("Sysroot path is: {}", better_path);
        Sysroot::execute_sysroot_command(&format!("cd {}; {}", better_path, command), _options);
    }
}

impl SetupThing for Sysroot {
    fn name(&self) -> &'static str {
        "sysroot"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p(self.name());
        Ok(())
    }

    fn is_built(&self) -> bool {
        // This sucks but whatever
        path_exists(&format!("root/.bashrc"))
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let cur_dir = dir_current();
        dir_change("../");
        // TODO: someone make this list smaller, not all is needed
        let package_vec: Vec<&str> = vec![
            "busybox",
            "busybox-extras",
            "libxkbcommon",
            "eudev",
            "libinput",
            "libgcc",
            "musl",
            "mtdev",
            "libevdev",
            "openssl",
            "fontconfig",
            "pkgconf",
            "openssl-dev",
            "eudev-dev",
            "libinput-dev",
            "libxkbcommon-dev",
            "curl",
            "gcc",
            "bash",
            "musl-dev",
            "clang",
            "clang19-libclang",
            "clang19-dev",
        ];
        AlpineChrootInstall::setup_alpine_chroot(
            _options,
            &format!("{}", self.name()),
            package_vec,
            "aarch64",
        );
        AlpineChrootInstall::turn_on_chroot(_options, &format!("{}/", self.name()));
        dir_change(&cur_dir);

        Sysroot::execute_sysroot_command(
            "curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y",
            _options,
        );

        let mut file = File::create("root/.bashrc").unwrap();
        file.write_all(". \"$HOME/.cargo/env\"".to_string().as_bytes())
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
