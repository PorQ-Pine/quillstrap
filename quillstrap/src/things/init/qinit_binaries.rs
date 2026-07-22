use crate::prelude::*;

const QINIT_BINARIES_FILE: &str = "qinit_binaries.squashfs";
const WORK_DIR: &str = "work";
const OUT_DIR: &str = "out";

#[derive(Clone, Copy, Default, Debug)]
pub struct QinitBinaries;

impl SetupThing for QinitBinaries {
    fn name(&self) -> &'static str {
        "qinit_binaries"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["firmware", "procedural_wallpapers", "core_settings"]
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p(self.name());
        Ok(())
    }

    fn is_built(&self) -> bool {
        return true;
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        remove_dir_all(&OUT_DIR).ok();
        remove_dir_all(&WORK_DIR).ok();
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let qinit_binaries_squashfs_path = format!("{}/{}", &OUT_DIR, &QINIT_BINARIES_FILE);
        remove_dir_all(&WORK_DIR).ok();
        mkdir_p(&WORK_DIR);
        mkdir_p(&OUT_DIR);
        copy_file(
            &format!(
                "../procedural_wallpapers/out/{}",
                &PROCEDURAL_WALLPAPERS_BINARY
            ),
            &format!("{}/{}", &WORK_DIR, &PROCEDURAL_WALLPAPERS_BINARY)
        )
        .unwrap();
        copy_file(
            &format!("../core_settings/out/{}", &CORE_SETTINGS_BINARY),
            &format!("{}/{}", &WORK_DIR, &CORE_SETTINGS_BINARY)
        )
        .unwrap();

        let mksquashfs_cmd = if _options.config.compression_enabled {
            format!(
                // TODO: mksquashfs options should go into a single variable somewhere, not be duplicated like that (from rootfs.rs) here
                "mksquashfs {} {} -b 32768 -comp zstd -Xcompression-level 22 -no-xattrs",
                &WORK_DIR, &qinit_binaries_squashfs_path
            )
        } else {
            format!(
                "mksquashfs {} {} -no-compression -no-xattrs",
                &WORK_DIR, &qinit_binaries_squashfs_path
            )
        };
        if std::fs::exists(&qinit_binaries_squashfs_path).unwrap() {
            std::fs::remove_file(&qinit_binaries_squashfs_path).unwrap();
        }
        run_command(&mksquashfs_cmd, true).unwrap();
        sign(
            &qinit_binaries_squashfs_path,
            &format!("{}.dgst", &qinit_binaries_squashfs_path),
            _options,
        );

        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
