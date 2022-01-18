use kwindex::KWIndex;

#[test]
fn test_hello_world() {
    let index = KWIndex::new().extend_from_text("Hello world");
    assert_eq!(2, index.len());
    assert_eq!(1, index.count_matches("world"));
}

#[test]
fn test_example_sentence() {
    let index = KWIndex::new().extend_from_text("It ain't over until it ain't, over.");
    assert_eq!(5, index.len());
}

#[test]
fn test_word_count() {
    let index = KWIndex::new().extend_from_text("It ain't over until it ain't, over.");
    assert_eq!(2, index.count_matches("over"));
}

#[test]
fn test_example1() {
    let kwindex = KWIndex::new().extend_from_text("I am THE WALRUS");
    assert_eq!(Some("THE"), kwindex.nth_uppercase(1));
}

#[test]
fn test_example2() {
    let kwindex = KWIndex::new().extend_from_text("b b b-banana b");
    assert_eq!(3, kwindex.count_matches("b"));
}

#[test]
fn test_example3() {
    let kwindex = KWIndex::new().extend_from_text("Can't stop this!");
    assert_eq!(2, kwindex.len());
}
