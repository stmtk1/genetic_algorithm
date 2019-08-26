use crate::polynomial::Polynomial;

#[derive(Clone, Debug)]
pub struct Gene {
    pub death_prob: Polynomial,
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
