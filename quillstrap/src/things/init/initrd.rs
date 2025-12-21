use std::{fs::remove_dir_all, io::Write};

use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct InitRD;

impl InitRD {}

impl SetupThing for InitRD {
    fn name(&self) -> &'static str {
        "initrd"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        // Also kernel repo for .commit, but that's only get() I hope
        // While alpine-chroot-install doesn't need to be built, it makes sense to put it here
        vec!["alpine-chroot-install"]
    }

    fn git(&self) -> &'static str {
        "initrd"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("initrd_alpine/.commit")
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        remove_dir_all("initrd_alpine").ok();
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let cur_dir = dir_current();
        // TODO: someone make this list smaller, not all is needed
        let mut packages: Vec<&str> = vec![
            "busybox",
            "busybox-extras",
            "libxkbcommon",
            "eudev",
            "udev-init-scripts",
            "libinput",
            "libgcc",
            "musl",
            "mtdev",
            "libevdev",
            "openssl",
            "fontconfig",
            "openrc",
            "xz",
            "iwd",
            "openresolv",
            "gocryptfs",
            "tzdata",
            "gptfdisk",
        ];

        if _options.config.unsecure_debug {
            let mut debug_packages = vec![
                "dropbear",
                "dropbear-ssh",
                "dropbear-scp",
                "openssh-sftp-server",
            ];
            packages.append(&mut debug_packages);
        }

        if !path_exists("initrd_alpine") {
            AlpineChrootInstall::setup_alpine_chroot(
                _options,
                &format!("initrd_alpine"),
                packages,
                "aarch64",
            );
        } else {
            warn!("If you want to actually rebuild initrd alpine, you need to clean it first!");
        }

        // AlpineChrootInstall::turn_on_chroot(_options, &format!("{}/", self.name()));
        dir_change("initrd_alpine");

        run_command(
            &format!("chmod 555 bin/bbsuid"),
            _options.config.command_output,
        )
        .unwrap();

        remove_file(&format!("env.sh"), false).ok();
        remove_file(&format!("destroy"), false).ok();
        remove_file(&format!("enter-chroot"), false).ok();
        remove_file(&format!("etc/motd"), false).ok();
        remove_file(&format!("etc/resolv.conf"), false).ok();

        remove_dir_all("var/cache").ok();

        mkdir_p("opt/key");
        generate_public_key("opt/key/public.pem", _options);

        // initrd_base is in kernel!
        // copy_file("../quill-init/out/qinit", "etc/init.d/qinit").unwrap();
        // copy_file("../quill-init/out/init", "sbin/init").unwrap();

        let commit = Kernel::get_kernel_commit(_options);
        let mut file = File::create(".commit").unwrap();
        file.write_all(commit.as_bytes()).unwrap();

        dir_change(&cur_dir);
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
