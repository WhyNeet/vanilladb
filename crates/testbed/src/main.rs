use comet::mem::{Collection, Document};

fn main() {
    let mut collection = Collection::new();

    let document = Document::new(0, "whyneet", "example@example.com");

    let slot = collection.create_document_slot();
    collection.num_documents += 1;

    document.serialize(unsafe { slot.as_mut().unwrap() });
}
