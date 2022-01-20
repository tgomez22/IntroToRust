## Tristan Gomez

# Keyword Index - HW 3

### What I did

### How It Went
I felt as though I got through this assignment fairly painlessly, but I don't believe that this was an 'easy' assignment. I just think that I'm starting to get a good feel for this language and its unique aspects. I am a bit embarrassed that I didn't initially recognize `pub struct KWIndex<'a>(Vec<&'a str>);` as declaring a tuple struct. I thought this may have been some weird mix of a struct and function definition. Initially, I rewrote this line to be... 
```
pub struct KWIndex<'a>{
    index: Vec<&'a str>
}
```

After receiving some clarification on the assignment from Cassaundra I was able to recognize the tuple struct for what it was. I then changed my implementation back to using the original struct definition/declaration. 

I didn't struggle with lifetimes or reference types much in this assignment. The toughest part of the assignment was coming up with the logic for some of the methods for the KWIndex struct. For example, I encountered some initial difficulty with every case for handling input to `extend_from_text()`; however, I realized that there are only really two cases for this method, accept or reject a string. I struggled with deciding where I should try to remove non-internal punctuation, and settled on it being immediately after words are split based on whitespace. It was very interesting to use the `trim_end_matches()` method and develop my own closure as its argument. 

The `nth_uppercase()` method was interesting to implement because I it was the first method returning an `Option` that I had implemented. In this method, I examine the first character of each 'stored' word in the index and determine if it is uppercase. I did this using...
```
 let mut uppercase_words: Vec<&str> = Vec::new();
        for &word in &self.0 {
            match word.chars().next() {
                Some(letter) => {
                    if letter.is_ascii_uppercase() {
                        uppercase_words.push(word)
                    }
                }
                None => continue,
            }
        }
        //more code below
```
In the above code, I 'get' the first letter of each stored word using `word.chars().next()` which was not the original way I wrote it. I originally wrote `word.chars().nth(0)`. Clippy gave me a warning that using `nth(0)` is equivalent to `.next()`, so I changed my code to use the suggested code.

### How I Tested My Work
For this homework assignment, I used all of the given examples in the assignment sheet for my automated tests suite. I also created several other tests that test items not covered in the given cases. For example, I created...
```
#[test]
fn test_example4() {
    let kwindex = KWIndex::new().extend_from_text("I am THE WALRUS");
    assert_eq!(None, kwindex.nth_uppercase(100));
}
```
The above test case checks that the `nth_uppercase()` method returns None (instead of panicking) if the argument attempts to access an out-of-range/non-existant index in the vector storing the str slices.