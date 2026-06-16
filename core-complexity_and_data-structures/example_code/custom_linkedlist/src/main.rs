// Import the reusable linked-list library defined in `src/lib.rs`.
// `first_or_default` demonstrates lifetimes.
// `LinkedList` is our custom data structure.
// `Summary` is a custom trait implemented for the list.
use custom_linkedlist::{LinkedList, Summary, first_or_default};

// These standard library imports are used to implement the Display trait.
use std::fmt::{self, Display};

// This struct is used only by the demo program.
// It stores borrowed string slices, so it needs a lifetime parameter.
#[derive(Debug)]
struct CourseTopic<'a> {
    // `&'a str` means this field borrows text that must live at least as long as the topic.
    name: &'a str,

    // Plain owned data does not need a lifetime.
    minutes: u32,
}

// Implementation block for our demo struct.
impl<'a> CourseTopic<'a> {
    // Constructor function.
    // The returned `CourseTopic` borrows the same string slice passed in as `name`.
    fn new(name: &'a str, minutes: u32) -> Self {
        Self { name, minutes }
    }
}

// Implement Display so a CourseTopic can be printed with `{topic}`.
// The lifetime is written as `'_` because Rust can infer the exact borrowed lifetime here.
impl Display for CourseTopic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} min)", self.name, self.minutes)
    }
}

fn main() {
    // Because `LinkedList` is generic, Rust will infer `LinkedList<CourseTopic>`.
    let mut topics = LinkedList::new();

    // Add several topics to the back of the list.
    // Each topic borrows a string literal, which lives for the whole program.
    topics.push_back(CourseTopic::new("lifetime", 12));
    topics.push_back(CourseTopic::new("struct", 8));
    topics.push_back(CourseTopic::new("implementation", 10));
    topics.push_back(CourseTopic::new("traits", 9));
    topics.push_back(CourseTopic::new("Result and Option", 14));
    topics.push_back(CourseTopic::new("generics", 11));

    // This uses our custom `Summary` trait.
    println!("{}", topics.summary());

    // `get` returns Option<&T>.
    // We must handle both cases: Some(value) and None.
    match topics.get(2) {
        Some(topic) => println!("Option example: topic at index 2 is {topic}"),
        None => println!("Option example: no topic exists at index 2"),
    }

    // `insert` returns Result<(), ListError>.
    // Result forces us to handle success and failure explicitly.
    match topics.insert(2, CourseTopic::new("custom LinkedList", 20)) {
        Ok(()) => println!("Result example: inserted a linked-list topic"),
        Err(error) => println!("Result example: could not insert topic: {error}"),
    }

    // This value is used only if the list is empty.
    let fallback = CourseTopic::new("fallback reference", 0);

    // `first_or_default` returns a reference.
    // Its lifetime signature guarantees the returned reference is valid.
    let first = first_or_default(&topics, &fallback);
    println!("Lifetime example: first visible topic is {first}");

    // `find` accepts a closure.
    // This demonstrates a generic function parameter with the trait bound `Fn(&T) -> bool`.
    if let Some(topic) = topics.find(|topic| topic.name.contains("traits")) {
        println!("Generic predicate example: found {topic}");
    }

    // This list stores integers instead of CourseTopic values.
    // That is possible because our linked list is generic.
    let mut iterator_demo = LinkedList::from_iter([1, 2, 3]);

    // `LinkedList<T>` implements the Iterator trait directly.
    // Calling `next()` consumes one item from the front.
    println!(
        "Iterator trait example: next item is {:?}",
        iterator_demo.next()
    );

    // Printing the list uses the Display implementation for LinkedList<T>.
    // This does not consume the list because Display uses the borrowing iterator.
    println!("Final list: {topics}");
}
