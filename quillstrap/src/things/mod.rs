use crate::prelude::*;

pub mod common;
pub mod init;
pub mod low;
pub mod os;
pub mod tests;

#[derive(Clone, Copy, Debug)]
pub enum TraitWrapper {
    TWUboot(Uboot),
    TWRkbin(Rkbin),
    TWBackup(Backup),
    TWQuillInit(QuillInit),
    TWSysroot(Sysroot),
    TWAlpineChrootInstall(AlpineChrootInstall),
    TWBranding(Branding),
    TWInitRD(InitRD),
    TWKernel(Kernel),
    TWExposeMmc(ExposeMmc),
    TwBackupMmc(BackupMmc),
    TwParitionSetup(PartitionSetup),
    TwBootPartition(BootPartition),
    TwFirmware(Firmware),
    TwEinkKernelMagic(EinkKernelMagic),
    TwRootfs(Rootfs),
    TwRootfsConfigs(RootfsConfigs),
    TwSerialLaunch(SerialLaunch),
    TwRootfsSysroot(RootfsSysroot),
    TwQoms(Qoms),
    TwSlintGallery(SlintGallery),
    TwGreetd(Greetd),
    TwEwwConfig(EwwConfig),
    TwLibQuillCom(LibQuillCom),
    TwNiri(Niri),
    TwEww(Eww),
    TwEwwNiriToolbar(EwwNiriToolbar),
    TwKoreader(Koreader),
    TwEwwDataProvider(QuillDataProvider),
    TwAnvil(Anvil),
    TwPinenoteService(PinenoteService),
    TwSqueekboard(Squeekboard),
    TwXournalpp(Xournalpp),
    TwXwaylandSatellite(Xwaylandsatellite),
    TwProceduralWallpapers(ProceduralWallpapers),
    TwCoreSettings(CoreSettings),
}

// This is weird but I won't kill you with lifetimes at least
macro_rules! forward {
    ($self:ident.$method:ident $( ( $($arg:expr),* ) )? ) => {
        match $self {
            TraitWrapper::TWUboot(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWRkbin(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWBackup(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWQuillInit(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWSysroot(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWAlpineChrootInstall(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWBranding(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWInitRD(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWKernel(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TWExposeMmc(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwBackupMmc(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwParitionSetup(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwBootPartition(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwFirmware(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwEinkKernelMagic(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwRootfs(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwRootfsConfigs(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwSerialLaunch(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwRootfsSysroot(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwQoms(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwSlintGallery(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwGreetd(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwEwwConfig(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwLibQuillCom(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwNiri(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwEww(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwEwwNiriToolbar(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwKoreader(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwAnvil(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwEwwDataProvider(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwPinenoteService(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwSqueekboard(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwXournalpp(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwXwaylandSatellite(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwProceduralWallpapers(inner) => inner.$method($($($arg),*)?),
            TraitWrapper::TwCoreSettings(inner) => inner.$method($($($arg),*)?),
        }
    };
}

impl SetupThing for TraitWrapper {
    fn name(&self) -> &'static str {
        forward!(self.name())
    }

    fn path(&self) -> &'static str {
        forward!(self.path())
    }

    fn deps(&self) -> Vec<&'static str> {
        forward!(self.deps())
    }

    fn git(&self) -> &'static str {
        forward!(self.git())
    }

    fn get(&self, options: &Options) -> Result<(), String> {
        forward!(self.get(options))
    }

    fn is_built(&self) -> bool {
        forward!(self.is_built())
    }

    fn clean(&self, options: &Options) -> Result<(), String> {
        forward!(self.clean(options))
    }

    fn build(&self, options: &Options) -> Result<(), String> {
        forward!(self.build(options))
    }

    fn deploy(&self, options: &Options) -> Result<(), String> {
        forward!(self.deploy(options))
    }

    fn run(&self, options: &Options) -> Result<(), String> {
        forward!(self.run(options))
    }
}

pub fn get_things() -> Vec<TraitWrapper> {
    vec![
        TWUboot(Uboot::default()),
        TWRkbin(Rkbin::default()),
        TWBackup(Backup::default()),
        TWQuillInit(Default::default()),
        TWSysroot(Default::default()),
        TWAlpineChrootInstall(Default::default()),
        TWBranding(Default::default()),
        TWInitRD(Default::default()),
        TWKernel(Default::default()),
        TWExposeMmc(Default::default()),
        TwBackupMmc(Default::default()),
        TwParitionSetup(Default::default()),
        TwBootPartition(Default::default()),
        TwFirmware(Default::default()),
        TwEinkKernelMagic(Default::default()),
        TwRootfs(Default::default()),
        TwRootfsConfigs(Default::default()),
        TwSerialLaunch(Default::default()),
        TwRootfsSysroot(Default::default()),
        TwQoms(Default::default()),
        TwSlintGallery(Default::default()),
        TwGreetd(Default::default()),
        TwEwwConfig(Default::default()),
        TwLibQuillCom(Default::default()),
        TwNiri(Default::default()),
        TwEww(Default::default()),
        TwEwwNiriToolbar(Default::default()),
        TwKoreader(Default::default()),
        TwEwwDataProvider(Default::default()),
        TwAnvil(Default::default()),
        TwPinenoteService(Default::default()),
        TwSqueekboard(Default::default()),
        TwXournalpp(Default::default()),
        TwXwaylandSatellite(Default::default()),
        TwProceduralWallpapers(Default::default()),
        TwCoreSettings(Default::default()),
    ]
}

pub fn get_thing_by_name(name: &str, things: &Vec<TraitWrapper>) -> TraitWrapper {
    for thing in things {
        if name == thing.name() {
            return *thing;
        }
    }
    let mut names = Vec::new();
    for thing in things.iter() {
        names.push(thing.name());
    }
    panic!(
        "You probably mistyped this: {}. Possible options: {}",
        name,
        names.join(" ")
    );
}
