// We use only the Rust standard library.
// `fmt` gives access to formatting traits, and `Display` lets our types print nicely.
use std::fmt::{self, Display};

// A linked list is built from links.
// Each link is either:
// - `None`, meaning "there is no next node"
// - `Some(Box<Node<T>>)` meaning "there is a heap-allocated node here"
//
// `T` is a generic type parameter. It means this linked list can store any type:
// integers, strings, structs, or any other user-defined value.
type Link<T> = Option<Box<Node<T>>>;

// A Node is private because users of the list should not need to manipulate nodes directly.
// They should use the public `LinkedList<T>` methods instead.
#[derive(Debug)]
struct Node<T> {
    // The actual data stored in this node.
    value: T,

    // The next pointer is an `Option`.
    // This is how Rust represents the end of the list without using null.
    next: Link<T>,
}

// This implementation block works for every possible `T`.
impl<T> Node<T> {
    // Creates one isolated node.
    // It has a value, but it does not point to any next node yet.
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

// This enum describes errors that can happen when using the list.
// It is public because methods like `insert` and `remove` return it.
#[derive(Debug, PartialEq, Eq)]
pub enum ListError {
    // This variant stores useful context for the caller:
    // the invalid index and the current list length.
    IndexOutOfBounds { index: usize, len: usize },
}

// Implementing `Display` lets us print the error with `{error}`.
// This is a trait implementation from the standard library.
impl Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `match` is commonly used with enums.
        // Here there is only one error variant, but the pattern scales well.
        match self {
            Self::IndexOutOfBounds { index, len } => {
                write!(
                    f,
                    "index {index} is out of bounds for a list of length {len}"
                )
            }
        }
    }
}

/// A custom singly linked list.
///
/// The list owns all of its nodes. Each node owns the next node through a `Box`.
/// The generic parameter `T` means the list can store any one type chosen by the caller.
#[derive(Debug, Default)]
pub struct LinkedList<T> {
    // The first node in the list.
    // If this is `None`, the list is empty.
    head: Link<T>,

    // We store the length so `len()` is fast.
    // Without this field, we would need to walk the whole list to count nodes.
    len: usize,
}

// Main implementation block for the linked list.
// `impl<T>` means all these methods are available for `LinkedList<i32>`,
// `LinkedList<String>`, `LinkedList<CourseTopic>`, and so on.
impl<T> LinkedList<T> {
    /// Creates an empty linked list.
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// Returns true when the list has no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements currently stored in the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Adds a value at the front of the list.
    pub fn push_front(&mut self, value: T) {
        // Create a new heap-allocated node.
        let mut new_node = Box::new(Node::new(value));

        // `take()` moves the current head out and leaves `None` behind.
        // This is useful because Rust does not allow moving directly out of `self.head`.
        new_node.next = self.head.take();

        // The new node becomes the head of the list.
        self.head = Some(new_node);

        // Keep the stored length in sync.
        self.len += 1;
    }

    /// Adds a value at the back of the list.
    pub fn push_back(&mut self, value: T) {
        // A cursor is a mutable reference to a link.
        // It starts at the head link and walks forward until it reaches the empty tail link.
        let mut cursor = &mut self.head;

        // While the current link contains a node, move the cursor to that node's `next` link.
        // This pattern avoids holding a mutable borrow while also trying to assign to it.
        while let Some(node) = cursor {
            cursor = &mut node.next;
        }

        // When the loop ends, cursor points to the final `None`.
        // Replacing that `None` with `Some(node)` appends the value.
        *cursor = Some(Box::new(Node::new(value)));
        self.len += 1;
    }

    /// Removes and returns the first value in the list.
    ///
    /// Returns `None` if the list is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        // `take()` removes the head from the list and leaves `None` temporarily.
        self.head.take().map(|old_head| {
            // `old_head` is a `Box<Node<T>>`.
            // `*old_head` moves the Node out of the Box, because we own the Box.
            let old_head = *old_head;

            // The second node becomes the new head.
            self.head = old_head.next;

            // The list now has one fewer element.
            self.len -= 1;

            // Return the stored value.
            old_head.value
        })
    }

    /// Borrows the first value without removing it.
    ///
    /// Returns `Option<&T>` because the list may be empty.
    pub fn front(&self) -> Option<&T> {
        // `as_ref()` converts `Option<Box<Node<T>>>` into `Option<&Box<Node<T>>>`.
        // `map()` then turns the borrowed node into a borrowed value.
        self.head.as_ref().map(|node| &node.value)
    }

    /// Mutably borrows the first value without removing it.
    ///
    /// Returns `Option<&mut T>` because the list may be empty.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        // `as_mut()` gives a mutable borrow of the node inside the Option.
        self.head.as_mut().map(|node| &mut node.value)
    }

    /// Borrows the value at a given index.
    ///
    /// Returns `None` when the index is outside the list.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.node_at(index).map(|node| &node.value)
    }

    /// Inserts a value at a given index.
    ///
    /// This method returns `Result` because the operation can fail.
    /// - `Ok(())` means the value was inserted.
    /// - `Err(ListError)` means the index was invalid.
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), ListError> {
        // Insertion is allowed at index `len` because that means "append at the end".
        if index > self.len {
            return Err(ListError::IndexOutOfBounds {
                index,
                len: self.len,
            });
        }

        // Inserting at the front is exactly the same operation as `push_front`.
        if index == 0 {
            self.push_front(value);
            return Ok(());
        }

        // To insert at index N, we need the node at index N - 1.
        // This `expect` is safe because the bounds check above already proved the index is valid.
        let previous = self
            .node_at_mut(index - 1)
            .expect("index was checked above");

        // Create the new node.
        let mut new_node = Box::new(Node::new(value));

        // The new node should point to what the previous node used to point to.
        new_node.next = previous.next.take();

        // The previous node now points to the new node.
        previous.next = Some(new_node);

        self.len += 1;
        Ok(())
    }

    /// Removes and returns the value at a given index.
    ///
    /// This method returns `Result<T, ListError>` because removal can fail.
    pub fn remove(&mut self, index: usize) -> Result<T, ListError> {
        // Removal is only valid for indexes already inside the list.
        if index >= self.len {
            return Err(ListError::IndexOutOfBounds {
                index,
                len: self.len,
            });
        }

        // Removing the first item is exactly the same operation as `pop_front`.
        if index == 0 {
            return Ok(self.pop_front().expect("index was checked above"));
        }

        // To remove index N, we need to change the `next` pointer of index N - 1.
        let previous = self
            .node_at_mut(index - 1)
            .expect("index was checked above");

        // Take ownership of the node being removed.
        let removed = previous.next.take().expect("index was checked above");
        let removed = *removed;

        // Skip over the removed node.
        previous.next = removed.next;

        self.len -= 1;

        // Return the removed value to the caller.
        Ok(removed.value)
    }

    /// Finds the first value that matches a predicate.
    ///
    /// `P` is a generic function-like type.
    /// `P: Fn(&T) -> bool` means the caller can pass a closure.
    pub fn find<P>(&self, predicate: P) -> Option<&T>
    where
        P: Fn(&T) -> bool,
    {
        // `iter()` borrows values, so this method does not consume the list.
        self.iter().find(|value| predicate(value))
    }

    /// Returns an iterator that borrows each value.
    ///
    /// The lifetime `'_` means "the iterator cannot outlive this borrow of the list".
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    // Private helper: returns the node at an index by shared reference.
    fn node_at(&self, index: usize) -> Option<&Node<T>> {
        // `as_deref()` turns `Option<Box<Node<T>>>` into `Option<&Node<T>>`.
        let mut current = self.head.as_deref();

        // Walk forward `index` times.
        for _ in 0..index {
            // The `?` operator works with Option.
            // If `current` is None, this whole function returns None immediately.
            current = current?.next.as_deref();
        }

        current
    }

    // Private helper: returns the node at an index by mutable reference.
    fn node_at_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
        // `as_deref_mut()` turns `Option<Box<Node<T>>>` into `Option<&mut Node<T>>`.
        let mut current = self.head.as_deref_mut();

        for _ in 0..index {
            // Again, `?` exits early if the traversal reaches the end.
            current = current?.next.as_deref_mut();
        }

        current
    }
}

// `FromIterator` lets us build a LinkedList from any iterator.
// Example: `LinkedList::from_iter([1, 2, 3])`.
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();

        // Push every incoming value to the back so the order is preserved.
        for value in iter {
            list.push_back(value);
        }

        list
    }
}

// This implementation lets a list print as `[a -> b -> c]`.
// The `T: Display` bound means this only works when the stored values can be displayed.
impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        // Use the borrowing iterator so printing does not consume the list.
        for (index, value) in self.iter().enumerate() {
            if index > 0 {
                write!(f, " -> ")?;
            }

            write!(f, "{value}")?;
        }

        write!(f, "]")
    }
}

/// A borrowing iterator over `LinkedList<T>`.
///
/// The lifetime `'a` connects the iterator to the list it borrows from.
/// This prevents the iterator from outliving the list.
pub struct Iter<'a, T> {
    // This stores the next node that will be visited.
    next: Option<&'a Node<T>>,
}

// This is the standard library `Iterator` trait.
// For `Iter<'a, T>`, each item is a borrowed `&'a T`.
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // If there is no next node, return None.
        let current = self.next?;

        // Move the iterator forward before returning the current value.
        self.next = current.next.as_deref();

        // Return a borrowed value.
        Some(&current.value)
    }
}

// This implementation makes `LinkedList<T>` itself a consuming iterator.
// Calling `next()` removes values from the front of the list.
impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

/// A custom trait used to demonstrate how we can define our own behavior.
pub trait Summary {
    /// Returns a short human-readable description of a value.
    fn summary(&self) -> String;
}

// Implement our custom trait for any linked list whose values implement Display.
impl<T: Display> Summary for LinkedList<T> {
    fn summary(&self) -> String {
        format!("LinkedList with {} item(s): {}", self.len(), self)
    }
}

/// Returns the first item in the list, or a fallback reference if the list is empty.
///
/// The lifetime `'a` tells Rust that the returned reference comes either from `list`
/// or from `default`, and both must live at least as long as the returned reference.
pub fn first_or_default<'a, T>(list: &'a LinkedList<T>, default: &'a T) -> &'a T {
    list.front().unwrap_or(default)
}
