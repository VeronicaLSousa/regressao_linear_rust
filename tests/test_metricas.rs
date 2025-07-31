use regressao_linear::metricas::{calcular_mse, calcular_r2};

#[test]
fn test_mse_correto() {
    let y = vec![3.0, 5.0, 7.0];
    let y_pred = vec![2.5, 5.5, 7.5];
    let mse = calcular_mse(&y, &y_pred).unwrap();
    assert!((mse - 0.25).abs() < 1e-6);
}

#[test]
fn test_r2_correto() {
    let y = vec![1.0, 2.0, 3.0];
    let y_pred = vec![1.0, 2.0, 3.0];
    let r2 = calcular_r2(&y, &y_pred).unwrap();
    assert_eq!(r2, 1.0);
}
