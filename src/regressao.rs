pub fn regressao_linear(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }

    let n = x.len() as f64;
    let soma_x = x.iter().sum::<f64>();
    let soma_y = y.iter().sum::<f64>();
    let soma_xy = x.iter().zip(y).map(|(xi, yi)| xi * yi).sum::<f64>();
    let soma_x2 = x.iter().map(|xi| xi * xi).sum::<f64>();

    let numerador = n * soma_xy - soma_x * soma_y;
    let denominador = n * soma_x2 - soma_x * soma_x;

    if denominador == 0.0 {
        return None;
    }

    let a = numerador / denominador;
    let b = (soma_y - a * soma_x) / n;

    Some((a, b))
}
