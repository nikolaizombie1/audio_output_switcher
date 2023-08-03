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
    #[arg(short, long, group = "func")]
    change: bool,

    #[arg(short, long, group = "func")]
    view: bool,

    #[arg(short = 'f', long, required = true, value_parser = is_jason_config)]
    devices_file: PathBuf,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Device {
    device_name: String,
    sink_name: String,
}

fn main() {
    let args = Args::parse();
    match which::which("pactl") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
    match which::which("bash") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
    let current_default_sink = get_default_sink();

    let mut devices = get_devices(&args.devices_file).unwrap();

    let available_sinks = get_available_sinks();

    for sink in &available_sinks {
	for device in &mut devices {
	    if sink.contains(&device.sink_name) {
		device.sink_name = sink.clone();
	    }
	}
    }

    let devices = devices.into_iter().filter(|d| device_available(d,&available_sinks) ).collect::<Vec<Device>>();
    
    let curr_default_device = match devices
        .clone()
        .into_iter()
        .find(|s| current_default_sink.contains(&s.sink_name))
    {
        Some(device) => device,
        None => Device {
            device_name: current_default_sink[0..3].to_owned(),
            sink_name: current_default_sink,
        },
    };

    if args.change {
        match curr_default_device.device_name == "Unknown Device" {
            true => {
		set_default_sink(&devices.clone().first().unwrap().sink_name);
            }
            false => {
		let pos = devices.clone().into_iter().position(|d| d == curr_default_device ).unwrap();
		let new_sink = &devices[(pos+1)%devices.clone().len()];
		set_default_sink(&new_sink.sink_name);
		}
	    }
    } else if args.view {
	let new_sink = &devices.into_iter().find(|d| get_default_sink().contains(&d.sink_name)).unwrap();
	println!("{}",format!("|{}|",new_sink.device_name));
    }

    std::process::exit(exitcode::OK)
}

fn set_default_sink(sink_name: &str) {
                Command::new("bash")
                    .arg("-c")
                    .arg(format!(
                        "pactl set-default-sink {}",
                        sink_name
                    ))
                    .output()
                    .expect("failed to execute pactl process");
}

fn get_available_sinks() -> Vec<String>  {
    String::from_utf8(
        Command::new("bash")
            .arg("-c")
            .arg("pactl list sinks short | awk '{print $2}'")
            .output()
            .expect("failed to execute pactl process")
            .stdout
    )
    .unwrap().trim().to_owned().split('\n').map(|s| s.to_owned()).collect::<Vec<String>>()
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
    .unwrap().trim().to_owned()
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

fn device_available(device: &Device,sinks: &[String]) -> bool {
    for sink in sinks {
	if &device.sink_name == sink {
	    return true;
	}
    }
   false 
}
