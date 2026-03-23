# quancoms
A high-performance, full-state vector quantum simulator written in Rust.

"Where bitwise magic meets quantum mechanics."

# Features
 - $O(2^{n-k})$ Optimization: Uses a specialized Bit-Insertion Technique to skip unnecessary state scans, making controlled gates ($CNOT$, $CCNOT$, $MCU$) significantly faster than naive implementations.
 - Memory Safety: Built-in exponential RAM requirement pre-check. It knows your hardware limits before it even tries to allocate.
 - Comprehensive Gate Set:
  - Unary: $X, H, R_x, R_y, R_z$, and the universal $U$ gate.
  - Controlled: $CNOT, CCNOT, CSWAP$.
  - Universal: $MCU$ (Multi-Controlled Unitary) for any $k$-control configuration
 - Flexible Observation: Supports full collapse, partial measurement, and "God-eye" non-destructive probability inspection.

 # Quick Start
 Create a Bell State ($|00\rangle + |11\rangle$)

 ```
 use quancoms::qubit::QuantumRegister;

fn main() -> Result<(), String> {
    let mut reg = QuantumRegister::new(2)?;

    reg.H(0)?       // Superposition on qubit 0
       .CNOT(0, 1)?; // Entangle qubit 0 and 1

    let result = reg.observe()?;
    println!("Measured state: {:?}", result);
    Ok(())
}
```

# Optimisation
Most simulators loop through all $2^n$ states for every gate. BIQ is different.For a gate with $k$ control qubits, we only iterate through $2^{n-(k+1)}$ subspaces. We use a "Triple Hole" (or N-Hole) injection strategy to reconstruct the target indices on the fly using bitwise XOR and shifts.
Performance Gain:
| Gate | Naive Loop | BIQ (Bit-Injected) |
| :--- | :--- | :--- |
| X / H / U | $2^n$ | $2^{n-1}$ (2x Faster) |
| CNOT | $2^n$ | $2^{n-2}$ (4x Faster) |
| CCNOT / CSWAP | $2^n$ | $2^{n-3}$ (8x Faster) |

# Hardware limit
Quantum simulation is memory-intensive. Each additional qubit doubles the RAM requirement.
| Qubits | RAM Required | Note |
| 20 | 16 MiB | Runs on a toaster|
| 30 | 16 GiB | Standard PC limit |
| 40 | 16 TiB | Data center required| 
| 100 | $\infty$ | Beyond the observable universe's storage| 
Note: BIQ will panic! safely if your available RAM cannot accommodate the requested state vector.

# Testing
We take physics seriously. Our test suite (currently 12+ passed) includes:
 - Bell State correlation tests.
 - GHZ State ($|000\rangle + |111\rangle$) multi-measurement consistency.
 - Unitary matrix reversibility.

 ```
  cargo test
 ```