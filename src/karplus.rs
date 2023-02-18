use std::f64::consts::PI;


pub trait Synth {
    fn step(&mut self) -> f64;
    fn set_freq(&mut self, frequency: f64);
}

const SAMPLE_RATE: f64 = 44_100.0;
const TABLE_SIZE: usize = 4410;

pub struct KarplusStrong {
    sine: [f64; TABLE_SIZE],
    phase: usize,
    count: i32,
    loc: f64,
    loc_int: i32,
    freq: f64,
}

impl KarplusStrong {
    pub fn new() -> KarplusStrong {
        let mut value = Self {
            sine: [0.0; TABLE_SIZE],
            phase: 0,
            count: 0,
            loc: 0.0,
            loc_int: 0,
            freq: 660.0,
        };

        for i in 0..20 {
            value.sine[i] = 1.0;
        }

        return value;
    }
}

impl Synth for KarplusStrong {
    fn step(self: &mut KarplusStrong) -> f64 {
        self.count += 1;
        let freq = self.freq + (self.count as f64 / 20000.0 * PI * 2.0).sin() * 2.0;
        // let speed = 2.0 + (self.count as f64 / 10000.0 * PI * 2.0).sin() * (10000.0 / (self.count as f64 + 100000.0));
        // let vibrato = (self.count as f64 / 10000.0 * PI * 2.0).sin() * (10000.0 / (self.count as f64 + 100000.0));
        let speed = freq * (TABLE_SIZE as f64 / SAMPLE_RATE);
        self.loc += speed;

        let new_iterations = self.loc as i32 - self.loc_int;
        self.loc_int = self.loc as i32;

        let new_value = (self.sine[self.phase] + self.sine[(self.phase + 1) % TABLE_SIZE]+ self.sine[(self.phase + 2) % TABLE_SIZE]) / 3.001;
        for _ in 0..new_iterations {
            self.sine[(self.phase + TABLE_SIZE - 1) % TABLE_SIZE] = new_value;

            self.phase += 1;
            if self.phase >= TABLE_SIZE {
                self.phase -= TABLE_SIZE;
            }
        }

        new_value
    }

    fn set_freq(&mut self, frequency: f64) {
        self.freq = frequency;
    }
}
