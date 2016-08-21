extern crate num;
extern crate num_traits;
extern crate crypto_int;

mod field;
mod elliptic;

#[cfg(test)]
mod tests {
    use field;
    use elliptic;
    use crypto_int::U512;

    #[test]
    fn it_works() {
        let i = U512::from_bytes_be(vec![
            0xA9, 0xFB, 0x57, 0xDB, 0xA1, 0xEE, 0xA9, 0xBC,
            0x3E, 0x66, 0x0A, 0x90, 0x9D, 0x83, 0x8D, 0x72,
            0x6E, 0x3B, 0xF6, 0x23, 0xD5, 0x26, 0x20, 0x28,
            0x20, 0x13, 0x48, 0x1D, 0x1F, 0x6E, 0x53, 0x77,
        ]);
        let gf = field::GF::new(i);
        println!("{}", i);
        let x = gf.el(i);
        println!("{}", x);
    }

    #[test]
    fn basic_addition() {
        let f = field::GF::new(U512::from_u64(11));
        let e = elliptic::EllipticCurve::new(
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
