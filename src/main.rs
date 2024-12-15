use clap::{ArgGroup, Parser};
use serde::Deserialize;
use std::{path::PathBuf, process::Command};

#[derive(Parser, Debug)]
#[clap(group(
    ArgGroup::new("funcs")
	.required(true)
	.args(&["change","view"])
))]
struct Args {
    /// Change current sink to the next available one.
    #[arg(short, long, group = "func")]
    change: bool,

    /// View the current sink.
    #[arg(short, long, group = "func")]
    view: bool,

    /// Specify a devices file to overwrite default names.
    #[arg(short = 'f', long, value_parser = is_jason_config)]
    devices_file: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Device {
    device_name: String,
    sink_name: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match which::which("pactl") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(exitcode::UNAVAILABLE);
        }
    }
    match which::which("bash") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(exitcode::UNAVAILABLE);
        }
    }
    let current_default_sink = get_default_sink();

    let current_device = Device {
        device_name: get_short_sink_name(&current_default_sink),
        sink_name: current_default_sink,
    };


    let mut available_devices = get_available_devices();

    if args.devices_file.is_some() {
        let devices = get_devices(&args.devices_file.unwrap())
            .expect("Unable to get divices from device file");
        for device in &mut available_devices {
            for overide_device in &devices {
                if device.sink_name.contains(&overide_device.sink_name) {
                    device.device_name = overide_device.device_name.clone();
                }
            }
        }
        available_devices = available_devices
            .iter()
            .filter(|d| {
                devices
                    .iter()
                    .map(|o| o.sink_name.clone())
                    .any(|s| d.sink_name.contains(&s))
            })
            .map(|d| d.to_owned())
            .collect::<Vec<_>>();
    }

    if available_devices.len() == 0 {
        eprintln!("No matching sinks found from device file.");
        std::process::exit(exitcode::CONFIG)
    }

    let current_device = &available_devices[available_devices.iter().position(|d| d.sink_name == current_device.sink_name).unwrap()];

    if args.view {
        println!("{}", current_device.device_name);
        std::process::exit(exitcode::OK)
    }

    if args.change {
        let current_device_index = available_devices
            .iter()
            .position(|d| {
                    d.sink_name == current_device.sink_name
            })
            .expect("Unexpected error");
        let next_device = if current_device_index == available_devices.len() - 1 {
            &available_devices[0]
        } else {
            &available_devices[current_device_index + 1]
        };
        set_default_sink(next_device);
    }

    Ok(())
}

fn get_short_sink_name(sink: &str) -> String {
    let sink_name = sink.split("_").collect::<Vec<_>>()[1]
        .split(".")
        .collect::<Vec<_>>()[1]
        .split("-")
        .collect::<Vec<_>>()[1]
        .split("_")
        .collect::<Vec<_>>()[0];
    sink_name.to_owned()
}

fn set_default_sink(device: &Device) {
    Command::new("bash")
        .arg("-c")
        .arg(format!("pactl set-default-sink {}", device.sink_name))
        .output()
        .expect("failed to execute pactl process");
}

fn get_available_devices() -> Vec<Device> {
    String::from_utf8(
        Command::new("bash")
            .arg("-c")
            .arg("pactl list sinks short | awk '{print $2}'")
            .output()
            .expect("failed to execute pactl process")
            .stdout,
    )
    .unwrap()
    .trim()
    .split('\n')
    .map(|s| s.to_owned())
    .map(|s| Device {
        device_name: get_short_sink_name(&s),
        sink_name: s,
    })
    .collect::<Vec<Device>>()
}

fn get_default_sink() -> String {
    String::from_utf8(
        Command::new("bash")
            .arg("-c")
            .arg("pactl info | awk '/Default Sink: /{print $3}'")
            .output()
            .expect("failed to execute pactl process")
            .stdout,
    )
    .unwrap()
    .trim()
    .to_owned()
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

fn get_devices(devices_file: &PathBuf) -> anyhow::Result<Vec<Device>> {
    let file_contents = std::fs::read_to_string(devices_file)?;
    Ok(serde_json::from_str::<Vec<Device>>(&file_contents)?)
}
