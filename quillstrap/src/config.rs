use crate::prelude::*;

use serde::{Deserialize, Serialize};

use crate::prelude::read_file_str;
use std::{io::ErrorKind::*, path::Path};

const CONFIG_PATH: &str = "../qstrap.ron";
const CONFIG_PATH_FRESH: &str = "../qstrap_fresh.ron";

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
pub enum GitLinkType {
    Ssh,
    #[default]
    Https,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
pub enum GitPlatform {
    // Gitlab is untested!
    Gitlab,
    #[default]
    Github,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct QinitOptions {
    pub deploy_ssh_port: u16,
    pub deploy_ftp_port: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct RootfsOptions {
    pub deploy_ssh_port: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Config {
    pub git_link_type: GitLinkType,
    pub git_platform: GitPlatform,
    pub git_username: String,
    // Show underlaying command output too
    pub command_output: bool,
    pub main_private_key_path: String, // Relative to root of the quillstrap repo!
    pub unrestricted: bool,
    // Also applied when unrestricted is true
    pub root_password: String,
    // Rootfs ssh login. Needs unrestricted to true too
    pub unsecure_debug: bool,
    pub deploy_ip_addr: [u8; 4], // Default for us is 192.168.3.2
    pub qinit_options: QinitOptions,
    pub rootfs_options: RootfsOptions,
    pub compression_enabled: bool,
    // "Optional apps to install in rootfs, possible apps: anki, obsidian",
    pub optional_apps: Vec<String>,
    pub optional_packages: Vec<String>, // Just more packages to install in rootfs
    // Both need to be set to take effect
    pub rootfs_user: String,
    pub rootfs_user_password: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            git_link_type: Default::default(),
            git_platform: Default::default(),
            git_username: "PorQ-Pine".to_string(),
            command_output: true,
            main_private_key_path: String::from("other/private/private.pem"),
            unrestricted: true, // :)
            root_password: "root".to_string(),
            unsecure_debug: false, // :(
            deploy_ip_addr: [0, 0, 0, 0],
            qinit_options: QinitOptions {
                deploy_ssh_port: 2222,
                deploy_ftp_port: 2221,
            },
            rootfs_options: RootfsOptions {
                deploy_ssh_port: 22,
            },
            compression_enabled: true,
            optional_apps: Vec::new(),
            optional_packages: Vec::new(),
            rootfs_user: String::new(),
            rootfs_user_password: String::new(),
        }
    }
}

impl Config {
    pub fn save(&self, path: String) {
        // Unwrap, as I doubt it will ever fail, we also have color-eyre for better unwrap
        let str = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap();
        if let Err(err) = std::fs::write(&path, str) {
            error!("Failed to save config, this is bad!: {:?}", err);
        }
    }

    pub fn load() -> Self {
        match read_file_str(CONFIG_PATH.to_string()) {
            Ok(str) => match ron::from_str::<Config>(&str) {
                Ok(conf) => {
                    if Path::new(CONFIG_PATH_FRESH).exists() {
                        warn!("Config file is good but fresh exists, I will remove fresh");
                        remove_file(CONFIG_PATH_FRESH, false).ok();
                    }
                    return conf;
                }
                Err(err) => {
                    Config::default().save(CONFIG_PATH_FRESH.to_string());
                    // TODO this in the future to insert them automatically, thats stupid as hell
                    error!("For config possible value options, look into quillstrap/src/config.rs");
                    panic!(
                        "Failed to load config file, it's probably outdated, new variables were added. Creating a fresh config file at {}, you need to repair the config at {} manually. This is the error: {:?}",
                        CONFIG_PATH_FRESH, CONFIG_PATH, err
                    );
                }
            },
            Err(err) => match err.kind() {
                NotFound => {
                    warn!("File not found, creating default config and continuing with it");
                    let conf = Config::default();
                    conf.save(CONFIG_PATH.to_string());
                    return conf;
                }
                _ => {
                    panic!(
                        "Something is wrong with accessing the config file, this is not recoverable."
                    );
                }
            },
        }
    }

    pub fn validate(&self) {
        if self.deploy_ip_addr == [0, 0, 0, 0] {
            warn!("The qinit deploy ip address is not set, it will not work!");
        }
    }
}
