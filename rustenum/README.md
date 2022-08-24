# RustEnum - A Web Page Enumeration Tool

### RustEnum and site scanner video demo!
https://media.pdx.edu/media/t/1_a0opn1a2

## Guides Used (More details in the "Lessons Learned" section below):
* https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest
* https://kerkour.com/rust-worker-pool/
* https://github.com/console-rs/indicatif/blob/main/examples/download.rs

## What Was Built
In CTF competitions and penetration testing engagements web page enumeration plays a vital role. The goal of the project was to provide similar functionality and interface of GoBuster, a popular web page enumeration tool, while also implementing more QOL features.

Features Provided by RustEnum:
* Enumerate web pages and directories using a user-supplied wordlist.
* Users can supply a list of HTTP status codes to ignore.
* Users can write scan output to a file of their choice.
* Users can specify(up to 14) the number of concurrent GET requests being sent.
* Users can supply a list of file extensions to be appended to each webpage.
* Terminal output is color-coded as a QOL feature for users. 
* Sorted output from low to high http status codes for ease of use.
* Added a loading/progress bar that updates during scan.

RustEnum uses the `tokio` runtime with the `reqwest` crate to send (10 by default) concurrent GET requests for pages in the user-supplied wordlist. Specifically, 10 requests are sent initially. As responses are received by RustEnum, they are processed and removed from a queue of requests. As a fulfilled request is removed from the queue a new, unfulfilled request is added to the queue. By default there can be a maximum of 10 requests being awaited in the queue.

## How RustEnum Works
A user needs to supply at least two command-line arguments for this program to run. The first command-line argument is `-w <path_to_wordlist>` or `--wordlist <path_to_wordlist>` (an absolute file path). The second command-line argument is `-u <base_url>` or `--url <base_url>`. RustEnum takes the base url and appends a word from the user-supplied wordlist to it. RustEnum attempts to perform an HTTP GET request for each word in the wordlist. If the user supplied a list of file extensions to search for, then every single word in the wordlist has each file extension appended to it. The base_url + word, without any extensions, is still checked too. When all HTTP responses are received, then RustEnum sorts them in ascending order by HTTP status code. The sorted results are also color coded in the terminal. Green signifies status codes in the range of 200-299, which are the success/found codes. Blue signifies status codes in the range of 300-399, which are the redirect status codes. With the redirect status codes, the `Location` header in the response will be displayed as well to show the user where the redirect is going to. Red signifies status codes in the range of 400-599 which are the "failure" codes, client or server. Yellow signifies a catch all for status codes that are not any of the previously listed ones. These would be very rare and the yellow signifies caution/look more closely at this.

There are several optional command-line flags.

* --ignore/-ig
* --threads/-t
* --output/-o
* --extensions/-x

The `--ignore/-ig` flag takes a comma-separated list of HTTP status codes (403, 404, 200, 301, etc.). These status codes are added to a list inside of the `Scanner` object. When an HTTP response from a potential page is received, its status code is checked against the ignore list. If the status code is in the ignore list then the response is ignored by the `Scanner`. If the status code is *not* in the ignore list, then the response is added to the `Scanner` object's `found` BTreeMap.

The `--threads/-t` flag takes a whole number ranging from 1 to 14 inclusive. This number represents the number of concurrent HTTP GET requests being sent. The default is 10 concurrent requests. 14 is the cap to prevent DoS. These aren't actually threads being spawned. This is mirroring terminology used by other enumeration tools to indicate concurrent requests.

The `--output/-o` flag takes an absolute file path as an argument. This file path can be to a file that exists or doesn't yet exist. If the file exists then the results of the scan will be appended to that file. If the file does not exist, then it will be created at that path with the contents of the scan in it. 

The `--extensions/-x` flag takes a comma-separated list of file extensions. These extensions are appended to each url generated in the program. The extensions can be prepended with a `.`, like `.php`. The extensions do **not** need to be prepended with a `.`, `php` works as well. 


## What Didn't Work
When I began work on this program/library, I had a few unanswered questions. I was unsure how Rust's borrowing rules would work in my initial design of the program. I made some slight alterations to my initial design, but it has remained mostly the same. This became an issue when I attempted to implement an intensive search option. The intensive search option was a stretch goal, so I didn't fully think out how it would have to be implemented. As a result, when I got around to attempting to implement it, I felt as though I had engineered myself into a corner. I didn't have a good answer as to what data structure I should use to store the results of an intensive scan. Depending on a website's structure the scan could be technically boundless. As long as a 300-399 status code is found, then the entire wordlist would've been run against that redirect link. If a redirect was found in that redirected search then again, the entire wordlist would be run. This would've continued as long as redirects would've been found. I could've tried to make a maximum depth of the search and have a user supplied depth argument, but this was an open question to me if it would've worked out in the end. I would've had to rewrite most of the already implemented functionality, and I don't know if those rewrites would've worked. The potential exists that I would've found more structural defects in my implementation, requiring even more rewrites. I decided that users would most likely prefer the current implementation, without the `intensive scan` functionality. Users could re-run this tool, passing in a desired redirect link to be enumerated. Some redirects lead to dead-ends or files that just aren't interesting to a penetration tester. For example, red-teams/pentesters don't care about CSS files, so why should they be forced to enumerate them?

Another failing of my program is that the stretch goal of writing scan results to a file in JSON or XML wasn't implemented. To be clear, the program can write the results of a scan to a file, but it just won't write the results in JSON or XML. This wasn't a Rust issue, but rather, when I tried to implement it, it felt forced. When thinking about the functionality, it seemed like a good idea when writing my proposal, but bad when I tried to implement it. It felt forced when I tried to write this functionality. I couldn't find a good way to structure the JSON/XML output in a way that makes sense and is helpful. My idea just seemed to try to force the output into JSON or XML format when it didn't add anything helpful to the end user. 


## What Lessons Were Learned

I learned that borrowing rules can be a pain to work around, but ultimately are helpful. Working with the borrowing rules in a concurrent, asynchronous program using closures is hard. I had to reference a few guides and a StackOverflow post to get this program to run. This project has helped me takes steps towards using a more functional programming style. To work within the borrowing rules, I did a lot of extra cloning of Strings in this program, especially for using words from the wordlist in closures. Recently, I was able to remove the cloning and replace it with an iterator and borrowing references from that iterator. Also, I found that I was cloning the base url for every single request being sent. After a recent lecture, I changed this field from a String to an `Arc<String>`. With this change, I was able to cut about 6 seconds off of my benchmark tests by eliminating the unnecessary cloning.

I had initial difficulty in implementing a counter to be used in a loading/progress bar for the program since I had to increment the counter in a closure using concurreny and async/await functionality. I used an `AtomicUsize` for the counter after learning about it in lecture, and it make this part much more easy to implement. 

I used this guide: https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest when I was working on how to send concurrent GET requests. I lifted the code from the provided answer which was written by user `Shepmaster`. For reference the lines in main.rs are 158-210. I had to make some alterations to the code because the original code segment takes the body of each GET response and prints them to the terminal. I needed to get each status code from the response, and the page requested to be stored in the Scanner object. When I was implementing this, I struggled a lot. The borrow checker was fighting me at every turn. I couldn't figure out the return type of this code block. I couldn't figure out how to make the functionality work in the closure of `map()`. After many hours of reading Rust Docs, I was able to slowly make it work piece by piece. 

First, I needed to fix the return type and to do this I made potentially a bad decision. No matter what response I get, this closure returns a `Vec<(String,u16)>`. If a request is successful then the correct HTTP status code and page is returned. If there is some sort of error then the tuple ("failed", 404) is returned. `RustEnum` ignores 404 status codes and does not store them, so errors would be ignored in this way. If I had more time, I would like to go back and properly handle potential errors. 

Second, I had difficulty getting the `Vec<(String, u16)>` but this was an easy fix. After reading more documentation, I found that I needed to add `.collect()` because I was working with an iterator. 

For the progress bar functionality, I used the code from this: https://github.com/console-rs/indicatif/blob/main/examples/download.rs . It is the official docs/examples from the `indicatif` crate. Their `ProgressBar` looks beautiful and I thought it would add a lot to the user experience of the program. I also referenced their RustDocs heavily when adapting their code to my uses. Lines 97 to 108 were the lines that were mostly copied from the file mentioned above. I had to make slight adjustments to the arguements for `.template()` because I am measuring progress by words scanned, not bytes downloaded. I then had to reference their docs to determine how to update the progress bar and have it end with a completion message.