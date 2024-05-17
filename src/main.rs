use btree::Tree;

fn main() {
    let tree = Tree::init(3);

    let tree = tree.insert(1);
    let tree = tree.insert(9);
    let tree = tree.insert(17);
    let tree = tree.insert(19);
    let tree = tree.insert(21);
    let tree = tree.insert(23);
    let tree = tree.insert(25);
    let tree = tree.insert(27);
    let tree = tree.insert(31);
    let tree = tree.insert(32);
    let tree = tree.insert(39);
    let tree = tree.insert(41);
    let tree = tree.insert(47);
    let tree = tree.insert(50);
    let tree = tree.insert(72);
    let tree = tree.insert(90);
    let tree = tree.insert(56);
    let tree = tree.insert(60);
    let tree = tree.insert(15);
    let tree = tree.insert(22);
    let tree = tree.insert(30);
    let tree = tree.insert(55);
    let tree = tree.insert(63);
    let tree = tree.insert(40);
    let tree = tree.insert(69);
    let mut tree = tree.insert(70);
    
    tree.print_btree(&tree.root, 0);

    tree.root = tree.delete(&mut tree.to_owned().root, 56);
    tree.print_btree(&tree.root, 0);    
    match tree.search( &tree.root,60){
        Some(a) => println!("key is at index {} in {}",a.0,a.1),
        None => println!("key is not found")
    }
}