use std::collections::LinkedList;

fn main() {
   let mut list = LinkedList::new();
   list.push_back(1);
   list.push_back(3);
   list.push_back(6);
   list.push_back(2);

   println!("List {:?}", list);
}