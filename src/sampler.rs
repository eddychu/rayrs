use rand::Rng;

pub trait Sampler {
    fn get_1d(&mut self) -> f64;
    fn get_2d(&mut self) -> (f64, f64);
}

pub struct RandomSampler {
    rng: rand::rngs::ThreadRng,
}

impl RandomSampler {
    pub fn new() -> RandomSampler {
        RandomSampler { rng: rand::thread_rng() }
    }
}

impl Sampler for RandomSampler {
    fn get_1d(&mut self) -> f64 {
        self.rng.gen()
    }

    fn get_2d(&mut self) -> (f64, f64) {
        (self.rng.gen(), self.rng.gen())
    }
}