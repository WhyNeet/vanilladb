use comet::mem::{Database, Document};

fn main() {
    let mut database = Database::new("primary".to_string());

    let collection = database.create_collection("users".to_string());

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
