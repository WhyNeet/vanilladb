use std::io;

use comet::{comet::Comet, document::Document, io::io_config::IoConfig};
use trail::field::Field;

fn main() -> io::Result<()> {
    println!("--- writing data ---");

    let config = IoConfig::builder()
        .data_dir(".comet_data".to_string())
        .build();
    let mut comet = Comet::new(config);
    comet.initialize().unwrap();

    let database = comet.create_database("primary".to_string())?;
    let collection = database.create_collection("users".to_string()).unwrap();

    for i in 0..1000 {
        let mut document = Document::new();
        document.append_field("id".to_string(), Field::uint32(i));
        document.append_field("username".to_string(), Field::string(format!("user {i}")));
        document.append_field(
            "email".to_string(),
            Field::string(format!("user.{i}@example.com")),
        );

        collection.insert_document(&document).unwrap();
    }

    println!("--- done ---");

    Ok(())
}
