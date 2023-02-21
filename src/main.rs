
extern crate portaudio;

use portaudio as pa;
use std::f64::consts::PI;

mod karplus;
use karplus::KarplusStrong;
use crate::karplus::Synth;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 10;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), pa::Error> {
    println!(
        "PortAudio Test: output sine wave. SR = {}, BufSize = {}",
        SAMPLE_RATE, FRAMES_PER_BUFFER
    );

    let pa = pa::PortAudio::new()?;

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    // we won't output out of range samples so don't bother clipping them.
    settings.flags = pa::stream_flags::CLIP_OFF;

    let mut synth = KarplusStrong::new();
    let mut synth2 = KarplusStrong::new();
    let mut synth3 = KarplusStrong::new();

    // synth.set_freq(1020.0);
    // synth2.set_freq(20.0);
    // synth3.set_freq(440.0);

    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let mut count: i64 = 0;
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        for i in 0..frames {
            let intensity = if count < 100 { 0.1 } else { 0.0 };
            let new_value = (synth.play(330.0, intensity) + synth2.play(440.0, intensity) + synth3.play(550.0, intensity)) / 3.0;

            buffer[i*2] = new_value as f32;
            buffer[i*2 + 1] = new_value as f32;
            count += 1;
        }
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    println!("Play for {} seconds.", NUM_SECONDS);
    pa.sleep(NUM_SECONDS * 1_000);

    stream.stop()?;
    stream.close()?;

    println!("Test finished.");

    Ok(())
}
