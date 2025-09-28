use crate::prelude::*;

// You also need to implement new()
pub trait SetupThing: Copy + Clone {
    fn name(&self) -> &'static str; // By this name, other setup things will specify their dependencies
    // Relative path to this folder's path, created one folder up (../), ends with / (will still be in the repository) - it should almost always be the path like its in quillstrap
    // So like uboot is in things/low/uboot in this project then his path is low/
    fn path(&self) -> &'static str;
    // List of deps (depending on arguments, to be built before a target)
    fn deps(&self) -> Vec<&'static str>;
    // Not sure
    // fn deep_deps() -> Vec< &'static str>; // Deep deps, where deps is just what is necceserry to run, deep deps is everything that is needed to run
    fn git(&self) -> &'static str; // Git repo name, the link and type (ssh, https) depends on the configuration
    fn get(&self, options: &Options) -> Result<(), String>; // How to get it, also updates it (git pull or else)
    // All of those functions assume we are in the proper directory / repo already
    // Except get!
    fn clean(&self, options: &Options) -> Result<(), String>; // Cleans all build-related files (DO NOT do git reset --hard)
    fn build(&self, options: &Options) -> Result<(), String>; // Builds everything
    fn deploy(&self, options: &Options) -> Result<(), String>; // The deployment will be based on a tree of deps, so the deepest things first
    fn run(&self, options: &Options) -> Result<(), String>;
}

#[derive(Clone, Copy)]
struct _ExampleImpl;

impl SetupThing for _ExampleImpl {
    fn name(&self) -> &'static str {
        "example"
    }

    fn path(&self) -> &'static str {
        // root
        ""
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["example1", "example2"]
    }

    fn git(&self) -> &'static str {
        "example_repo_name"
    }

    fn get(&self, _options: &Options) -> Result<(), String> {
        todo!()
    }

    fn clean(&self, _options: &Options) -> Result<(), String> {
        todo!()
    }

    fn build(&self, _options: &Options) -> Result<(), String> {
        todo!()
    }

    fn deploy(&self, _options: &Options) -> Result<(), String> {
        todo!()
    }

    fn run(&self, _options: &Options) -> Result<(), String> {
        todo!()
    }
}
