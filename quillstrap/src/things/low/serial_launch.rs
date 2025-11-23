use crate::{prelude::*};

#[derive(Clone, Copy, Default)]
pub struct SerialLaunch {
    
}

// https://github.com/PorQ-Pine/rkbin
impl SetupThing for SerialLaunch {
    fn name(&self) -> &'static str {
        "serial"
    }

    fn path(&self) -> &'static str {
        "low/"
    }

    fn deps(&self) -> Vec< &'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        todo!()
    }

    fn get(&self, _options: &Options) -> std::result::Result<(), String> {
        mkdir_p(self.name());
        Ok(())
    }

    fn is_built(&self) -> bool {
        true
    }
    
    fn clean(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }

    fn build(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }

    fn deploy(&self, _options: &Options) -> std::result::Result<(), String> {
        Ok(())
    }

    fn run(&self, _options: &Options) -> std::result::Result<(), String> {
        let port = choose_serial_port();
        run_shell_command(&format!("tio -b 1500000 {}", port), _options.config.command_output).unwrap();
        Ok(())
    }
}
