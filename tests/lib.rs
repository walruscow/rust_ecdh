extern crate ecdh;
extern crate rand;

use ecdh::GenDH;

#[test]
fn dh_interface() {
    let ecc: ecdh::ECC = ecdh::BRAINPOOL_P256_R1.into();
    let mut rng = rand::OsRng::new().unwrap();

    let pair1 = rng.gen_dh_pair(&ecc);
    let pair2 = rng.gen_dh_pair(&ecc);

    let key1 = pair1.get_shared_key(&pair2.public);
    let key2 = pair2.get_shared_key(&pair1.public);

    for (x, y) in key1.iter().zip(key2.iter()) {
        assert_eq!(*x, *y);
    }
}
