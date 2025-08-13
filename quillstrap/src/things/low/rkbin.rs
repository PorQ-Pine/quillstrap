use crate::{prelude::*};

#[derive(Clone, Copy, Default)]
pub struct Rkbin {
    
}

// https://github.com/PorQ-Pine/rkbin
impl SetupThing for Rkbin {
    fn name(&self) -> &'static str {
        "rkbin"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec< &'static str> {
        Vec::new()
    }

    fn git(&self) -> &'static str {
        "rkbin"
    }

    fn get(&self, options: &Options) -> std::result::Result<(), String> {
        git_get_manage(self, &options);
        Ok(())
    }

    fn clean(&self) -> std::result::Result<(), String> {
        // TODO: not sure
        Ok(())
    }

    fn build(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }

    fn run(&self) -> std::result::Result<(), String> {
        Ok(())
    }
}