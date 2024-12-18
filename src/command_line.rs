use crate::audio_server_utils::get_devices;
use clap::{ArgGroup, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(group(
    ArgGroup::new("funcs")
	.required(true)
	.args(&["change","view"])
))]
pub struct Cli {
    /// Change current sink to the next available one.
    #[arg(short, long, group = "func")]
    pub change: bool,

    /// View the current sink.
    #[arg(short, long, group = "func")]
    pub view: bool,

    /// List currently available devices
    #[arg(short, long, group = "func")]
    pub list: bool,

    /// Specify a devices file to overwrite default names.
    #[arg(short, long = "devices-file", value_parser = is_jason_config)]
    pub devices_file: Option<PathBuf>,
}

fn is_jason_config(devices_file: &str) -> anyhow::Result<PathBuf> {
    let path = devices_file.parse::<PathBuf>()?;
    match path.is_file() {
        true => {
            let devices = get_devices(&path)?;
            if !devices.is_empty() {
                Ok(devices_file.parse::<PathBuf>()?)
            } else {
                eprintln!("Devices file is empty");
                std::process::exit(exitcode::NOINPUT);
            }
        }
        false => {
            eprintln!("device file argument is not a file");
            std::process::exit(2);
        }
    }
}
