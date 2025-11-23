use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct PartitionSetup;

impl SetupThing for PartitionSetup {
    fn name(&self) -> &'static str {
        "partition_setup"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["expose_mmc"]
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p("partition_setup");
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
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        show_wait_toast(
            "This process can brick your pinenote, destroy your data and kill a crab, make sure you took a backup. Do you wish to continue?",
        );

        warn!("We assume because of expose_mmc deploy, the mmc is exposed as a block device");

        let disk = choose_disk();

        info!("Look:");
        run_command(
            &format!("gdisk -l {}", disk),
            _options.config.command_output,
        )
        .unwrap();

        let partitions = get_disk_partitions(&disk);
        if partitions.len() != 7 {
            panic!("Wrong partition count, this is not the default partition set up, aborting");
        }

        let good_partitions = vec![
            "uboot",
            "waveform",
            "uboot_env",
            "logo",
            "os1",
            "os2",
            "data",
        ];
        // Well, we assume here no one has this naming
        show_wait_toast(&format!(
            "Please confirm none of your partitions on your device have these names in the gpt partition table: {}",
            good_partitions.join(", ")
        ));

        for i in 0..7 {
            let label_expected = good_partitions[i];
            let label = get_partition_label(&partitions[i]);
            if label != label_expected {
                panic!(
                    "Wrong partition label: {} at: {}. Expected: {}, Aborting!",
                    label,
                    i + 1,
                    label_expected
                );
            }
        }

        info!("This is the default expected partition set, good");

        // Aaaa why
        run_command(
            &format!("sgdisk -e {}", disk),
            _options.config.command_output,
        )
        .unwrap();
        // Someone send help
        sleep_millis(2000);

        remove_partition("os2");
        move_partition_left("data");
        resize_partition("data", 10 * 1024);

        create_partition("data", 256, "quill_boot");
        create_partition("quill_boot", 1024 * 10, "quill_recovery");
        create_partition("quill_recovery", 1024 * 80, "quill_main");

        run_command(
            &format!("sfdisk -r {}", disk),
            _options.config.command_output,
        )
        .unwrap();
        sleep_millis(200);

        Ok(())
    }
}
