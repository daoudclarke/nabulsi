use std::f64::consts::PI;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::prelude::*;


pub trait Synth {
    fn play(&mut self, freq: f64, intensity: f64) -> f64;
}

const SAMPLE_RATE: f64 = 44_100.0;
const TABLE_SIZE: usize = 4410;
const RAND_SIZE: usize = 1000;

pub struct KarplusStrong {
    sine: [f64; TABLE_SIZE],
    phase: usize,
    count: i32,
    loc: f64,
    loc_int: i32,
    // rng: [f64; 1000],
    rng: SmallRng,
}

impl KarplusStrong {
    pub fn new() -> KarplusStrong {
        let mut random = [0.0; RAND_SIZE];
        let mut rng = SmallRng::from_entropy();
        for i in 0..RAND_SIZE {
            for _ in 0..10 {
                random[i] += rng.gen::<f64>() - 0.5;
            }
        }

        let mut value = Self {
            sine: [0.0; TABLE_SIZE],
            phase: 0,
            count: 0,
            loc: 0.0,
            loc_int: 0,
            // rng: random,
            rng,
        };

        return value;
    }
}

impl Synth for KarplusStrong {
    fn play(self: &mut KarplusStrong, freq: f64, intensity: f64) -> f64 {
        self.count += 1;
        let freq = freq + (self.count as f64 / 20000.0 * PI * 2.0).sin() * 2.0;
        // let speed = 2.0 + (self.count as f64 / 10000.0 * PI * 2.0).sin() * (10000.0 / (self.count as f64 + 100000.0));
        // let vibrato = (self.count as f64 / 10000.0 * PI * 2.0).sin() * (10000.0 / (self.count as f64 + 100000.0));
        let speed = freq * (TABLE_SIZE as f64 / SAMPLE_RATE);
        self.loc += speed;

        let new_iterations = self.loc as i32 - self.loc_int;
        self.loc_int = self.loc as i32;

        // let rand_value = self.rng[self.loc_int as usize % RAND_SIZE];
        let mut rand_value = 0.0;
        for _ in 0..10 {
            rand_value += self.rng.gen::<f64>() - 0.5;
        }
        let new_value = (self.sine[self.phase] + self.sine[(self.phase + 1) % TABLE_SIZE]+ self.sine[(self.phase + 2) % TABLE_SIZE]) / 3.001 + rand_value * intensity;
        for _ in 0..new_iterations {
            self.sine[(self.phase + TABLE_SIZE - 1) % TABLE_SIZE] = new_value;

            self.phase += 1;
            if self.phase >= TABLE_SIZE {
                self.phase -= TABLE_SIZE;
            }
        }

        new_value
    }
}
