use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct EinkKernelMagic;

impl SetupThing for EinkKernelMagic {
    fn name(&self) -> &'static str {
        "eink_kernel_magic"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        // Backup from low
        vec![]
    }

    fn git(&self) -> &'static str {
        "eink-kernel-magic"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &_options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        run_command("chmod +x wbf_to_custom.py", false).unwrap();
        run_command("./wbf_to_custom.py ../../low/backup/waveform.bin", true).unwrap();
        // TODO check
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
