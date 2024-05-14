use std::fmt;

#[derive(Clone,Debug)]
struct Node{
    keys: Vec<i32>,
    children: Vec<Node>,
    leaf: bool,
}

impl Node{
    fn new(leaf:bool) -> Self{
        return Node{keys: vec![],children:vec![],leaf}
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, key) in self.keys.iter().enumerate() {
            write!(f, "{}", key)?;
            if i < self.keys.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}   

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Tree{
    root: Node,
    t: usize
}
impl Tree{
    fn init(t:usize) -> Self{
        return Tree{root: Node::new(true),t:t}
    }

    fn print_btree(&self,node: &Node, level: usize) {
        println!("{}{}", "  ".repeat(level), node);
        for child in &node.children {
            self.print_btree(child, level + 1);
        }
    }

    fn search<'a>(&'a self, s:i32,node:&'a Node) -> Option<(usize,&Node)>{
        let mut i = 0;
        while i < node.keys.len() && s > node.keys[i]{
            i = i+1;
        }
        if i < node.keys.len() && s == node.keys[i]{
            return Some((i,node))
        }
        else if node.leaf{return None}
        else {return self.search(s,&node.children[i]);}
    }

    fn split<'a>(&'a self, mut x:Node,i:usize) -> Node{
        let t = self.t;
        let mut z = Node::new(x.children[i].leaf);
        x.keys.insert(i, x.children[i].keys[t-1]);
        z.keys = x.children[i].keys[t..2*t-1].to_vec();
        x.children[i].keys = x.children[i].keys[0..t-1].to_vec();
        if !x.children[i].leaf{
            let mut j:usize = 0;
            while j < t {
                z.children.insert(j, x.children[i].children[j+t].clone());
                j+=1
            }
            while j>0{
                x.children[i].children.pop();
                j-=1;
            }
        }
        x.children.insert(i+1,z);
        // println!("node split\n{:?}",x);
        x
    }

    fn insert(mut self,k:i32) -> Self{
        if self.root.keys.is_empty(){
            self.root.keys.push(k); self
        } else {
        let root = self.root.clone();
        let t = self.t;
        if root.keys.len() == (2*t - 1){
            let mut new_root = Node::new(false);
            new_root.children.insert(0, root);
            self.root = new_root.clone();
            self.root = self.split(new_root, 0);
            // println!("after split\n{:?}",self);
            self.root = self.clone().insert_non_full(self.root,k);
            return self;
        }
        else {
            self.root = self.clone().insert_non_full(self.root,k);
            return self;
        }
    }
    }

    fn insert_non_full(&mut self, mut x:Node,k:i32) -> Node{
        let t = self.t;
        let mut i = x.keys.len()-1;
        if x.leaf {
            x.keys.append(&mut vec![0]);
            // println!("{:?}",x.keys);
            while i>=usize::MIN && k < x.keys[i]{
                x.keys[i+1]= x.keys[i];
                if i == 0 {
                    x.keys[i] = k;
                    return x
                }
                i -=1;
            }
            x.keys[i+1] = k;
            x
        }
        else {
            while i >=usize::MIN && k < x.keys[i]{
                if i == 0 {
                    if x.children[i].keys.len() == (2*t-1){
                        x = self.split(x, i);
                        if k > x.keys[i]{
                            i+=1
                        }
                    }
                    x.children[i] = self.insert_non_full(x.children[i].clone(), k);
                    return x;        
                }
                i -= 1;
            }
            i += 1;
            // if full
            if x.children[i].keys.len() == (2*t-1){
                x = self.split(x, i);
                if k > x.keys[i]{
                    i+=1
                }
            }
            x.children[i] = self.insert_non_full(x.children[i].clone(), k);
            x
        }
    }

}

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