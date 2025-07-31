pub fn calcular_mse(y_verdadeiros: &[f64], y_previstos: &[f64]) -> Option<f64> {
    if y_verdadeiros.len() != y_previstos.len() || y_verdadeiros.is_empty() {
        return None;
    }

    let mse = y_verdadeiros.iter()
        .zip(y_previstos)
        .map(|(y, y_pred)| (y - y_pred).powi(2))
        .sum::<f64>() / y_verdadeiros.len() as f64;

    Some(mse)
}

pub fn calcular_r2(y_verdadeiros: &[f64], y_previstos: &[f64]) -> Option<f64> {
    if y_verdadeiros.len() != y_previstos.len() || y_verdadeiros.is_empty() {
        return None;
    }

    let media_y = y_verdadeiros.iter().sum::<f64>() / y_verdadeiros.len() as f64;

    let ss_total: f64 = y_verdadeiros.iter().map(|y| (y - media_y).powi(2)).sum();
    let ss_res: f64 = y_verdadeiros.iter()
        .zip(y_previstos)
        .map(|(y, y_pred)| (y - y_pred).powi(2))
        .sum();

    if ss_total == 0.0 {
        return None;
    }

    Some(1.0 - (ss_res / ss_total))
}
