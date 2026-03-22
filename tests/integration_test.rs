use quancoms::qubit::{self, QuantumRegister};
use quancoms::complex;

#[test]
fn test_X() {
    let mut q = QuantumRegister::new(2);
    q.X(0).unwrap();

    let expected = [0.0, 1.0, 0.0, 0.0];

    assert!((q.god_observe()[1] - expected[1]).abs() < 1e-6);
}

#[test]
fn test_H() {
    let mut q = QuantumRegister::new(2);
    q.H(0).unwrap();
    q.H(1).unwrap();

    let expected = 0.25;
    let result = q.god_observe();

    for x in result.iter() {
        assert!((x - expected).abs() < 1e-6);
    }
}

#[test]
fn test_CNOT() {
    let mut q = QuantumRegister::new(2);
    q.H(0).unwrap();
    q.CNOT(0, 1).unwrap();

    let expected = 0.5;
    let result = q.god_observe();

    assert!((result[0] - expected).abs() < 1e-6);
    assert!((result[3] - expected).abs() < 1e-6);
}

#[test]
fn test_CCNOT() {
    let mut q = QuantumRegister::new(3);
    q.H(0).unwrap();
    q.H(1).unwrap();
    q.CCNOT(0, 1, 2).unwrap();

    let expected = 0.25;
    let result = q.god_observe();

    assert!((result[0] - expected).abs() < 1e-6);
    assert!((result[1] - expected).abs() < 1e-6);
    assert!((result[2] - expected).abs() < 1e-6);
    assert!((result[7] - expected).abs() < 1e-6);
}

#[test]
fn test_Rx() {
    let mut q = QuantumRegister::new(2);
    q.Rx(0, std::f64::consts::PI).unwrap();

    let expected = 1.0;
    let result = q.god_observe();

    assert!((result[0] - 0.0).abs() < 1e-6);
    assert!((result[1] - expected).abs() < 1e-6);
}

#[test]
fn test_Ry() {
    let mut q = QuantumRegister::new(2);
    q.Ry(0, std::f64::consts::PI).unwrap();

    let result = q.god_observe();

    assert!((result[0] - 0.0).abs() < 1e-6);
    assert!((result[1] - 1.0).abs() < 1e-6);
}

#[test]
fn test_Rz() {
    let mut q = QuantumRegister::new(2);
    q.Rz(0, std::f64::consts::PI / 2.0).unwrap();

    let result = q.god_observe();

    assert!((result[0] - 1.0).abs() < 1e-6);
}

#[test]
fn test_U()
{
    let mut q = QuantumRegister::new(2);
    let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
    let c_val = complex::Complex::new(inv_sqrt2, 0.0);
    let c_neg = complex::Complex::new(-inv_sqrt2, 0.0);

    // 使用通用 U 门模拟 H 门
    q.U(0, c_val, c_val, c_val, c_neg).unwrap();

    let result = q.god_observe();

    assert!((result[0] - 0.5).abs() < 1e-6);
    assert!((result[1] - 0.5).abs() < 1e-6);
}

#[test]
fn test_MCU_triple_control() {
    let mut q = QuantumRegister::new(4);
    // 设置三个控制位为 1
    q.X(0).unwrap();
    q.X(1).unwrap();
    q.X(2).unwrap();

    let u00 = complex::Complex::new(0.0, 0.0);
    let u01 = complex::Complex::new(1.0, 0.0);
    let u10 = complex::Complex::new(1.0, 0.0);
    let u11 = complex::Complex::new(0.0, 0.0);

    // 当 0, 1, 2 全为 1 时，翻转 3
    q.MCU(vec![0, 1, 2], 3, u00, u01, u10, u11).unwrap();

    let result = q.god_observe();
    // 状态应该是 |1111>，即索引 15 的概率为 1.0
    assert!((result[15] - 1.0).abs() < 1e-6);
}