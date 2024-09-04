use comet::mem::{Comet, Document};

fn main() {
    println!("--- writing data ---");
    write_data();

    println!("--- reading data ---");
    read_data();
}

fn write_data() {
    let mut comet = Comet::new(".comet_data".to_string());
    comet.initialize().unwrap();

    let database = comet.create_database("primary".to_string());
    let collection = database.create_collection("users".to_string());

    for i in 0..10000 {
        collection.insert_document(&Document::new(
            i,
            &format!("user {i}"),
            &format!("user{i}@example.com"),
        ))
    }

    comet.flush().unwrap();
}

fn read_data() {
    let mut comet = Comet::new(".comet_data".to_string());
    comet.initialize().unwrap();
    comet.load().unwrap();

    let database = comet.database("primary").unwrap();
    let collection = database.collection("users").unwrap();

    let document_id = 100;
    let stored_document = collection.retrieve_document(document_id);
    if let Some(doc) = stored_document {
        println!("Document stored with id: {document_id}");
        doc.display();
    } else {
        println!("No document with id: {document_id}");
    }
}
