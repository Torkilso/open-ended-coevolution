use crate::config;
use crate::maze::maze_genotype::MazeGenome;
use crate::mcc::maze::maze_queue::MazeQueue;

#[derive(Debug, Clone)]
pub struct MazeSpeciesStatistics {
    average_sizes: Vec<f64>,
    maximum_sizes: Vec<u32>,
    minimum_sizes: Vec<u32>,
    average_size_increases: Vec<f64>,
    average_path_complexities: Vec<f64>,
    maximum_path_complexities: Vec<u32>,
    minimum_path_complexities: Vec<u32>,
    average_path_complexity_increases: Vec<f64>,
}

impl MazeSpeciesStatistics {
    pub fn get_overall_average_increase(&self) -> f64 {
        if self.average_size_increases.is_empty() {
            return 0.0;
        }

        self.average_size_increases.iter().sum::<f64>() as f64
            / self.average_size_increases.len() as f64
    }

    pub fn get_overall_average_path_complexity_increase(&self) -> f64 {
        if self.average_path_complexity_increases.is_empty() {
            return 0.0;
        }

        self.average_path_complexity_increases.iter().sum::<f64>() as f64
            / self.average_path_complexity_increases.len() as f64
    }

    pub fn get_current_average_size_increase(&self) -> f64 {
        let last = self.average_size_increases.last();
        if last.is_some() {
            *last.unwrap()
        } else {
            0.0
        }
    }

    pub fn get_current_average_complexity_increase(&self) -> f64 {
        let last = self.average_path_complexity_increases.last();
        if last.is_some() {
            *last.unwrap()
        } else {
            0.0
        }
    }

    pub fn get_overall_score(&self) -> f64 {
        self.get_overall_average_increase() + self.get_overall_average_path_complexity_increase()
    }
}

#[derive(Debug, Clone)]
pub struct MazeSpecies {
    centroid: MazeGenome,
    pub maze_queue: MazeQueue,
    pub id: u32,
    pub statistics: MazeSpeciesStatistics,
}

impl MazeSpecies {
    pub fn new(maze: MazeGenome, max_items_limit: u32, id: u32) -> MazeSpecies {
        MazeSpecies {
            maze_queue: MazeQueue::new(vec![maze.clone()], max_items_limit),
            centroid: maze.clone(),
            id,
            statistics: MazeSpeciesStatistics {
                average_sizes: vec![],
                maximum_sizes: vec![],
                minimum_sizes: vec![],
                average_size_increases: vec![],
                average_path_complexities: vec![],
                maximum_path_complexities: vec![],
                minimum_path_complexities: vec![],
                average_path_complexity_increases: vec![],
            },
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &MazeGenome> {
        self.maze_queue.iter()
    }

    pub fn len(&self) -> usize {
        self.maze_queue.len()
    }

    pub fn push(&mut self, agent: MazeGenome) {
        self.maze_queue.push(agent);
    }

    pub fn get_children(&mut self, amount: usize) -> Vec<MazeGenome> {
        self.maze_queue.get_children(amount)
    }

    pub fn distance(&self, other: &MazeGenome) -> f64 {
        self.centroid.distance(other)
    }

    pub fn save_state(&mut self) {
        let last_average_size = self.statistics.average_sizes.last();
        if last_average_size.is_some() {
            let new_average_increase =
                self.maze_queue.get_average_size() - *last_average_size.unwrap();
            self.statistics
                .average_size_increases
                .push(new_average_increase);
        } else {
            let new_average_increase =
                self.maze_queue.get_average_size() - config::MCC.default_maze_size as f64;
            self.statistics
                .average_size_increases
                .push(new_average_increase);
        }
        let last_average_complexity = self.statistics.average_path_complexities.last();
        if last_average_complexity.is_some() {
            let new_average_complexity =
                self.maze_queue.get_average_path_size() - *last_average_complexity.unwrap();
            self.statistics
                .average_path_complexity_increases
                .push(new_average_complexity);
        }

        self.statistics
            .average_sizes
            .push(self.maze_queue.get_average_size());
        self.statistics
            .maximum_sizes
            .push(self.maze_queue.get_largest_size());
        self.statistics
            .minimum_sizes
            .push(self.maze_queue.get_smallest_size());

        self.statistics
            .average_path_complexities
            .push(self.maze_queue.get_average_path_size());
        self.statistics
            .maximum_path_complexities
            .push(self.maze_queue.get_largest_path_size());
        self.statistics
            .minimum_path_complexities
            .push(self.maze_queue.get_smallest_path_size());
    }
}
