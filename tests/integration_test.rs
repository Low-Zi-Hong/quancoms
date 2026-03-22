use quancoms::qubit::{self, QuantumRegister};

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
fn Rx() {
    let mut q = QuantumRegister::new(2);
    q.Rx(0, std::f64::consts::PI).unwrap();

    let expected = 1.0;
    let result = q.god_observe();

    assert!((result[0] - 0.0).abs() < 1e-6);
    assert!((result[1] - expected).abs() < 1e-6);
}

#[test]
fn Ry() {
    let mut q = QuantumRegister::new(2);
    q.Ry(0, std::f64::consts::PI).unwrap();

    let result = q.god_observe();

    assert!((result[0] - 0.0).abs() < 1e-6);
    assert!((result[1] - 1.0).abs() < 1e-6);
}

#[test]
fn Rz() {
    let mut q = QuantumRegister::new(2);
    q.Rz(0, std::f64::consts::PI / 2.0).unwrap();

    let result = q.god_observe();

    assert!((result[0] - 1.0).abs() < 1e-6);
}
