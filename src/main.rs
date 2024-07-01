use std::io;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use num_integer::Integer;
use rand::prelude::*;
use base64::{encode, decode};



const BIT_SIZE: usize = 2048;
const TEST_ROUNDS: usize = 16;

fn big(n: usize) -> BigUint {
    BigUint::from(n)
}

fn generate_prime(bits: usize) -> BigUint {
    let mut rng = thread_rng();
    loop {
        let prime_candidate = rng.gen_biguint(bits.try_into().unwrap());
        if is_prime(&prime_candidate, TEST_ROUNDS) {
            return prime_candidate;
        }
    }
}

fn is_prime(n: &BigUint, k: usize) -> bool {
    if *n == big(2) || *n == big(3) {
        return true;
    }
    if n.is_even() || *n < big(2) {
        return false;
    }

    let mut d = n - 1u32;
    let mut r = 0usize;
    while d.is_even() {
        d >>= 1;
        r += 1;
    }

    'outer: for _ in 0..k {
        let a = thread_rng().gen_biguint_range(&big(2), &(n - 2u32)) + 1u32;
        let mut x = a.modpow(&d, n);
        if x == BigUint::one() || x == n - 1u32 {
            continue;
        }
        for _ in 0..r - 1 {
            x = x.modpow(&big(2), n);
            if x == n - 1u32 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn mod_inv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let mut mn: (BigUint, BigUint) = (m.clone(), a.clone());
    let mut xy: (BigUint, BigUint) = (BigUint::zero(), BigUint::one());

    while mn.1 != BigUint::zero() {
        let quot = (&mn.0 / &mn.1).clone();

        let (new_mn0, new_mn1) = (mn.1.clone(), (&mn.0 + (&mn.1 * &quot)) % &mn.1);
        mn = (new_mn0, new_mn1);

        let (new_xy0, new_xy1) = (xy.1.clone(), (&xy.0 + (&xy.1 * &quot)) % m);
        xy = (new_xy0, new_xy1);
    }

    if mn.0 > BigUint::one() {
        None
    } else {
        Some((xy.0 + m) % m)
    }
}

fn generate_rsa_keys() -> (BigUint, BigUint, BigUint) {
    let p = generate_prime(BIT_SIZE / 2);
    let q = generate_prime(BIT_SIZE / 2);
    let n = &p * &q;
    let phi = (&p - 1u32) * (&q - 1u32);
    let e = BigUint::from(65537u32);
    let d = mod_inv(&e, &phi).expect("Modular inverse does not exist.");

    (e, d, n)
}

fn encrypt(m: &BigUint, e: &BigUint, n: &BigUint) -> BigUint {
    m.modpow(e, n)
}

fn decrypt(c: &BigUint, d: &BigUint, n: &BigUint) -> BigUint {
    c.modpow(d, n)
}


fn main() {
    let (e, d, n) = generate_rsa_keys();

    println!("Enter a message to encrypt:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let message_bytes = input.trim().as_bytes();
    let message = BigUint::from_bytes_be(message_bytes);

    let encrypted = encrypt(&message, &e, &n);
    let encrypted_base64 = encode(&encrypted.to_bytes_be());
    println!("Original: {}", input.trim());
    println!("Encrypted (Base64): {}", encrypted_base64);

    let decrypted_bytes = decode(&encrypted_base64).unwrap();
    let decrypted_biguint = BigUint::from_bytes_be(&decrypted_bytes);
    let decrypted = decrypt(&decrypted_biguint, &d, &n);
    let binding = decrypted.to_bytes_be();
    let decrypted_message = String::from_utf8_lossy(&binding);

    println!("Decrypted: {}", decrypted_message);
}

