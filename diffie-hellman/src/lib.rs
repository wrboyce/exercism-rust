pub fn private_key(p: u64) -> u64 {
    p - 1
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

// see https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
fn mod_exp(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1;

    b %= m;
    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % m
        }
        e >>= 1;
        b = b.pow(2) % m;
    }

    r
}
