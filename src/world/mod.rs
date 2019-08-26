use crate::animal::Animal;

#[derive(Clone, Debug)]
pub struct World {
    pub animals: Vec<Animal>,
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
