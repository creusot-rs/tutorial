//! # Exercise 1: Gnome sort
//!
//! A sorting function with the simplicity of a single loop.
//!
//! ## Tutorial summary
//!
//! 1. Formalize and prove the following specification:
//!
//!     - The final value of `v` (`^v`) contains elements in increasing order.
//!     - The final value of `v` is a permutation of the initial value (`*v`).
//!
//! 2. Generalize `gnome_sort` to sort slices of any ordered type.

#![allow(unused)] // TODO: Remove this

use creusot_std::prelude::*;

// #[ensures(TODO)]
pub fn gnome_sort(v: &mut [usize]) {
    let mut n = 0;
    // #[invariant(TODO)]
    while n < v.len() {
        if n == 0 || v[n - 1] <= v[n] {
            n += 1;
        } else {
            v.swap(n - 1, n);
            n -= 1;
        }
    }
}

#[test]
fn test_1() {
    let mut v = [4, 2, 1, 3];
    gnome_sort(&mut v);
    assert_eq!(v, [1, 2, 3, 4]);
}

/* TODO: Uncomment this after generalizing gnome_sort
#[test]
fn test_2() {
    let mut v = [(4, 4), (2, 2), (1, 1), (3, 3)];
    gnome_sort(&mut v);
    assert_eq!(v, [(1, 1), (2, 2), (3, 3), (4, 4)]);
}
*/
