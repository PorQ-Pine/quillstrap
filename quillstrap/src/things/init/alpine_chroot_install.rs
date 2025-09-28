use std::fs::remove_dir_all;

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct AlpineChrootInstall;

impl AlpineChrootInstall {
    pub fn execute(_options: &crate::Options, command: &str) {
        let thing = get_thing_by_name("alpine-chroot-install", &_options.things);
        let path = format!(
            "{}alpine-chroot-install",
            get_path_of_thing(&thing, _options)
        );
        let str = &format!("{} {}", path, command);
        info!("Running command: {}", str);
        run_command(str, _options.config.command_output)
            .expect("Failed to run alpine-chroot-install");
    }

    // Chroot dir without / at the end
    pub fn setup_alpine_chroot(
        _options: &crate::Options,
        chroot_dir: &str,
        alpine_packages: Vec<&str>,
        arch: &str,
    ) {
        umount_recursive(chroot_dir);

        let chroot_dir_absolute = format!("{}/{}", dir_current(), chroot_dir);
        warn!("Chroot dir absolute is: {}", chroot_dir_absolute);

        remove_dir_all(chroot_dir).ok();
        AlpineChrootInstall::execute(
            _options,
            &format!(
                "-d {} -p {} -a {}",
                chroot_dir_absolute,
                alpine_packages.join(" -p "),
                arch
            ),
        );
        umount_recursive(chroot_dir);
    }

    // Dir, with / at the end. It's the path to the actual sysroot
    pub fn turn_on_chroot(_options: &crate::Options, dir: &str) {
        // let thing = get_thing_by_name("alpine-chroot-install", &_options.things);
        mount_point(&format!("{}proc", dir), "proc");
        mount_point(&format!("{}sys", dir), "sysfs");
        mount_point(&format!("{}tmp", dir), "tmpfs");
        mount_point(&format!("{}run", dir), "tmpfs");
        mount_point(&format!("{}dev", dir), "devtmpfs");
        let dev_pts_path = &format!("{}dev/pts", dir);
        mkdir_p(dev_pts_path);
        mount_point(dev_pts_path, "devpts");
    }
}

impl SetupThing for AlpineChrootInstall {
    fn name(&self) -> &'static str {
        "alpine-chroot-install"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "alpine-chroot-install"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &_options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
