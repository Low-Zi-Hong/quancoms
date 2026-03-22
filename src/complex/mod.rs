/*Define structure for complex number. */
#[repr(C)]
#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Complex{
    pub re:f64,
    pub im:f64,
}

/*For printing complex number in a better way. */
use std::fmt;
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im >= 0.0{
            write!(f,"{:.4} + {:.4}i",self.re,self.im)
        } else {
            write!(f,"{:.4} - {:.4}i", self.re,-self.im)
        }
    }
}

/*Define operators for Complex numbers */
use std::ops::{Add, Div, Mul, Neg, Sub};

impl Add for Complex{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {re: self.re + other.re,im:self.im + other.im}
    }
}

impl Sub for Complex{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {re: self.re - other.re,im:self.im - other.im}
    }
}

impl Neg for Complex{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {re: - self.re, im: - self.im}
    }
}

impl Mul for Complex{
    type Output = Self;
    #[inline]
    fn mul(self,other:Self)-> Self::Output {
        Self {re: (self.re * other.re) - (self.im * other.im), im: (self.re*other.im) + (self.im * other.re)}
    }
}

impl Mul<f64> for Complex{
    type Output = Self;
    fn mul(self,other:f64)->Self::Output {
        Self {re: self.re * other, im: self.im * other}
    }
}

impl Div for Complex{
    type Output = Self;
    #[inline]
    fn div(self,other:Self)-> Self::Output{
        let denominator = other.re * other.re + other.im * other.im;
        Self{re:((self.re * other.re) + (self.im * other.im))/denominator,im:((self.im * other.re) - (self.re * other.im))/denominator}
    }
}

impl Div<f64> for Complex{
    type Output = Self;
    fn div(self,other:f64)->Self::Output {
        Self {re: self.re / other, im: self.im / other}
    }
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self{re,im}
    }
}
