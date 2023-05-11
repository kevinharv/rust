
enum List {
    Cons(i32, Box<List>),
    Null
}

impl List {
    fn new() -> List {
        List::Null
    }

    fn add(self, x: i32) -> List {
        List::Cons(x, Box::new(self))
    }

    fn stringify(&self) -> String {
        match self {
            List::Cons(data, next) => {
                format!("Data: {}\n{}", data, next.stringify())
            },

            List::Null => format!("N/A\n")
        }
    }
}



enum Node {
    Cons(f64, Box<Node>, Box<Node>),
    Null
}

impl Node {
    // Create root with null data type
    fn new() -> Node {
        Node::Null
    }

    // Add data as new node, return root
    fn add(&mut self, x: f64) {
        match self {
            Node::Cons(data, left, right) => {
                if &x <= &data {
                    // Insert on left
                    left.add(x);
                }
                else {
                    // Insert on right
                    right.add(x);
                }
            },
            Node::Null => {
                *self = Node::Cons(x, Box::new(Node::Null), Box::new(Node::Null));
            }
        }
        // self
    }

    fn stringify(&self) -> String {
        match self {
            Node::Cons(data, left, right) => {
                format!("Data: {}\n{}{}", data, left.stringify(), right.stringify())
            }
            Node::Null => format!("")
        }
        
    }
}

fn main() {
    let mut l = List::new();
    l = l.add(13);
    l = l.add(26);
    l = l.add(44);

    print!("Linked List:\n{}\n", l.stringify());

    let mut n = Node::new();
    n.add(5f64);
    n.add(4f64);
    n.add(6f64);
    n.add(16f64);
    n.add(44f64);
    n.add(-1f64);

    println!("Binary Tree:\n{}\n", n.stringify());

}