//! Rustenum - a command-line webpage enumeration tool!
//!
//! Tristan Gomez - Winter 2022

use futures::{stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use rustenum::*;
use std::collections::HashMap;
use std::env;
use std::sync::atomic::AtomicUsize;
use std::time::Instant;

#[tokio::main]
async fn main() {
    // Initialize a new reqwest::Client object that will eventually send
    // GET requests. Reqwest recommends using a Client object over the 'reqwest::get()'
    // method when making large numbers of GET requests.
    let client = Client::builder()
        // Set the client to never follow redirects.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // skip past the name of the program in argv and collect the rest of the command-line args.
    let cmd_args: Vec<String> = env::args().skip(1).collect();

    // process the command-line arguments and store them in a `processed_args` hashmap.
    let processed_args = process_arguments(&cmd_args);

    // Create a new scanner object.
    let mut scanner = Scanner::new();

    // Check if website exists and can be correctly parsed.
    if processed_args.contains_key("-u") {
        match scanner.try_add_site(processed_args.get("-u").unwrap()) {
            // Url successfully parsed, do nothing but continue program execution.
            Ok(_msg) => {}

            // Error when parsing url, we cannot continue program execution.
            // Provide diagnostic message to user, call usage() and exit.
            Err(err) => {
                eprintln!("{}", err);
                usage();
            }
        }
    }

    // Valid site, next step is to check if wordlist is valid.
    // If it is then load its contents into the Scanner object.
    let mut infile = String::new();

    // User provided wordlist flag and argument. Store the file name into `infile`.
    if processed_args.contains_key("-w") {
        if let Some(val) = processed_args.get("-w") {
            infile += val;
        }

    // User did *not* provide a wordlist flag and argument. Program execution cannot
    // continue. Call usage() and exit.
    } else {
        usage();
    }

    scanner.build_wordlist_from_file(infile);

    // Add "404" to the ignore list by default.
    scanner.use_default_ignore_list();

    // The user provided the flag for adding a list of file extensions to each webpage in the scan.
    if processed_args.contains_key("-x") {
        if let Some(val) = processed_args.get("-x") {
            // process the provided list of extensions and add them to the scanner.
            scanner.add_extensions_to_wordlist(val);
        }
    }

    // Print a nice message to the user.
    println!("---------------------------------------------------------");
    println!("\u{1f50e} RustEnum - A webpage enumeration tool \u{1f50D}");
    println!("By Tristan Gomez");
    println!("---------------------------------------------------------");

    // This variable is used to keep track of how many webpages have been enumerated.
    // It is an AtomicUsize because it needs to be safely shared between threads.
    let mut words_used = AtomicUsize::new(0);

    // This variable represents the total number of pages needed to be enumerated.
    // It is cast from a usize to a u64 for use in the progress_bar object.
    let wordlist_size = scanner.wordlist.len() as u64;

    // We need to initialize a progress bar with a length that is the number of pages to enumerate through.
    let progress_bar = ProgressBar::new(wordlist_size);

    // This code in `progress_bar.set_style()` is almost exactly taken from the `indicatif` documentation/examples.
    // Please see the README for extra information, including a link to the code where this segment was taken from.
    progress_bar.set_style(
        ProgressStyle::default_bar()
            // I am designating the progress bar to have a spinning icon with a color of green that helps indicate
            // progress to the user. I am then displaying the elapsed time for the scan. The color of the progress bar is cyan/blue.
            // I then display the percent of pages which have already been enumerated through. Finally, I have a message
            // field which is empty until the scan is completed. It will eventually say 'scan complete' when the scan is over.
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% {msg} ")
            .unwrap()
            .with_key("eta", |state| format!("{:.1}s", state.eta().as_secs_f64()))
            // This sets the progress bar to fill with '#' as it progresses, with the leading character being a '>'.
            .progress_chars("#>-"),
    );

    // The user has designated more HTTP status codes to ignore. We will process them
    // and add them to the scanner's ignore list.
    if processed_args.contains_key("-ig") {
        let ignore_list = parse_ignore(processed_args.get("-ig").unwrap());
        scanner.add_to_ignore_list(&ignore_list);
    }

    // The default number of concurrent/parallel requests that can be sent is 10.
    let mut thread_count = 10_usize;

    // If the user wants to change the default number of concurrent/parallel requests being sent.
    if processed_args.contains_key("-t") {
        thread_count = processed_args
            // get the desired thread count. Attempt to parse it as a usize. If the parse is successful
            // then assign the number to thread_count. It not, display a diagnostic message, call usage()
            // and exit.
            .get("-t")
            .unwrap()
            .parse::<usize>()
            .unwrap_or_else(|err| {
                eprintln!("Error ocurred when attempting to get thread count: {}", err);
                usage();
            });

        // To prevent DoS (malicious or accidental), I am capping the number of concurrent/parallel
        // requests to 14.
        if thread_count > 14 {
            println!("Cannot specify a thread count higher than 14.");
            usage();
        } else if thread_count < 1 {
            println!(
                "The minimum number of threads needs to be in the range of 1 to 14 inclusive."
            );
            usage();
        }
    }

    println!("Starting Scan.");

    // Scan is beginning take the time to be used later to determine how long
    // the scan took.
    let now = Instant::now();

    // The code immediately below assigning the Vec<(String, u16)> to `temp` is mostly
    // taken from a Stack Overflow post by user Shepmaster. The url to the post/solution
    // is in the README.

    // I am creating an iterator over the scanner's wordlist then turning it into a stream iterator.
    let temp: Vec<(String, u16)> = stream::iter(scanner.wordlist.0.iter())
        // Each word in the stream iterator is then mapped to the block of code that scans a web page.
        .map(|word| {
            // We need to take a reference to the client object so we can use it to send GET requests.
            let client = &client;

            // We need to get the main url/site path from the scanner object.
            let path = scanner.site.clone();

            // Increment the words_used AtomicUsize variable to progress the
            // progress bar.
            let words_used = &mut words_used;
            *words_used.get_mut() += 1;

            // update the progress_bar with the new count of words_used.
            progress_bar.set_position(*words_used.get_mut() as u64);

            async move {
                // construct the full url by concatenating the url_path + "/" + word_from_wordlist
                let path = &path;
                let mut url;
                if path.ends_with('/') {
                    url = path.to_string();
                    url += word;
                } else {
                    url = path.to_string();
                    url += "/";
                    url += word;
                }

                // Send the get request for the url and await a response.
                match client.get(url).send().await {
                    // This isn't the best solution because it does very "bad" error handling.
                    // It is explained in the comments for both 'process_response' and 'failed_response()'.
                    // Both methods return a (String, u16), but the 'failed_response' method
                    // returns ("failed", 404) as a work around.
                    // 'process_response' returns the results of the request.
                    // In this program, the 404 status code from a failed request will prevent the tuple from being
                    // added to the results list.
                    Ok(resp) => process_response(word.to_string(), resp),
                    Err(_) => failed_response(),
                }
            }
        })
        // buffer_unordered caps the number of parallel/concurrent GET requests being sent to the value of 'thread_count'/
        // This should prevent DoS for a website by preventing every request from being sent at once. The default limit is
        // 10 requests at a time, but it can be changed to any value from 1 ..=14.
        // I am using buffer_unordered because I do not care about the order of my GET responses. I only care that they arrived.
        // When a response is received, then a new GET request is added to the buffer_unordered queue.
        .buffer_unordered(thread_count)
        // Await all responses and collect the responses at the end of the scan.
        .collect()
        .await;

    // For every result found, process them.
    // `add_to_found()` ignores any tuple that has
    // a u16 in the scanner's ignore list.
    for result in temp {
        scanner.add_to_found(result);
    }

    // Scan is over, finish and clear the progress bar.
    progress_bar.finish_and_clear();

    println!("Scan Complete");
    println!("Time elapsed: {} seconds", now.elapsed().as_secs());
    scanner.display_found();

    // if the user gave the option to write the results to a file then attempt to do so.
    if processed_args.contains_key("-o") {
        // Get the file path for the outfile and attempt to write to it. This will
        // attempt to create a new file if the one at the provided path doesn't exist.
        match scanner.write_results_to_file(processed_args.get("-o").unwrap()) {
            Ok(msg) => println!("{}", &msg),
            Err(err) => {
                eprintln!("{}", &err);
            }
        }
    }
}

/// This method displays the manual of flags used in the program. This method exits the program without an error upon finishing.
fn display_man_page() -> ! {
    println!("RustEnum - A website enumeration tool!");
    println!("by Tristan Gomez - Winter 2022 - Intro to Rust Programming\n");
    println!("[REQUIRED FLAG(S)]");
    println!("  -w ,  --wordlist      Provide website page wordlist for scan.");
    println!("  -u ,  --url           The address of the host to scan. If 'http://' or 'https://' is not prepended to the url then 'http://' is used by default.");
    println!("[Options]");
    println!("  -ig , --ignore        Provide a comma separated list of HTTP status codes to ignore. Default without this option is 404. If flag is set, 404 is still by default included in the ignore list.");
    println!(
        "  -o ,  --output        Provide a file name/path for the scan to write its results to."
    );
    println!("  -t ,  --threads       Provide a thread count for number of concurrent requests. Default is 10, Max is 14.");
    println!("  -x ,  --extensions    Provide a comma separated of file extensions to append to each word in the wordlist.");
    std::process::exit(0);
}

/// This displays some examples on how to use this program. It also points the user to the use the -h/--help flag to
/// get more help with using this program.
fn usage() -> ! {
    println!("./rustenum [optional flags] -w /usr/share/wordlist/dirb/common.txt [optional flags] -u [host]");
    println!("Remember to use the correct HTTP scheme (HTTP/HTTPS) for the --url argument.");
    println!("EXAMPLE: ./rustenum -w /usr/share/wordlists/common.txt -t 4 -u http://10.10.10.10");
    println!("EXAMPLE: ./rustenum -w /usr/share/wordlists/common.txt -u https://10.10.10.10");
    println!("EXAMPLE: ./rustenum -w /usr/share/wordlists/common.txt -u http://example.com");
    println!("EXAMPLE: ./rustenum -w /usr/share/wordlists/common.txt -x .php,html,js -u http://example.com");
    println!("Use the '-h' or '--help' flags for man page.");
    std::process::exit(1);
}

/// This method processes the command-line arguments for the program.
/// It takes a slice of an array of Strings as its argument and after
/// processing the arguments will return them formatted in a HashMap.
fn process_arguments(cmd_args: &[String]) -> HashMap<String, String> {
    let mut cmd_options: HashMap<String, String> = HashMap::new();
    match cmd_args.len() {
        // No flags given. This is incorrect, call `usage()` and exit.
        0 => usage(),

        // 1 argument/flag given. Check if it is -h/--help.
        1 => {
            // if argument is -h or --help then display man page and exit.
            if cmd_args[0] == *"-h" || cmd_args[0] == "--help" {
                display_man_page();

            // some incorrect value was provided so call `usage` and exit.
            } else {
                usage();
            }
        }

        // Multiple command-line arguments were provided, process them.
        _ => {
            let mut i: usize = 0;
            while i < cmd_args.len() {
                if cmd_args[i] == "-w" || cmd_args[i] == "--wordlist" {
                    cmd_options.insert("-w".to_string(), cmd_args[i + 1].clone());
                    i += 2;
                } else if cmd_args[i] == "-u" || cmd_args[i] == "--url" {
                    cmd_options.insert("-u".to_string(), cmd_args[i + 1].clone());
                    i += 2;
                } else if cmd_args[i] == "-ig" || cmd_args[i] == "--ignore" {
                    cmd_options.insert("-ig".to_string(), cmd_args[i + 1].clone());
                    i += 2;
                } else if cmd_args[i] == "-o" || cmd_args[i] == "--output" {
                    cmd_options.insert("-o".to_string(), cmd_args[i + 1].clone());
                    i += 2;
                } else if cmd_args[i] == "-t" || cmd_args[i] == "--threads" {
                    cmd_options.insert("-t".to_string(), cmd_args[i + 1].clone());
                    i += 2;
                } else if cmd_args[i] == "-x" || cmd_args[i] == "--extensions" {
                    cmd_options.insert("-x".to_string(), cmd_args[i + 1].clone());
                    i += 2;
                } else {
                    usage();
                }
            }
            cmd_options
        }
    }
}

/// This method takes a string slice as its argument and returns a vector of u16s.
/// The argument should be a comma-separated list of http status codes to ignore.
fn parse_ignore(ignore_args: &str) -> Vec<u16> {
    let mut codes_to_use: Vec<u16> = Vec::new();

    // Split the string slice at the commas and for each word, attempt to add them to the
    // codes_to_use vector.
    for status_code in ignore_args.split(',') {
        // Try to parse status code as u16. If this fails then call `usage()` and exit program.
        codes_to_use.push(status_code.parse().unwrap_or_else(|err| {
            eprintln!("{}", err);
            usage();
        }));
    }
    codes_to_use
}

/// This method processes a reqwest::Response object received from the Client.get() in the scan.
/// This method takes 'ext'(webpage that was requested) and 'resp' as arguments.
pub fn process_response(ext: String, resp: reqwest::Response) -> (String, u16) {
    // If the response is a 301 redirect
    if resp.status().as_u16() == 301 {
        // If the location header was provided in the response object then
        // we need to format the returned object to reflect that. Attempt to
        // get the location header's value.
        if let Ok(location) = resp
            .headers()
            .get(reqwest::header::LOCATION)
            .unwrap()
            .to_str()
        {
            // display the requested page, the page the request
            // should be redirected to, and the http status code.
            (
                ext + "   [REDIRECTED TO: " + location + "]",
                u16::from(resp.status()),
            )

        // Location header doesn't exist. Return (page enumerated, http status code)
        } else {
            (ext, u16::from(resp.status()))
        }

    // If the response was a 302 redirect.
    } else if resp.status().as_u16() == 302 {
        // If the location header was provided in the response object then
        // we need to format the returned object to reflect that. Attempt to
        // get the location header's value.
        if let Ok(location) = resp
            .headers()
            .get(reqwest::header::LOCATION)
            .unwrap()
            .to_str()
        {
            // display the requested page, the page the request
            // should be redirected to, and the http status code.
            (ext + "   [" + location + "]", u16::from(resp.status()))

        // Location header doesn't exist. Return (page enumerated, http status code)
        } else {
            (ext, u16::from(resp.status()))
        }
    } else {
        // Normal case. Return the page requested and its http response.
        (ext, u16::from(resp.status()))
    }
}

/// Generic failed response. Returns a 404 status code and a placeholder value of "failed".
/// Since the status code is 404, it will be ignored by the scanner in this program.
pub fn failed_response() -> (String, u16) {
    ("failed".to_string(), 404u16)
}
