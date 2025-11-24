use crate::prelude::*;
use clap::Parser;

const GENERAL_OPTIONS: &str = "Run options";
const QUILL_INIT_OPTIONS: &str = "Quill Init options";
const FIRMWARE_OPTIONS: &str = "Firmware options";

#[derive(Parser, Clone, Debug)]
#[command(about = "Quill OS build and bootstrap tool")]
pub struct Args {
    #[arg(
        short,
        long,
        help = "You need to specify everything what quillstrap should do manually.",
        default_value_t = false, help_heading = GENERAL_OPTIONS
    )]
    pub manual_mode: bool,
    #[arg(
        short,
        long,
        help = "Specify a single action, then everything that's needed will be done to achieve it",
        default_value_t = false, help_heading = GENERAL_OPTIONS
    )]
    pub auto_mode: bool,
    #[arg(
        short,long,
        help = "Things to get (or check for updates), seperated by space. Possible all option",
        num_args = 1.., help_heading = GENERAL_OPTIONS
    )]
    pub get: Vec<String>,
    #[arg(
        short,long,
        help = "Things to build, seperated by space",
        num_args = 1..,help_heading = GENERAL_OPTIONS
    )]
    pub build: Vec<String>,
    #[arg(
        short,long,
        help = "Things to clean, seperated by space",
        num_args = 1..,help_heading = GENERAL_OPTIONS
    )]
    pub clean: Vec<String>,
    #[arg(
        short,long,
        help = "Things to deploy, seperated by space",
        num_args = 1..,help_heading = GENERAL_OPTIONS
    )]
    pub deploy: Vec<String>,
    #[arg(short, long, help = "A single thing to run", help_heading = GENERAL_OPTIONS)]
    pub run: Option<String>,
    #[arg(
        long,
        help = "Things to check if they are built, seperated by space. Possible all option",
        num_args = 1.., help_heading = GENERAL_OPTIONS
    )]
    pub is_built: Vec<String>,

    #[arg(
        long,
        help = "Generate a D2 graph of things and their dependencies",
        default_value_t = false, help_heading = GENERAL_OPTIONS
    )]
    pub d2_graph: bool,

    #[command(flatten)]
    pub quill_init_options: QuillInitOptions,

    #[command(flatten)]
    pub firmware_options: FirmwareOptions,
}

#[derive(Parser, Clone, Debug)]
pub struct QuillInitOptions {
    #[arg(long, help = "For quill_init ssh build", help_heading = QUILL_INIT_OPTIONS)]
    pub qi_ssh_build: bool,
}

#[derive(Parser, Clone, Debug)]
pub struct FirmwareOptions {
    #[arg(long, help = "Fix for disconnecting bluetooth devices", help_heading = FIRMWARE_OPTIONS)]
    pub remove_bt_firmware: bool,
}

impl Args {
    pub fn parse_validate() -> Self {
        let mut args = Args::parse();
        debug!("Initial args structure: {:#?}", args);

        // TODO: this is cool https://crates.io/crates/human-panic
        if args.auto_mode && args.manual_mode {
            panic!("Select only one mode!");
        }

        if !args.auto_mode && !args.manual_mode {
            warn!("Select a mode! Defaulting to manual mode!");
            args.manual_mode = true;
        }

        if args.build.is_empty()
            && args.get.is_empty()
            && args.clean.is_empty()
            && args.deploy.is_empty()
            && args.run.is_none()
            && args.is_built.is_empty()
            && !args.d2_graph
        {
            panic!("No action selected to be done!");
        }

        if !args.is_built.is_empty() {
            if !args.build.is_empty()
                || !args.get.is_empty()
                || !args.clean.is_empty()
                || !args.deploy.is_empty()
                || args.run.is_some()
            {
                panic!("When --is-built is used, no other action can be selected!");
            }
            if !args.manual_mode {
                panic!("--is-built can only be used in manual mode!");
            }
        }

        if args.get.len() == 1 && args.get[0] == "all" {
            args.get.clear();
            for thing in get_things() {
                args.get.push(thing.name().to_string());
            }
        }

        if args.is_built.len() == 1 && args.is_built[0] == "all" {
            args.is_built.clear();
            for thing in get_things() {
                args.is_built.push(thing.name().to_string());
            }
        }

        if args.auto_mode {
            if !args.get.is_empty()
                || !args.clean.is_empty()
                || !args.deploy.is_empty()
                || args.run.is_some()
                || !args.is_built.is_empty()
                || args.d2_graph
            {
                panic!("Only build is supported in auto mode");
            }
        }

        args
    }
}
