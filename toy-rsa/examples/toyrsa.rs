use toy_rsa::encrypt;

fn main() {
    let message: u32 = 0x12345f;
    let pubkey: u64 = 0xde9c5816141c8ba9;

    println!("{}", encrypt(pubkey, message))
}
