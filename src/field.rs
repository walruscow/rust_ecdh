extern crate num;
extern crate num_traits;

use std::cmp;
use std::fmt;
use std::ops;

use num::BigUint;
use self::num_traits::identities::Zero;

/// We need a Modular Number struct, to do arithmetic mod P
///
/// This should hopefully be created by some function or other
/// GF(BigUint) -> Fn<BigUint, ModularNumber>

// TODO: What we actually want is a ModularNumber thing that has
// an arbitrary container, maybe of a specified bit size or whatever
// and that container should support a bunch of magical things
// so that we can say ModularNumber * 3 or ModularNumber + 1
// or whatever, hopefully
//
// TODO: Create container like BigNum256 which uses u64 or u32 or whatever
// to do addition and stuff. It doesn't have to be *that* fast but pretty fast
// would be nice...
// The type could also be semi-resistant to timing attacks, by having few
// branches based on the numbers (always doing overflow addition or w.e.)
//
// TODO: How to handle multiplication with the new containers? Because if
// we use u32 for the base layer we could convert each to u64 to multiply
// (since the result could be 64 bits), but that would be slower than
// having u64 from the beginning
#[derive(Clone)]
pub struct ModularNumber {
    modulus: BigUint,
    pub value: BigUint,
}

impl ModularNumber {
    pub fn new(value: BigUint, modulus: BigUint) -> ModularNumber {
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
        write!(f, "{:#0x} mod {:#0x}", &self.value, &self.modulus)
    }
}

// TODO: Need Inverse
impl ops::Add for ModularNumber {
    type Output = ModularNumber;

    fn add(self, other: ModularNumber) -> ModularNumber {
        ModularNumber {
            modulus: self.modulus.clone(),
            value: (self.value + other.value) % self.modulus,
        }
    }
}

impl<'a> ops::Add<&'a ModularNumber> for &'a ModularNumber {
    type Output = ModularNumber;

    fn add(self, other: &'a ModularNumber) -> ModularNumber {
        ModularNumber {
            modulus: self.modulus.clone(),
            value: (&self.value + &other.value) % &self.modulus,
        }
    }
}

impl ops::Sub for ModularNumber {
    type Output = ModularNumber;

    fn sub(self, other: ModularNumber) -> ModularNumber {
        let m = &self.modulus;
        ModularNumber {
            modulus: m.clone(),
            // Handle overflow
            value: (&self.value + m - &other.value) % m,
        }
    }
}

impl<'a> ops::Sub for &'a ModularNumber {
    type Output = ModularNumber;

    fn sub(self, other: &'a ModularNumber) -> ModularNumber {
        let m = &self.modulus;
        ModularNumber {
            modulus: m.clone(),
            // Handle overflow
            value: (&self.value + m - &other.value) % m,
        }
    }
}


impl ops::Div for ModularNumber {
    type Output = ModularNumber;

    fn div(self, other: ModularNumber) -> ModularNumber {
        // TODO
        self
    }
}

impl<'a> ops::Div for &'a ModularNumber {
    type Output = ModularNumber;

    fn div(self, other: &'a ModularNumber) -> ModularNumber {
        // TODO
        self.clone()
    }
}

impl ops::Mul for ModularNumber {
    type Output = ModularNumber;

    fn mul(self, other: ModularNumber) -> ModularNumber {
        // TODO; We would like a good implementation...
        ModularNumber {
            modulus: self.modulus.clone(),
            value: (&other.value * &self.value) % &self.modulus
        }
    }
}

impl<'a> ops::Mul for &'a ModularNumber {
    type Output = ModularNumber;

    fn mul(self, other: &ModularNumber) -> ModularNumber {
        // TODO; We would like a good implementation...
        ModularNumber {
            modulus: self.modulus.clone(),
            value: (&other.value * &self.value) % &self.modulus
        }
    }
}


impl ops::Shl<usize> for ModularNumber {
    type Output = ModularNumber;

    fn shl(self, other: usize) -> ModularNumber {
        ModularNumber {
            modulus: self.modulus.clone(),
            value: (self.value << other) % self.modulus,
        }
    }
}

impl<'a> ops::Shl<usize> for &'a ModularNumber {
    type Output = ModularNumber;

    fn shl(self, other: usize) -> ModularNumber {
        ModularNumber {
            modulus: self.modulus.clone(),
            value: (&self.value << other) % &self.modulus,
        }
    }
}


impl ops::Neg for ModularNumber {
    type Output = ModularNumber;
    fn neg(self) -> ModularNumber {
        ModularNumber {
            modulus: self.modulus.clone(),
            value: (&self.modulus - &self.value) % &self.modulus,
        }
    }
}

impl<'a> ops::Neg for &'a ModularNumber {
    type Output = ModularNumber;
    fn neg(self) -> ModularNumber {
        ModularNumber {
            modulus: self.modulus.clone(),
            value: (&self.modulus - &self.value) % &self.modulus,
        }
    }
}

impl cmp::PartialEq for ModularNumber {
    fn eq(&self, other: &ModularNumber) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &ModularNumber) -> bool {
        self.value != other.value
    }
}

pub struct GF {
    pub size: BigUint,
}

impl GF {
    pub fn new(size: &BigUint) -> GF {
        GF {
            size: size.clone(),
        }
    }

    pub fn el(&self, x: BigUint) -> ModularNumber {
        ModularNumber::new(x, self.size.clone())
    }
}
