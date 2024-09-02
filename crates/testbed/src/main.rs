use comet::mem::{TableEntity, TOTAL_DOCUMENT_SIZE};

fn main() {
    let entity = TableEntity::new(0, "whyneet", "example@example.com");

    let mut output_buffer = [0u8; TOTAL_DOCUMENT_SIZE];
    entity.serialize(&mut output_buffer);

    println!("output: {:?}", output_buffer);
}
