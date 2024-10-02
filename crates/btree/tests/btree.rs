use btree::tree::BTree;

#[test]
pub fn insertion() {
    let btree = BTree::<u32, u32>::new(4);

    btree.insert((0, 1));
    btree.insert((1, 2));
    btree.insert((3, 4));
    btree.insert((2, 3));

    println!("{btree:?}");
}