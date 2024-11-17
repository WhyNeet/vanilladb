use btree::tree::file::node::FileBTreeNode;
use llio::util::record_id::RecordId;
use trail::{deserialize::Deserialize, serialize::Serialize};

#[test]
pub fn node_serialization_works() {
    let mut node = FileBTreeNode::empty(false);
    // node.append(FileBTreeNodeItem::Key(Rc::new(Field::ubyte(10))));
    // node.append(FileBTreeNodeItem::Pair(
    //     Rc::new(Field::ubyte(11)),
    //     vec![Rc::new(Field::string("value".to_string()))],
    // ));

    node.append(RecordId::new("/hello/world".to_string(), 512));
    node.append(RecordId::new("/hello/there".to_string(), 52));

    let buffer = node.serialize();
    assert!(buffer.is_ok());

    let buffer = buffer.unwrap();
    assert_eq!(
        &buffer[..],
        [
            53, 0, 0, 0, 0, 24, 0, 0, 0, 47, 104, 101, 108, 108, 111, 47, 119, 111, 114, 108, 100,
            0, 2, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 47, 104, 101, 108, 108, 111, 47, 116, 104, 101,
            114, 101, 52, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
pub fn node_deserialization_works() {
    let buffer = [
        53, 0, 0, 0, 0, 24, 0, 0, 0, 47, 104, 101, 108, 108, 111, 47, 119, 111, 114, 108, 100, 0,
        2, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 47, 104, 101, 108, 108, 111, 47, 116, 104, 101, 114, 101,
        52, 0, 0, 0, 0, 0, 0, 0,
    ];
    let node = FileBTreeNode::deserialize(&buffer);
    assert!(node.is_ok());

    let node = node.unwrap();
    assert_eq!(node.is_internal(), false);

    assert!(node.get(0).is_some());
    assert_eq!(node.get(0).unwrap().path(), "/hello/world");
    assert_eq!(node.get(0).unwrap().offset(), 512);

    assert!(node.get(1).is_some());
    assert_eq!(node.get(1).unwrap().path(), "/hello/there");
    assert_eq!(node.get(1).unwrap().offset(), 52);
}
