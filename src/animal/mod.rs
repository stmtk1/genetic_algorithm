use crate::gene::Gene;

#[derive(Clone, Debug)]
pub struct Animal {
    pub genes: Vec<Gene>,
    pub age: f64,
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
