use crate::qubit::QuantumRegister;
mod complex;
mod qubit;

#[allow(unused)]
fn main() {
    println!("Hello, world!");

    let mut q1 = QuantumRegister::new(2).unwrap();
    let mut q2 = QuantumRegister::new(2).unwrap();

    q1.X(0).unwrap();
    q2.X_test(0).unwrap();

    println!("{:?}", q1.god_observe().unwrap());
    println!("{:?}", q2.god_observe().unwrap());
}
