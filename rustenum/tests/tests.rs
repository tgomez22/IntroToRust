use rustenum::Scanner;
use std::collections::BTreeMap;

#[test]
fn check_ignore_capabilities() {
    let mut results = Scanner::new();

    let ignore: Vec<u16> = [404, 301, 200].to_vec();

    results.add_to_ignore_list(&ignore);

    assert_eq!(results.status_code_ignore_list, ignore);
}

#[test]
fn check_default_ignore() {
    let mut results = Scanner::new();
    let ignore: Vec<u16> = [404].to_vec();
    results.use_default_ignore_list();
    assert_eq!(results.status_code_ignore_list, ignore)
}

#[test]
fn check_found_results() {
    let mut results = Scanner::new();

    let mut test_data: BTreeMap<String, u16> = BTreeMap::new();
    test_data.insert("wp-admin".to_string(), 403);
    test_data.insert("wp-login.php".to_string(), 200);
    test_data.insert("resources".to_string(), 301);
    test_data.insert("robots.txt".to_string(), 200);

    results.add_to_found(("wp-admin".to_string(), 403u16));
    results.add_to_found(("wp-login.php".to_string(), 200u16));
    results.add_to_found(("resources".to_string(), 301u16));
    results.add_to_found(("robots.txt".to_string(), 200u16));
    assert_eq!(results.found, test_data);
}

#[test]
fn validate_adding_extensions() {
    let mut scanner = Scanner::new();
    let extensions = ".php,html,.js".to_string();

    scanner.add_extensions_to_wordlist(&extensions);

    assert!(scanner.extension_list.0.contains(&".php".to_string()));
    assert!(scanner.extension_list.0.contains(&".html".to_string()));
    assert!(scanner.extension_list.0.contains(&".js".to_string()));

    assert!(!scanner.wordlist.0.contains(&".htaccess..php".to_string()));
}
