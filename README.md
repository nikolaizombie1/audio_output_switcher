# audio_output_switcher
Instantly switch between audio outputs on Pipewire and Pulse Audio

audio_output_switcher is a command line application used to quickly cycle through audio outputs on Pulse and Pipewire audio servers and view the current audio output.

This application is intended to be an audio server agnostic way to cycle through your audio outputs, view the currently active audio output and even customize which audio
outputs will be cycled.

## Usage examples
```bash
audio_output_switcher --view
```
```bash
audio_output_switcher --change
```
## Configuration
### Device Name Configuration
1. Run audio_output_switcher at least once before
2. Execute `audio_output_switcher -l` in a terminal.
3. Take note of the sink name of the device you would like to change.
4. Open `~/.config/audio_output_switcher` in a text editor.
5. Write the sink name for the audio output you would like to customize.
6. Enter the device name you desire.

### Output Device Filtering
You can also choose which audio output devices will be cycled through by adding more entries inside of `devices.json` like so:
```json
[
  {
    "device_name": "Schiit Modi 3+",
    "sink_name": "Schiit_Audio_Schiit_Modi_3"
  },
  {
    "device_name": "Monitor",
    "sink_name": "hdmi-stereo"
  },
  {
    "device_name": "Bluetooth",
    "sink_name": "bluez"
  },
  {
    "device_name": "Built-in Speakers",
    "sink_name": "alsa_output.pci-0000_0c_00.1.hdmi-stereo"
  }
]
```
