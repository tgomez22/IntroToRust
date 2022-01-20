//! Toy-RSA crate (key-gen, encryption, decryption) test cases
//!
//! Tristan Gomez - Winter 2022
use toy_rsa::*;
use toy_rsa_lib::*;

#[test]
fn test_encryption() {
    let public_key: u64 = 0xde9c5816141c8ba9;
    let message: u32 = 0x12345f;

    assert_eq!(0x6418280e0c4d7675_u64, encrypt(public_key, message));
}

#[test]
fn test_decryption() {
    let ciphertext: u64 = 0x6418280e0c4d7675;
    let key: (u32, u32) = (0xed23e6cd, 0xf050a04d);
    assert_eq!(0x12345f, decrypt(key, ciphertext));
}

#[test]
fn test_all_functions() {
    let (p, q): (u32, u32) = genkey();

    let plaintext: u32 = 0xfa56db77;

    let ciphertext: u64 = encrypt(u64::from(p) * u64::from(q), plaintext);

    let result: u32 = decrypt((p, q), ciphertext);

    assert_eq!(result, plaintext);
}

#[test]
fn test_p_q_in_range() {
    let (p, q): (u32, u32) = genkey();

    //check p, q in range 2**31 to (2**32) - 1
    assert!(p >= u32::pow(2, 31) && p < u32::MAX);
    assert!(q >= u32::pow(2, 31) && q < u32::MAX);

    assert!(
        EXP < lcm(u64::from(p - 1), u64::from(q - 1))
            && gcd(EXP, lcm(u64::from(p - 1), u64::from(q - 1))) == 1
    );
}

#[test]
fn test_if_encrypt_overflows() {
    let result = encrypt(u64::MAX, u32::MAX);
    assert_eq!(0xffffffff, result);
}

#[test]
fn test_if_decrypt_overflows() {
    let result = decrypt((u32::MAX, u32::MAX), u64::MAX);
    assert_eq!(0, result);
}
