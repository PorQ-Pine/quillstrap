use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Firmware;

impl SetupThing for Firmware {
    fn name(&self) -> &'static str {
        "firmware"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "firmware"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &_options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        remove_dir_all("out").ok();
        mkdir_p("out/wifi_bt");
        remove_file("/tmp/firmware.squashfs", false).ok();
        copy_file("wifi_bt/firmware.squashfs", "/tmp/firmware.squashfs").unwrap();

        // TODO: compression level (config)
        if _options.args.firmware_options.remove_bt_firmware {
            info!("Appluing remove bt firmware fix");
            remove_dir_all("/tmp/firmware.tmp").ok();
            run_command(
                "unsquashfs -d /tmp/firmware.tmp /tmp/firmware.squashfs",
                _options.config.command_output,
            )
            .unwrap();
            remove_file("/tmp/firmware.tmp/brcm/BCM4345C0.hcd", true).unwrap();
            // Firmware should be always compressed. It's just... firmware
            run_command(
                "mksquashfs /tmp/firmware.tmp /tmp/firmware.squashfs -noappend",
                _options.config.command_output,
            )
            .unwrap();
            remove_dir_all("/tmp/firmware.tmp").unwrap();
        }

        copy_file("/tmp/firmware.squashfs", "out/wifi_bt/firmware.squashfs").unwrap();
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
