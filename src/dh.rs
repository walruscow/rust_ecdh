use std::io::{Read, Write};

use crypto_int::U512;
use rand;
use sha::Sha256;

use elliptic;
use super::ECCParams;
use encoding::*;

#[derive(Debug)]
struct DHSecret(U512);

#[derive(Debug)]
pub struct DHPublic(elliptic::Point);

impl DHPublic {
    pub fn encode_to(&self, writer: &mut Write) -> EncodingResult {
        self.0.encode_to(writer)
    }

    pub fn decode_from(
        reader: &mut Read,
        ecc: &ECCParams,
    ) -> DecodingResult<DHPublic> {
        match elliptic::Point::decode_from(reader, &ecc.curve) {
            Ok(p) => Ok(DHPublic(p)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug)]
pub struct DHPair {
    secret: DHSecret,
    pub public: DHPublic,
}

impl DHPair {
    pub fn get_shared_key(&self, their_public: &DHPublic) -> [u8; 32] {
        let publ = &(their_public.0);
        let key_bytes = (*publ * self.secret.0).x.value.to_bytes_le();

        let mut sha = Sha256::new();
        for b in &key_bytes[0..32] {
            sha.add_byte(*b);
        }
        sha.digest()
    }
}

pub trait GenDH {
    fn gen_dh_pair(&mut self, ecc_params: &ECCParams) -> DHPair;
}

// We only want this on OsRng because it is secure
impl GenDH for rand::os::OsRng {
    fn gen_dh_pair(&mut self, ecc_params: &ECCParams) -> DHPair {
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
