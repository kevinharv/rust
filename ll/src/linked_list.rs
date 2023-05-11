pub mod linked_list {
    
    pub struct Node<'node_lt> {
        pub data: i32,
        pub next: Option<&'node_lt Node<'node_lt>>
    }
}