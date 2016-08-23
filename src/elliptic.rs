use std::cmp;
use std::fmt;
use std::ops;

use crypto_int::U512;

use field::{ModularNumber, GF};

#[derive(Copy, Clone, Debug)]
pub struct Point<'a> {
    x: ModularNumber,
    y: ModularNumber,
    curve: &'a EllipticCurve,
}

impl<'a> ops::Add<Point<'a>> for Point<'a> {
    type Output = Point<'a>;

    fn add(mut self, other: Point<'a>) -> Point<'a> {
        if other.x.is_zero() && other.y.is_zero() {
            // Other is zero
            return self;
        } else if self.x.is_zero() && self.y.is_zero() {
            // We are zero
            return other;
        } else if self.x == other.x && self.y == -other.y {
            // Inverses
            return self.curve.pt(U512::zero(), U512::zero());
        }

        // Both points are the same
        let m: ModularNumber;
        if self == other {
            let a = self.curve.num(self.curve.a);
            let x_sqr = self.x * self.x;
            // Do a really weird * 3, because multiplying is real slow
            m = ((x_sqr << 1) + x_sqr + a) / (self.y << 1);
        } else {
            m = (other.y - self.y) / (other.x - self.x);
        }
        let sum_x = (m * m) - self.x - other.x;
        let sum_y = -(m * (sum_x - self.x) + self.y);
        self.x = sum_x;
        self.y = sum_y;
        self
    }
}

// TODO: Why is it so slow?
impl<'a> ops::Mul<U512> for Point<'a> {
    type Output = Point<'a>;

    fn mul(mut self, rhs: U512) -> Point<'a> {
        if rhs.is_zero() {
            self.curve.pt(U512::zero(), U512::zero())
        } else if rhs.is_even() {
            self = self * (rhs >> 1);
            self + self
        } else {
            self * (rhs - U512::from_u64(1)) + self
        }
    }
}

impl<'a> cmp::PartialEq for Point<'a> {
    fn eq(&self, rhs: &Point) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }

    fn ne(&self, rhs: &Point) -> bool {
        !self.eq(rhs)
    }
}

impl<'a> cmp::Eq for Point<'a> {}

impl<'a> fmt::Display for Point<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EllipticCurve {
    a: U512,
    b: U512,
    gf: GF,
}

// TODO: Use some trait like "IntoModular" or whatever for overloading
impl EllipticCurve {
    pub fn new(a: U512, b: U512, gf: GF) -> EllipticCurve {
        EllipticCurve {
            a: a,
            b: b,
            gf: gf,
        }
    }
    pub fn pt<'a>(&'a self, x: U512, y: U512) -> Point {
        Point {
            x: self.gf.el(x),
            y: self.gf.el(y),
            curve: &self,
        }
    }

    pub fn num<'a>(&'a self, x: U512) -> ModularNumber {
        self.gf.el(x)
    }
}
