use std::cmp;
use std::fmt;
use std::io::{Read, Write};
use std::ops;

use crypto_int::U512;

use field::{ModularNumber, GF};
use encoding::*;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: ModularNumber,
    pub y: ModularNumber,
    curve: Curve,
}

impl Point {
    pub fn encode_to(&self, writer: &mut Write) -> EncodingResult {
        let bytes = self.x.value.to_bytes_le();
        match writer.write_all(&bytes) {
            Ok(_) => (),
            Err(e) => return Err(EncodingError::IoError(e)),
        }
        let bytes = self.y.value.to_bytes_le();
        match writer.write_all(&bytes) {
            Ok(_) => Ok(()),
            Err(e) => Err(EncodingError::IoError(e)),
        }
    }

    pub fn decode_from(
        reader: &mut Read,
        curve: &Curve,
    ) -> DecodingResult<Point> {
        let mut x = (&[0u8; 64]).to_vec();
        match reader.read_exact(&mut x) {
            Ok(_) => (),
            Err(e) => return Err(DecodingError::IoError(e)),
        }
        let x = U512::from_bytes_le(x);

        let mut y = (&[0u8; 64]).to_vec();
        match reader.read_exact(&mut y) {
            Ok(_) => (),
            Err(e) => return Err(DecodingError::IoError(e)),
        }
        let y = U512::from_bytes_le(y);
        Ok(curve.pt(x, y))
    }
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
    use std::io::{Read, Write, Result};

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

    // Some basic read/write stuff because idk how to use a vec...
    struct Writer { pub buf: Vec<u8> }
    impl Write for Writer {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            let mut x = 0;
            for b in buf.iter() {
                self.buf.push(*b);
                x += 1;
            }
            Ok(x)
        }
        fn flush(&mut self) -> Result<()> { Ok(()) }
    }

    struct Reader { pub buf: Vec<u8>, pub rc: usize }
    impl Read for Reader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut x = 0;
            for (a, b) in buf.iter_mut().zip(self.buf.iter().skip(self.rc)) {
                *a = *b;
                x += 1;
                self.rc += 1;
            }
            Ok(x)
        }
    }

    #[test]
    fn encode_decode() {
        let f = GF::new(U512::from_u64(11));
        let e = Curve::new(
            U512::from_u64(1),
            U512::from_u64(6),
            f,
        );
        let point = e.pt(U512::from_u64(2), U512::from_u64(4));
        let buf = {
            let mut writer = Writer { buf: Vec::new(), };
            point.encode_to(&mut writer).unwrap();
            writer.buf
        };
        let mut reader = Reader { buf: buf, rc: 0};
        let decoded_pt = Point::decode_from(&mut reader, &e).unwrap();
        assert_eq!(decoded_pt, point);
    }
}
