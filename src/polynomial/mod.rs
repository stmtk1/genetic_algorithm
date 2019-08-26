use rand::prelude::*;

fn max(a: f64, b: f64) -> f64 {
    if a < b {
        b
    } else {
        a
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a 
    } else {
        b
    }
}

#[derive(Clone, Debug)]
pub struct Polynomial {
    pub coeffecient: Vec<f64>,
}

impl Polynomial {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut coeffecient = Vec::with_capacity(100);
        for _ in 0..100 {
            coeffecient.push(rng.gen::<f64>() * 2.0 - 1.0);
        }
        Self {
            coeffecient,
        }
    }
    
    pub fn eval(&self, x: f64) -> f64 {
        let ret = self.coeffecient
            .clone()
            .into_iter()
            .fold(0.0, |a, b| a * (x - 1.0) + b)
            * (x - 1.0)
            + 1.0;
        max(min(x * ret, 1.0), 0.0)
    }
    
    pub fn mutate(&self) -> Self {
        let mut rng = rand::thread_rng();
        let coeffecient = self.coeffecient.clone()
            .into_iter()
            .map(|c| c + (2.0 * rng.gen::<f64>() * 2.0 - 1.0))
            .collect();
        Self {
            coeffecient,
        }
    }
}
