use crate::qubit::QuantumRegister;

use std::time::Instant;

#[allow(unused)]
fn main() {
    println!("Hello, world!");

    let mut sec = vec![0.0; 35];

    for (n, x) in sec.iter_mut().enumerate() {
        let mut q_ok = QuantumRegister::new(n);
        if let Ok(mut q) = q_ok {
            let start = Instant::now();
            for i in 0..n {
                q.H(i).unwrap();
            }
            for i in 0..n.saturating_sub(1) {
                q.CNOT(i, i + 1_usize).unwrap();
            }
            let duration = start.elapsed();
            println!(
                "Qubits: {:2} | {} | Time: {:?}",
                n,
                q.observe().unwrap(),
                duration
            );
            *x = duration.as_micros() as f64;
        } else {
            *x = 0 as f64;
        }
    }
}
