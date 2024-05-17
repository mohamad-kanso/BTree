use std::fmt;

#[derive(Clone,Debug)]
#[derive(PartialEq)]
pub struct Node{
   keys: Vec<i32>,
   child: Vec<Node>,
   leaf: bool,
}

impl Node{
   pub fn new(leaf:bool) -> Self{
      return Node{keys: vec![],child:vec![],leaf}
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

#[derive(Clone, Debug)]
pub struct Tree{
   pub root: Node,
   t: usize
}
impl Tree{
   pub fn init(t:usize) -> Self{
      return Tree{root: Node::new(true),t}
   }

   pub fn print_btree(&self,node: &Node, level: usize) {
      println!("{}{}", "  ".repeat(level), node);
      for child in &node.child {
         self.print_btree(child, level + 1);
      }
   }

   pub fn search<'a>(&'a self, node:&'a Node, s:i32) -> Option<(usize,&Node)>{
      let mut i = 0;
      while i < node.keys.len() && s > node.keys[i]{
         i = i+1;
      }
      if i < node.keys.len() && s == node.keys[i]{
         return Some((i,node))
      }
      else if node.leaf{return None}
      else {return self.search(&node.child[i],s);}
   }

   fn split<'a>(&'a self, mut x:Node,i:usize) -> Node{
      let t = self.t;
      let mut z = Node::new(x.child[i].leaf);
      x.keys.insert(i, x.child[i].keys[t-1]);
      z.keys = x.child[i].keys[t..2*t-1].to_vec();
      x.child[i].keys = x.child[i].keys[0..t-1].to_vec();
      if !x.child[i].leaf{
         let mut j:usize = 0;
         while j < t {
               z.child.insert(j, x.child[i].child[j+t].clone());
               j+=1
         }
         while j>0{
               x.child[i].child.pop();
               j-=1;
         }
      }
      x.child.insert(i+1,z);
      // println!("node split\n{:?}",x);
      x
   }

   pub fn insert(mut self,k:i32) -> Self{
      if self.root.keys.is_empty(){
         self.root.keys.push(k); self
      } else {
      let t = self.t;
      if self.root.keys.len() == (2*t - 1){
         let mut new_root = Node::new(false);
         new_root.child.insert(0, self.root);
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
               if x.child[i].keys.len() == (2*t-1){
                  x = self.split(x, i);
                  if k > x.keys[i]{
                     i+=1
                  }
               }
               x.child[i] = self.insert_non_full(x.child[i].clone(), k);
               return x;        
            }
            i -= 1;
         }
         i += 1;
         if x.child[i].keys.len() == (2*t-1){
            x = self.split(x, i);
            if k > x.keys[i]{i+=1}
         }
         x.child[i] = self.insert_non_full(x.child[i].clone(), k);
         x
      }
   }

   pub fn delete (&mut self, x:&mut Node, k:i32) -> Node{
      let t: usize = self.t;
      let mut i: usize = 0;
      while i < x.keys.len() && k > x.keys[i] {
         i+=1;
      }
      if x.leaf{
         if i < x.keys.len() && x.keys[i] == k {
            x.keys.remove(i);
         }
         return x.to_owned();
      }

      if i < x.keys.len() && x.keys[i] == k {
         self.delete_internal_node(x,k,i);
         return x.to_owned();
      }
      else if x.child[i].keys.len() >= t {
         x.child[i] = self.delete(&mut x.child[i].to_owned(), k);
         return x.to_owned();
      }
      else {
         if i!=0 && i+2 < x.child.len(){
            if x.child[i-1].keys.len() >= t {self.delete_sibling(x,i,i-1)}
            else if x.child[i+1].keys.len() >=t {self.delete_sibling(x,i,i+1)}
            else {self.delete_merge (x,i,i+1)}
         }
         else if i == 0{
            if x.child[i+1].keys.len() >= t {self.delete_sibling(x, i, i+1);}
            else {println!("here");self.delete_merge(x, i, i+1);}
         }
         else if i+1 == x.child.len(){
            if x.child[i-1].keys.len() >= t {self.delete_sibling(x, i, i-1)}
            else {self.delete_merge(x, i, i-1);}
         }
      }
      x.child[i] = self.delete(&mut x.child[i], k); 
      return x.to_owned();
   }

   fn delete_internal_node(&mut self,x:&mut Node, k:i32,i:usize){
      let t = self.t;
      if x.leaf{
         if x.keys[i] == k {
            x.keys.remove(i);
            return;
         }
         return;
      }
      if x.child[i].keys.len() >= t {
         x.keys[i] = self.clone().delete_predecessor(&mut x.child[i]);
         return
      }
      else if x.child[i+1].keys.len() >= t {
         x.keys[i] = self.clone().delete_successor(&mut x.child[i]);
         return
      }
      else{
         self.delete_merge(x, i, i+1);
         self.delete_internal_node(&mut x.child[i], k, t-1);
      }
   }
   fn delete_predecessor(mut self, x:&mut Node) -> i32{
      if x.leaf {return x.keys.pop().unwrap();}
      let n = x.keys.len() - 1;
      if x.child[n].keys.len() >= self.t {
         self.clone().delete_sibling(x, n+1, n);
      }
      else{
         self.delete_merge(x, n, n+1);
      }
      self.delete_predecessor(&mut x.child[n])
   }

   fn delete_successor(mut self, x:&mut Node) -> i32{
      if x.leaf {
         return x.keys.remove(0);
      }
      if x.child[1].keys.len() >= self.t{self.clone().delete_sibling(x, 0, 1)}
      else {self.delete_merge(x, 0, 1)}
      self.delete_successor(&mut x.child[0])
   }

   fn delete_sibling(&self, x:&mut Node,i:usize,j:usize){
      if i < j {
         x.child[i].keys.push(x.keys[i]);
         x.keys[i] = x.child[j].keys[0];
         if x.child[j].child.len() > 0 {
            let a = x.child[j].child[0].clone();
            x.child[i].child.push(a);
            x.child[j].child.remove(0);
         }
         x.child[j].keys.remove(0);
      }
      else {
         x.child[i].keys.insert(0, x.keys[i-1]);
         x.keys[i-1] = x.child[j].keys.pop().unwrap();
         if x.child[j].child.len() > 0 {
            let a = x.child[j].child.pop();
            x.child[i].child.insert(0, a.unwrap());
         }
      }
   }

   fn delete_merge <'a>(&'a mut self, x:&mut Node, i:usize, j:usize){
      if j > i{
         x.child[i].keys.push(x.keys[i]);
         for k in 0..x.child[j].keys.len(){
            let a = x.child[j].keys[k];
            x.child[i].keys.push(a);
            if x.child[j].child.len() > 0 {
               let a = x.child[j].child[k].clone();
               x.child[i].child.push(a);
            }
         }
         if x.child[j].child.len() > 0 {
            let a = x.child[j].child.pop().unwrap();
            x.child[i].child.push(a);
         }
         x.keys.remove(i);
         x.child.remove(j);
         *x = x.child[i].to_owned();
      }
      else {
         x.child[j].keys.push(x.keys[j]);
         for k in 0..x.child[i].keys.len(){
            let a = x.child[i].keys[k];
            x.child[j].keys.push(a);
            if x.child[j].child.len() > 0 {
               let a = x.child[i].child[k].clone();
               x.child[j].child.push(a);
            }
         }
         if x.child[j].child.len() > 0 { 
            let a = x.child[i].child.pop();
            x.child[j].child.push(a.unwrap());
         }
         x.keys.remove(j);
         x.child.remove(i);
         *x = x.child[j].to_owned();
      }
      if x == &mut self.root && x.keys.len() == 0 {
         self.root = x.child[i].clone();
      }
   }

}