extern crate crypto_int;
extern crate rand;
extern crate sha;

mod dh;
mod elliptic;
pub mod encoding;
mod field;

use crypto_int::U512;

pub use dh::{DHPair, DHPublic, GenDH};

#[derive(Debug, Clone, Copy)]
struct Generator {
    point: elliptic::Point,
    order: U512,
}

/// Basic parameters for elliptic curve cryptography.
/// Some of these are provided as constants, like P-256
/// To perform ECC operations, ECCParams can be converted to ECC,
/// which has the actual curve and generator structs.
#[derive(Debug, Clone, Copy)]
pub struct ECCParams {
    pub p: U512,
    pub a: U512,
    pub b: U512,
    pub g: (U512, U512),
    pub n: U512,
}

/// A bunch of parameters wrapped in fairly usable structs for doing ECC operations
#[derive(Debug, Clone, Copy)]
pub struct ECC {
    curve: elliptic::Curve,
    generator: Generator,
}

impl From<ECCParams> for ECC {
    fn from(params: ECCParams) -> Self {
        let curve = elliptic::Curve::new(params.a, params.b, field::GF::new(params.p));
        ECC {
            curve,
            generator: Generator {
                point: curve.pt(params.g.0, params.g.1),
                order: params.n,
            },
        }
    }
}

pub const P_256: ECCParams = ECCParams {
    p: U512::from_hex_be(b"ffffffff00000001000000000000000000000000ffffffffffffffffffffffff"),
    a: U512::from_hex_be(b"ffffffff00000001000000000000000000000000fffffffffffffffffffffffc"),
    b: U512::from_hex_be(b"5ac635d8aa3a93e7b3ebbd55769886bc651d06b0cc53b0f63bce3c3e27d2604b"),
    g: (
        U512::from_hex_be(b"6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296"),
        U512::from_hex_be(b"4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5"),
    ),
    n: U512::from_hex_be(b"ffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551"),
};

pub const BRAINPOOL_P256_R1: ECCParams = ECCParams {
    a: U512::from_hex_be(b"7d5a0975fc2c3057eef67530417affe7fb8055c126dc5c6ce94a4b44f330b5d9"),
    b: U512::from_hex_be(b"26dc5c6ce94a4b44f330b5d9bbd77cbf958416295cf7e1ce6bccdc18ff8c07b6"),
    p: U512::from_hex_be(b"a9fb57dba1eea9bc3e660a909d838d726e3bf623d52620282013481d1f6e5377"),
    g: (
        U512::from_hex_be(b"8bd2aeb9cb7e57cb2c4b482ffc81b7afb9de27e1e3bd23c23a4453bd9ace3262"),
        U512::from_hex_be(b"547ef835c3dac4fd97f8461a14611dc9c27745132ded8e545c1d54c72f046997"),
    ),
    n: U512::from_hex_be(b"a9fb57dba1eea9bc3e660a909d838d718c397aa3b561a6f7901e0e82974856a7"),
};

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_int::U512;

    #[test]
    fn it_works() {
        let ecc: ECC = BRAINPOOL_P256_R1.into();
        let expected = ecc.curve.pt(U512::zero(), U512::zero());
        assert_eq!(ecc.generator.point * ecc.generator.order, expected);
    }
}
