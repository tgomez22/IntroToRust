## Tristan Gomez - Winter 2022


# HW 2: Toy RSA



### What I did 
For this assignment, I wrote a toy RSA encryption/decryption library that implements an interface provided in the assignment sheet. I implemented

* `genkey() -> (u32, u32)`
* `encrypt(key: u64, msg: u32) -> u64`
* `decrypt(key:(u32, u32), msg: u64) -> u32`

The `genkey()` function takes no arguments and returns a tuple of two prime u32 values. This method generates two prime u32 numbers, p and q, repeatedly until these values meet two conditions. First, the least common multiple of p-1 and q-1 must be greater than the fixed RSA encryption exponent of 65,537. Second, the greatest common divisor of the fixed RSA encryption exponent and the least common multiple of p-1 and q-1 must equal 1. When both conditions are met then p and q are returned.

The `encrypt()` function takes two arguments, a u64 key (which should be p * q using the values from the genkey() function) and a u32 message that you want to encrypt. The function returns the u64 value of the computation `(msg**EXP) mod key`, known as the ciphertext.

The `decrypt()` function takes two arguments, a tuple of two u32 values and a u64 value. The message should be the ciphertext generated from the encrypt function. The decryption function generates a value `d` that is the inverse of `EXP mod lcm(p-1,q-1)`. The value `d` is then used in the decryption equation `(msg ** d) mod (p * q)`. The value of the decryption equation is converted back into a u32 and is returned. 

### How it went
I think this assignment went more smoothly than the first one. I didn't have many problems with the conversions between u32 and u64 that are all through the program. I struggled more with logic errors while converting the pseudocode into actual code. 

For example, in the `decrypt()` function I kept getting panics. 
```
let plaintext: u64 = modexp(msg, d, u64::from(key.0) * u64::from(key.1));
```

The above line is the correct code that does not panic. My mistake (below) results in an 'attempt to multiply with overflow' error.
```
let plaintext: u64 = modexp(msg, d, u64::from(key.0 * key.1);
```
This error is a mix of a logic error and a conversion one. 

Another error that I encountered was that I attempted to perform the modexp operation without using the provided modexp function in the toy_rsa_lib. I kept getting panics that I didn't understand until I stopped programming for awhile and examined the given pseudocode. I also needed to look at the docs for the toy_rsa_lib. Once I did this I was able to see that I could use the modexp function instead of trying to implement my own. Once I used the given function all my problems cleared up.

### How I tested my work
I wrote four automated tests for this assignment. 

* `test_encryption()` uses part of the given test case in the assignment sheet. It uses a given u64 public key and a u32 message. The result of the encryption function using the provided values is checked against the expected answer of 0x6418280e0c4d7675.

* `test_decryption()` uses part of the given test case in the assignment sheet. It uses a given u64 ciphertext value and a tuple of two u32 values representing p and q. The result of the decryption function using the provided values is checked against the expected answer of 0x12345f.

* `test_all_functions()` is a function that I wrote. It generates a tuple of two u32s using my `genkey()` implementation. I created an arbitrary u32 value for the plaintext message. I encrypt the plaintext message using the `encrypt()` method, passing in the plaintext and the generated (p,q). Finally, I pass in the ciphertext generated from the encrypt function into the decrypt function along with the (p,q) tuple. I then check that the resulting value from the decrypt method is the same as the original plaintext message.

* `test_p_q_in_range()` is a function that I wrote. I use the genkey() function to create a (p, q) tuple. I then check that both p and q are in the range of `2**31 to (2**32) - 1`. Then I check that EXP is less than the least common multiple of p-1 and q-1, and that the greatest common divisor of EXP and the least common multiple of p-1 and q-1 is equal to 1.