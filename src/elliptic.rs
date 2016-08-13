extern crate num;
extern crate num_traits;

use std::ops::Add;

use num::BigUint;
use self::num_traits::identities::Zero;

use field::{ModularNumber, GF};

#[derive(Clone)]
pub struct Point<'a> {
    x: ModularNumber,
    y: ModularNumber,
    // HOW TO DOOOO this?????????????
    curve: &'a EllipticCurve,
}

impl<'a> Add<Point<'a>> for Point<'a> {
    type Output = Point<'a>;

    // TODO: HOLY SHIT is there some gnarly borrowing happening here...
    // We REALLY need to do something about that...
    fn add(self, other: Point<'a>) -> Point<'a> {
        // TODO: Check that the curves are eq
        //assert_eq!(self.modulus, other.modulus);
        if other.x.is_zero() && other.y.is_zero() {
            // Other is zero
            return self.clone();
        } else if self.x.is_zero() && self.y.is_zero() {
            // We are zero
            return other.clone();
        } else if self.x == other.x && self.y == -(&other.y) {
            // Inverses
            return self.curve.pt(BigUint::zero(), BigUint::zero());
        }

        // Both points are the same
        let m: ModularNumber;
        if self.x == other.x && self.y == other.y {
            let a = ModularNumber::new(self.curve.a.clone(), self.curve.gf.size.clone());
            let x = &self.x * &self.x;
            // Do a really weird * 3, because BigUint doesn't support
            // multiplying by a normal int...
            m = (((&x << 1) + x) + a) / (&self.y << 1);
        } else {
            m = (&other.y - &self.y) / (&other.x - &self.x);
        }
        let sum_x = (&m * &m) - (&self.x - &other.x);
        let sum_y = -(m * (&sum_x - &self.x) + self.y);
        self.curve.pt(sum_x.value, sum_y.value)
    }
}

pub struct EllipticCurve {
    a: BigUint,
    b: BigUint,
    gf: GF,
}

// TODO: Use some trait like "IntoModular" or whatever for overloading
impl EllipticCurve {
    pub fn pt<'a>(&'a self, x: BigUint, y: BigUint) -> Point {
        Point {
            x: self.gf.el(x),
            y: self.gf.el(y),
            curve: &self,
        }
    }
}
