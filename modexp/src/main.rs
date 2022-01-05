//! Command-line modular exponentiation tool
//!
//! Tristan Gomez 2022

/// Lib inclusions.
/// Ordering for use in match control flow.
/// Env for use in obtaining command line args.
use std::cmp::Ordering;
use std::env;

/// This method performs an initial match on the number of given command line args.
/// If there are too few or too many arguments given then a specific error message is given to help the user.
/// A generic usage error message is also displayed.
///
/// If the proper number of arguments is given then the arguments are parsed from strings into u64. If the conversion fails
/// a specific error message for why the conversion failed is displayed to the user, as well as, a generic usage error message.
///
/// If the arguments are successfully parsed then modexp() is called and the parsed arguments are passed in. Once modexp returns,
/// display its return value to the user and exit the program successfully.
fn main() {
    // Gather command line arguments to be stored in a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Check if there is the expected number of arguments. Call error() if incorrect number of arguments. Else call modexp().
    match args.len().cmp(&4) {
        Ordering::Less => {
            println!(
                "Too few arguments. This program takes 3 arguments and {} were provided",
                args.len() - 1
            );
            error()
        }
        Ordering::Greater => {
            println!(
                "Too many arguments. This program takes 3 arguments and {} were provided",
                args.len() - 1
            );
            error()
        }
        Ordering::Equal => {
            // Found expected number of arguments. Convert them from String to u64 using parsenum(), and store result in vector.
            // If any inputs fail to parse successfully then display both a specific err message with exact type of error, as well as,
            // a generic usage error.
            let converted_args: Vec<u64> =
                vec![parsenum(&args[1]), parsenum(&args[2]), parsenum(&args[3])];

            let ret_val = modexp(converted_args[0], converted_args[1], converted_args[2]);
            println!("{}", ret_val);
            std::process::exit(0);
        }
    }
}

/// This method performs the modular exponentiation. It takes 3, u64 arguments and returns a u64
/// upon success. If there is some sort of error then specific and generic error messages are displayed to the user.
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    let mut y = y;
    let m: u128 = u128::from(m);
    let mut x: u128 = u128::from(x);

    match m {
        0 => {
            println!(
                "The third argument cannot be 0. It must be a positive integer greater than 0."
            );
            error();
        }
        1 => 0,
        _ => {
            let mut ret_val: u128 = 1;
            while y > 0 {
                if y % 2 == 1 {
                    ret_val = (ret_val * x) % m;
                }
                y /= 2;
                x = (u128::pow(x, 2)) % m;
            }

            // Convert the ret_val into a u64 and return it
            // or
            // Somehow the converstion from u128 to u64 failed even though the return value
            // should be <= m, which fits into a u64.
            u64::try_from(ret_val).unwrap_or_else(|caught_err| {
                eprintln!("{}", caught_err);
                error()
            })
        }
    }
}

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");

    #[cfg(test)]
    panic!("error");

    #[cfg(not(test))]
    std::process::exit(1);
}

/// Parse the given string as a `u64`
fn parsenum(s: &str) -> u64 {
    // I changed the provided code to display the ParseIntError given if the conversion
    // fails, so that way users can have more helpful error messages rather than the standard
    // error message.
    s.parse().unwrap_or_else(|ret_val| {
        eprintln!("{}", ret_val);
        error()
    })
}

/// Test cases below
#[test]
fn test_modexp1() {
    // Largest prime less than 2**64
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
}

#[test]
fn test_modexp2() {
    let bigm = u64::max_value() - 58;
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
}

#[test]
fn test_modexp3() {
    let bigm = u64::max_value() - 58;
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
}

#[test]
fn test_modexp4() {
    assert_eq!(4, modexp(10, 9, 6));
}

#[test]
fn test_modexp5() {
    assert_eq!(34, modexp(450, 768, 517));
}

#[test]
fn test_modexp6() {
    assert_eq!(16, modexp(2, 20, 17));
}

#[test]
fn test_modexp7() {
    let num = u64::max_value();
    assert_eq!(166, modexp(num, num, 517));
}

#[test]
fn test_modexp8() {
    let num = u64::max_value();
    assert_eq!(0, modexp(num, num, num));
}
