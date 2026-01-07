use crate::prelude::*;
use std::str::FromStr;

pub fn install_optional_apps(options: &crate::Options, rd: &str) {
    if options
        .config
        .optional_apps
        .contains(&String::from_str("anki").unwrap())
    {
        info!("Installing anki");
        install_anki(options, rd);
    }

    if options
        .config
        .optional_apps
        .contains(&String::from_str("obsidian").unwrap())
    {
        info!("Installing obsidian");
        install_obsidian(options, rd);
    }

    if options
        .config
        .optional_apps
        .contains(&String::from_str("syncthing").unwrap())
    {
        info!("Installing syncthing");
        install_syncthing(options, rd);
    }
}

fn install_anki(options: &crate::Options, rd: &str) {
    if !Rootfs::package_is_installed(rd, "anki-bin") {
        Rootfs::execute(
            rd,
            "dnf copr enable hazel-bunny/anki -y",
            options.config.command_output,
        );
        Rootfs::execute(rd, "dnf install anki-bin -y", options.config.command_output);
    }
}

pub const OBSIDIAN_DESKTOP: &str = "[Desktop Entry]
Name=Obsidian
Exec=/usr/bin/obsidian.AppImage --no-sandbox --enable-features=UseOzonePlatform --ozone-platform=wayland
Icon=obsidian
Type=Application
Terminal=false
Categories=Office;";

fn install_obsidian(options: &crate::Options, rd: &str) {
    if !path_exists("other/obsidian.AppImage") {
        mkdir_p("other");
        download_file(
            "https://github.com/obsidianmd/obsidian-releases/releases/download/v1.10.6/Obsidian-1.10.6-arm64.AppImage",
            "other/obsidian.AppImage",
        );
    }
    run_command(
        "chmod +x other/obsidian.AppImage",
        options.config.command_output,
    )
    .unwrap();

    copy_file(
        "other/obsidian.AppImage",
        &format!("{}usr/bin/obsidian.AppImage", rd),
    )
    .unwrap();

    remove_file("other/obsidian.desktop", false).ok();
    append_to_file("other/obsidian.desktop", OBSIDIAN_DESKTOP);
    copy_file(
        "other/obsidian.desktop",
        &format!("{}usr/share/applications/obsidian.desktop", rd),
    )
    .unwrap();
}

fn install_syncthing(options: &crate::Options, rd: &str) {
    if !Rootfs::package_is_installed(rd, "syncthing") {
        Rootfs::execute(
            rd,
            "dnf install syncthing -y",
            options.config.command_output,
        );
    }
}
