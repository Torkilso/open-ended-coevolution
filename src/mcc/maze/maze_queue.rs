use crate::config;
use crate::maze::maze_genotype::MazeGenome;

pub struct MazeQueue {
    mazes: Vec<MazeGenome>,
    current_maze_index: usize,
    max_items_limit: usize
}

impl MazeQueue {
    pub fn new(mazes: Vec<MazeGenome>) -> MazeQueue {
        MazeQueue { mazes, current_maze_index: 0, max_items_limit: config::MCC.maze_population_capacity }
    }

    pub fn len(&self) -> usize {
        self.mazes.len()
    }

    pub fn push(&mut self, maze: MazeGenome) {
        if self.mazes.len() >= self.max_items_limit {
            self.remove_oldest(self.mazes.len() - self.max_items_limit);
        }

        self.mazes.push(maze);
    }

    fn remove_oldest(&mut self, amount: usize) {
        for _ in 0..amount {
            self.mazes.remove(0);
        }
        if amount > self.current_maze_index {
            self.current_maze_index = 0;
        } else {
            self.current_maze_index -= amount;
        }
    }

    pub fn get_children(&mut self) -> Vec<MazeGenome> {
        let mut children: Vec<MazeGenome> = vec!();

        for _ in 0..config::MCC.maze_selection_limit {

            if self.current_maze_index >= self.mazes.len() {
                self.current_maze_index = 0;
            }

            children.push(self.mazes.get(self.current_maze_index).unwrap().clone());
            self.current_maze_index = (self.current_maze_index + 1) % self.max_items_limit;
        }

        for child in children.iter_mut() {
            child.mutate();
        }

        children
    }
}
