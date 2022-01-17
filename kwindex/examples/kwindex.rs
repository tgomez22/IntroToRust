fn main() {
    let index = kwindex::KWIndex::new().extend_from_text("It ain't over until it ain't over.");
    println!("{}", index.len());
    index.display_stored_words();
}
