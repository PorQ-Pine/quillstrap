use crate::prelude::*;

const QUILL_BOOT_MOUNT_PATH: &str = "/mnt/quill_boot/";
const QINIT_BINARIES_FILE: &str = "qinit_binaries.squashfs";
const TMP_PATH: &str = "/tmp/boot_partition_temp/";

#[derive(Clone, Copy, Default, Debug)]
pub struct BootPartition;

impl SetupThing for BootPartition {
    fn name(&self) -> &'static str {
        "boot_partition"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        // It needs them to deploy, but makes sense to keep them here
        vec!["kernel", "eink_kernel_magic", "firmware", "procedural_wallpapers", "core_settings"]
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p(self.name());
        Ok(())
    }

    fn is_built(&self) -> bool {
        true
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        warn!("We assume that because of expose_mmc deploy, the MMC is exposed as a block device");

        let _disk = choose_disk();

        let partition = get_partition("quill_boot");
        let qinit_binaries_squashfs_path = format!("{}{}", &QUILL_BOOT_MOUNT_PATH, &QINIT_BINARIES_FILE);

        mkdir_p(&QUILL_BOOT_MOUNT_PATH);

        run_command(
            &format!("mount {} {}", partition, QUILL_BOOT_MOUNT_PATH),
            _options.config.command_output,
        )
        .unwrap();
        run_command("sync", false).unwrap();

        if std::fs::exists(&TMP_PATH).unwrap() {
            std::fs::remove_dir_all(&TMP_PATH).unwrap();
        }
        mkdir_p(&TMP_PATH);
        copy_file(&format!("../procedural_wallpapers/out/{}", &PROCEDURAL_WALLPAPERS_BINARY), &format!("{}procedural_wallpapers", &TMP_PATH)).unwrap();
        let mksquashfs_cmd = if _options.config.compression_enabled {
            format!(
                // TODO: mksquashfs options should go into a single variable somewhere, not be duplicated like that (from rootfs.rs) here
                "mksquashfs {} {} -b 32768 -comp zstd -Xcompression-level 22 -no-xattrs",
                &TMP_PATH, &qinit_binaries_squashfs_path
            )
        } else {
            format!(
                "mksquashfs {} {} -no-compression -no-xattrs",
                &TMP_PATH, &qinit_binaries_squashfs_path
            )
        };
        if std::fs::exists(&qinit_binaries_squashfs_path).unwrap() {
            std::fs::remove_file(&qinit_binaries_squashfs_path).unwrap();
        }
        run_command(&mksquashfs_cmd, true).unwrap();
        sign(&qinit_binaries_squashfs_path, &format!("{}.dgst", &qinit_binaries_squashfs_path), _options);

        copy_file("../kernel/out/Image.gz", &format!("{}Image.gz", &QUILL_BOOT_MOUNT_PATH)).unwrap();
        copy_file("../kernel/out/DTB", &format!("{}DTB", &QUILL_BOOT_MOUNT_PATH)).unwrap();

        copy_file(
            "../firmware/out/wifi_bt/firmware.squashfs",
            &format!("{}firmware.squashfs", &QUILL_BOOT_MOUNT_PATH),
        )
        .unwrap();
        sign(
            &format!("{}firmware.squashfs", &QUILL_BOOT_MOUNT_PATH),
            &format!("{}firmware.squashfs.dgst", &QUILL_BOOT_MOUNT_PATH),
            _options,
        );

        mkdir_p(&format!("{}waveform", &QUILL_BOOT_MOUNT_PATH));
        copy_file(
            "../eink_kernel_magic/custom_wf.bin",
            &format!("{}waveform/custom_wf.bin", &QUILL_BOOT_MOUNT_PATH),
        )
        .unwrap();

        run_command("sync", false).unwrap();
        run_command(
            &format!("umount {}", partition),
            _options.config.command_output,
        )
        .unwrap();

        info!("Done, in theory, reboot the device now manually via the power button");
        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
