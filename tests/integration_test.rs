use quancoms::complex::Complex;
use quancoms::qubit::{self, QuantumRegister};

#[test]
fn test_X() {
    let mut q = QuantumRegister::new(2).unwrap();
    q.X(0).unwrap();

    let expected = [0.0, 1.0, 0.0, 0.0];

    assert!((q.god_observe().unwrap()[1] - expected[1]).abs() < 1e-6);
}

#[test]
fn test_H() {
    let mut q = QuantumRegister::new(2).unwrap();
    q.H(0).unwrap();
    q.H(1).unwrap();

    let expected = 0.25;
    let result = q.god_observe().unwrap();

    for x in result.iter() {
        assert!((x - expected).abs() < 1e-6);
    }
}

#[test]
fn test_CNOT() {
    let mut q = QuantumRegister::new(2).unwrap();
    q.H(0).unwrap();
    q.CNOT(0, 1).unwrap();

    let expected = 0.5;
    let result = q.god_observe().unwrap();

    assert!((result[0] - expected).abs() < 1e-6);
    assert!((result[3] - expected).abs() < 1e-6);
}

#[test]
fn test_CCNOT() {
    let mut q = QuantumRegister::new(3).unwrap();
    q.H(0).unwrap();
    q.H(1).unwrap();
    q.CCNOT(0, 1, 2).unwrap();

    let expected = 0.25;
    let result = q.god_observe().unwrap();

    assert!((result[0] - expected).abs() < 1e-6);
    assert!((result[1] - expected).abs() < 1e-6);
    assert!((result[2] - expected).abs() < 1e-6);
    assert!((result[7] - expected).abs() < 1e-6);
}

#[test]
fn test_Rx() {
    let mut q = QuantumRegister::new(2).unwrap();
    q.Rx(0, std::f64::consts::PI).unwrap();

    let expected = 1.0;
    let result = q.god_observe().unwrap();

    assert!((result[0] - 0.0).abs() < 1e-6);
    assert!((result[1] - expected).abs() < 1e-6);
}

#[test]
fn test_Ry() {
    let mut q = QuantumRegister::new(2).unwrap();
    q.Ry(0, std::f64::consts::PI).unwrap();

    let result = q.god_observe().unwrap();

    assert!((result[0] - 0.0).abs() < 1e-6);
    assert!((result[1] - 1.0).abs() < 1e-6);
}

#[test]
fn test_Rz() {
    let mut q = QuantumRegister::new(2).unwrap();
    q.Rz(0, std::f64::consts::PI / 2.0).unwrap();

    let result = q.god_observe().unwrap();

    assert!((result[0] - 1.0).abs() < 1e-6);
}

#[test]
fn test_U() {
    let mut q = QuantumRegister::new(2).unwrap();
    let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
    let c_val = Complex::new(inv_sqrt2, 0.0);
    let c_neg = Complex::new(-inv_sqrt2, 0.0);

    // 使用通用 U 门模拟 H 门
    q.U(0, c_val, c_val, c_val, c_neg).unwrap();

    let result = q.god_observe().unwrap();

    assert!((result[0] - 0.5).abs() < 1e-6);
    assert!((result[1] - 0.5).abs() < 1e-6);
}

#[test]
fn test_MCU_triple_control() {
    let mut q = QuantumRegister::new(4).unwrap();
    // 设置三个控制位为 1
    q.X(0).unwrap();
    q.X(1).unwrap();
    q.X(2).unwrap();

    let u00 = Complex::new(0.0, 0.0);
    let u01 = Complex::new(1.0, 0.0);
    let u10 = Complex::new(1.0, 0.0);
    let u11 = Complex::new(0.0, 0.0);

    // 当 0, 1, 2 全为 1 时，翻转 3
    q.MCU(vec![0, 1, 2], 3, u00, u01, u10, u11).unwrap();

    let result = q.god_observe().unwrap();
    // 状态应该是 |1111>，即索引 15 的概率为 1.0
    assert!((result[15] - 1.0).abs() < 1e-6);
}

#[test]
fn test_bell_state_observation() {
    let mut reg = QuantumRegister::new(2).unwrap();
    // 构造贝尔态 (|00> + |11>) / sqrt(2)
    // 假设你已经写好了这些门，如果没有，就手动改 state
    reg.state[0] = Complex::new(0.707106, 0.0);
    reg.state[3] = Complex::new(0.707106, 0.0);

    let result = reg.observe().unwrap();
    // 验证：贝尔态观测结果只能是 0 或 3
    assert!(result.value == 0 || result.value == 3);

    // 验证：坍缩后，选中的那个概率幅必须是 1.0
    assert!((reg.state[result.value].re - 1.0).abs() < 1e-6);
}

#[test]
fn test_observe_one_consistency() {
    let mut reg = QuantumRegister::new(2).unwrap();
    reg.state[0] = Complex::new(0.707106, 0.0);
    reg.state[3] = Complex::new(0.707106, 0.0);

    let res = reg.observe_one(0).unwrap();
    // 如果第 0 位观测到 0，那么整个态必须坍缩到 |00>
    if res.value == 0 {
        assert!((reg.state[0].re - 1.0).abs() < 1e-6);
        assert!(reg.state[3].re == 0.0);
    } else {
        assert!((reg.state[3].re - 1.0).abs() < 1e-6);
        assert!(reg.state[0].re == 0.0);
    }
}

#[test]
fn test_observe_mul_ghz_correlation() {
    // 1. Setup a 3-qubit register
    let mut reg = QuantumRegister::new(3).unwrap();

    // 2. Manually set state to GHZ: (|000> + |111>) / sqrt(2)
    // index 0 is |000>, index 7 is |111>
    reg.state.fill(Complex::new(0.0, 0.0));
    reg.state[0] = Complex::new(0.70710678, 0.0);
    reg.state[7] = Complex::new(0.70710678, 0.0);

    // 3. Measure non-contiguous qubits: [0, 2]
    let targets = vec![0, 2];
    let result = reg.observe_mul(targets).unwrap();

    // 4. VERIFY LOGIC
    if result.value == 0 {
        // If result is |00>, state must be |000>
        assert!((reg.state[0].re.abs() - 1.0) < 1e-6);
        assert_eq!(reg.state[7].re, 0.0);
    } else if result.value == 3 {
        // If result is |11> (binary 3 for qubits 0 & 2), state must be |111>
        assert!((reg.state[7].re.abs() - 1.0) < 1e-6);
        assert_eq!(reg.state[0].re, 0.0);
    } else {
        panic!(
            "Physics broken! Observed impossible state index {}",
            result.value
        );
    }
}
