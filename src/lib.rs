mod errors;

use std::mem;
pub struct List{
    head:Link,
}
enum Link{
    Empty,
    More(Box<Node>),
}
struct Node{
    elem:i32,
    next:Link,
}
// box is a smart pointer for dynamic allocation on the heap 
//impl block allows for the implementation of types such as struct or enums
impl List{
    pub fn new () -> Self{
        List{head: Link::Empty}
    }

    pub fn push(&mut self, elem:i32) {
        let new_node =Box::new(Node{
            elem:elem,
            next:mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }
    
    pub fn pop(&mut self) -> Option<i32>{
        let result;
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty  => {
                result = None; 
            }
            Link::More(Node) => {
                result = Some(Node.elem);
                self.head = Node.next;
            }
        };
        result
    }
}

//to test a function you annotate it with #[test]

mod test{
    use super::List;
    #[test]
    fn basic_test() {
        let mut list = List::new(); //creating a new list and making it mutable
        assert_eq! (list.pop() , None);
        list.push(10);
        list.push(20);
        list.push(30);
        list.push(40);
        list.push(50);

        assert_eq!(list.pop(), Some(50)); //testing the pop function
        assert_eq!(list.pop() , Some(40));

        list.push(60);
        list.push(70);

        assert_eq!(list.pop(), Some(70)); //testing the pop function
        assert_eq!(list.pop() , Some(60));

        assert_eq!(list.pop(), Some(30)); //testing the pop function
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop() , None);
    }
}