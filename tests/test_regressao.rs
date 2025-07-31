use regressao_linear::regressao::regressao_linear;

#[test]
fn test_regressao_linear_basico() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![2.0, 4.0, 6.0];
    let resultado = regressao_linear(&x, &y);
    assert_eq!(resultado, Some((2.0, 0.0)));
}

#[test]
fn test_regressao_com_entrada_invalida() {
    let x = vec![1.0, 2.0];
    let y = vec![3.0]; // tamanho diferente
    assert_eq!(regressao_linear(&x, &y), None);
}
