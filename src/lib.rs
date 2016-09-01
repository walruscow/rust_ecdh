extern crate crypto_int;
extern crate rand;
extern crate sha;

mod field;
pub mod elliptic;

use crypto_int::U512;
use sha::Sha256;

pub struct Generator {
    pub point: elliptic::Point,
    pub order: U512,
}

pub struct ECC {
    pub curve: elliptic::Curve,
    pub generator: Generator,
}

pub fn brainpool_p256_r1() -> ECC {
    let brainpool_p256_r1_curve: elliptic::Curve = elliptic::Curve::new(
        U512::from_bytes_be(vec![
            0x7D, 0x5A, 0x09, 0x75, 0xFC, 0x2C, 0x30, 0x57,
            0xEE, 0xF6, 0x75, 0x30, 0x41, 0x7A, 0xFF, 0xE7,
            0xFB, 0x80, 0x55, 0xC1, 0x26, 0xDC, 0x5C, 0x6C,
            0xE9, 0x4A, 0x4B, 0x44, 0xF3, 0x30, 0xB5, 0xD9,
        ]),
        U512::from_bytes_be(vec![
            0x26, 0xDC, 0x5C, 0x6C, 0xE9, 0x4A, 0x4B, 0x44,
            0xF3, 0x30, 0xB5, 0xD9, 0xBB, 0xD7, 0x7C, 0xBF,
            0x95, 0x84, 0x16, 0x29, 0x5C, 0xF7, 0xE1, 0xCE,
            0x6B, 0xCC, 0xDC, 0x18, 0xFF, 0x8C, 0x07, 0xB6,
        ]),
        field::GF::new(U512::from_bytes_be(vec![
            0xA9, 0xFB, 0x57, 0xDB, 0xA1, 0xEE, 0xA9, 0xBC,
            0x3E, 0x66, 0x0A, 0x90, 0x9D, 0x83, 0x8D, 0x72,
            0x6E, 0x3B, 0xF6, 0x23, 0xD5, 0x26, 0x20, 0x28,
            0x20, 0x13, 0x48, 0x1D, 0x1F, 0x6E, 0x53, 0x77,
        ])),
    );
    ECC {
        curve: brainpool_p256_r1_curve,
        generator: Generator {
            point: brainpool_p256_r1_curve.pt(
                U512::from_bytes_be(vec![
                    0x8B, 0xD2, 0xAE, 0xB9, 0xCB, 0x7E, 0x57, 0xCB,
                    0x2C, 0x4B, 0x48, 0x2F, 0xFC, 0x81, 0xB7, 0xAF,
                    0xB9, 0xDE, 0x27, 0xE1, 0xE3, 0xBD, 0x23, 0xC2,
                    0x3A, 0x44, 0x53, 0xBD, 0x9A, 0xCE, 0x32, 0x62,
                ]),
                U512::from_bytes_be(vec![
                    0x54, 0x7E, 0xF8, 0x35, 0xC3, 0xDA, 0xC4, 0xFD,
                    0x97, 0xF8, 0x46, 0x1A, 0x14, 0x61, 0x1D, 0xC9,
                    0xC2, 0x77, 0x45, 0x13, 0x2D, 0xED, 0x8E, 0x54,
                    0x5C, 0x1D, 0x54, 0xC7, 0x2F, 0x04, 0x69, 0x97,
                ]),
            ),
            order: U512::from_bytes_be(vec![
                0xA9, 0xFB, 0x57, 0xDB, 0xA1, 0xEE, 0xA9, 0xBC,
                0x3E, 0x66, 0x0A, 0x90, 0x9D, 0x83, 0x8D, 0x71,
                0x8C, 0x39, 0x7A, 0xA3, 0xB5, 0x61, 0xA6, 0xF7,
                0x90, 0x1E, 0x0E, 0x82, 0x97, 0x48, 0x56, 0xA7,
            ]),
        },
    }
}

pub struct DHSecret(U512);
// TODO: Make this serializable or something....
pub struct DHPublic(elliptic::Point);

pub struct DHPair {
    pub secret: DHSecret,
    pub public: DHPublic,
}

pub fn get_shared_key(
    my_secret: &DHSecret,
    their_public: &DHPublic,
) -> [u8; 32] {
    let sec = &(my_secret.0);
    let publ = &(their_public.0);
    let key_bytes = (*publ * *sec).x.value.to_bytes_le();

    // Hash the coordinate just in case :)
    let mut sha = Sha256::new();
    for b in &key_bytes[0..32] {
        sha.add_byte(*b);
    }
    sha.digest()
}

trait GenDH {
    fn gen_dh_pair(&mut self, ecc_params: &ECC) -> DHPair;
}

// We only want this on OsRng because it is secure
impl GenDH for rand::os::OsRng {
    fn gen_dh_pair(&mut self, ecc_params: &ECC) -> DHPair {
        let secret = U512::random_in_range(
            U512::from_u64(1),
            ecc_params.generator.order,
            self,
        );
        let public = ecc_params.generator.point * secret;
        DHPair {
            secret: DHSecret(secret),
            public: DHPublic(public),
        }
    }
}

#[cfg(test)]
mod tests {
    use crypto_int::U512;
    use rand;
    use GenDH;
    use super::*;

    #[test]
    fn it_works() {
        let ecc = brainpool_p256_r1();
        let expected = ecc.curve.pt(U512::zero(), U512::zero());
        assert_eq!(ecc.generator.point * ecc.generator.order, expected);
    }

    #[test]
    fn diffie() {
        let ecc = brainpool_p256_r1();

        let mut rng = rand::OsRng::new().unwrap();

        let pair1 = rng.gen_dh_pair(&ecc);
        let pair2 = rng.gen_dh_pair(&ecc);

        let key1 = get_shared_key(&pair1.secret, &pair2.public);
        let key2 = get_shared_key(&pair2.secret, &pair1.public);

        for (x, y) in key1.iter().zip(key2.iter()) {
            assert_eq!(*x, *y);
        }
    }
}
