use crate::{common::commands::run_command, prelude::*};

// We use commands instead of library because for support for ssh key managers, I don't feel like reinventing the wheel for no benefit

// Here we assume we are in the directory where the git repo should be
pub fn git_get_manage(thing: &impl SetupThing, _options: &Options) {
    if path_exists(thing.name()) {
        dir_change(thing.name());
        // Because the config can change, we change the remote url every time to be sure
        set_remote_url(thing, _options);
        pull(thing, _options);
        dir_change("../");
    } else {
        clone(thing, _options);
    }
}

// Here we assume we are in the directory already
pub fn pull(thing: &impl SetupThing, options: &Options) {
    // Note: no submodules, we don't use submodules, ever!
    let url = assemble_git_link(thing.git(), options);
    info!("Pulling existing repo: {}", url);
    run_command(&format!("git pull"), options.config.command_output)
        .expect(&format!("Failed to pull repo: {}", url));
}

pub fn clone(thing: &impl SetupThing, options: &Options) {
    let url = assemble_git_link(thing.git(), options);
    info!("Cloning repo: {}", url);
    run_command(
        &format!("git clone --recurse-submodules {} {}", url, thing.name()),
        options.config.command_output,
    )
    .expect(&format!("Failed to clone repo: {}", url));
}

pub fn set_remote_url(thing: &impl SetupThing, options: &Options) {
    let url = assemble_git_link(thing.git(), options);
    debug!("Setting remote url: {}", url);
    run_command(
        &format!("git remote set-url origin {}", url),
        options.config.command_output,
    )
    .expect(&format!("Failed to clone repo: {}", url));
}

/*
https://github.com/PorQ-Pine/u-boot-pinenote.git
git@github.com:PorQ-Pine/u-boot-pinenote.git
*/
pub fn assemble_git_link(name: &str, options: &Options) -> String {
    let mut link = String::new();
    match options.config.git_link_type {
        crate::config::GitLinkType::Ssh => link.push_str("git@"),
        crate::config::GitLinkType::Https => link.push_str("https://"),
    }

    match options.config.git_platform {
        crate::config::GitPlatform::Gitlab => link.push_str("gitlab.com"),
        crate::config::GitPlatform::Github => link.push_str("github.com"),
    }

    match options.config.git_link_type {
        crate::config::GitLinkType::Ssh => link.push_str(":"),
        crate::config::GitLinkType::Https => link.push_str("/"),
    }

    link.push_str(&options.config.git_username);
    link.push_str("/");
    link.push_str(name);
    link.push_str(".git");

    link
}
