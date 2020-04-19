pub fn n_euclidean(a: Vec<f64>, b: Vec<f64>) -> f64 {
    let mut distance: f64 = 0.0;

    for (a, b) in a.iter().zip(b.iter()) {
        distance += (b - a).powi(2);
    }

    distance.sqrt()
}
