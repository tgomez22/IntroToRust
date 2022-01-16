//! Toy RSA encryption library
//!
//! Tristan Gomez - Winter 2022

/// Include library functions from this crate.
use toy_rsa::*;

/// This example program is just a short demo of the my RSA implementation.
/// A key is generated using the genkey() function. Then an arbitrary, hardcoded message is
/// encrypted using the encrypt() function. Then the resulting ciphertext is decrypted using the decrypt()
/// function. This program prints its results like the given test case in the assignment sheet.
fn main() {
    println!("This is a short demo program of my toy RSA implementation.");

    let private_key: (u32, u32) = genkey();
    println!(
        "Private Key: p = 0x{:x} q = 0x{:x}",
        private_key.0, private_key.1
    );

    let public_key: u64 = u64::from(private_key.0) * u64::from(private_key.1);
    println!("Public key: p * q = 0x{:x}", public_key);

    let message: u32 = 0x12345f;
    println!("Message: 0x{:x}", message);

    let ciphertext = encrypt(public_key, message);
    println!("Encrypted: 0x{:x}", ciphertext);

    let decrypted: u32 = decrypt(private_key, ciphertext);
    println!("Decrypted: 0x{:x}", decrypted);
}
