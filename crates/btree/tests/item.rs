use std::rc::Rc;

use btree::tree::file::item::FileBTreeNodeItem;
use llio::util::record_id::RecordId;
use trail::{field::Field, serialize::Serialize};

#[test]
pub fn item_key_serialization_works() {
    let item: FileBTreeNodeItem<String> = FileBTreeNodeItem::Key("username".to_string());

    let buffer = item.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        &buffer[..],
        [0, 8, 0, 0, 0, 117, 115, 101, 114, 110, 97, 109, 101]
    );
}

#[test]
pub fn item_pair_serialization_works() {
    let item: FileBTreeNodeItem<String> = FileBTreeNodeItem::Pair(
        "cities".to_string(),
        vec![
            Rc::new(Field::string("NY".to_string())),
            Rc::new(Field::string("LA".to_string())),
        ],
    );

    let buffer = item.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        &buffer[..],
        [
            1, 21, 0, 0, 0, 99, 105, 116, 105, 101, 115, 0, 14, 0, 0, 0, 0, 2, 0, 0, 0, 78, 89, 0,
            2, 0, 0, 0, 76, 65
        ]
    )
}

#[test]
pub fn item_pointer_serialization_works() {
    let item: FileBTreeNodeItem<String> =
        FileBTreeNodeItem::Pointer(RecordId::new("/hello/world".to_string(), 512));

    let buffer = item.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        &buffer[..],
        [
            2, 22, 0, 0, 0, 12, 0, 47, 104, 101, 108, 108, 111, 47, 119, 111, 114, 108, 100, 0, 2,
            0, 0, 0, 0, 0, 0
        ]
    );
}
