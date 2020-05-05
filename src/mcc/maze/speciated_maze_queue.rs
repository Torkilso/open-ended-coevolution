use crate::config;
use crate::maze::maze_genotype::MazeGenome;
use crate::mcc::maze::maze_species::MazeSpecies;

pub struct SpeciatedMazeQueue {
    pub species: Vec<MazeSpecies>,
}

impl SpeciatedMazeQueue {
    pub fn new(mazes: Vec<MazeGenome>) -> SpeciatedMazeQueue {
        let mut queue = SpeciatedMazeQueue { species: vec![] };

        let species_max_mazes_limit: u32 =
            config::MCC.maze_population_capacity / mazes.len() as u32;

        for (i, maze) in mazes.iter().enumerate() {
            let species = MazeSpecies::new(maze.clone(), species_max_mazes_limit, i as u32);
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

    pub fn iter_species(&self) -> impl Iterator<Item = &MazeSpecies> {
        self.species.iter()
    }

    pub fn iter_species_mut(&mut self) -> impl Iterator<Item = &mut MazeSpecies> {
        self.species.iter_mut()
    }
    /*pub fn iter_individuals(&self) -> impl Iterator<Item = &MazeGenome> {
        self.species.iter().map(|species| species.iter()).flatten()
    }*/

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

        // should always be 1
        let amount: usize = config::MCC.maze_selection_limit / self.species.len();

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
                sum += m.get_amount_of_junctures();
            }
        }

        sum as f64 / self.len() as f64
    }

    pub fn get_average_size_increase(&self) -> f64 {
        let mut sum = 0.0;
        for s in self.species.iter() {
            sum += s.statistics.get_overall_average_increase();
        }

        sum as f64 / self.species.len() as f64
    }

    pub fn get_average_complexity_increase(&self) -> f64 {
        let mut sum = 0.0;
        for s in self.species.iter() {
            sum += s.statistics.get_overall_average_path_complexity_increase();
        }

        sum as f64 / self.species.len() as f64
    }

    pub fn get_last_average_size_increase(&self) -> f64 {
        let mut sum = 0.0;
        for s in self.species.iter() {
            sum += s.statistics.get_current_average_size_increase();
        }

        sum as f64 / self.species.len() as f64
    }

    pub fn get_last_average_complexity_increase(&self) -> f64 {
        let mut sum = 0.0;
        for s in self.species.iter() {
            sum += s.statistics.get_current_average_complexity_increase();
        }

        sum as f64 / self.species.len() as f64
    }

    pub fn save_state(&mut self) {
        for s in self.species.iter_mut() {
            s.save_state();
        }
    }
}
