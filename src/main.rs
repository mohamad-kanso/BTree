#[derive(Clone,Debug)]
struct Node{
    keys: Vec<i32>,
    children: Vec<Node>,
    leaf: bool,
}

impl Node{
    fn new(leaf:bool) -> Self{
        return Node{keys: vec![],children:vec![],leaf:leaf}
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

    fn print_tree(&self, root:&Node, level: i32){
        println!("Level: {level}",);
        for k in &root.keys{
            print!("{k} \t");
        }
        println!();
        let level=level + 1;
        if root.children.len() > 0{
            for i in &root.children{
                self.print_tree(&i, level)
            }
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
                z.children[j] = x.children[i].children[j+t].clone();
                j+=1
            }
        }
        x.children.insert(i+1,z);
        // println!("node split\n{:?}",x);
        x
    }

    fn insert(mut self,k:i32) -> Self{
        if self.root.keys.is_empty(){
            self.root.keys.push(k); return self;
        } else {
        let root = self.root.clone();
        let t = self.t;
        if root.keys.len() == (2*t - 1){
            let mut new_root = Node::new(false);
            new_root.children.insert(0, root);
            // println!("{:?}",new_root);
            self.root = new_root.clone();
            self.root = self.split(new_root, 0);
            // println!("after split\n{:?}",self);
            self = self.clone().insert_non_full(self.root,k);
            return self;
        }
        else {
            self = self.clone().insert_non_full(self.root,k);
            return self;
        }
    }
    }

    fn insert_non_full(&mut self, x:Node,k:i32) -> Self{
        let t = self.t;
        let mut i = x.keys.len()-1;
        if x.leaf {
            self.root.keys.append(&mut vec![0]);
            while i>=usize::MIN && k < self.root.keys[i]{
                if i+1 != self.root.keys.len(){self.root.keys[i+1]= x.keys[i]}
                else {self.root.keys.insert(i+1, self.root.keys[i])}
                if i==0{
                    self.root.keys[i] = k;
                    return self.clone(); 
                }
                i -=1;
            }
            if i+1 != self.root.keys.len(){self.root.keys[i+1]= k}
                else {self.root.keys.insert(i+1, k)}
            self.clone()
        }
        else {
            while i >=usize::MIN && k < x.keys[i]{
                i -= 1;
            }
            i += 1;
            // if full
            if x.children[i].keys.len() == (2*t-1){
                self.split(x.clone(), i);
                if k > x.keys[i]{
                    i+=1
                }
            }
            self.insert_non_full(x.children[i].clone(), k);
            self.clone()
        }
    }

}

fn main() {
    let tree = Tree::init(2);
    let tree = tree.insert(5);
    let tree = tree.insert(9);
    let tree = tree.insert(3);
    let tree = tree.insert(7);
    // let tree = tree.insert(60);
    // let tree = tree.insert(25);
    
    tree.print_tree(&tree.root, 0);
    match tree.search(1, &tree.root){
        Some(_) => println!("key is found"),
        None => println!("key is not found")
    }
}