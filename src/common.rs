use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Device {
    pub device_name: String,
    pub sink_name: String,
}
pub const APP_NAME: &str = "audio_output_switcher";
