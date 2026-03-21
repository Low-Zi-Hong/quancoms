use core::panic;

use crate::complex::Complex;
use log::warn;
use std::{f64::consts::FRAC_1_SQRT_2, vec};

/*Define the Quantum register with include all the states. */
#[repr(C)]
#[derive(PartialEq, Clone, Debug, Default)]
pub struct QuantumRegister{
    pub qubits: usize,
    pub state: Vec<Complex>,
    size:usize,
}

impl QuantumRegister{
    pub fn new(n:usize) -> Self{
        let _size = 1 << n;
        let mut v = vec![Complex::new(0.0,0.0);_size];
        //let |0...0> equals 1
        v[0] = Complex::new(1.0,0.0);
        Self { qubits: n, state: v,size:_size }
    }

    #[allow(dead_code)]
    pub fn observe(&mut self) -> i32 {
        let mut prob = vec![0.0;self.size];
        for x in 0..prob.len(){
            let real = self.state[x].re ;
            let imag = self.state[x].im ;

           prob[x] = real * real + imag * imag;
        }

        
        100
    }

    pub fn god_observe(&mut self) -> Vec<f64> {
        let mut prob = vec![0.0;self.size];
        for x in 0..prob.len(){
            let real = self.state[x].re ;
            let imag = self.state[x].im ;

           prob[x] = real * real + imag * imag;
        }
        prob
    }

    /*This block is slightly slower */
    #[allow(dead_code)]
    pub fn impl_X(&mut self,target:usize){
        if target >= self.qubits {
            panic!("target out of range!");
        }
        else{
            for x in 0..(self.size){
                /*Find the bit where at the target location is 1 just by bit shift to the right and "and" 1 if 1 then return 1 else 0 */
                if(x>>target)&1 == 1{
                    /*The opposite is just XOR the bit of the original index at the target position. */
                    let opposite = x ^ (1<<target);
                    self.state.swap(x,opposite);
                }
            }
        }
    }

    /*Swaping the target qubit. Swap both state at index "target" bit. */
    pub fn X(&mut self, target:usize) -> Result<&mut Self, String>{
        if target >= self.qubits {
            return Err("...".into());
        }
        else{
            let bit = self.size>>1_usize;
            for x in 0..bit{
                let low = ((x>>target) <<( target+1_usize) ) ^ (x&((1_usize << target) - 1_usize));
                let high = low | (1_usize<<target);
                self.state.swap(low, high);
            }
        }
        Ok(self)
    }

    /*Making the possibility of state at bit "target" to the same. */
    pub fn H(&mut self,target:usize) -> Result<&mut Self, String> {
        if target >= self.qubits {
            return Err("...".into());
        }
        else{
            let bit = self.size>>1_usize;
            for x in 0..bit{
                let low = ((x>>target) <<( target+1_usize) ) ^ (x&((1_usize << target) - 1_usize));
                let high = low | (1_usize<<target);
                
                let a = self.state[low];
                let b = self.state[high];

                self.state[low] = ( a +  b) * FRAC_1_SQRT_2;
                self.state[high] = (a -  b) * FRAC_1_SQRT_2;
            }
        }
        Ok(self)
    }

    /*If the "control" bit of the state is 1, then swap at the "target" bit.  */
    pub fn CNOT(&mut self,control:usize,target:usize) -> Result<&mut Self, String> {
         if target >= self.qubits  || control >= self.qubits || control == target{
            return Err("...".into());
        }
        else{
            let bit = self.size>>2_usize;
            for x in 0..bit{
                if(control < target)
                {   
                    //slice
                    let mut s1 = ((x>>control)<<(control +1_usize)) ^ (x&((1_usize<<control) -1_usize));
                    //insert control bit
                    s1 = s1 | (1_usize <<control);
                    //insert target
                    let low = ((s1>>target) <<( target+1_usize) ) ^ (s1&((1_usize << target) - 1_usize));
                    let high = low | (1_usize<<target);
                    self.state.swap(low, high); 
                }
                else{
                    let mut low = ((x>>target) <<( target+1_usize) ) ^ (x&((1_usize << target) - 1_usize));
                    low = ((low>>control)<<(control +1_usize)) ^ (low&((1_usize<<control) - 1_usize)) | (1_usize<<control);
                    let high = low | (1_usize<<target);
                    self.state.swap(low, high); 
                }
            }

        }
        Ok(self)
    }

    pub fn CCNOT(&mut self,control1:usize,control2:usize,target:usize)-> Result<&mut Self, String> {
         if target >= self.qubits  || control1 >= self.qubits|| control2 >= self.qubits || control1 == target|| control1 == control2{
            return Err("...".into());
        }
        else{
            let bit = self.size>>3_usize;
            let mut arr = [control1,control2,target];
            arr.sort();
            for x in 0..bit{
                let mut high = (((x>>arr[0]) << (arr[0] + 1_usize)) ^ (x & ((1_usize << arr[0]) - 1_usize)) )| (1_usize<<arr[0]);
                high = (((high>>arr[1]) << (arr[1] + 1_usize)) ^ (high & ((1_usize << arr[1]) - 1_usize)) )| (1_usize<<arr[1]);
                high = (((high>>arr[2]) << (arr[2] + 1_usize)) ^ (high & ((1_usize << arr[2]) - 1_usize)) )| (1_usize<<arr[2]);
                
                let low = high ^ (1_usize << target);
                self.state.swap(low, high);
            }
        }
        Ok(self)
    }

}

