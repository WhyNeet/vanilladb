use comet::mem::{Collection, Document};

fn main() {
    let mut collection = Collection::new();

    let document1 = Document::new(0, "whyneet", "example@example.com");
    let document2 = Document::new(1, "test", "test@gmail.com");

    collection.insert_document(&document1);
    collection.insert_document(&document2);

    let document_id = 1;
    let stored_document = collection.retrieve_document(document_id);
    if let Some(doc) = stored_document {
        println!("Document stored with id: {document_id}");
        doc.display();
    } else {
        println!("No document with id: {document_id}");
    }
}
