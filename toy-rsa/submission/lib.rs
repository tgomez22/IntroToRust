//! Toy RSA encryption library
//!
//! Tristan Gomez - Winter 2022

/// Included library provides gcd(), lcm(), modexp(), modinverse(), and rsa_prime() methods.
use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    loop {
        let p: u32 = rsa_prime();
        let q: u32 = rsa_prime();
        if EXP < lcm(u64::from(p - 1), u64::from(q - 1))
            && gcd(EXP, lcm(u64::from(p - 1), u64::from(q - 1))) == 1
        {
            return (p, q);
        }
    }
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    let msg = u64::from(msg);
    modexp(msg, EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
///
/// May panic!() if an error occurs when converting the u64 msg into a u32.
/// This panic!() technically shouldn't ever occur but the u32::try_from()
/// method has the potential to return an Err().
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d: u64 = modinverse(EXP, lcm(u64::from(key.0 - 1), u64::from(key.1 - 1)));
    let plaintext: u64 = modexp(msg, d, u64::from(key.0) * u64::from(key.1));

    u32::try_from(plaintext).unwrap_or_else(|caught_err| {
        eprintln!("{}", caught_err);
        panic!("Internal logic error converting u64 plaintext into u32.")
    })
}
