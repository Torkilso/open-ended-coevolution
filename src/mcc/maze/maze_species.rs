use crate::maze::maze_genotype::MazeGenome;
use crate::mcc::maze::maze_queue::MazeQueue;

pub struct MazeSpecies {
    centroid: MazeGenome,
    pub maze_queue: MazeQueue,
    id: u32,
}

impl MazeSpecies {
    pub fn new(maze: MazeGenome, max_items_limit: usize, id: u32) -> MazeSpecies {
        MazeSpecies {
            maze_queue: MazeQueue::new(vec![maze.clone()], max_items_limit),
            centroid: maze.clone(),
            id,
        }
    }

    /*pub fn iter(&self) -> impl Iterator<Item = &MazeGenome> {
        self.maze_queue.iter()
    }*/

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
}
