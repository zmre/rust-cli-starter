use anyhow::Result;
use clap_verbosity_flag::Verbosity;
use confy::ConfyError;
use log::debug;
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::io::Write;
use std::path::PathBuf;
use structopt::clap::{crate_version, AppSettings};
use structopt::StructOpt;
use text_io::read;

const APP_NAME: &'static str = "CHANGEME";
const APP_DESCRIPTION: &'static str = "CHANGEME";

#[derive(Debug, StructOpt)]
#[structopt(name = APP_NAME, version = crate_version!(), about = APP_DESCRIPTION, rename_all = "kebab-case", setting = AppSettings::InferSubcommands)]
struct Cli {
    /// The config file to use
    #[structopt(short, long, parse(from_os_str))]
    pub config_file: Option<PathBuf>,

    /// Set verbosity default is just errors
    #[structopt(flatten)]
    verbose: Verbosity,

    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Something
    Home,
    /// Something
    List {
        /// Something
        #[structopt(short, long)]
        name: String,
    },
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    secret: String,
}

#[tokio::main]
async fn main() {
    ::std::process::exit(match run().await {
        Ok(_) => 0,
        Err(err) => {
            println!("Error: {}", err);
            1
        }
    });
}

async fn run() -> Result<()> {
    // Check command line params
    let args = Cli::from_args();
    setup_logging(&args.verbose).expect("Failed to initialize logging");
    debug!("Got args {:?}", args);
    let cfg: MyConfig = get_config_from_file(&args.config_file)
        .and_then(|cfg: MyConfig| {
            // If reading the config didn't throw an error, but produced a default
            // config with no values, then prompt the user.
            if cfg.secret == "" {
                Ok(get_config_from_user(&args.config_file)?)
            } else {
                Ok(cfg)
            }
        })
        // And if some error happened on reading, try to prompt the user and write.
        .or_else(|_| get_config_from_user(&args.config_file))?;

    debug!("Got config {:?}", cfg);

    match args.cmd {
        None | Some(Command::Home) => {
            unimplemented!()
        }
        Some(Command::List { name }) => {
            unimplemented!()
        }
    }

    Ok(())
}
fn setup_logging(v: &Verbosity) -> Result<()> {
    Ok(TermLogger::init(
        match v.log_level().unwrap_or(log::Level::Error) {
            log::Level::Trace => LevelFilter::Trace,
            log::Level::Debug => LevelFilter::Debug,
            log::Level::Info => LevelFilter::Info,
            log::Level::Warn => LevelFilter::Warn,
            log::Level::Error => LevelFilter::Error,
        },
        // LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?)
}

fn get_config_from_file(config_file: &Option<PathBuf>) -> Result<MyConfig> {
    Ok(match config_file {
        Some(ref config_file) => confy::load_path(config_file)?,
        None => confy::load(APP_NAME)?,
    })
}

fn get_config_from_user(
    config_file: &Option<PathBuf>,
) -> core::result::Result<MyConfig, ConfyError> {
    // No preference file found so prompt the user
    let mut stdo = std::io::stdout();
    print!("Enter the secret: ");
    let _ = stdo.flush();
    let secret: String = read!("{}\n");
    let cfg = MyConfig { secret };

    // And then save to the preference file so we don't have to prompt again
    match config_file {
        Some(ref config_file) => confy::store_path(config_file, &cfg),
        None => confy::store(APP_NAME, &cfg),
    }
    .map(|_| cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio_test::block_on;

    #[test]
    fn test_the_thing() {}
}
