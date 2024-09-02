use comet::mem::{Collection, Document};

fn main() {
    let mut collection = Collection::new();

    let document = Document::new(0, "whyneet", "example@example.com");

    collection.insert_document(&document);
}
