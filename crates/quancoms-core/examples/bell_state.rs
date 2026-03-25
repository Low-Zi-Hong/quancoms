use quancoms_core::qubit::QuantumRegister;

fn main() {
    let mut q = QuantumRegister::new(2).unwrap();
    q.H(0).unwrap().CNOT(0, 1).unwrap();
    println!("{:?}", q.god_observe().unwrap());
    print!("{}", q.observe().unwrap());
}
