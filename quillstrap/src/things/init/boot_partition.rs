use crate::prelude::*;

const QUILL_BOOT_MOUNT_PATH: &str = "/mnt/quill_boot/";
const QINIT_BINARIES_DIR: &str = "qinit_binaries/";

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
        vec!["kernel", "eink_kernel_magic", "firmware"]
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
        warn!("We assume because of expose_mmc deploy, the mmc is exposed as a block device");

        let _disk = choose_disk();

        let partition = get_partition("quill_boot");
        let qinit_binaries_dir_path = &format!("{}{}", &QUILL_BOOT_MOUNT_PATH, &QINIT_BINARIES_DIR);
        let procedural_wallpapers_binary_final_path = &format!("{}{}", &qinit_binaries_dir_path, &PROCEDURAL_WALLPAPERS_BINARY);

        mkdir_p(&QUILL_BOOT_MOUNT_PATH);

        run_command(
            &format!("mount {} {}", partition, QUILL_BOOT_MOUNT_PATH),
            _options.config.command_output,
        )
        .unwrap();
        run_command("sync", false).unwrap();

        mkdir_p(&qinit_binaries_dir_path);
        copy_file(&format!("../procedural_wallpapers/out/{}", &PROCEDURAL_WALLPAPERS_BINARY), &procedural_wallpapers_binary_final_path).unwrap();
        sign(&procedural_wallpapers_binary_final_path, &format!("{}.dgst" &procedural_wallpapers_binary_final_path), _options);

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
