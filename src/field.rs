use std::cmp;
use std::fmt;
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

impl ModularNumber {
    pub fn new(value: U512, modulus: U512) -> ModularNumber {
        ModularNumber {
            modulus: modulus.clone(),
            value: value % modulus,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl fmt::Display for ModularNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} mod {:?}", &self.value, &self.modulus)
    }
}

// TODO: Need Inverse
impl ops::Add for ModularNumber {
    type Output = ModularNumber;
    fn add(mut self, rhs: ModularNumber) -> ModularNumber {
        self.value = (self.value + rhs.value) % self.modulus;
        self
    }
}

impl ops::Sub for ModularNumber {
    type Output = ModularNumber;
    fn sub(mut self, rhs: ModularNumber) -> ModularNumber {
        self.value = (self.value + self.modulus - rhs.value) % self.modulus;
        self
    }
}

impl ops::Div for ModularNumber {
    type Output = ModularNumber;
    fn div(self, rhs: ModularNumber) -> ModularNumber {
        // TODO
        self
    }
}

impl ops::Mul for ModularNumber {
    type Output = ModularNumber;
    fn mul(mut self, rhs: ModularNumber) -> ModularNumber {
        self.value = (self.value * rhs.value) % self.modulus;
        self
    }
}

impl ops::Shl<usize> for ModularNumber {
    type Output = ModularNumber;
    fn shl(mut self, rhs: usize) -> ModularNumber {
        self.value = (self.value << rhs) % self.modulus;
        self
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

pub struct GF {
    pub size: U512,
}

impl GF {
    pub fn new(size: U512) -> GF {
        GF {
            size: size,
        }
    }

    pub fn el(&self, x: U512) -> ModularNumber {
        ModularNumber::new(x, self.size)
    }
}
