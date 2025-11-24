
use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct Uboot {}

impl Uboot {
    pub fn new() -> Self {
        Uboot {}
    }
}

// https://github.com/PorQ-Pine/u-boot-pinenote
impl SetupThing for Uboot {
    fn name(&self) -> &'static str {
        "uboot"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "u-boot-pinenote"
    }

    fn get(&self, options: &Options) -> std::result::Result<(), String> {
        git_get_manage(self, &options);
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("uboot.img")
    }

    fn clean(&self, _options: &Options) -> std::result::Result<(), String> {
        todo!()
    }

    fn build(&self, options: &Options) -> std::result::Result<(), String> {
        set_var("CROSS_COMPILE", "/usr/bin/aarch64-linux-gnu-");

        info!("Building U-boot defconfig");
        run_command(
            "make rk3566-pinenote_defconfig",
            options.config.command_output,
        )
        .expect("Failed to make rk3566-pinenote_defconfig");

        info!("Building U-boot rk3566-pinenote");
        run_command("./make.sh rk3566-pinenote", options.config.command_output)
            .expect("Failed to make.sh rk3566-pinenote");
        info!("Running U-boot trust");
        run_command("./make.sh trust", options.config.command_output)
            .expect("Failed to make.sh trust");

        info!("Building U-boot spl");
        run_command("./make.sh spl", options.config.command_output).expect("Failed to make.sh spl");
        info!("Running U-boot trust, again");
        run_command("./make.sh trust", options.config.command_output)
            .expect("Failed to make.sh trust (second time)");

        info!("Running build of logotool_mod");
        let cur_dir = dir_current();
        mkdir_p("pinenote_ui/logotool_mod/build");
        dir_change("pinenote_ui/logotool_mod/build");
        run_command("cmake ..", options.config.command_output).expect("logotool_mod cmake failed");
        run_command("cmake --build .", options.config.command_output)
            .expect("logotool_mod cmake failed");
        dir_change(&cur_dir);

        info!("Running build of boot menu");
        run_shell_command(
            "pushd pinenote_ui/; ./build_all.sh; popd",
            options.config.command_output,
        )
        .expect("Failed to build U-boot boot menu");

        // Clean the var
        set_var("CROSS_COMPILE", "");
        let uboot_img_file = "uboot.img";
        if !path_exists(&uboot_img_file) {
            let err = format!(
                "Failed to build U-boot, everything seems fine but output file {} is missing",
                &uboot_img_file
            );
            error!("{}", err);
            return Err(err.to_string());
        } else {
            info!("Built U-Boot succesfully!");
            return Ok(());
        }
    }

    fn deploy(&self, options: &Options) -> std::result::Result<(), String> {
        let todo = uboot_cli_rockusb(options).expect("Failed to enter rock usb mode");

        if todo == UbootStuffStatus::SkipAll {
            warn!(
                "ToRuckUSBStatus is skip all, we don't flash spl loader, uboot deploy just exits, we assume we are already there"
            );
            return Ok(());
        }
        /*
        info!("Ok, Putting pinenote into download mode");
        run_command(
            "rkdeveloptool reboot-maskrom",
            options.config.command_output,
        )
        .expect("Failed to run: rkdeveloptool reboot-maskrom");
        */

        info!("Running rkdeveloptool boot rk356x_spl_loader_v1.20.114.bin");
        run_command(
            "rkdeveloptool boot rk356x_spl_loader_v1.20.114.bin",
            options.config.command_output,
        )
        .expect("Failed to write uboot");

        match rkdevelop_test(options) {
            Ok(_) => {
                info!("Rkdevelop second test worked");
            }
            Err(_) => {
                let mess = "Rkdevelop second test failed";
                error!("{}", mess);
                return Err(mess.to_string());
            }
        }

        let w = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("in theory rkdevelop works now, do you want to flash new uboot?\nOtherwise you will stay in this mode (for example, for other tools that need rkdeveloptool)")
        .default(true)
        .show_default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap();

        if !w {
            return Ok(());
        }

        info!("Running rkdeveloptool write-partition uboot uboot.img");
        run_command(
            "rkdeveloptool write-partition uboot uboot.img",
            options.config.command_output,
        )
        .expect("Failed to write uboot");

        info!("Running rkdeveloptool write-partition logo pinenote_ui/logo_new.img");
        run_command(
            "rkdeveloptool write-partition logo pinenote_ui/logo_new.img",
            options.config.command_output,
        )
        .expect("Failed to write logo partition");

        let cur_dir = dir_current();
        dir_change("../expose_mmc");
        info!("Running rkdeveloptool write 0x35800 Image.gz");
        run_command(
            "rkdeveloptool write 0x35800 Image.gz",
            options.config.command_output,
        )
        .expect("Failed to write logo partition");

        info!("Running rkdeveloptool write 0x39800 dtb");
        run_command(
            "rkdeveloptool write 0x39800 dtb",
            options.config.command_output,
        )
        .expect("Failed to write logo partition");
        dir_change(&cur_dir);

        Ok(())
    }

    fn run(&self, _options: &Options) -> std::result::Result<(), String> {
        info!("Rebooting...");

        info!("Running rkdeveloptool reboot");
        run_command("rkdeveloptool reboot", _options.config.command_output).expect("Failed to write logo partition");

        Ok(())
    }
}
