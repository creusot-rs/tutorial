//! # Exercise 1: verify `gnome_sort`
//!
//! ## Part A
//!
//! Formalize and prove the following specification:
//!
//! - The final value of `v` (`^v`) contains elements in increasing order.
//! - The final value of `v` is a permutation of the initial value (`*v`).
//!
//! ## Part B
//!
//! Generalize `gnome_sort` to sort slices of any ordered type.

use creusot_contracts::prelude::*;

#[trusted] // TODO: Remove this
// #[ensures(TODO)]
pub fn gnome_sort(v: &mut [usize]) {
    let _old_v: Snapshot<&mut [usize]> = snapshot! { v };
    let mut i = 0;
    // #[invariant(TODO)]
    while i < v.len() {
        if i == 0 || v[i - 1] <= v[i] {
            i += 1;
        } else {
            v.swap(i - 1, i);
            i -= 1;
        }
    }
}

// Unit tests

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
