use btree::Tree;

fn main() {
    let tree = Tree::init(2);

    let tree = tree.insert(5);
    let tree = tree.insert(9);
    let tree = tree.insert(3);
    let tree = tree.insert(7);
    let tree = tree.insert(1);
    let tree = tree.insert(2);
    let tree = tree.insert(8);
    let tree = tree.insert(6);
    let tree = tree.insert(0);
    let tree = tree.insert(4);
    
    tree.print_btree(&tree.root, 0);

    match tree.search(6, &tree.root){
        Some(_) => println!("key is found"),
        None => println!("key is not found")
    }
}