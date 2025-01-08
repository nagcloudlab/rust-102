use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

struct Person {
    name: String,
    friends: Vec<Rc<RefCell<Person>>>,
    parent: Option<Weak<RefCell<Person>>>,
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Limit recursion by only printing friend names and parent name
        let friend_names: Vec<String> = self
            .friends
            .iter()
            .map(|friend| friend.borrow().name.clone())
            .collect();
        let parent_name = self
            .parent
            .as_ref()
            .and_then(|weak_parent| weak_parent.upgrade())
            .map(|parent| parent.borrow().name.clone());

        f.debug_struct("Person")
            .field("name", &self.name)
            .field("friends", &friend_names)
            .field("parent", &parent_name)
            .finish()
    }
}

fn main() {
    let alice = Rc::new(RefCell::new(Person {
        name: String::from("Alice"),
        friends: vec![],
        parent: None, // Alice has no parent
    }));

    let bob = Rc::new(RefCell::new(Person {
        name: String::from("Bob"),
        friends: vec![Rc::clone(&alice)],
        parent: Some(Rc::downgrade(&alice)), // Bob's parent is Alice (Weak reference)
    }));

    // Alice becomes friends with Bob
    alice.borrow_mut().friends.push(Rc::clone(&bob));

    println!("Alice: {:?}", alice.borrow());
    println!("Bob: {:?}", bob.borrow());
    println!("Reference count of Alice: {}", Rc::strong_count(&alice)); // 2
    println!("Reference count of Bob: {}", Rc::strong_count(&bob)); // 2
}
