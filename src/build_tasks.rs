use std::fs;
use anyhow::{Context, Result};
use log::{info, warn, error};
use sys_mount::{unmount, Mount, MountFlags, UnmountFlags};
use git2::Repository;

use crate::common;
use crate::signing;

use common::{run_command, download_file, bind_mount};

/* BEGIN U-BOOT */
pub fn u_boot(build_dir: &str) -> Result<()> {
    Ok(())
}
/* END U-BOOT */

/* BEGIN KERNEL */
pub fn kernel(build_dir: &str) -> Result<()> {
    Ok(())
}
/* END KERNEL */

/* BEGIN ROOTFS */
pub fn rootfs(build_dir: &str, private_key_path: &str) -> Result<()> {
    let rootfs_build_dir_path = format!("{}/{}", &build_dir, &common::ROOTFS_BUILD_DIR);
    let base_rootfs_archive_path = format!("{}/../{}/{}", &build_dir, &common::DOWNLOADS_DIR, &common::BASE_ROOTFS_FILE);
    let base_rootfs_path = format!("{}/{}/{}", &build_dir, &common::ROOTFS_BUILD_DIR, &common::ROOTFS_BASE_DIR);
    let merged_rootfs_path = format!("{}/{}", &rootfs_build_dir_path, &common::MERGED_ROOTFS_DIR);

    fs::create_dir_all(&rootfs_build_dir_path)?;
    fs::create_dir_all(&base_rootfs_path)?;
    fs::create_dir_all(&merged_rootfs_path)?;

    // Download base
    if !fs::exists(&base_rootfs_archive_path)? {
        info!("Downloading root filesystem base");
        download_file(&common::BASE_ROOTFS_URL, &base_rootfs_archive_path)?;
    }
    // Extract it
    info!("Extracting root filesystem base");
    run_command("tar", &["-C", &base_rootfs_path, "-xvf", &base_rootfs_archive_path])?;
    // Set up the chroot
    rootfs_setup_chroot(&rootfs_build_dir_path, &base_rootfs_path, &merged_rootfs_path)?;
    // Installing packages/updating the chroot
    rootfs_manage_packages(&merged_rootfs_path)?;
    rootfs_tear_down_chroot(&merged_rootfs_path)?;
    rootfs_compress_and_sign(&rootfs_build_dir_path, &merged_rootfs_path, &private_key_path)?;

    Ok(())
}

pub fn rootfs_setup_chroot(rootfs_build_dir_path: &str, base_rootfs_path: &str, merged_rootfs_path: &str) -> Result<()> {
    let rootfs_overlay_path = format!("{}/{}", &rootfs_build_dir_path, &common::ROOTFS_OVERLAY_DIR);
    let cow_directory_path = format!("{}/{}", &rootfs_build_dir_path, &common::ROOTFS_COW_DIR);
    fs::create_dir_all(&rootfs_overlay_path)?;
    fs::create_dir_all(&cow_directory_path)?;

    // Clone rootfs overlay repository for seamless packaging of static/custom files into final root filesystem
    info!("Cloning rootfs-overlay repository");
    Repository::clone(&common::ROOTFS_OVERLAY_REPO_URL, &rootfs_overlay_path)?;
    // Merge the two fileystems
    run_command("unionfs", &["-o", "cow", "-o", "allow_other", &format!("{}=RW:{}=RO:{}=RO", &cow_directory_path, &rootfs_overlay_path, &base_rootfs_path), &merged_rootfs_path])?;

    info!("Mounting chroot filesystems");
    Mount::builder().fstype("proc").mount("proc", &format!("{}/proc", &merged_rootfs_path))?;
    Mount::builder().fstype("sysfs").mount("sysfs", &format!("{}/sys", &merged_rootfs_path))?;
    Mount::builder().fstype("tmpfs").mount("tmpfs", &format!("{}/tmp", &merged_rootfs_path))?;
    Mount::builder().fstype("tmpfs").mount("tmpfs", &format!("{}/run", &merged_rootfs_path))?;
    Mount::builder().fstype("devtmpfs").mount("devtmpfs", &format!("{}/dev", &merged_rootfs_path))?;
    let resolv_conf_path = format!("{}/etc/resolv.conf", &merged_rootfs_path);
    fs::File::create(&resolv_conf_path).with_context(|| "Could not touch resolv.conf in chroot")?;
    bind_mount("/etc/resolv.conf", &resolv_conf_path).with_context(|| "Could not bind-mount resolv.conf")?;

    Ok(())
}

pub fn rootfs_tear_down_chroot(merged_rootfs_path: &str) -> Result<()> {
    unmount(format!("{}/proc", &merged_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/sys", &merged_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/tmp", &merged_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/run", &merged_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/dev", &merged_rootfs_path), UnmountFlags::DETACH)?;
    unmount(format!("{}/etc/resolv.conf", &merged_rootfs_path), UnmountFlags::DETACH)?;

    Ok(())
}

pub fn rootfs_run_chroot_command(merged_rootfs_path: &str, command: &[&str]) -> Result<()> {
    info!("Running the following command in rootfs base chroot: '{}'", &command.join(" "));
    let mut args: Vec<&str> = Vec::with_capacity(1 + command.len());
    args.push(&merged_rootfs_path);
    args.extend_from_slice(&command);

    run_command("chroot", &args)?;

    Ok(())
}

pub fn rootfs_manage_packages(merged_rootfs_path: &str) -> Result<()> {
    rootfs_run_chroot_command(&merged_rootfs_path, &["pacman", "-R", "--noconfirm", "linux-aarch64", "linux-aarch64-headers", "linux-aarch64-lts", "linux-aarch64-lts-headers", "linux-firmware", "linux-firmware-whence"])?;
    rootfs_run_chroot_command(&merged_rootfs_path, &["pacman", "--noconfirm", "-Syu", "pacman-contrib"])?;
    rootfs_run_chroot_command(&merged_rootfs_path, &["paccache", "-rk0"])?;

    Ok(())
}

pub fn rootfs_compress_and_sign(rootfs_build_dir_path: &str, merged_rootfs_path: &str, private_key_path: &str) -> Result<()> {
    let archive_path = format!("{}/{}", &rootfs_build_dir_path, &common::ROOTFS_FILE);

    info!("Compressing root filesystem");
    run_command("mksquashfs", &[&merged_rootfs_path, &archive_path, "-b", "32768", "-comp", "zstd", "-Xcompression-level", "22", "-no-xattrs"])?;
    signing::sign_file(&private_key_path, &archive_path)?;

    Ok(())
}
/* END ROOTFS */
