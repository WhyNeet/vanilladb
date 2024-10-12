use btree::tree::BTree;

#[test]
pub fn insertion() {
    let mut btree = BTree::<u32, u32>::new(4, false);

    btree.insert((0, 1));
    btree.insert((1, 2));
    btree.insert((3, 4));
    btree.insert((2, 3));
    btree.insert((7, 8));
    btree.insert((5, 6));

    btree.insert((6, 7));
    btree.insert((8, 9));
    btree.insert((9, 10));
    btree.insert((10, 11));

    btree.insert((3, 5));

    println!("{btree:?}");
}

#[test]
pub fn retrieval() {
    let mut btree = BTree::<u32, u32>::new(4, false);

    btree.insert((0, 1));
    btree.insert((1, 2));
    btree.insert((3, 4));
    btree.insert((2, 3));
    btree.insert((7, 8));
    btree.insert((5, 6));

    btree.insert((6, 7));
    btree.insert((8, 9));
    btree.insert((9, 10));
    btree.insert((10, 11));

    btree.insert((3, 5));

    let key_5 = btree.get(&5);

    assert!(key_5.is_some());

    let key_5 = key_5.unwrap();

    assert_eq!(key_5.len(), 1);
    assert_eq!(*key_5[0], 6);

    let key_3 = btree.get(&3);

    assert!(key_3.is_some());

    let key_3 = key_3.unwrap();

    assert_eq!(key_3.len(), 2);
    assert_eq!(*key_3[0], 4);
    assert_eq!(*key_3[1], 5);
}
