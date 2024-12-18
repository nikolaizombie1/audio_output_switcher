use crate::common::Device;
use std::path::PathBuf;
use std::process::Command;

pub fn get_devices(devices_file: &PathBuf) -> anyhow::Result<Vec<Device>> {
    let file_contents = std::fs::read_to_string(devices_file)?;
    Ok(serde_json::from_str::<Vec<Device>>(&file_contents)?)
}

pub fn are_dependencies_installed() -> Result<(), exitcode::ExitCode> {
    match which::which("pactl") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("pactl is not installed. {e}");
            return Err(exitcode::UNAVAILABLE);
        }
    }
    match which::which("bash") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("bash shell is not installed. {e}");
            return Err(exitcode::UNAVAILABLE);
        }
    }
    match which::which("awk") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("awk is not installed. {e}");
            return Err(exitcode::UNAVAILABLE);
        }
    }
    Ok(())
}

pub fn get_short_sink_name(sink: &str) -> String {
    let sink_name = sink.split("_").collect::<Vec<_>>()[1]
        .split(".")
        .collect::<Vec<_>>()[1]
        .split("-")
        .collect::<Vec<_>>()[1]
        .split("_")
        .collect::<Vec<_>>()[0];
    sink_name.to_owned()
}

pub fn set_default_sink(device: &Device) {
    Command::new("bash")
        .arg("-c")
        .arg(format!("pactl set-default-sink {}", device.sink_name))
        .output()
        .expect("failed to execute pactl process");
}

pub fn get_available_devices() -> Vec<Device> {
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

pub fn get_default_sink() -> String {
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
