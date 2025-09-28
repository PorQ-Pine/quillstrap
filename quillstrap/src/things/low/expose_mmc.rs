use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct ExposeMmc {}

impl SetupThing for ExposeMmc {
    fn name(&self) -> &'static str {
        "expose_mmc"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        Vec::new()
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, _options: &Options) -> std::result::Result<(), String> {
        mkdir_p(self.name());
        dir_change(self.name());
        if path_exists("Image.gz") && path_exists("dtb") {
            warn!("Files exist, if you want to redownload them, run clean");
            return Ok(());
        }
        download_file(
            "https://github.com/PorQ-Pine/initrd/releases/download/1/Image.gz",
            "Image.gz",
        );
        download_file(
            "https://github.com/PorQ-Pine/initrd/releases/download/1/dtb",
            "dtb",
        );
        dir_change("../");
        Ok(())
    }

    fn clean(&self, _options: &Options) -> std::result::Result<(), String> {
        remove_file("Image.gz", false).ok();
        remove_file("dtb", false).ok();
        Ok(())
    }

    fn build(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, _options: &Options) -> std::result::Result<(), String> {
        let (port, _status) = enter_uboot_cli().unwrap();

        // TODO: make error checks
        send_read_serial(port.clone(), "mmc read ${kernel_addr_c} 0x35800 0x4000");
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        send_read_serial(port.clone(), "mmc read ${fdt_addr_r} 0x39800 0x400");
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        send_read_serial(port.clone(), "unzip ${kernel_addr_c} ${kernel_addr_r}");
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        send_read_serial(port.clone(), "booti ${kernel_addr_r} - ${fdt_addr_r}");
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        show_wait_toast(
            "Now in theory, if you saw Waiting for USB to be plugged in, unplug the serial, plug in usb, the eemc should be exposed!",
        );

        show_wait_toast(
            "Make sure no partitions on another disk have the same labels as the one on the pinenote!",
        );

        Ok(())
    }

    fn run(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }
}

/*
// Xmodem way, never worked
        // Kernel
        let output = send_read_serial(port.clone(), "loadb ${kernel_addr_r}");
        if output.contains("Ready for binary (xmodem) download") {
            info!("Loading kernel now!");
            run_shell_command(
                &format!("rmodem -f Image.gz -d {} -b 1500000", port.clone()),
                options.config.command_output,
            )
            .unwrap();
        } else {
            error!("kernel loady failed, this is bad");
        }
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        // DTB
        let output = send_read_serial(port.clone(), "loadb ${fdt_addr_r}");
        if output.contains("Ready for binary (xmodem) download") {
            info!("Loading dtb now!");
            run_shell_command(
                &format!("rmodem -f dtb -d {} -b 1500000", port.clone()),
                options.config.command_output,
            )
            .unwrap();
        } else {
            error!("dtb loady failed, this is bad");
        }
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        // Unzip
        send_read_serial(port.clone(), "unzip ${kernel_addr_c} ${kernel_addr_r}");
        sleep_millis(1000);
        clear_uboot_cli(port.clone());

        // Boot
        let str = send_read_serial(port.clone(), "booti ${kernel_addr_r} - ${fdt_addr_r}");
        sleep_millis(1000);
        if str.contains("Bad Linux ARM64 Image magic!") {
            error!("Failed to boot expose eemc kernel, bad");
        }

        show_wait_toast(
            "Maybe it worked, maybe not, unplug serial, plug in normally, there should be another block device",
        );
*/
