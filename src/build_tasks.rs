use std::fs;
use std::process::Command;
use anyhow::{Context, Result};
use log::{info, warn, error};
use sys_mount::{unmount, Mount, UnmountFlags};
use git2::Repository;

use crate::common;
use crate::signing;

use common::{run_command, download_file, bind_mount, clean_dir};

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
pub fn rootfs(build_dir: &str, private_key_path: &str, unrestricted_system: bool) -> Result<()> {
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
        let latest_build = Command::new("bash").args(["-c", &format!("curl -s {} | grep href | tail -n 3 | cut -c 10-25 | tail -n 1", &common::BASE_ROOTFS_URL)]).output()?;
        let mut latest_build_str = String::from_utf8(latest_build.stdout)?;
        latest_build_str.pop();
        download_file(&format!("{}{}/rootfs.tar.xz", &common::BASE_ROOTFS_URL, &latest_build_str), &base_rootfs_archive_path)?;
    }
    // Extract it
    info!("Extracting root filesystem base");
    run_command("tar", &["-C", &base_rootfs_path, "-xvf", &base_rootfs_archive_path])?;
    // Set up the chroot
    rootfs_setup_chroot(&rootfs_build_dir_path, &base_rootfs_path, &merged_rootfs_path, unrestricted_system)?;
    // Install packages/update the chroot
    rootfs_manage_packages(&merged_rootfs_path, unrestricted_system)?;
    rootfs_misc(&merged_rootfs_path)?;
    rootfs_tear_down_chroot(&merged_rootfs_path)?;
    // Compress the rootfs
    rootfs_compress_and_sign(&rootfs_build_dir_path, &merged_rootfs_path, &private_key_path)?;

    Ok(())
}

pub fn rootfs_setup_chroot(rootfs_build_dir_path: &str, base_rootfs_path: &str, merged_rootfs_path: &str, unrestricted_system: bool) -> Result<()> {
    let rootfs_overlay_repository_path = format!("{}/{}", &rootfs_build_dir_path, &common::ROOTFS_OVERLAY_DIR);
    let rootfs_cow_dir_path = format!("{}/{}", &rootfs_build_dir_path, &common::ROOTFS_COW_DIR);
    let rootfs_work_dir_path = format!("{}/{}", &rootfs_build_dir_path, &common::ROOTFS_WORK_DIR);
    fs::create_dir_all(&rootfs_overlay_repository_path)?;
    fs::create_dir_all(&rootfs_cow_dir_path)?;
    fs::create_dir_all(&rootfs_work_dir_path)?;

    // Clone rootfs overlay repository for seamless packaging of static/custom files into final root filesystem
    info!("Cloning rootfs-overlay repository");
    Repository::clone_recurse(&common::ROOTFS_OVERLAY_REPO_URL, &rootfs_overlay_repository_path)?;
    let common_rootfs_overlay_path = format!("{}/common", &rootfs_overlay_repository_path);
    let specific_rootfs_overlay_path = match unrestricted_system {
        true => format!("{}/unrestricted", &rootfs_overlay_repository_path),
        false => format!("{}/restricted", &rootfs_overlay_repository_path),
    };
    // Merge the fileystems
    run_command("fuse-overlayfs", &["-o", &format!("allow_other,lowerdir={}:{}:{},upperdir={},workdir={}", &common_rootfs_overlay_path, &specific_rootfs_overlay_path, &base_rootfs_path, &rootfs_cow_dir_path, &rootfs_work_dir_path), &merged_rootfs_path])?;

    info!("Mounting chroot filesystems");
    Mount::builder().fstype("proc").mount("proc", &format!("{}/proc", &merged_rootfs_path))?;
    Mount::builder().fstype("sysfs").mount("sysfs", &format!("{}/sys", &merged_rootfs_path))?;
    Mount::builder().fstype("tmpfs").mount("tmpfs", &format!("{}/tmp", &merged_rootfs_path))?;
    Mount::builder().fstype("tmpfs").mount("tmpfs", &format!("{}/run", &merged_rootfs_path))?;
    Mount::builder().fstype("devtmpfs").mount("devtmpfs", &format!("{}/dev", &merged_rootfs_path))?;
    let resolv_conf_path = format!("{}/etc/resolv.conf", &merged_rootfs_path);
    // fs::File::create(&resolv_conf_path).with_context(|| "Could not touch resolv.conf in chroot")?;
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
    let mut args: Vec<&str> = Vec::with_capacity(1 + command.len());
    args.push(&merged_rootfs_path);
    args.extend_from_slice(&command);

    run_command("chroot", &args)?;

    Ok(())
}

pub fn rootfs_manage_packages(merged_rootfs_path: &str, unrestricted_system: bool) -> Result<()> {
    rootfs_run_chroot_command(&merged_rootfs_path, &["dnf", "--assumeyes", "update"])?;
    rootfs_run_chroot_command(&merged_rootfs_path, &["dnf", "--assumeyes", "install", "zsh", "NetworkManager", "NetworkManager-wifi", "vim", "nano", "busybox"])?;
    if unrestricted_system {
        rootfs_run_chroot_command(&merged_rootfs_path, &["dnf", "--assumeyes", "install", "dropbear"])?;
    }
    rootfs_run_chroot_command(&merged_rootfs_path, &["dnf", "clean", "all"])?;

    Ok(())
}

pub fn rootfs_misc(merged_rootfs_path: &str) -> Result<()> {
    fs::create_dir_all(&format!("{}/lib/modules", &merged_rootfs_path))?;
    fs::create_dir_all(&format!("{}/lib/firmware", &merged_rootfs_path))?;
    clean_dir(&format!("{}/var/log", &merged_rootfs_path))?;
    clean_dir(&format!("{}/var/cache", &merged_rootfs_path))?;
    rootfs_disable_service(&merged_rootfs_path, "systemd-networkd-wait-online")?;
    rootfs_disable_service(&merged_rootfs_path, "systemd-time-wait-sync")?;
    rootfs_disable_service(&merged_rootfs_path, "serial-getty@")?;
    rootfs_run_chroot_command(&merged_rootfs_path, &["chown", "-R", "quill", "/home/quill"])?;
    rootfs_run_chroot_command(&merged_rootfs_path, &["sed", "-i", "s/Fedora Linux 42 (Container Image)/Quill OS/g", "/etc/os-release"])?;

    Ok(())
}

pub fn rootfs_disable_service(merged_rootfs_path: &str, service: &str) -> Result<()> {
    rootfs_run_chroot_command(&merged_rootfs_path, &["systemctl", "disable", &service])?;
    rootfs_run_chroot_command(&merged_rootfs_path, &["systemctl", "mask", &service])?;

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
