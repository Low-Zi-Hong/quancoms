use quancoms::qubit::QuantumRegister;

fn main() {
    let mut q = QuantumRegister::new(2);
    q.H(0).unwrap().CNOT(0, 1).unwrap();
    println!("{:?}", q.god_observe());
    print!("{}", q.observe());
}
