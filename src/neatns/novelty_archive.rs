use crate::config;
use crate::simulator::Point;

pub struct NoveltyArchive {
    pub(crate) novelty_items: Vec<Point>,
}

impl NoveltyArchive {
    pub fn new() -> NoveltyArchive {
        NoveltyArchive {
            novelty_items: vec![],
        }
    }

    pub fn add_or_discard_position(&mut self, position: Point) {
        self.novelty_items.push(position);
    }

    pub fn evaluate_position_novelty(&self, position: &Point) -> f64 {
        let mut distances = self.find_distances_to_point(position);
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if distances.len() < 15 {
            return 0.0;
        }

        let closest_points = distances[0..config::NEATNS.amount_of_neighbors].to_vec();
        closest_points.iter().sum::<f64>() / closest_points.len() as f64
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
    ((point_b.x - point_a.x).powi(2) + (point_b.y - point_a.y).powi(2)).sqrt()
}
