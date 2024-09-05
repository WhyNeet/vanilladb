use comet::{
    comet::Comet,
    document::Document,
    io::{direct::direct_io::DirectIO, io_config::IOConfig},
};

fn main() {
    println!("--- writing data ---");
    write_data();

    println!("--- reading data ---");
    read_data();
}

fn write_data() {
    let config = IOConfig::builder()
        .data_dir(".comet_data".to_string())
        .build();
    let mut comet = Comet::new(DirectIO::new(config));
    comet.initialize().unwrap();

    let database = comet.create_database("primary".to_string());
    let collection = database.create_collection("users".to_string());

    for i in 0..1000 {
        collection.insert_document(&Document::new(
            i,
            &format!("user {i}"),
            &format!("user{i}@example.com"),
        ))
    }

    comet.flush().unwrap();
}

fn read_data() {
    let config = IOConfig::builder()
        .data_dir(".comet_data".to_string())
        .build();
    let mut comet = Comet::new(DirectIO::new(config));
    comet.initialize().unwrap();
    comet.load().unwrap();

    let database = comet.database("primary").unwrap();
    let collection = database.collection("users").unwrap();

    let document_id = 1;
    let stored_document = collection.retrieve_document(document_id);
    if let Some(doc) = stored_document {
        println!("Document stored with id: {document_id}");
        doc.display();
    } else {
        println!("No document with id: {document_id}");
    }
}
