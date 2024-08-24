use std::cmp;
use std::fmt;
use std::mem;
use std::ops;

use crypto_int::U512;

/// We need a Modular Number struct, to do arithmetic mod P
///
/// This should hopefully be created by some function or other
/// GF(BigUint) -> Fn<BigUint, ModularNumber>

#[derive(Clone, Copy, Debug)]
pub struct ModularNumber {
    modulus: U512,
    pub value: U512,
}

// TODO: Impl mul for u64, other ops for U512
impl ModularNumber {
    pub fn new(value: U512, modulus: U512) -> ModularNumber {
        ModularNumber {
            modulus,
            value: value % modulus,
        }
    }

    pub fn n(&self, x: U512) -> ModularNumber {
        ModularNumber::new(x, self.modulus)
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }

    pub fn invert(mut self) -> ModularNumber {
        let mut r: U512 = self.modulus;
        let mut new_r: U512 = self.value;

        self.value = U512::zero();
        let mut new_t = self.n(U512::from_u64(1));

        while !new_r.is_zero() {
            let quotient = r / new_r;
            self -= self.n(quotient) * new_t;
            mem::swap(&mut self, &mut new_t);

            r -= quotient * new_r;
            mem::swap(&mut r, &mut new_r);
        }
        if r > U512::from_u64(1) {
            panic!()
        }
        self
    }
}

impl fmt::Display for ModularNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl ops::Add for ModularNumber {
    type Output = ModularNumber;
    fn add(mut self, rhs: ModularNumber) -> ModularNumber {
        self.value = (self.value + rhs.value) % self.modulus;
        self
    }
}

impl ops::AddAssign for ModularNumber {
    fn add_assign(&mut self, rhs: ModularNumber) {
        self.value = (self.value + rhs.value) % self.modulus;
    }
}

impl ops::Sub for ModularNumber {
    type Output = ModularNumber;
    fn sub(mut self, rhs: ModularNumber) -> ModularNumber {
        self.value = (self.value + self.modulus - rhs.value) % self.modulus;
        self
    }
}

impl ops::SubAssign for ModularNumber {
    fn sub_assign(&mut self, rhs: ModularNumber) {
        self.value = (self.value + self.modulus - rhs.value) % self.modulus;
    }
}

impl ops::Div for ModularNumber {
    type Output = ModularNumber;
    fn div(mut self, rhs: ModularNumber) -> ModularNumber {
        self *= rhs.invert();
        self
    }
}

impl ops::DivAssign for ModularNumber {
    fn div_assign(&mut self, rhs: ModularNumber) {
        *self *= rhs.invert();
    }
}

impl ops::Mul for ModularNumber {
    type Output = ModularNumber;
    fn mul(mut self, rhs: ModularNumber) -> ModularNumber {
        self.value = (self.value * rhs.value) % self.modulus;
        self
    }
}

impl ops::MulAssign for ModularNumber {
    fn mul_assign(&mut self, rhs: ModularNumber) {
        self.value = (self.value * rhs.value) % self.modulus;
    }
}

impl ops::Shl<usize> for ModularNumber {
    type Output = ModularNumber;
    fn shl(mut self, rhs: usize) -> ModularNumber {
        self.value = (self.value << rhs) % self.modulus;
        self
    }
}

impl ops::ShlAssign<usize> for ModularNumber {
    fn shl_assign(&mut self, rhs: usize) {
        self.value = (self.value << rhs) % self.modulus;
    }
}

impl ops::Neg for ModularNumber {
    type Output = ModularNumber;
    fn neg(mut self) -> ModularNumber {
        self.value = (self.modulus - self.value) % self.modulus;
        self
    }
}

impl cmp::PartialEq for ModularNumber {
    fn eq(&self, rhs: &ModularNumber) -> bool {
        self.value == rhs.value
    }

    fn ne(&self, rhs: &ModularNumber) -> bool {
        self.value != rhs.value
    }
}

impl cmp::Eq for ModularNumber {}

#[derive(Copy, Clone, Debug)]
pub struct GF {
    pub size: U512,
}

impl GF {
    pub const fn new(size: U512) -> GF {
        GF { size }
    }

    pub fn el(&self, x: U512) -> ModularNumber {
        ModularNumber::new(x, self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_int::U512;

    #[test]
    fn basic_arithmetic() {
        let f = GF::new(U512::from_u64(13));
        let x = f.el(U512::from_u64(7));
        assert_eq!(x + x, f.el(U512::from_u64(1)));

        let y = f.el(U512::from_u64(6));
        assert_eq!(x + y, f.el(U512::from_u64(0)));

        let z = f.el(U512::from_u64(5));
        assert_eq!(x + z, f.el(U512::from_u64(12)));
    }
}
