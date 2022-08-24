//! Command-line web page/directory enumeration tool
//!
//! Tristan Gomez - Winter 2022

use colored::*;
use reqwest::Url;
use std::collections::BTreeMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::io::{self, BufRead};
use std::sync::Arc;

/// A generic function to call when an error occurs. It takes a string slice as its sole argument which is displayed to the user.
/// Then the program gracefully ends with an error status code.
pub fn error(e: &str) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}

/// The `Wordlist` tuple struct contains the contents of the user provided wordlist file in a vector of strings.
#[derive(Default, Debug, Clone)]
pub struct Wordlist(pub Vec<String>);

impl Wordlist {
    /// Make a new empty pages-to-search list.
    pub fn new() -> Self {
        Wordlist(Vec::new())
    }

    /// Fill the wordlist with the contents of the file at arg 'path', and
    /// return the new wordlist.
    pub fn extend_from_file(&mut self, path: String) -> Self {
        // Attempt to open the file at the provided `path`.
        let _file = match File::open(path) {
            // File successfully opened!
            Ok(file_handle) => {
                // Attempt to read in all words from file, line-by-line. Then attempt to push them to the vector of strings.
                let lines = io::BufReader::new(file_handle).lines();
                for line in lines {
                    if let Ok(word) = line {
                        self.0.push(word.clone());
                    } else {
                        error("Error when reading from file. Please check the contents of the provided wordlist.");
                    }
                }
                return self.clone();
            }

            // File could not be opened. Call the `error()` with a diagnostic
            // message and exit gracefully.
            Err(_err) => {
                error("Error when handling file. Please check the provided file path");
            }
        };
    }

    /// Returns the number of stored words.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true is the wordlist is empty
    /// Returns false if any values are contained within the vector.
    pub fn is_empty(&self) -> bool {
        match self.0.len() == 0 {
            true => true,
            false => false,
        }
    }
}

/// This struct represents the scanner/scanning process. It contains the Wordlist struct, a vec<u16> that represents HTTP status codes to ignore,
/// a BTreeMap of results from HTTP GET requests during the enumeration procerss, and a string that is the base url for the site in question.
#[derive(Default, Debug)]
pub struct Scanner {
    // Words to look for in the scan
    pub wordlist: Wordlist,

    // Ignore pages that return HTTP status codes within this list.
    pub status_code_ignore_list: Vec<u16>,

    // The results of the scan. If a HTTP GET request returns a status_code NOT in the
    // 'status_code_ignore_list' then add the page name and its corresponding HTTP status code.
    pub found: BTreeMap<String, u16>,

    // The base url to enumerate.
    pub site: Arc<String>,

    // File extensions to check
    pub extension_list: Wordlist,
}

impl Scanner {
    /// Make a new empty scanner object.
    pub fn new() -> Self {
        Scanner {
            wordlist: Wordlist::new(),
            status_code_ignore_list: Vec::new(),
            found: BTreeMap::new(),
            site: Arc::new(String::new()),
            extension_list: Wordlist::new(),
        }
    }

    /// Attempt to add the provided url to the `site` data member. If the provided
    /// url is successfully parsed by the `reqwest::Url` struct then add it to self.site.
    ///
    /// If the provided url cannot be parsed by the `reqwest::Url` struct than it cannot be
    /// used by reqwest, so we have to return an error.
    pub fn try_add_site(&mut self, provided_url: &str) -> Result<&str, &str> {
        // Make a new `host_addr` String and fill it with the provided_url argument.
        let mut host_addr = String::new();
        host_addr += provided_url;

        // If the provided url doesn't have `http://` or `https://` prepended to it then
        // we should add it. Most websites like `google.com` *should* still work, but
        // an IP address will *NOT* work. Adding an http(s) scheme will allow an ip address
        // to be successfully parsed. To make it easy on me, I will prepend every provided url
        // with "http://" if a scheme is not provided.
        if !provided_url.starts_with("http://") || !provided_url.starts_with("https://") {
            host_addr.insert_str(0, "http://");
        }

        // Attempt to parse the constructed `host_addr`. If okay, then convert it to a string, assign it to `self.site`, and return Ok.
        if let Ok(val) = Url::parse(&host_addr) {
            self.site = Arc::new(val.as_str().to_string());
            Ok("Successfully parsed given url.")

        // Url could not be parsed. Return an error.
        } else {
            Err("Unable to parse provided url. Please check your `-u`/`--url` argument.")
        }
    }

    /// This method wraps the `extend_from_file` method for the Wordlist struct. The sole argument is a
    /// String that represents an absolute file path. This method then returns a Scanner object with an
    /// initialized wordlist.
    pub fn build_wordlist_from_file(&mut self, path: String) -> Self {
        Scanner {
            wordlist: self.wordlist.extend_from_file(path),
            status_code_ignore_list: self.status_code_ignore_list.clone(),
            found: self.found.clone(),
            site: self.site.clone(),
            extension_list: self.extension_list.clone(),
        }
    }

    /// This method sorts the results by HTTP status code in ascending order. Then the results are formatted, given a color to represent
    /// their response type and then are printed to the screen.
    pub fn display_found(&self) {
        // Convert the BTreeMap into a Vec which can then be easily sorted by value.
        let mut sorted_results = Vec::from_iter(&self.found);

        // sort the vector by status_code in ascending order.
        sorted_results.sort_by(|a, b| a.1.cmp(b.1));

        // Print a nice QOL message to show the results section.
        println!("---------------------------------------------------------");
        println!("Scan Results");
        println!("Site: {}", &self.site);
        println!("Method: GET");
        self.display_ignore_list();
        println!("---------------------------------------------------------");

        // Print all stored, sorted results. Display the resulting line with a different color
        // depending on its associated http status_code.
        for (page, status_code) in sorted_results {
            // Match status_codes based on 'type'
            match status_code {
                // Okay -> Page found codes. Green to indicate success.
                200 => {
                    let output = format!("/{} --> Status: {}", page, status_code).green();
                    println!("{}", output);
                }

                // Redirects. Blue to indicate these are something to look into further.
                300..=399 => {
                    let output = format!("/{} --> Status: {}", page, status_code).blue();
                    println!("{}", output);
                }

                // Client & Server errors. Red to indicate that they are not immediately useful to the user.
                400..=599 => {
                    let output = format!("/{} --> Status: {}", page, status_code).red();
                    println!("{}", output);
                }

                // All other responses. Unlikely to see these codes, but this is a catch all.
                // Yellow indicates caution or that these are 'unusual' codes to receive.
                _ => {
                    let output = format!("/{} --> Status: {}", page, status_code).yellow();
                    println!("{}", output);
                }
            }
        }
    }

    /// This method displays the stored list of HTTP status codes to ignore.
    pub fn display_ignore_list(&self) {
        println!("Ignoring {:?}", self.status_code_ignore_list)
    }

    /// This method adds the contents of to_use into the status_code_ignore_list. The
    /// to_use argument is a borrowed array slice of u16s.
    pub fn add_to_ignore_list(&mut self, to_use: &[u16]) {
        for status in to_use {
            // If the status code already exists in the ignore_list then
            // skip the current loop iteration.
            if self.status_code_ignore_list.contains(status) {
                continue;
            }

            // The status is doesn't currently exist in the status_code_ignore_list
            // so add it.
            self.status_code_ignore_list.push(*status);
        }
    }

    /// Sets status_code_ignore_list to contain 404 which is the 'NOT FOUND' HTTP status code.
    pub fn use_default_ignore_list(&mut self) {
        if !self.status_code_ignore_list.contains(&404) {
            self.status_code_ignore_list.push(404);
        }
    }

    /// Returns true if the 'found_status' argument is in the status_code_ignore_list
    fn should_ignore(&self, found_status: &u16) -> bool {
        self.status_code_ignore_list.contains(found_status)
    }

    /// Adds a page and received status code to the self.found: BTreeMap.
    pub fn add_to_found(&mut self, (page, http_status): (String, u16)) {
        // The http_status is not in our ignore list so add it to self.found
        if !self.should_ignore(&http_status) {
            self.found.insert(page, http_status);
        }
    }

    /// This method takes a file path argument as a str slice. This method returns a result, both of which contain
    /// a string. The file path argument is used to create a new file which then has the contents of self.found written to it.
    pub fn write_results_to_file(self, path: &str) -> Result<String, String> {
        let mut write_error = false;

        // If the file at `path` exists and is successfully opened in append mode.
        if let Ok(mut file) = OpenOptions::new().append(true).open(&path) {
            // Convert the BTreeMap into a Vec which can then be easily sorted by value.
            let mut sorted_results = Vec::from_iter(&self.found);

            // Sort the vector by status_code in ascending order.
            sorted_results.sort_by(|a, b| a.1.cmp(b.1));

            // Write each stored result as a byte string into the file.
            for (path, status) in &sorted_results {
                let byte_str = format!("/{} -> Status: {}\n", path, status);
                match file.write(byte_str.as_bytes()) {
                    // successfully wrote line, so continue to next iteration.
                    Ok(_val) => continue,

                    // error writing line to file. Flip write_error to true
                    // and break loop.
                    Err(_err) => {
                        write_error = true;
                        break;
                    }
                }
            }

            if write_error {
                Err("Couldn't write results to ".to_string() + path)
            } else {
                Ok("Successfully wrote results to ".to_string() + path)
            }
        } else {
            // File doesn't exist, attempt to create a new one at `path`.
            match File::create(&path) {
                // File was successfully created
                Ok(mut file) => {
                    // For each stored result, convert it into a formatted byte string
                    // and write it to the file.
                    for (path, status) in &self.found {
                        let byte_str = format!("/{} -> Status: {}\n", path, status);
                        match file.write(byte_str.as_bytes()) {
                            // successfully wrote line, so continue to next iteration.
                            Ok(_val) => continue,

                            // error writing line to file. Flip write_error to true
                            // and break loop.
                            Err(_err) => {
                                write_error = true;
                                break;
                            }
                        }
                    }
                }
                Err(_err) => return Err(
                    "Could not create output file at provided path. Please check your file path."
                        .to_string(),
                ),
            }
            if write_error {
                Err("Couldn't write results to ".to_string() + path)
            } else {
                Ok("Successfully created and wrote results to ".to_string() + path)
            }
        }
    }

    /// This method either:
    /// 1) displays every stored extension that is being searched for
    /// 2) or displays a message stating that none are searched for.
    pub fn display_extension_list(&self) {
        if self.extension_list.is_empty() {
            println!("Extensions: N/A");
        } else {
            println!("Extensions: {:?}", self.extension_list);
        }
    }

    /// This method takes a string slice as an argument. The extension_args
    /// slice should be a comma separated list of file extensions. They do *not* need to be
    /// prepended with a '.' character.
    pub fn add_extensions_to_wordlist(&mut self, extension_args: &str) {
        // split the string slice at each comma and collect the split words into a vector
        // of string slices.
        let extension_list = extension_args.split(',').collect::<Vec<&str>>();

        for extension in &extension_list {
            // if the word does not have a '.' character then prepend the word
            // with it and add it to self.extension_list.
            if !extension.starts_with('.') {
                self.extension_list.0.push(".".to_string() + extension);
            } else {
                // The word already is prepended with the '.' character
                // so add it to self.extension_list
                self.extension_list.0.push(extension.to_string());
            }
        }

        // For every word in self.wordlist.0 (the web pages to enumerate),
        // clone the word and append the extension to the new word.
        for index in 0..self.wordlist.0.len() {
            for extension in &self.extension_list.0 {
                self.wordlist
                    .0
                    .push(self.wordlist.0[index].clone() + extension);
            }
        }
    }
}
