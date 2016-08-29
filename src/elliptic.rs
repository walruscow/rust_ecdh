use std::cmp;
use std::fmt;
use std::ops;

use crypto_int::U512;

use field::{ModularNumber, GF};

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: ModularNumber,
    pub y: ModularNumber,
    curve: Curve,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(mut self, other: Point) -> Point {
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

        let m: ModularNumber;
        if self == other {
            // Both points are the same
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
impl ops::Mul<U512> for Point {
    type Output = Point;

    fn mul(mut self, rhs: U512) -> Point {
        if rhs.is_zero() {
            self.curve.pt(U512::zero(), U512::zero())
        } else if rhs.is_even() {
            self = self * (rhs >> 1);
            self + self
        } else {
            // TODO: This can be faster b/c just need to zero the lowest bit
            self * (rhs - U512::from_u64(1)) + self
        }
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, rhs: &Point) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }

    fn ne(&self, rhs: &Point) -> bool {
        !self.eq(rhs)
    }
}

impl cmp::Eq for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Curve {
    a: U512,
    b: U512,
    gf: GF,
}

// TODO: Use some trait like "IntoModular" or whatever for overloading
impl Curve {
    pub fn new(a: U512, b: U512, gf: GF) -> Curve {
        Curve {
            a: a,
            b: b,
            gf: gf,
        }
    }

    pub fn pt(&self, x: U512, y: U512) -> Point {
        Point {
            x: self.gf.el(x),
            y: self.gf.el(y),
            curve: *self,
        }
    }

    pub fn num(&self, x: U512) -> ModularNumber {
        self.gf.el(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_int::U512;
    use field::GF;

    #[test]
    fn basic_addition() {
        let f = GF::new(U512::from_u64(11));
        let e = Curve::new(
            U512::from_u64(1),
            U512::from_u64(6),
            f,
        );
        let p1 = e.pt(U512::from_u64(2), U512::from_u64(4));
        let p2 = e.pt(U512::from_u64(5), U512::from_u64(2));

        let e1 = e.pt(U512::from_u64(2), U512::from_u64(7));
        assert_eq!(p1 + p2, e1);

        let e2 = e.pt(U512::from_u64(5), U512::from_u64(9));
        assert_eq!(p1 + p1, e2);
    }
}
