use crate::prelude::*;

const KOREADER_DEB_URL: &str = "https://github.com/koreader/koreader/releases/download/v2025.10/koreader-appimage-aarch64-v2025.10.AppImage";

// Too lazy to set proper icon
const KOREADER_DESKTOP: &str = r#"[Desktop Entry]
Name=KOReader
Exec=/usr/bin/koreader.AppImage
Icon=bookreader
Type=Application
Categories=Education;Utility;
Comment=Read eBooks on your device
Terminal=false
"#;

#[derive(Clone, Copy, Default)]
pub struct Koreader;

impl SetupThing for Koreader {
    fn name(&self) -> &'static str {
        "koreader"
    }

    fn path(&self) -> &'static str {
        "os/gui/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "koreader"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        mkdir_p("koreader");
        dir_change("koreader");
        download_file(KOREADER_DEB_URL, "koreader.AppImage");
        run_command("chmod +x koreader.AppImage", true).unwrap();
        append_to_file("koreader.desktop", KOREADER_DESKTOP);
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
