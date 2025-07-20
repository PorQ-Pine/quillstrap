use anyhow::Result;
use clap::{ArgGroup, Parser};
use log::info;
use std::fs;

use crate::common::mount_tmpfs_on_build_dir;

mod build_tasks;
mod common;
mod signing;

#[derive(Parser)]
#[command(about = "Quill OS build and bootstrap tool")]
#[command(group(
    ArgGroup::new("build_targets")
        .args(&["build_u_boot", "build_kernel", "build_rootfs"])
        .required(true)
        .multiple(true)
))]
pub struct Args {
    #[arg(long, help = "Build U-Boot", default_value_t = false)]
    build_u_boot: bool,
    #[arg(long, help = "Build kernel and initrd", default_value_t = false)]
    build_kernel: bool,
    #[arg(
        long,
        help = "Build root filesystem and package it",
        default_value_t = false
    )]
    build_rootfs: bool,
    #[arg(long, help = "Main build directory", required = true)]
    build_dir: String,
    #[arg(long, help = "Private key location", required = true)]
    private_key_path: String,
    #[arg(
        long,
        help = "Unrestricted system (allow SSH/root access)",
        default_value_t = false
    )]
    unrestricted_system: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    fs::create_dir_all(&args.build_dir)?;
    fs::create_dir_all(&format!(
        "{}/../{}",
        &args.build_dir,
        &common::DOWNLOADS_DIR
    ))?;
    mount_tmpfs_on_build_dir(&args.build_dir)?;

    let private_key = signing::read_private_key(&args.private_key_path)?;

    if args.build_u_boot {
        build_tasks::u_boot(&args.build_dir)?;
    }
    if args.build_kernel {
        build_tasks::kernel(&args.build_dir)?;
    }
    if args.build_rootfs {
        build_tasks::rootfs(&args.build_dir, &private_key, args.unrestricted_system)?;
    }

    info!("All done!");
    Ok(())
}
