use std::f64::consts::PI;


pub trait Synth {
    fn step(&mut self) -> f64;
}

const TABLE_SIZE: usize = 300;

pub struct KarplusStrong {
    sine: [f64; TABLE_SIZE],
    phase: usize,
    count: i32,
    loc: f64,
    loc_int: i32,
}

impl KarplusStrong {
    pub fn new() -> KarplusStrong {
        let mut value = Self {
            sine: [0.0; TABLE_SIZE],
            phase: 0,
            count: 0,
            loc: 0.0,
            loc_int: 0,
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
        let speed = 2.0 + (self.count as f64 / 10000.0 * PI * 2.0).sin() * (10000.0 / (self.count as f64 + 100000.0));
        self.loc += speed;

        let new_iterations = self.loc as i32 - self.loc_int;
        self.loc_int = self.loc as i32;

        let new_value = (self.sine[self.phase] + self.sine[(self.phase + 1) % TABLE_SIZE]) / 2.001;
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
