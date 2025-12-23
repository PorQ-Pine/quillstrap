use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct Backup {}

impl SetupThing for Backup {
    fn name(&self) -> &'static str {
        "backup"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        warn!("No git for backup, obviously");
        ""
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p("backup");
        Ok(())
    }

    fn is_built(&self) -> bool {
        let status = path_exists("uboot.bin")
            && path_exists("waveform.bin")
            && path_exists("uboot_env.bin")
            && path_exists("logo.bin");

        if !status {
            error!("Backup is an exception here, you need to run it (take the backup) to assume it's built.");
        }
        status
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        warn!("No clean for backup, obviously");
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        warn!("No build for backup, obviously");
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        warn!("No deploy for backup, obviously");
        Ok(())
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        warn!("We assume that because of expose_mmc deploy, the MMC is exposed as a block device");

        let disk = choose_disk();
        info!("Chosen disk: {}", disk);

        let partitions = vec!["uboot", "waveform", "uboot_env", "logo"];
        let mut wrong = false;
        for label in partitions {
            let partition = get_partition(label);
            run_shell_command(&format!("dd if={} of={}.bin bs=512 status=progress", partition, label), _options.config.command_output).unwrap();

            if !path_exists(&format!("{}.bin", label)) {
                error!(
                    "File doesn't exist: {}.bin - we have a problem probably, the backup is bad!",
                    label
                );
                wrong = true;
            }
        }

        if wrong {
            return Err(String::from("Failed to take backup!"));
        } else {
            info!("Backup taken succesfully!");
        }

        Ok(())
    }
}

/*
spl boot doesn't support reads it seems?

warn!("We assume because of uboot deploy, we are in rkdeveloptool state");

        run_command("rkdeveloptool read-partition uboot uboot.bin", true)
            .expect("Failed to read partition");
        run_command("rkdeveloptool read-partition waveform waveform.bin", true)
            .expect("Failed to read partition");
        run_command("rkdeveloptool read-partition uboot_env uboot_env.bin", true)
            .expect("Failed to read partition");
        run_command("rkdeveloptool read-partition logo logo.bin", true)
            .expect("Failed to read partition");

*/
