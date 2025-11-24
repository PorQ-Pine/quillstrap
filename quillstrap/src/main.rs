use std::env;
use crate::prelude::*;

use crate::logic::d2_graph::generate_d2_graph;
use crate::things::{TraitWrapper, get_things};
use crate::{args::Args, logic::choose_mode};

pub mod args;
pub mod common;
pub mod config;
pub mod logic;
pub mod prelude;
pub mod thetrait;
pub mod things;

#[derive(Clone)]
pub struct Options {
    pub args: Args,
    pub config: Config,
    pub things: Vec<TraitWrapper>,
    pub path_of_repo: String, // With / at the end
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // The env is named RUST_LOG
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );
    let args = Args::parse_validate();

    mkdir_p("../other/private");
    let config = Config::load();
    config.validate();
    let things = get_things();

    let options = Options {
        args,
        config,
        things,
        path_of_repo: format!("{}/../", env::current_dir().unwrap().display().to_string()),
    };

    // d2
    if options.args.d2_graph {
        generate_d2_graph(&PathBuf::from(&options.path_of_repo))?;
        std::process::exit(0);
    }

    let private_key_path = &get_private_key_path(&options);
    if !path_exists(private_key_path) {
        warn!("Private key doesn't exist at path, I will create it, no need to thank me");
        run_command(
            &format!("openssl genrsa -out {} 2048", private_key_path),
            options.config.command_output,
        )
        .expect("Failed to create private key");
    }

    choose_mode(options);

    info!("All done, bye!");
    Ok(())
}
