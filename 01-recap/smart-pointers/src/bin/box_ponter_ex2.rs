#[derive(Debug)]
enum LinkedList {
    Node(i32, Box<LinkedList>), // Node contains data and a pointer to the next node
    Empty,                      // Represents the end of the list
}

impl LinkedList {
    // Add a new node to the front of the list
    fn add_front(value: i32, list: LinkedList) -> LinkedList {
        LinkedList::Node(value, Box::new(list))
    }

    // Traverse the list and print its elements
    fn traverse(&self) {
        match self {
            LinkedList::Node(value, next) => {
                println!("Node value: {}", value);
                next.traverse(); // Recursively call traverse on the next node
            }
            LinkedList::Empty => {
                println!("End of the list.");
            }
        }
    }
}

fn main() {
    // Start with an empty list
    let list = LinkedList::Empty;

    // Add nodes to the list
    let list = LinkedList::add_front(3, list);
    let list = LinkedList::add_front(2, list);
    let list = LinkedList::add_front(1, list);

    // Traverse the list and print its elements
    println!("Traversing the linked list:");
    list.traverse();
}
