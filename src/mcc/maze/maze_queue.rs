use crate::maze::maze_genotype::MazeGenome;

pub struct MazeQueue {
    pub mazes: Vec<MazeGenome>,
    current_maze_index: usize,
    max_items_limit: usize,
    total_individuals_added: u32,
}

impl MazeQueue {
    pub fn new(mazes: Vec<MazeGenome>, max_items_limit: usize) -> MazeQueue {
        let total_individuals_added = mazes.len() as u32;

        MazeQueue {
            mazes,
            current_maze_index: 0,
            max_items_limit,
            total_individuals_added,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &MazeGenome> {
        self.mazes.iter()
    }

    pub fn len(&self) -> usize {
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
            child.successful_agent_id = None;
            child.viable = false;
            child.id = self.total_individuals_added;
            self.total_individuals_added += 1;
            child.mutate();
        }

        children
    }

    pub fn get_largest_size(&self) -> u32 {
        let max = self.mazes.iter().max_by_key(|p| p.width);
        return max.unwrap().width;
    }

    pub fn get_smallest_size(&self) -> u32 {
        let max = self.mazes.iter().min_by_key(|p| p.width);
        return max.unwrap().width;
    }

    pub fn get_average_size(&self) -> f64 {
        let mut size_sum = 0;
        for maze in self.mazes.iter() {
            size_sum += maze.width;
        }

        size_sum as f64 / self.mazes.len() as f64
    }

    pub fn get_largest_path_size(&self) -> u32 {
        let max = self.mazes.iter().max_by_key(|p| p.get_amount_of_junctures());

        max.unwrap().path_genes.len() as u32
    }

    pub fn get_smallest_path_size(&self) -> u32 {
        let max = self.mazes.iter().min_by_key(|p| p.get_amount_of_junctures());

        max.unwrap().path_genes.len() as u32
    }

    pub fn get_average_path_size(&self) -> f64 {
        let mut sum = 0;
        for maze in self.mazes.iter() {
            sum += maze.get_amount_of_junctures();
        }

        sum as f64 / self.mazes.len() as f64
    }
}
