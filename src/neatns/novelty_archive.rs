use crate::neatns::novelty_item::{NoveltyItem, NoveltyItemsByFitness};
use crate::neatns::population::Population;
use crate::neatns::agent::Agent;

use crate::config;
use crate::simulator::Point;

pub struct NoveltyArchive {
    novelty_items: Vec<Point>,
    fittest_items: NoveltyItemsByFitness,
    generation: u32,
    items_added_in_generation: u32,
    novelty_threshold: f64,
    generations_without_addition: u32,
}

impl NoveltyArchive {
    pub fn new() -> NoveltyArchive {
        NoveltyArchive {
            novelty_items: vec![],
            fittest_items: NoveltyItemsByFitness::new(),
            generation: 0,
            items_added_in_generation: 0,
            novelty_threshold: config::NEATNS.initial_novelty_threshold,
            generations_without_addition: 0,
        }
    }

    pub fn add_or_discard_position(&mut self, position: Point) {
        self.novelty_items.push(position);
    }

    pub fn evaluate_position_novelty(&self, position: &Point) -> f64 {
        let mut distances = self.find_distances_to_point(position);
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let closest_points = distances[0..config::NEATNS.amount_of_neighbors].to_vec();
        closest_points.iter().sum::<f64>() / closest_points.len() as f64
    }

    pub fn end_of_generation(&mut self) {
        self.generation += 1;
        self.adjust_archive_settings()
    }

    pub fn adjust_archive_settings(&mut self) {
        if self.items_added_in_generation == 0 {
            self.generations_without_addition += 1;
        } else {
            self.generations_without_addition = 0
        }

        if self.generations_without_addition == 10 {
            self.novelty_threshold *= 0.95;
            if self.novelty_threshold < config::NEATNS.novelty_floor {
                self.novelty_threshold = config::NEATNS.novelty_floor
            }
            self.generations_without_addition = 0
        }

        if self.items_added_in_generation >= 4 {
            self.novelty_threshold *= 1.2
        }

        self.items_added_in_generation = 0;
    }

    pub fn find_distances_to_point(&self, current_point: &Point) -> Vec<f64> {
        let mut distances: Vec<f64> = vec![];

        for point in self.novelty_items.iter() {
            distances.push(euclidean_distance(point, current_point));
        }
        distances
    }
}

pub fn euclidean_distance(point_a: &Point, point_b: &Point) -> f64 {
    ((point_b.x - point_a.x).powi(2) + (point_b.x - point_a.x).powi(2)).sqrt()
}
