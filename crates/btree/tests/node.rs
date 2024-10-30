use std::{ptr, rc::Rc};

use btree::tree::file::{item::FileBTreeNodeItem, node::FileBTreeNode};
use trail::{deserialize::Deserialize, field::Field, serialize::Serialize};

#[test]
pub fn node_serialization_works() {
    let mut node = FileBTreeNode::<u32, String>::empty(false);
    node.append(FileBTreeNodeItem::Key(10));
    node.append(FileBTreeNodeItem::Pair(
        11,
        vec![Rc::new(Field::string("value".to_string()))],
    ));

    let buffer = node.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        &buffer[..],
        [
            37, 0, 0, 0, 0, 0, 4, 0, 0, 0, 10, 0, 0, 0, 1, 18, 0, 0, 0, 4, 0, 0, 0, 11, 0, 0, 0, 0,
            5, 0, 0, 0, 118, 97, 108, 117, 101
        ]
    );
}

#[test]
pub fn node_deserialization_works() {
    let buffer = [
        37, 0, 0, 0, 0, 0, 4, 0, 0, 0, 10, 0, 0, 0, 1, 18, 0, 0, 0, 4, 0, 0, 0, 11, 0, 0, 0, 0, 5,
        0, 0, 0, 118, 97, 108, 117, 101,
    ];
    let node = FileBTreeNode::<u32, String>::deserialize(&buffer);
    assert!(node.is_ok());

    let node = node.unwrap();
    assert_eq!(node.is_internal(), false);

    assert!(node.items()[0].is_key());
    assert_eq!(node.items()[0].as_key(), &10);

    assert!(node.items()[1].is_pair());
    assert_eq!(node.items()[1].as_pair().0, &11);
    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(
                node.items()[1].as_pair().1[0].value() as *const Box<_>
            )) as *const String)
                .as_ref()
                .unwrap()
        },
        "value"
    );
}
