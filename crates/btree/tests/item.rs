use std::{ptr, rc::Rc};

use btree::tree::file::item::FileBTreeNodeItem;
use llio::util::record_id::RecordId;
use trail::{deserialize::Deserialize, field::Field, serialize::Serialize};

#[test]
pub fn item_key_serialization_works() {
    let item = FileBTreeNodeItem::Key(Rc::new(Field::string("username".to_string())));

    let buffer = item.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        &buffer[..],
        [0, 0, 8, 0, 0, 0, 117, 115, 101, 114, 110, 97, 109, 101]
    );
}

#[test]
pub fn item_pair_serialization_works() {
    let item = FileBTreeNodeItem::Pair(
        Rc::new(Field::string("cities".to_string())),
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
            1, 29, 0, 0, 0, 0, 6, 0, 0, 0, 99, 105, 116, 105, 101, 115, 0, 2, 0, 0, 0, 78, 89, 0,
            2, 0, 0, 0, 76, 65
        ]
    );
}

#[test]
pub fn item_pointer_serialization_works() {
    let item = FileBTreeNodeItem::Pointer(RecordId::new("/hello/world".to_string(), 512));

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

#[test]
pub fn item_key_deserialization_works() {
    let buffer = [0, 0, 8, 0, 0, 0, 117, 115, 101, 114, 110, 97, 109, 101];
    let item = FileBTreeNodeItem::deserialize(&buffer);
    assert!(item.is_ok());

    let item = item.unwrap();
    assert!(item.is_key());
    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(item.as_key().value() as *const Box<_>)) as *const String)
                .as_ref()
                .unwrap()
        },
        "username"
    );
}

#[test]
pub fn item_pair_deserialization_works() {
    let buffer = [
        1, 29, 0, 0, 0, 0, 6, 0, 0, 0, 99, 105, 116, 105, 101, 115, 0, 2, 0, 0, 0, 78, 89, 0, 2, 0,
        0, 0, 76, 65,
    ];
    let item = FileBTreeNodeItem::deserialize(&buffer);
    assert!(item.is_ok());

    let item = item.unwrap();
    assert!(item.is_pair());
    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(item.as_pair().0.value() as *const Box<_>)) as *const String)
                .as_ref()
                .unwrap()
        },
        "cities"
    );
    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(item.as_pair().1[0].value() as *const Box<_>))
                as *const String)
                .as_ref()
                .unwrap()
        },
        "NY"
    );
    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(item.as_pair().1[1].value() as *const Box<_>))
                as *const String)
                .as_ref()
                .unwrap()
        },
        "LA"
    );
}

#[test]
pub fn item_pointer_deserialization_works() {
    let buffer = [
        2, 22, 0, 0, 0, 12, 0, 47, 104, 101, 108, 108, 111, 47, 119, 111, 114, 108, 100, 0, 2, 0,
        0, 0, 0, 0, 0,
    ];
    let item = FileBTreeNodeItem::deserialize(&buffer);
    assert!(item.is_ok());

    let item = item.unwrap();
    assert!(item.is_pointer());
    assert_eq!(item.as_pointer().path(), "/hello/world");
    assert_eq!(item.as_pointer().offset(), 512);
}
