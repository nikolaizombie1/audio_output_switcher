#![deny(unused_extern_crates)]
#![warn(missing_docs)]

//! # audio_output_switcher
//! Instantly switch between audio outputs on Pipewire and Pulse Audio
//!
//! audio_output_switcher is a command line application used to quickly cycle through audio outputs on Pulse and Pipewire audio servers and view the current audio output.
//!
//! This application is intended to be an audio server agnostic way to cycle through your audio outputs, view the currently active audio output and even customize which audio
//! outputs will be cycled.
//!
//! #Usage examples
//! ```bash
//! audio_output_switcher --view
//! ```
//! ```bash
//! audio_output_switcher --change
//! ```

use anyhow::{anyhow, Context};
use audio_output_switcher::{
    audio_server_utils::{
        are_dependencies_installed, get_available_devices, get_default_sink, get_devices,
        get_short_sink_name, set_default_sink,
    },
    command_line::Cli,
    common::{Device, APP_NAME},
};
use clap::Parser;
use serde::ser::Error;
use std::{
    fs::File,
    io::{Read, Write},
};

fn main() -> anyhow::Result<()> {
    match are_dependencies_installed() {
        Ok(_) => {}
        Err(e) => std::process::exit(e),
    }
    let args = Cli::parse();
    let current_default_sink = get_default_sink();

    let current_device = Device {
        device_name: get_short_sink_name(&current_default_sink),
        sink_name: current_default_sink,
    };

    let mut available_devices = get_available_devices();

    if args.list {
        available_devices.iter().for_each(|d| println!("Device Name: {} Sink Name: {}", d.device_name, d.sink_name));
    }

    match &args.devices_file {
        Some(f) => {
            let devices =
                get_devices(f).with_context(|| "Unable to get devices from device file")?;
            for device in &mut available_devices {
                for overide_device in &devices {
                    if device.sink_name.contains(&overide_device.sink_name) {
                        device.device_name = overide_device.device_name.clone();
                    }
                }
            }
            filter_unavailable_devices(&mut available_devices, &devices)
        }
        None => {
            let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_NAME)?;
            let config_path = xdg_dirs.place_config_file("devices.json")?;
            let config = File::open(&config_path);
            let config = match config {
                Ok(mut f) => {
                    let mut buf = String::new();
                    f.read_to_string(&mut buf)?;
                    serde_json::from_str::<Vec<Device>>(&buf)
                }
                Err(_) => {
                    let vec = vec![Device::default()];
                    let json = serde_json::to_string(&vec)?;
                    let mut c = File::create(&config_path)?;
                    let bytes_written = c.write(json.as_bytes())?;
                    match bytes_written == json.as_bytes().len() {
                        true => Ok(vec),
                        false => Err(serde_json::error::Error::custom(
                            "unable to write device.json",
                        )),
                    }
                }
            }?;
            match config.len() {
                0 => return Err(anyhow!("Empty config file. Please delete.")),
                1 => {
                    if config[0].sink_name != String::default() {
                        filter_unavailable_devices(&mut available_devices, &config);
                    }
                }
                _ => filter_unavailable_devices(&mut available_devices, &config),
            }
        }
    }

    if available_devices.is_empty() {
        eprintln!("No matching sinks found from device file.");
        std::process::exit(exitcode::CONFIG)
    }

    let current_device = &available_devices[available_devices
        .iter()
        .position(|d| d.sink_name == current_device.sink_name)
        .unwrap()];

    if args.view {
        println!("{}", current_device.device_name);
        std::process::exit(exitcode::OK)
    }

    if args.change {
        let current_device_index = available_devices
            .iter()
            .position(|d| d.sink_name == current_device.sink_name)
            .expect("Unexpected error");
        let next_device = if current_device_index == available_devices.len() - 1 {
            &available_devices[0]
        } else {
            &available_devices[current_device_index + 1]
        };
        set_default_sink(next_device);
        std::process::exit(exitcode::OK)
    }

    Ok(())
}

fn filter_unavailable_devices(available_devices: &mut Vec<Device>, devices: &[Device]) {
    *available_devices = available_devices
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
