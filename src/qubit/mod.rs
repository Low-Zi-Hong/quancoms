use core::panic;

use crate::complex::{self, Complex};
use std::{f64::consts::FRAC_1_SQRT_2, vec};

/*Define the Quantum register with include all the states. */
#[repr(C)]
#[derive(PartialEq, Clone, Debug, Default)]
pub struct QuantumRegister {
    pub qubits: usize,
    pub state: Vec<Complex>,
    size: usize,
}

#[allow(non_snake_case)]
impl QuantumRegister {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let _size = 1 << n;
        let mut v = vec![Complex::new(0.0, 0.0); _size];
        //let |0...0> equals 1
        v[0] = Complex::new(1.0, 0.0);
        Self {
            qubits: n,
            state: v,
            size: _size,
        }
    }

    /*This will interact with the qubit and observe it. */
    #[allow(dead_code)]
    pub fn observe(&mut self) -> usize {
        let dart: f64 = rand::random();

        let mut current_pos = 0.0;
        let mut hit_index = 0;

        for x in 0..self.state.len() {
            let real = self.state[x].re;
            let imag = self.state[x].im;
            let prob = real * real + imag * imag;

            current_pos += prob;

            if dart <= current_pos {
                hit_index = x;
                break;
            }
        }

        for i in 0..self.size {
            self.state[i].re = 0.0;
            self.state[i].im = 0.0;
        }

        self.state[hit_index].re = 1.0;
        hit_index
    }

    /*This will observe the state of qubit but not collapsing it. */
    #[allow(dead_code)]
    pub fn god_observe(&mut self) -> Vec<f64> {
        let mut prob = vec![0.0; self.size];
        for (x, val) in prob.iter_mut().enumerate() {
            let real = self.state[x].re;
            let imag = self.state[x].im;

            *val = real * real + imag * imag;
        }
        prob
    }

    /*This block is slightly slower */
    #[allow(dead_code)]
    pub fn impl_x(&mut self, target: usize) {
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

    /*Swaping the target qubit. Swap both state at index "target" bit. */
    #[allow(dead_code)]
    pub fn X(&mut self, target: usize) -> Result<&mut Self, String> {
        if target >= self.qubits {
            return Err("...".into());
        } else {
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

    /*Making the possibility of state at bit "target" to the same. */
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

    /*If the "control" bit of the state is 1, then swap at the "target" bit.  */
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
}
