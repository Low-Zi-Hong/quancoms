use quancoms::qubit::QuantumRegister;

fn main() {
    let mut q = QuantumRegister::new(28).unwrap();
    q.X(1).unwrap();
    q.H(0).unwrap();
    q.H(1).unwrap();

    //Oracle
    q.CNOT(0, 1).unwrap();

    q.H(0).unwrap();

    // |0> as the test qubit
    let result = q.observe_one(0).unwrap();
    println!("测量结果: {}", result);
}
