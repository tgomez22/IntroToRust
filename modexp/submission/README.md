## Tristan Gomez - Winter 2022

# Project Name: Modular Exponentiation 

### What I did
For this assignment I wrote a command line modular exponentiation tool. The tool takes three non-negative, positive integers as command line arguments. The tool attempts to convert the arguments from strings into u64. If the conversion fails due to some bad input then both specific and generic error messages are displayed to the user. If the user provides too many or too few arguments then both specific and generic error messages are displayed to the user. The generic error is a usage message for the program, but the specific error message describes why the error occurred. Assuming all inputs are valid then the modular exponentiation is performed and the result of the computation is displayed to the user and the program exits.


### How it went
I found this homework assignment to be fairly challenging but fun. The C-like syntax made it easy to jump into this language but it also got me in trouble in some spots. For example, I initially forgot to add the `mut` keyword to several of my variables which resulted in compiler errors. 

Speaking of compiler errors, I have found that Rust's compiler errors are quite a bit nicer/more helpful when it comes to debugging. Generally the compiler provides good information on:
    * What the error is exactly.
    * What exactly can be done to remedy the error.

These compiler errors occurred most often: when I was attempting to compare different data types, when I would forget to make a variable mutable, when I would accidentally make a variable mutable that never changed its value, and a few times when I wrote a potentially infinite loop. The errors were initially difficult to figure out since they were new types of error messages, but ultimately they were very helpful and much more descriptive than C based error messages.

I really enjoy the `match` syntax and control flow. It was a little difficult to get a hang of initially. I struggled with when to use Ordering with match. Match is pretty intuitive and I found it interesting that I got a lot of compiler warnings initially when I wrote if-else-else if statements. The errors *strongly* suggested that I use match instead. 

### How I tested my work
I used the given assertions in the assignment sheet, as well as, wrote a few of my own test cases to test my program programatically. I also ran my own manual tests on the program using negative values or characters then converted these manual tests into test cases than can be ran with `cargo test`. The generic usage error displayed during each trial of invalid input. I changed the given `parsenum()` function to display the specific `ParseIntError` that resulted during the attempted conversion so that the user would get specific feedback as to why their input was invalid. 

I wrote test cases calling just `parsenum()` while passing in invalid inputs. For example, I attempted to pass in -5 as an argument and then assert that -5 would not equal the return value of `parsenum()`. The compiler would not compile after I wrote this test because -5 is not a valid value for a u64. I realized that if a parsing error occurs in this function then it doesn't return, but rather it calls `error()` and exits. I also attempted to write test cases passing invalid input directly into `modexp()`, but again I was prevented from doing so by the compiler because the values were not valid for a u64. I found this feature to be helpful and reassuring; however, It also frustrated me a bit. I was able to use the professor's code changes (below) to correctly write tests using invalid input. 
```
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");

    #[cfg(test)]
    panic!("error");
    #[cfg(not(test))]
    std::process::exit(1);
}
```

This change allowed me to write test cases with the `#[should_panic]` attribute, allowing me to properly validate that invalid input is being correctly handled.
