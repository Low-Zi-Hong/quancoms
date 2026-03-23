use core::panic;
use sysinfo::System;

use crate::complex::{self, Complex};
use std::{f64::consts::FRAC_1_SQRT_2, vec};

///Structure of a Quantum Register
#[repr(C)]
#[derive(PartialEq, Clone, Debug, Default)]
pub struct QuantumRegister {
    ///Number of qubits to simulate
    pub qubits: usize,

    ///State of the current quantum computer, this follow the number of qubits $2^n$
    pub state: Vec<Complex>,

    ///Size of the state vector
    size: usize,
}

#[allow(non_snake_case)]
impl QuantumRegister {
    ///Create new Quantum Register with init state of the first qubit to |1>
    ///
    /// # Arguments
    ///
    /// * `n` - Number of Qubit want to stimulate.
    ///
    /// # Note
    ///
    /// RAM needed to grow exponentially $2^n \times 16$
    ///
    /// # Panics
    /// Check your computer have enough RAM to simulate or else panic.
    #[allow(dead_code)]
    pub fn new(n: usize) -> Result<Self, String> {
        let _size = 1_usize << n;

        let mut sys = System::new_all();
        sys.refresh_memory();
        let memory = sys.available_memory();

        if (_size * 16_usize) as u64 > memory {
            panic!(
                "You have {} bytes of memory and Not enough memory to simulate {} qubits! This which needs.. {} bytes of memory! This need {} more memory",
                memory,
                n,
                _size * 16_usize,
                (_size * 16_usize) as u64 - memory
            );
        }

        let mut v = vec![Complex::new(0.0, 0.0); _size];
        //let |0...0> equals 1
        v[0] = Complex::new(1.0, 0.0);
        Ok(Self {
            qubits: n,
            state: v,
            size: _size,
        })
    }

    /// Measures the entire quantum register, causing the wavefunction to collapse into a single basis state.
    ///
    /// The collapse is determined by the probability amplitudes of the current state.
    /// According to Born's rule, the probability of collapsing into a specific state
    /// is given by $P(\text{state}) = |\text{amplitude}|^2$.
    ///
    /// # Returns
    ///
    /// Returns a [`DiracKet`] representing the observed classical value in $|X...X\rangle$ format.
    #[allow(dead_code)]
    pub fn observe(&mut self) -> Result<DiracKet, String> {
        let dart: f64 = rand::random();

        let mut current_pos = 0.0;
        let mut hit_index = 0;

        for x in 0..self.state.len() {
            //here follow Born's rule
            let real = self.state[x].re;
            let imag = self.state[x].im;
            let prob = real * real + imag * imag;

            current_pos += prob;

            if dart <= current_pos {
                hit_index = x;
                break;
            }
        }

        self.state.fill(Complex { re: 0.0, im: 0.0 });
        self.state[hit_index].re = 1.0;

        Ok(DiracKet {
            value: hit_index,
            width: self.qubits,
        })
    }

    /// Measures a single qubit in the register, causing a partial collapse of the wavefunction.
    ///
    /// The probability of the outcome is determined by the squared magnitudes of the
    /// current state's amplitudes. According to Born's rule:
    /// $P(\text{outcome}) = \sum |\text{amplitude}_{\text{match}}|^2$.
    ///
    /// After measurement, the amplitudes of states inconsistent with the result are
    /// zeroed out. The remaining amplitudes are then normalized by a factor of $1/\sqrt{P}$
    /// to ensure the total probability remains 1.
    ///
    /// # Arguments
    ///
    /// * `target` - The index of the qubit to be measured.
    ///
    /// # Returns
    ///
    /// Returns a [`DiracKet`] containing the classical result (0 or 1) in $|X\rangle$ format.
    ///
    /// # Panics
    ///
    /// Panics if the `target` index is out of scope.
    #[allow(dead_code)]
    pub fn observe_one(&mut self, target: usize) -> Result<DiracKet, String> {
        if target >= self.qubits {
            panic!("target out of scope!");
        }
        let mut high_prob = 0.0;
        for x in 0..self.size {
            if (x >> target) & 1_usize == 1 {
                high_prob += Complex::prob(self.state[x]);
            }
        }

        let dart: f64 = rand::random();

        let hit_index = if dart < high_prob { 1 } else { 0 };

        let final_prob = if hit_index == 0 {
            1.0 - high_prob
        } else {
            high_prob
        };

        let norm_factor = final_prob.sqrt();

        let mask = (1_usize << target) - 1_usize;
        let bit = self.size >> 1_usize;

        for x in 0..bit {
            let low = ((x >> target) << (target + 1_usize)) ^ (x & mask);
            let high = low | (1_usize << target);

            if hit_index == 0 {
                // 结果是 0：保留 low 并放大，抹除 high
                self.state[low] = self.state[low] / norm_factor;
                self.state[high] = Complex { re: 0.0, im: 0.0 };
            } else {
                // 结果是 1：抹除 low，保留 high 并放大
                self.state[low] = Complex { re: 0.0, im: 0.0 };
                self.state[high] = self.state[high] / norm_factor;
            }
        }

        Ok(DiracKet {
            value: hit_index,
            width: 1,
        })
    }

    /// Simultaneously measures multiple qubits in the register, leading to a partial collapse.
    ///
    /// The probability of each possible multi-qubit outcome is calculated by summing the
    /// squared magnitudes of all corresponding state-vector components.
    /// According to Born's rule:
    /// $P(\text{outcome}) = \sum |\text{amplitude}_{\text{match}}|^2$.
    ///
    /// This function performs a projection onto the subspace defined by the measurement result.
    /// States inconsistent with the observed pattern are zeroed out, and the remaining
    /// amplitudes are normalized by $1/\sqrt{P}$ to maintain unit probability.
    ///
    /// # Arguments
    ///
    /// * `targets` - A vector of indices of the qubits to be measured.
    ///
    /// # Returns
    ///
    /// Returns a [`DiracKet`] containing the observed multi-bit value in $|X...X\rangle$ format.
    ///
    /// # Panics
    ///
    /// Panics if any index in `targets` is out of scope.
    ///
    /// /// # Examples
    ///
    /// ```
    /// use quancoms::qubit::QuantumRegister;
    ///
    /// let mut reg = QuantumRegister::new(2).unwrap();
    /// // ... apply gates ...
    /// let result = reg.observe_mul(vec![0, 1]).unwrap();
    /// println!("Measured: {}", result);
    /// ```
    #[allow(dead_code)]
    pub fn observe_mul(&mut self, mut targets: Vec<usize>) -> Result<DiracKet, String> {
        if targets.iter().any(|&c| c >= self.qubits) {
            panic!("target out of scope!");
        }
        //sort the targets
        targets.sort();
        let mut prob = vec![0.0; 1_usize << targets.len()];

        //loop through the 2^n states
        for x in 0..self.size {
            //find the target probability... we find 0 enough
            let mut num: usize = 0;
            //we get the state of the targets qubit with |0> state
            for (n, i) in targets.iter().enumerate() {
                num |= (x >> *i & 1_usize) << n;
            }
            prob[num] += Complex::prob(self.state[x]);
        }

        let dart: f64 = rand::random();

        let mut current_pos = 0.0;
        let mut hit_index = 0;

        for (n, x) in prob.iter().enumerate() {
            current_pos += *x;

            if dart <= current_pos {
                hit_index = n;
                break;
            }
        }

        let norm_factor = prob[hit_index].sqrt();
        let mut mask: usize = 0;
        let mut full_mask: usize = 0;
        for (n, t) in targets.iter().enumerate() {
            if (hit_index >> n) & 1_usize == 1_usize {
                mask |= 1_usize << *t;
            } //this make the mask
            full_mask |= 1_usize << *t; //this make the full mask masking all the bit we need
        }

        for x in 0..self.size {
            //we and with the full mask to get the pure bits then XOR to detect it match or not
            if (x & full_mask) ^ mask == 0_usize {
                self.state[x] = self.state[x] / norm_factor;
            } else {
                self.state[x] = Complex { re: 0.0, im: 0.0 };
            }
        }

        Ok(DiracKet {
            value: hit_index,
            width: targets.len(),
        })
    }

    /// Provides a "god-eye" view of the register by calculating the probability distribution
    /// without collapsing the wavefunction.
    ///
    /// Unlike a standard measurement, this operation is non-destructive and
    /// preserves the current quantum superposition. It is primarily used for
    /// debugging and visualization (e.g., Manim animations).
    ///
    /// # Returns
    ///
    /// Returns a [`Vec<f64>`] where each element represents the probability
    /// of a specific basis state, calculated as $P(i) = |c_i|^2$.
    #[allow(dead_code)]
    pub fn god_observe(&mut self) -> Result<Vec<f64>, String> {
        Ok(self
            .state
            .iter()
            .map(|c| c.re * c.re + c.im * c.im)
            .collect())
    }

    /*This block is slightly slower do the same thing with the bellow block but slower */
    #[allow(dead_code)]
    pub fn x_native(&mut self, target: usize) {
        if target >= self.qubits {
            panic!("target out of range!");
        } else {
            for x in 0..(self.size) {
                /*Find the bit where at the target location is 1 just by bit shift to the right and "and" 1 if 1 then return 1 else 0 */
                if (x >> target) & 1 == 1 {
                    /*The opposite is just XOR the bit of the original index at the target position. */
                    let opposite = x ^ (1 << target);
                    self.state.swap(x, opposite);
                }
            }
        }
    }

    /// Applies the Pauli-X gate (NOT gate) to the target qubit.
    ///
    /// The X gate maps the basis state $|0\rangle$ to $|1\rangle$ and vice-versa.
    /// Mathematically, it swaps the probability amplitudes of states where the target qubit is 0
    /// with those where the target qubit is 1.
    ///
    /// # Optimization: Bit Insertion Technique
    ///
    /// Standard simulation requires iterating through all $2^n$ states. However, this
    /// implementation optimizes the runtime to $O(2^{n-1})$ by using a bit-insertion
    /// technique. Instead of checking every state, we iterate through $2^{n-1}$ states
    /// and "inject" the target bit to find the corresponding `low` (0) and `high` (1) indices.
    ///
    /// ### Bit Logic Example:
    /// To insert a bit at index 3 of the value `10010` (binary):
    /// 1. **Shift High Part**: `(x >> target) << (target + 1)` creates a "hole" at the target position.
    /// 2. **Mask Low Part**: `x & ((1 << target) - 1)` extracts the bits below the target.
    /// 3. **Combine**: XORing these two results gives the `low` index with a 0 at the target.
    /// 4. **Set High**: ORing the `low` index with `1 << target` gives the `high` index with a 1 at the target.
    ///
    /// # Arguments
    ///
    /// * `target` - The index of the qubit to apply the X gate.
    ///
    /// # Errors
    ///
    /// Returns an error string if the `target` index is out of the register's scope.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut reg = QuantumRegister::new(2).unwrap();
    /// reg.X(0).expect("Scope error");
    /// ```
    #[allow(dead_code)]
    pub fn X(&mut self, target: usize) -> Result<&mut Self, String> {
        if target >= self.qubits {
            return Err("...".into());
        } else {
            //This block decrease the loop from 2^n to 2^n-1
            let bit = self.size >> 1_usize;
            for x in 0..bit {
                let low =
                    ((x >> target) << (target + 1_usize)) ^ (x & ((1_usize << target) - 1_usize));
                let high = low | (1_usize << target);
                self.state.swap(low, high);
            }
        }
        Ok(self)
    }

    /// Applies the Hadamard (H) gate to the target qubit.
    ///
    /// The Hadamard gate creates a balanced superposition. It transforms the basis
    /// states according to:
    /// $H|0\rangle = \frac{|0\rangle + |1\rangle}{\sqrt{2}}$ and $H|1\rangle = \frac{|0\rangle - |1\rangle}{\sqrt{2}}$.
    ///
    /// After applying this gate to a qubit in a definite state, measuring it will
    /// yield $|0\rangle$ or $|1\rangle$ with an equal probability of $0.5$.
    ///
    /// # Optimization: Bit Insertion Technique
    ///
    /// This implementation utilizes the same bit-insertion optimization as the [`Self::X`] gate,
    /// reducing the required iterations from $2^n$ to $2^{n-1}$.
    /// Please refer to the [`Self::X`] documentation for a detailed explanation of the indexing logic.
    ///
    /// # Arguments
    ///
    /// * `target` - The index of the qubit to apply the H gate.
    ///
    /// # Errors
    ///
    /// Returns an error string if the `target` index is out of the register's scope.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut reg = QuantumRegister::new(1).unwrap();
    /// reg.H(0).expect("Scope error");
    /// ```
    #[allow(dead_code)]
    pub fn H(&mut self, target: usize) -> Result<&mut Self, String> {
        if target >= self.qubits {
            return Err("...".into());
        } else {
            let bit = self.size >> 1_usize;
            for x in 0..bit {
                let low =
                    ((x >> target) << (target + 1_usize)) ^ (x & ((1_usize << target) - 1_usize));
                let high = low | (1_usize << target);

                let a = self.state[low];
                let b = self.state[high];

                self.state[low] = (a + b) * FRAC_1_SQRT_2;
                self.state[high] = (a - b) * FRAC_1_SQRT_2;
            }
        }
        Ok(self)
    }

    /// Applies a generic $2 \times 2$ unitary gate (U gate) to the target qubit.
    ///
    /// This is the generalized form of all single-qubit operations. It performs
    /// the following matrix multiplication on the subspace of the target qubit:
    ///
    /// $$
    /// \begin{pmatrix} \psi_{low}' \\ \psi_{high}' \end{pmatrix} =
    /// \begin{pmatrix} U_{00} & U_{01} \\ U_{10} & U_{11} \end{pmatrix}
    /// \begin{pmatrix} \psi_{low} \\ \psi_{high} \end{pmatrix}
    /// $$
    ///
    /// # Optimization: Bit Insertion Technique
    ///
    /// Similar to the [`Self::X`] and [`Self::H`] gates, this function uses bit-insertion
    /// to achieve $O(2^{n-1})$ efficiency by only iterating through half of the state vector.
    ///
    /// # Arguments
    ///
    /// * `target` - The index of the qubit to apply the U gate.
    /// * `U00`, `U01`, `U10`, `U11` - The four complex components of the unitary matrix.
    ///
    /// # Errors
    ///
    /// Returns an error string if the `target` index is out of the register's scope.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut reg = QuantumRegister::new(1).unwrap();
    /// // Example: Identity gate (does nothing)
    /// let c1 = Complex::new(1.0, 0.0);
    /// let c0 = Complex::new(0.0, 0.0);
    /// reg.U(0, c1, c0, c0, c1).expect("Scope error");
    /// ```
    #[allow(dead_code)]
    pub fn U(
        &mut self,
        target: usize,
        U00: Complex,
        U01: Complex,
        U10: Complex,
        U11: Complex,
    ) -> Result<&mut Self, String> {
        if target >= self.qubits {
            return Err("...".into());
        } else {
            let bit = self.size >> 1_usize;
            for x in 0..bit {
                let low =
                    ((x >> target) << (target + 1_usize)) ^ (x & ((1_usize << target) - 1_usize));
                let high = low | (1_usize << target);

                let a = self.state[low];
                let b = self.state[high];

                self.state[low] = (U00 * a) + (U01 * b);
                self.state[high] = (U10 * a) + (U11 * b);
            }
        }
        Ok(self)
    }

    /// Applies the Controlled-NOT (CNOT) gate to the target qubit, controlled by the control qubit.
    ///
    /// The CNOT gate flips the target qubit if and only if the control qubit is in the $|1\rangle$ state.
    /// It maps the basis states as follows:
    /// - $|0, 0\rangle \to |0, 0\rangle$
    /// - $|0, 1\rangle \to |0, 1\rangle$
    /// - $|1, 0\rangle \to |1, 1\rangle$
    /// - $|1, 1\rangle \to |1, 0\rangle$
    ///
    /// # Optimization: Double Bit Insertion Technique
    ///
    /// Instead of a naive $O(2^n)$ scan, this implementation optimizes the process to $O(2^{n-2})$
    /// by isolating the subspace where the control qubit is fixed to $|1\rangle$.
    ///
    /// ### The "Two Holes" Logic:
    /// 1. We iterate through $2^{n-2}$ states, effectively "skipping" the two qubits involved.
    /// 2. **Stable Insertion**: We perform bit-insertion for the `control` and `target` bits
    ///    sequentially. We sort the indices to ensure that inserting a bit at a lower position
    ///    does not shift the intended index of a higher-order bit.
    /// 3. **Fixing the Control**: After the first insertion at the `control` position,
    ///    we force that bit to 1. This ensures we only operate on the states that satisfy
    ///    the CNOT condition.
    /// 4. **Target Mapping**: The second insertion creates a pair of indices:
    ///    - `low`: Where `control=1` and `target=0`.
    ///    - `high`: Where `control=1` and `target=1`.
    /// 5. **The Swap**: We swap the amplitudes of `low` and `high` to perform the NOT operation.
    ///
    /// # Arguments
    ///
    /// * `control` - The index of the qubit used as the control.
    /// * `target` - The index of the qubit to be flipped.
    ///
    /// # Errors
    ///
    /// Returns an error if the indices are out of scope or if `control == target`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut reg = QuantumRegister::new(2).unwrap();
    /// // Create state |10>, CNOT(0, 1) will result in |11>
    /// reg.X(0)?.CNOT(0, 1).expect("Scope error");
    /// ```
    #[allow(dead_code)]
    pub fn CNOT(&mut self, control: usize, target: usize) -> Result<&mut Self, String> {
        if target >= self.qubits || control >= self.qubits || control == target {
            return Err("...".into());
        } else {
            let bit = self.size >> 2_usize;
            for x in 0..bit {
                if control < target {
                    //slice
                    let mut s1 = ((x >> control) << (control + 1_usize))
                        ^ (x & ((1_usize << control) - 1_usize));
                    //insert control bit
                    s1 |= 1_usize << control;
                    //insert target
                    let low = ((s1 >> target) << (target + 1_usize))
                        ^ (s1 & ((1_usize << target) - 1_usize));
                    let high = low | (1_usize << target);
                    self.state.swap(low, high);
                } else {
                    let mut low = ((x >> target) << (target + 1_usize))
                        ^ (x & ((1_usize << target) - 1_usize));
                    low = ((low >> control) << (control + 1_usize))
                        ^ (low & ((1_usize << control) - 1_usize))
                        | (1_usize << control);
                    let high = low | (1_usize << target);
                    self.state.swap(low, high);
                }
            }
        }
        Ok(self)
    }

    /// Applies the CCNOT (Toffoli) gate to the target qubit, controlled by two qubits.
    ///
    /// The target qubit is flipped (X gate) if and only if both `control1` and `control2`
    /// are in the |1> state. This is a universal gate for classical logic.
    ///
    /// # Optimization: Triple Bit Insertion Technique
    ///
    /// To avoid scanning all $2^n$ states, this implementation uses a triple-layer
    /// bit-insertion technique, reducing the loop count by a factor of $2^3 = 8$
    /// (from $2^n$ to $2^{n-3}$).
    ///
    /// # Logic Breakdown:
    /// 1. **Space Reduction**: We loop through 2^(n-3) states, focusing only on the
    ///    subspace where the three bits (c1, c2, target) are flexible.
    /// 2. **Sequential Injection**: By sorting `arr`, we can inject a '1' into each
    ///    target position one by one. Sorting ensures that inserting a bit at a
    ///    lower index doesn't shift the intended position of a higher index.
    /// 3. **The "High" State**: After three injections, the `high` index represents
    ///    the state where `control1=1`, `control2=1`, and `target=1`.
    /// 4. **The "Low" State**: By XORing the `target` bit, we derive the `low` index
    ///    where `control1=1`, `control2=1`, and `target=0`.
    /// 5. **The Swap**: We swap these two amplitudes to perform the flip.
    ///
    /// # Arguments
    ///
    /// * `control1` - The first control qubit index.
    /// * `control2` - The second control qubit index.
    /// * `target` - The target qubit index to be flipped.
    ///
    /// # Errors
    ///
    /// Returns an error if any index is out of scope or if indices overlap.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut reg = QuantumRegister::new(3).unwrap();
    /// // If state is |110>, CCNOT(0, 1, 2) results in |111>
    /// reg.X(0)?.X(1)?.CCNOT(0, 1, 2).expect("Scope error");
    /// ```
    #[allow(dead_code)]
    pub fn CCNOT(
        &mut self,
        control1: usize,
        control2: usize,
        target: usize,
    ) -> Result<&mut Self, String> {
        if target >= self.qubits
            || control1 >= self.qubits
            || control2 >= self.qubits
            || control1 == target
            || control1 == control2
        {
            return Err("...".into());
        } else {
            let bit = self.size >> 3_usize;
            let mut arr = [control1, control2, target];
            arr.sort();
            for x in 0..bit {
                let mut high = (((x >> arr[0]) << (arr[0] + 1_usize))
                    ^ (x & ((1_usize << arr[0]) - 1_usize)))
                    | (1_usize << arr[0]);
                high = (((high >> arr[1]) << (arr[1] + 1_usize))
                    ^ (high & ((1_usize << arr[1]) - 1_usize)))
                    | (1_usize << arr[1]);
                high = (((high >> arr[2]) << (arr[2] + 1_usize))
                    ^ (high & ((1_usize << arr[2]) - 1_usize)))
                    | (1_usize << arr[2]);

                let low = high ^ (1_usize << target);
                self.state.swap(low, high);
            }
        }
        Ok(self)
    }

    /*Theta here use radian ha :D */
    /// Rotates the target qubit around the Z-axis of the Bloch sphere.
    ///
    /// The $R_z$ gate introduces a relative phase shift between $|0\rangle$ and $|1\rangle$.
    /// The transformation matrix is:
    /// $$R_z(\theta) = \begin{pmatrix} e^{-i\theta/2} & 0 \\ 0 & e^{i\theta/2} \end{pmatrix}$$
    ///
    /// # Arguments
    ///
    /// * `target` - The index of the qubit to apply the rotation.
    /// * `theta` - The rotation angle in **radians**.
    ///
    /// # Optimization: Bit Insertion Technique
    ///
    /// Achieves $O(2^{n-1})$ complexity by isolating the subspace where the target bit is toggled,
    /// performing the diagonal phase multiplication without full-state iteration.
    #[allow(dead_code)]
    pub fn Rz(&mut self, target: usize, theta: f64) -> Result<&mut Self, String> {
        if target >= self.qubits {
            return Err("...".into());
        } else {
            let bit = self.size >> 1_usize;
            let neg_half_theta =
                complex::Complex::new(f64::cos(-theta / 2.0), f64::sin(-theta / 2.0));
            let pos_half_theta =
                complex::Complex::new(f64::cos(theta / 2.0), f64::sin(theta / 2.0));
            for x in 0..bit {
                let low =
                    ((x >> target) << (target + 1_usize)) ^ (x & ((1_usize << target) - 1_usize));
                let high = low | (1_usize << target);
                self.state[low] = self.state[low] * neg_half_theta;
                self.state[high] = self.state[high] * pos_half_theta;
            }
        }
        Ok(self)
    }

    /*Theta here use radian ha :D */
    /// Rotates the target qubit around the X-axis of the Bloch sphere.
    ///
    /// This gate creates a superposition by rotating the state between the
    /// computational basis states $|0\rangle$ and $|1\rangle$.
    /// The transformation matrix is:
    /// $$R_x(\theta) = \begin{pmatrix} \cos(\theta/2) & -i\sin(\theta/2) \\ -i\sin(\theta/2) & \cos(\theta/2) \end{pmatrix}$$
    ///
    /// # Arguments
    ///
    /// * `target` - The index of the qubit to apply the rotation.
    /// * `theta` - The rotation angle in **radians**.
    #[allow(dead_code)]
    pub fn Rx(&mut self, target: usize, theta: f64) -> Result<&mut Self, String> {
        if target >= self.qubits {
            return Err("...".into());
        } else {
            let cos_half_theta = complex::Complex::new(f64::cos(theta / 2.0), 0.0);
            let sin_half_theta = complex::Complex::new(0.0, -f64::sin(theta / 2.0));
            let bit = self.size >> 1_usize;
            for x in 0..bit {
                let low =
                    ((x >> target) << (target + 1_usize)) ^ (x & ((1_usize << target) - 1_usize));
                let high = low | (1_usize << target);

                let low_state = self.state[low];

                self.state[low] =
                    (low_state * cos_half_theta) + (self.state[high] * sin_half_theta);
                self.state[high] =
                    (low_state * sin_half_theta) + (self.state[high] * cos_half_theta);
            }
        }
        Ok(self)
    }

    /*Theta here use radian ha :D */
    /// Rotates the target qubit around the X-axis of the Bloch sphere.
    ///
    /// This gate creates a superposition by rotating the state between the
    /// computational basis states $|0\rangle$ and $|1\rangle$.
    /// The transformation matrix is:
    /// $$R_x(\theta) = \begin{pmatrix} \cos(\theta/2) & -i\sin(\theta/2) \\ -i\sin(\theta/2) & \cos(\theta/2) \end{pmatrix}$$
    ///
    /// # Arguments
    ///
    /// * `target` - The index of the qubit to apply the rotation.
    /// * `theta` - The rotation angle in **radians**.
    #[allow(dead_code)]
    pub fn Ry(&mut self, target: usize, theta: f64) -> Result<&mut Self, String> {
        if target >= self.qubits {
            return Err("...".into());
        } else {
            let cos_half_theta = complex::Complex::new(f64::cos(theta / 2.0), 0.0);
            let sin_half_theta = complex::Complex::new(f64::sin(theta / 2.0), 0.0);
            let bit = self.size >> 1_usize;
            for x in 0..bit {
                let low =
                    ((x >> target) << (target + 1_usize)) ^ (x & ((1_usize << target) - 1_usize));
                let high = low | (1_usize << target);

                let low_state = self.state[low];

                self.state[low] =
                    (low_state * cos_half_theta) + (self.state[high] * -sin_half_theta);
                self.state[high] =
                    (low_state * sin_half_theta) + (self.state[high] * cos_half_theta);
            }
        }
        Ok(self)
    }

    /// Swaps the quantum states of two qubits.
    ///
    /// The SWAP gate exchanges the probability amplitudes between states where
    /// the two qubits differ ($|01\rangle \leftrightarrow |10\rangle$), while
    /// leaving $|00\rangle$ and $|11\rangle$ unchanged.
    ///
    /// # Optimization: Double Bit Insertion
    ///
    /// By using double bit-insertion, this implementation reduces the loop count
    /// from $2^n$ to $2^{n-2}$.
    /// 1. **Two Holes**: We iterate through $2^{n-2}$ states and insert "holes"
    ///    at the positions of `q1` and `q2`.
    /// 2. **Targeting Asymmetry**: We construct two specific indices:
    ///    - `low`: Qubit 1 is 1, Qubit 2 is 0.
    ///    - `high`: Qubit 1 is 0, Qubit 2 is 1.
    /// 3. **The Swap**: We swap the amplitudes of these two indices.
    ///
    /// # Arguments
    ///
    /// * `q1`, `q2` - The indices of the two qubits to be swapped.
    #[allow(dead_code)]
    pub fn SWAP(&mut self, q1: usize, q2: usize) -> Result<&mut Self, String> {
        if q1 >= self.qubits || q2 >= self.qubits || q1 == q2 {
            return Err("...".into());
        } else {
            let mut q = [q1, q2];
            q.sort();
            let bit = self.size >> 2_usize;
            for x in 0..bit {
                let mut qq: usize =
                    ((x >> q[0]) << (q[0] + 1_usize)) ^ (x & ((1_usize << q[0]) - 1_usize));
                qq = ((qq >> q[1]) << (q[1] + 1_usize)) ^ (qq & ((1_usize << q[1]) - 1_usize));
                let low = qq | (1_usize << q[0]);
                let high = qq | (1_usize << q[1]);

                self.state.swap(low, high);
            }
        }
        Ok(self)
    }

    /// Applies a Controlled-SWAP gate, also known as the Fredkin gate.
    ///
    /// This gate swaps the states of `target_1` and `target_2` if and only if
    /// the `control` qubit is $|1\rangle$.
    ///
    /// # Optimization: Triple Bit Insertion
    ///
    /// This implementation achieves $O(2^{n-3})$ efficiency by isolating the subspace
    /// where the `control` bit is fixed to 1 and the two targets are flexible.
    ///
    /// ### Logic Breakdown:
    /// 1. **Triple Insertion**: We insert three bits at the `control`, `target_1`,
    ///    和 `target_2` positions.
    /// 2. **Fixing Control**: We force the `control` bit to 1.
    /// 3. **Defining the Pair**:
    ///    - `low`: `control=1`, `target_1=1`, `target_2=0`
    ///    - `high`: `control=1`, `target_1=0`, `target_2=1`
    /// 4. **Conditional Swap**: Swapping these two indices effectively performs
    ///    the swap only when the control condition is met.
    ///
    /// # Arguments
    ///
    /// * `control` - The index of the control qubit.
    /// * `target_1`, `target_2` - The indices of the two qubits to swap.
    #[allow(dead_code)]
    pub fn CSSWAP(
        &mut self,
        control: usize,
        target_1: usize,
        target_2: usize,
    ) -> Result<&mut Self, String> {
        if control >= self.qubits
            || target_1 >= self.qubits
            || target_2 >= self.qubits
            || target_1 == target_2
            || control == target_1
            || control == target_2
        {
            return Err("...".into());
        } else {
            let mut q = [control, target_1, target_2];
            q.sort();
            let bit = self.size >> 3_usize;
            for x in 0..bit {
                //insert 3 0s
                let mut qq: usize =
                    ((x >> q[0]) << (q[0] + 1_usize)) ^ (x & ((1_usize << q[0]) - 1_usize));
                qq = ((qq >> q[1]) << (q[1] + 1_usize)) ^ (qq & ((1_usize << q[1]) - 1_usize));
                qq = ((qq >> q[2]) << (q[2] + 1_usize)) ^ (qq & ((1_usize << q[2]) - 1_usize));
                //insert control
                qq |= 1_usize << control;

                let low = qq | (1_usize << target_1);
                let high = qq | (1_usize << target_2);

                self.state.swap(low, high);
            }
        }
        Ok(self)
    }

    /// Applies a Multi-Controlled Unitary (MCU) gate to the target qubit.
    ///
    /// This is the most generalized gate in the simulator. It applies a $2 \times 2$
    /// unitary matrix $U$ to the target qubit if and only if **all** control qubits
    /// are in the $|1\rangle$ state.
    ///
    /// # Mathematical Transformation
    ///
    /// For the subspace where all control bits = 1:
    /// $$ \begin{pmatrix} \psi_{low}' \\ \psi_{high}' \end{pmatrix} =
    /// \begin{pmatrix} U_{00} & U_{01} \\ U_{10} & U_{11} \end{pmatrix}
    /// \begin{pmatrix} \psi_{low} \\ \psi_{high} \end{pmatrix} $$
    ///
    /// # Optimization: N-Level Bit Injection
    ///
    /// This function generalizes the bit-insertion technique to $k$ control qubits.
    /// It achieves an optimal complexity of $O(2^{n-(k+1)})$, where $n$ is the total
    /// number of qubits and $k$ is the number of control qubits.
    ///
    /// ### Logic Breakdown:
    /// 1. **Index Compression**: We iterate through a reduced state space of size $2^{n-(k+1)}$.
    /// 2. **Sequential Injection**: We collect all involved qubits (controls + target) into
    ///    a `buffer` and sort them. We then inject a '1' bit into each position sequentially
    ///    to build the `high` index.
    /// 3. **High/Low Pairing**:
    ///    - `high`: The index where all controls = 1 and target = 1.
    ///    - `low`: Derived by flipping the `target` bit of `high` (where all controls = 1 and target = 0).
    /// 4. **Unitary Application**: Performs the complex matrix multiplication on the identified pair.
    ///
    /// # Arguments
    ///
    /// * `control` - A vector of indices for the control qubits.
    /// * `target` - The index of the target qubit.
    /// * `U00`, `U01`, `U10`, `U11` - Components of the $2 \times 2$ unitary matrix.
    #[allow(dead_code)]
    pub fn MCU(
        &mut self,
        control: Vec<usize>,
        target: usize,
        U00: Complex,
        U01: Complex,
        U10: Complex,
        U11: Complex,
    ) -> Result<&mut Self, String> {
        if target >= self.qubits || control.iter().any(|&c| c >= self.qubits || c == target) {
            return Err("...".into());
        } else {
            let mut buffer = control.clone();
            buffer.push(target);
            buffer.sort();
            let bit = self.size >> (buffer.len());
            for x in 0..bit {
                let mut high = x;
                for val in buffer.iter() {
                    high = (((high >> *val) << (*val + 1_usize))
                        ^ (high & ((1_usize << *val) - 1_usize)))
                        | (1_usize << *val);
                }

                let low = high ^ (1_usize << target);

                let a = self.state[low];
                let b = self.state[high];

                self.state[low] = (U00 * a) + (U01 * b);
                self.state[high] = (U10 * a) + (U11 * b);
            }
        }
        Ok(self)
    }
}

/**
 * Display Structure
 */
#[derive(Debug, Clone, Copy)]
pub struct DiracKet {
    pub value: usize,
    pub width: usize,
}

use std::fmt;
impl fmt::Display for DiracKet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // {:0>width$b} 的意思是：
        // 0: 补零
        // >: 右对齐
        // width$: 动态宽度
        // b: 二进制格式
        write!(f, "|{:0>width$b}>", self.value, width = self.width)
    }
}
