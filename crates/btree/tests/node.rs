use std::{ptr, rc::Rc};

use btree::tree::file::{item::FileBTreeNodeItem, node::FileBTreeNode};
use trail::{deserialize::Deserialize, field::Field, serialize::Serialize};

#[test]
pub fn node_serialization_works() {
    let mut node = FileBTreeNode::empty(false, None);
    node.append(FileBTreeNodeItem::Key(Rc::new(Field::ubyte(10))));
    node.append(FileBTreeNodeItem::Pair(
        Rc::new(Field::ubyte(11)),
        vec![Rc::new(Field::string("value".to_string()))],
    ));

    let buffer = node.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        &buffer[..],
        [
            33, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 10, 1, 20, 0, 0, 0, 2, 1, 0, 0, 0, 11, 0, 5, 0, 0, 0,
            118, 97, 108, 117, 101,
        ]
    );
}

#[test]
pub fn node_deserialization_works() {
    let buffer = [
        33, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 10, 1, 20, 0, 0, 0, 2, 1, 0, 0, 0, 11, 0, 5, 0, 0, 0,
        118, 97, 108, 117, 101,
    ];
    let node = FileBTreeNode::deserialize(&buffer);
    assert!(node.is_ok());

    let node = node.unwrap();
    assert_eq!(node.is_internal(), false);
    assert!(node.items()[0].is_key());
    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(node.items()[0].as_key().value() as *const Box<_>))
                as *const u8)
                .as_ref()
                .unwrap()
        },
        &10
    );

    assert!(node.items()[1].is_pair());
    assert_eq!(
        unsafe {
            (Box::into_raw(ptr::read(
                node.items()[1].as_pair().0.value() as *const Box<_>
            )) as *const u8)
                .as_ref()
                .unwrap()
        },
        &11
    );
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
