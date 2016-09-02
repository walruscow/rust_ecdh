use std::io::{Read, Write};
use std::io::Error as IoError;

use crypto_int::U512;
use rand;
use sha::Sha256;

use super::{elliptic, ECCParams};

pub enum SerializationError {
    IoError(IoError),
}

#[derive(Debug)]
struct DHSecret(U512);

#[derive(Debug)]
pub struct DHPublic(elliptic::Point);

impl DHPublic {
    pub fn serialize_to<W: Write>(w: &mut W) -> Result<(), SerializationError> {
        Ok(())
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
