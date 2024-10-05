use btree::tree::BTree;

#[test]
pub fn insertion() {
    let mut btree = BTree::<u32, u32>::new(4);

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

    println!("{btree:?}");
}
