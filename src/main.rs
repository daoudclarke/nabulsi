
extern crate portaudio;

use portaudio as pa;
// use std::f64::consts::PI;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 100;

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

    // Initialise sinusoidal wavetable.
    let mut sine = [0.0; TABLE_SIZE];
    for i in 0..20 {
        sine[i] = 1.0;
        // sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
    }
    // let mut left_phase = 0;
    // let mut right_phase = 0;
    let mut phase = 0;

    let pa = pa::PortAudio::new()?;

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    // we won't output out of range samples so don't bother clipping them.
    settings.flags = pa::stream_flags::CLIP_OFF;

    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            let new_value = (sine[phase] + sine[(phase + 1) % TABLE_SIZE]) / 2.01;
            buffer[idx] = new_value;
            buffer[idx + 1] = new_value;
            // for i in 0..TABLE_SIZE - 1 {
            //     sine[i] = sine[i+1];
            // }
            sine[(phase + TABLE_SIZE - 1) % TABLE_SIZE] = new_value;

            phase += 1;
            if phase >= TABLE_SIZE {
                phase -= TABLE_SIZE;
            }

            // buffer[idx] = sine[left_phase];
            // buffer[idx + 1] = sine[right_phase];
            // left_phase += 1;
            // if left_phase >= TABLE_SIZE {
            //     left_phase -= TABLE_SIZE;
            // }
            // right_phase += 3;
            // if right_phase >= TABLE_SIZE {
            //     right_phase -= TABLE_SIZE;
            // }
            idx += 2;
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
