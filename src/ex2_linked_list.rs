//! # Exercise 2: Linked list
//!
//! A linked list with methods `push_back`, `pop_front`, `push_front`.
//!
//! This implementation uses unsafe pointer operations,
//! and we will use Creusot to verify their safety conditions
//! thanks to ghost pointer permissions.
//!
//! ## Tutorial summary
//!
//! 1. Add a field to `List<T>` containing the pointer permissions.
//! 2. Add a type invariant (`impl Invariant for List<T>`).
//! 3. Add a view (`impl View for List<T>`).
//! 4. Verify `new`, `push_back`, `pop_front`, `push_front`.

#![allow(unused)] // TODO: remove this

use creusot_contracts::{ghost::perm::Perm, prelude::*};

struct Link<T> {
    value: T,
    next: *const Link<T>,
}

pub struct List<T> {
    first: *const Link<T>,
    last: *const Link<T>,
}

impl<T> List<T> {
    /// Create an empty list.
    #[trusted] // TODO: Remove this
    pub fn new() -> List<T> {
        List {
            first: std::ptr::null_mut(),
            last: std::ptr::null_mut(),
        }
    }

    /// Push an element to the back of the list.
    #[trusted] // TODO: Remove this after rewriting away `&mut *self.last`
    pub fn push_back(&mut self, value: T) {
        // Allocate a new `Link`
        let link = Box::new(Link {
            value,
            next: std::ptr::null_mut(),
        });
        // Cast the `Box` into a raw pointer
        let link_ptr = Box::into_raw(link);
        if self.last.is_null() {
            self.first = link_ptr;
            self.last = link_ptr;
        } else {
            // Modify the `next` field of the `last` link to point to the newly allocated `Link`
            let link_last = unsafe { &mut *(self.last as *mut Link<T>) };
            link_last.next = link_ptr;
            self.last = link_ptr;
        }
    }

    /// Pop an element from the front of the list.
    #[trusted] // TODO: Remove this
    pub fn pop_front(&mut self) -> Option<T> {
        if self.first.is_null() {
            return None;
        }
        // Cast the `self.first` pointer to a `Box` and take out its contents.
        // The `Box` is deallocated implicitly.
        let link = *unsafe { Box::from_raw(self.first as *mut Link<T>) };
        self.first = link.next;
        if self.first.is_null() {
            self.last = std::ptr::null_mut();
        }
        Some(link.value)
    }

    /// Push an element to the front of the list.
    #[trusted] // TODO: Remove this
    pub fn push_front(&mut self, value: T) {
        let link = Box::new(Link {
            value,
            next: self.first,
        });
        let link_ptr = Box::into_raw(link);
        self.first = link_ptr;
        if self.last.is_null() {
            self.last = link_ptr;
        }
    }
}

#[test]
fn test_1() {
    const LIST: [usize; 3] = [1, 2, 3];
    let mut x = List::new();
    for i in LIST {
        x.push_back(i);
    }
    for i in LIST {
        assert_eq!(x.pop_front(), Some(i));
    }
    assert_eq!(x.pop_front(), None)
}
