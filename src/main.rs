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

#[derive(Clone, Debug)]
struct Gene {
    death_prob: Polynomial,
}

impl Gene {
    pub fn new() -> Self {
        //let mut rng = rand::thread_rng();
        Gene {
            death_prob: Polynomial::new(),
        }
    }
    
    pub fn is_dead(&self, age: f64) -> bool {
        self.death_prob.eval(age) >= 1.0
    }
    
    pub fn next_month(&self) -> Self {
        Gene {
            death_prob: self.death_prob.clone(),
        }
    }
    
    pub fn next_generation(&self) -> Self {
        Self {
            death_prob: self.death_prob.mutate()
        }
    }
}

#[derive(Clone, Debug)]
struct Animal {
    genes: Vec<Gene>,
    age: f64,
}

impl Animal {
    pub fn new(n: usize) -> Self {
        let mut genes = Vec::with_capacity(n);
        for _ in 0..n {
            genes.push(Gene::new());
        }
        Animal { 
            age: 0.0,
            genes, 
        }
    }

    pub fn next_month(&self) -> Self {
        let genes = self.genes.clone()
            .into_iter()
            .map(|gene| gene.next_month())
            .collect();
        Animal {
            age: self.age + 0.01,
            genes,
        }
    }
    
    pub fn is_dead(&self) -> bool {
        self.genes.clone()
            .into_iter()
            .fold(false, |acc, gene| gene.is_dead(self.age) || acc)
    }

    pub fn next_generation(&self) -> Self {
        let genes = self.genes.clone()
            .into_iter()
            .map(|gene| gene.next_generation())
            .collect();

        Animal {
            age: 0.0,
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
    
    pub fn next_month(&self) -> Self {
        let animals = self.animals.clone()
            .into_iter()
            .filter(|animal| animal.is_dead())
            .map(|animal| animal.next_month())
            .collect();
        World {
            animals,
        }
    }
    
    pub fn next_generation(&self) -> Self {
        let animals = self.animals.clone()
            .into_iter()
            .map(|animal| animal.next_generation())
            .collect();
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
    World::new(100).next_month().next_generation();
}
