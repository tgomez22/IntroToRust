//! Command-line modular exponentation tool
//! 
//! Tristan Gomez 2022

use std::env;
use std::cmp::Ordering;

fn main() {
    
    let args: Vec<String> = env::args().collect();

    // Check if there is the expected number of arguments. Call error() if incorrect number of arguments. Else call modexp().
    match args.len().cmp(&4) {
        Ordering::Less => error(),
        Ordering::Greater => error(),
        Ordering::Equal => {

            // Found expected number of arguments. Convert them from String to u64 using parsenum(), and store result in vector.
            let converted_args: Vec<u64> = vec![
                parsenum(&args[1]),
                parsenum(&args[2]),
                parsenum(&args[3])
            ];

            let ret_val = modexp(converted_args[0], converted_args[1], converted_args[2]);
            println!("{}", ret_val);
            std::process::exit(0);
        }
    }
}


fn modexp(x: u64, y: u64, m: u64) -> u64{
    let mut y = y;
    let m: u128 = u128::from(m);
    let mut x: u128 = u128::from(x);

    match m {
        0 => error(),
        1 => 0,
        _ => {
            let mut ret_val: u128 = 1;   
            while  y > 0 {
                if y % 2 == 1 {
                    ret_val = (ret_val * x) % m;
                }
                y /= 2;
                x = (u128::pow(x, 2)) % m;
            }
            u64::try_from(ret_val).unwrap()
        }
    } 
}

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

/// Parse the given string as a `u64`
fn parsenum(s: &str) -> u64{
    s.parse().unwrap_or_else(|_| error())
}


/// Test cases
#[test]
fn test_modexp(){
    // Largest prime less than 2**64

    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));

    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));
    assert_eq!(16, modexp(2, 20, 17));
}