mod linked_list;

use linked_list::linked_list::Node;

fn main() {
    let mut a = Node{ data: 10, next: None };
    let b = Node{ data: 15, next: Some(&a) };

    println!("The value of a is: {}", b.next.unwrap().data);

    a.data = 4;

}