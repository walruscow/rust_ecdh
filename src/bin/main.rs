extern crate crypto_int;
extern crate ecdh;

use crypto_int::U512;

fn main() {
    let ecc = ecdh::brainpool_p256_r1();
    let p = ecc.curve.pt(U512::zero(), U512::zero());
    for _ in 0..100 {
        let x = ecc.generator.point * ecc.generator.order;
        if x != p {
            println!("uhoh");
        }
    }
}
