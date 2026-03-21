use log::{warn, info};
use env_logger;
use crate::qubit::QuantumRegister;

mod qubit;
mod complex;

//test
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    env_logger::init();

    let qu = complex::Complex::new(0.0,1.0);
    let qu2 = complex::Complex::new(1.0,1.0);

    print!("{}",(qu / qu2));

    let n = 3;
    let mut quantum = QuantumRegister::new(n);
    let mut quantum2 = QuantumRegister::new(n);
    quantum.X(0);
    print!("{:?}",quantum.god_observe());
    quantum.H(0);
    quantum.H(1);
    quantum.CNOT(0, 1);
    quantum.CCNOT(0, 1, 2).unwrap();
    print!("{:?}",quantum.god_observe());

    //print!("{:?} and {:?}", quantum, quantum2);
    // 看看快了多少倍
}
