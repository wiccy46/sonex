use sonex::analytic::Meter;
use sonex::io::AudioReader;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("audio/sin_100Hz_-3dBFS_3s.wav");

    let mut reader = AudioReader::new(path).expect("Failed to create audio reader");

    println!("Audio file info:");
    println!("Sample rate: {} Hz", reader.sample_rate());
    println!("Channels: {}", reader.channels());

    let mut all_samples = Vec::new();
    while let Ok(Some(samples)) = reader.read_packet() {
        all_samples.extend(samples);
    }

    println!("\nTotal samples read: {}", all_samples.len());

    let meter = Meter::new(&all_samples, reader.channels() as u32, reader.sample_rate());

    let integrated_lufs: f64 = meter.lufs_integrated().unwrap();
    let short_term_lufs: f64 = meter.lufs_shortterm().unwrap();
    let true_peaks: Vec<f64> = meter.true_peaks().unwrap();

    println!("Integrated LUFS: {:?}", integrated_lufs);
    println!("Short-term LUFS: {:?}", short_term_lufs);
    println!("True peaks: {:?}", true_peaks); // This is a vec per channel
    Ok(())
}
