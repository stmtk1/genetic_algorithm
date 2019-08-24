use rand::prelude::*;

#[derive(Clone, Debug)]
struct Polynomial {
    coeffecient: Vec<f64>,
}

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
}

#[derive(Clone, Debug)]
struct Gene {
    death_prob: Polynomial,
    value: f64,
}

impl Gene {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Gene {
            death_prob: Polynomial::new(),
            value: rng.gen::<f64>(),
        }
    }
    
    pub fn is_dead(&self) -> f64 {
        self.death_prob.eval(self.value)
    }
}

#[derive(Clone, Debug)]
struct Animal {
    genes: Vec<Gene>,
}

impl Animal {
    pub fn new(n: usize) -> Self {
        let mut genes = Vec::with_capacity(n);
        for _ in 0..n {
            genes.push(Gene::new());
        }
        Animal { 
            genes, 
        }
    }
}

#[derive(Clone, Debug)]
struct World {
    animals: Vec<Animal>,
}

impl World {
    pub fn new(n: usize) -> Self {
        let mut animals = Vec::with_capacity(n);
        for _ in 0..n {
            animals.push(Animal::new(100));
        }
        World {
            animals,
        }
    }
}

fn main() {
    /*
    let a = Polynomial::new();
    for i in 0..100 {
        println!("{}, {}", i, a.eval((i as f64) / 100.0));
    }
    */
    World::new(100);
}
