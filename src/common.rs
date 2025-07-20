use anyhow::{Context, Result};
use log::info;
use std::process::Command;
use std::{
    fs::{self, File},
    io::copy,
};
use sys_mount::{Mount, UnmountFlags, unmount};

pub const ROOTFS_BUILD_DIR: &str = "rootfs/";
pub const ROOTFS_BASE_DIR: &str = "base/";
pub const BASE_ROOTFS_URL: &str =
    "https://images.linuxcontainers.org/images/fedora/42/arm64/default/";
pub const BASE_ROOTFS_FILE: &str = "base_rootfs.tar.xz";
pub const DOWNLOADS_DIR: &str = "download/";
pub const ROOTFS_FILE: &str = "rootfs.squashfs";
pub const ROOTFS_OVERLAY_REPO_URL: &str = "https://github.com/PorQ-Pine/rootfs-overlay";
pub const ROOTFS_OVERLAY_DIR: &str = "overlay/";
pub const MERGED_ROOTFS_DIR: &str = "merged/";
pub const ROOTFS_COW_DIR: &str = "write/";
pub const ROOTFS_WORK_DIR: &str = "work/";

pub fn run_command(command: &str, args: &[&str]) -> Result<()> {
    info!(
        "Running command '{}' with arguments '{}'",
        &command,
        &args.join(" ")
    );
    let status = Command::new(&command)
        .args(args)
        .status()
        .with_context(|| format!("Failed to execute command: {}", &command))?;

    if status.success() {
        Ok(())
    } else {
        return Err(anyhow::anyhow!(
            "Command `{}` exited with status: {}",
            &command,
            &status
        ));
    }
}

// https://www.thorsten-hans.com/weekly-rust-trivia-download-an-image-to-a-file/
pub fn download_file(url: &str, file_name: &str) -> Result<()> {
    info!("Fetching '{}' and writing it to '{}'", &url, &file_name);
    // Send an HTTP GET request to the URL
    let mut response = reqwest::blocking::get(url)?;
    // Create a new file to write the downloaded image to
    let mut file = File::create(file_name)?;
    // Copy the contents of the response to the file
    copy(&mut response, &mut file)?;

    Ok(())
}

pub fn mount_tmpfs_on_build_dir(build_dir: &str) -> Result<()> {
    // Unmount previous build directories, if any
    info!("Unmounting previous build directories, if any");
    loop {
        if let Err(_e) = unmount(&build_dir, UnmountFlags::DETACH) {
            break;
        }
    }
    // Mount new tmpfs on build directory
    info!("Mounting tmpfs on build directory");
    Mount::builder()
        .fstype("tmpfs")
        .data("size=50%")
        .mount("tmpfs", &build_dir)?;

    Ok(())
}

pub fn bind_mount(source: &str, mountpoint: &str) -> Result<()> {
    // Please figure out why Mount::builder() does not work for this kind of mount
    run_command("mount", &["--rbind", &source, &mountpoint])?;

    Ok(())
}

pub fn clean_dir(path: &str) -> Result<()> {
    fs::remove_dir_all(&path)?;
    fs::create_dir_all(&path)?;

    Ok(())
}
