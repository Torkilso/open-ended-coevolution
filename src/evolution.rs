use crate::maze_genotype::MazeGenome;
use crate::navigator::Navigator;

pub fn evolve_seed_navigators(mazes: &Vec<MazeGenome>, seed_amount: i32) -> Vec<Navigator> {
    vec![Navigator::new()]
}

pub fn reproduce_navigators(parents: &Vec<Navigator>) -> Vec<Navigator> {
    vec![Navigator::new()]
}

pub fn dequeue<T>(queue: &Vec<T>, amount: i32) -> &Vec<T> {
    queue
}

pub fn enqueue<T>(queue: &Vec<T>, items: &Vec<T>) {}

pub fn remove_oldest(queue: &Vec<Navigator>, size: usize) {}
