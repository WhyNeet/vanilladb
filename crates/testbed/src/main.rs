use comet::mem::{CollectionEntity, TOTAL_DOCUMENT_SIZE};

fn main() {
    let entity = CollectionEntity::new(0, "whyneet", "example@example.com");

    println!("the original entity:");
    entity.display();

    let mut output_buffer = [0u8; TOTAL_DOCUMENT_SIZE];
    entity.serialize(&mut output_buffer);

    let mut new_entity = CollectionEntity {
        id: 0,
        username: [0u8; 32],
        email: [0u8; 255],
    };

    new_entity.deserialize(&output_buffer);

    println!("the deserialized entity:");

    new_entity.display();
}
