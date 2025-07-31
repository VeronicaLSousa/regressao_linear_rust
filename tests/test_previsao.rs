use regressao_linear::previsao::prever_proximos;

#[test]
fn test_prever_proximos() {
    let x = vec![5.0, 6.0];
    let a = 2.0;
    let b = 1.0;
    let resultados = prever_proximos(&x, a, b);
    assert_eq!(resultados, vec![11.0, 13.0]);
}
carg