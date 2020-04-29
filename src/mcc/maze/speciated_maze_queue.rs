use crate::config;
use crate::maze::maze_genotype::MazeGenome;
use crate::mcc::maze::maze_species::MazeSpecies;

pub struct SpeciatedMazeQueue {
    species: Vec<MazeSpecies>,
}

impl SpeciatedMazeQueue {
    pub fn new(mazes: Vec<MazeGenome>) -> SpeciatedMazeQueue {
        let mut queue = SpeciatedMazeQueue { species: vec![] };

        let species_max_mazes_limit: usize = config::MCC.maze_population_capacity / mazes.len();

        for maze in mazes {
            let species = MazeSpecies::new(maze, species_max_mazes_limit);
            queue.species.push(species);
        }

        queue
    }

    pub fn len(&self) -> usize {
        let mut length = 0;

        for species in self.species.iter() {
            length += species.len();
        }

        length
    }

    pub fn iter(&self) -> impl Iterator<Item = &MazeGenome> {
        self.species.iter().map(|species| species.iter()).flatten()
    }

    pub fn push(&mut self, maze: MazeGenome) {
        let mut distances: Vec<f64> = vec![];

        for species in self.species.iter() {
            distances.push(species.distance(&maze));
        }

        let mut highest = 0.0;
        let mut index: usize = 0;

        for (i, value) in distances.iter().enumerate() {
            if *value > highest {
                highest = *value;
                index = i;
            }
        }

        self.species[index].push(maze);
    }

    pub fn get_children(&mut self) -> Vec<MazeGenome> {
        let mut children: Vec<MazeGenome> = vec![];

        let amount: usize = config::MCC.agent_selection_limit / self.species.len();

        for species in self.species.iter_mut() {
            for child in species.get_children(amount) {
                children.push(child);
            }
        }

        for child in children.iter_mut() {
            child.mutate();
            child.viable = false;
        }

        children
    }

    pub fn get_largest_size(&self) -> u32 {
        let max = self
            .species
            .iter()
            .max_by_key(|p| p.maze_queue.get_largest_size());
        max.unwrap().maze_queue.get_largest_size()
    }

    pub fn get_smallest_size(&self) -> u32 {
        let min = self
            .species
            .iter()
            .min_by_key(|p| p.maze_queue.get_smallest_size());
        min.unwrap().maze_queue.get_smallest_size()
    }

    pub fn get_average_size(&self) -> f64 {
        let mut sum = 0;
        for s in self.species.iter() {
            for m in s.maze_queue.iter() {
                sum += m.width;
            }
        }

        sum as f64 / self.len() as f64
    }

    pub fn get_largest_path_size(&self) -> u32 {
        let max = self
            .species
            .iter()
            .max_by_key(|p| p.maze_queue.get_largest_path_size());
        max.unwrap().maze_queue.get_largest_path_size() as u32
    }

    pub fn get_smallest_path_size(&self) -> u32 {
        let min = self
            .species
            .iter()
            .min_by_key(|p| p.maze_queue.get_smallest_path_size());
        min.unwrap().maze_queue.get_smallest_path_size() as u32
    }

    pub fn get_average_path_size(&self) -> f64 {
        let mut sum = 0;
        for s in self.species.iter() {
            for m in s.maze_queue.iter() {
                sum += m.path_genes.len();
            }
        }

        sum as f64 / self.len() as f64
    }
}
