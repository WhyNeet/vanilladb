use llio::util::record_id::RecordId;
use trail::{deserialize::Deserialize, serialize::Serialize};

#[test]
pub fn record_id_serialization_works() {
    let record_id = RecordId::new("/serialization/test".to_string(), 1024);

    let buffer = record_id.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        &buffer[..],
        [
            31, 0, 0, 0, 47, 115, 101, 114, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 47,
            116, 101, 115, 116, 0, 4, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
pub fn record_id_deserialization_works() {
    let buffer = [
        31, 0, 0, 0, 47, 115, 101, 114, 105, 97, 108, 105, 122, 97, 116, 105, 111, 110, 47, 116,
        101, 115, 116, 0, 4, 0, 0, 0, 0, 0, 0,
    ];

    let record_id = RecordId::deserialize(&buffer);
    // assert!(record_id.is_ok());

    let record_id = record_id.unwrap();
    assert_eq!(record_id.path(), "/serialization/test");
    assert_eq!(record_id.offset(), 1024);
}
