pub fn prever_proximos(x: &[f64], a: f64, b: f64) -> Vec<f64> {
    x.iter().map(|xi| a * xi + b).collect()
}
