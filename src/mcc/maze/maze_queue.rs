use crate::maze::maze_genotype::MazeGenome;

pub struct MazeQueue {
    mazes: Vec<MazeGenome>,
    current_maze_index: usize,
    max_items_limit: usize,
}

impl MazeQueue {
    pub fn new(mazes: Vec<MazeGenome>, max_items_limit: usize) -> MazeQueue {
        MazeQueue {
            mazes,
            current_maze_index: 0,
            max_items_limit,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&MazeGenome> {
        self.mazes.iter()
    }

    pub fn len(&self) -> usize {
        println!("queue length: {}/{}", self.mazes.len(), self.max_items_limit);

        self.mazes.len()
    }

    pub fn push(&mut self, maze: MazeGenome) {
        self.mazes.push(maze);

        if self.mazes.len() >= self.max_items_limit {
            self.remove_oldest(self.mazes.len() - self.max_items_limit);
        }
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

    pub fn get_children(&mut self, amount: usize) -> Vec<MazeGenome> {
        let mut children: Vec<MazeGenome> = vec![];

        for _ in 0..amount {
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

    pub fn get_largest(&self) -> MazeGenome {
        let max = self.mazes.iter().max_by_key(|p| p.width);

        return max.unwrap().clone();
    }

    pub fn get_average_size(&self) -> f64 {
        let mut size_sum = 0;
        for maze in self.mazes.iter() {
            size_sum += maze.width;
        }

        size_sum as f64 / self.mazes.len() as f64
    }
}
