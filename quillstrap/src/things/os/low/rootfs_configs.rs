use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct RootfsConfigs;

impl SetupThing for RootfsConfigs {
    fn name(&self) -> &'static str {
        "rootfs_configs"
    }

    fn path(&self) -> &'static str {
        "os/low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "rootfs-configs"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn clean(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        todo!();
    }

    fn run(&self, _options: &Options) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
