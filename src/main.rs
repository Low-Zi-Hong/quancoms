use crate::qubit::QuantumRegister;
mod complex;
mod qubit;

#[allow(unused)]
fn main() {
    println!("Hello, world!");
    env_logger::init();

    let qu = complex::Complex::new(0.0, 1.0);
    let qu2 = complex::Complex::new(1.0, 1.0);

    print!("{}", (qu / qu2));

    let n = 3;
    let mut quantum = QuantumRegister::new(n).unwrap();
    //let mut quantum2 = QuantumRegister::new(n);
    quantum.X(0);
    print!("{:?}", quantum.god_observe());
    quantum.H(0);
    quantum.H(1);
    quantum.CNOT(0, 1);
    quantum.CCNOT(0, 1, 2).unwrap();
    print!("{:?}", quantum.god_observe().unwrap());
    print!("{}", quantum.observe().unwrap());

    //print!("{:?} and {:?}", quantum, quantum2);
    // 看看快了多少倍
    print!("\n\n\n\n");
    let mut q = QuantumRegister::new(2).unwrap();
    q.H(0).unwrap();
    q.Rz(0, std::f64::consts::PI / 2.0).unwrap();
    print!("{:?}", q.god_observe());
    let mut q2 = QuantumRegister::new(5).unwrap();
    q2.H(0).unwrap().H(1).unwrap().H(3).unwrap();
    q2.CCNOT(0, 2, 1).unwrap();
    q2.X(4).unwrap();
    q2.Rx(0, 2.55);
    q2.Ry(0, 2.3);
    q2.Rz(0, 2.3);

    println!("{:?}", q2.god_observe());
    let r = q2.observe_one(0).unwrap();

    println!("{}", r);
    println!("{:?}", q2.god_observe());

    let mut qqq = QuantumRegister::new(100);
    print!("{}", qqq.unwrap().observe().unwrap());
}
