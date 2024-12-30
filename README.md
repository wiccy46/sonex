# pcaster - An audio processing tool for podcasts

## Feature Overview

- Analyze your podcast audio and report statistics relevant to publishing major platforms
- Audio signal processing focused on complying to platforms' audio requirements
- Speech quality analysis and enhancement

## Library Overview

- io: Crate for File I/O
- analytic: Create for audio analysis, metrics
- process: Crate for audio processing and enhancement

## Usage

Check `examples` folder for example usage. e.g. run `cargo run --example calc_lufs_example` to 
calculate LUFS and true peaks of an audio file.