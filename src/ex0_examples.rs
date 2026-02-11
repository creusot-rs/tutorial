//! A couple of short exercises.
//!
//! Objective: remove all `#[trusted]` with an "Exercise: ..." comment
//! and make `cargo creusot prove` happy!
//!
//! Some exercises make use of the `creusot_std` API, which you can
//! browse here: https://creusot-rs.github.io/creusot/doc/creusot_std/
//!
//! The main goal is to get comfortable with Creusot's syntax.
//! It's perfectly fine to directly skip to the solutions and learn by osmosis.
//! ([`crate::solutions::ex0_examples`])
//!
//! Most of these also appear in the tutorial slides linked in `README.md`.

use creusot_std::prelude::*;

/// Swap the contents of two mutable borrows
#[trusted] // Exercise: write the contract
pub fn swap<T>(x: &mut T, y: &mut T) {
    // The naive definition would require some constraints on T.
    // Here we use the general swap (which uses some unsafe primitives internally).
    std::mem::swap(x, y)
}

/// Choose one of two mutable borrows
#[trusted] // Exercise: write the contract
pub fn choose<'a, T>(b: bool, x: &'a mut T, y: &'a mut T) -> &'a mut T {
    if b { x } else { y }
}

/// Set a slice to zero
#[trusted] // Exercise: write the contract and the invariant
pub fn all_zero(v: &mut [u32]) {
    // Hint: In the invariant, a special variable `produced: Seq<&mut u32>` is available,
    // containing all past values produced by the iterator (not including the current `x` at the start of the loop)
    for x in v.iter_mut() {
        *x = 0;
    }
}

/// Set a slice to zero, using `Iterator::map`.
#[trusted] // Exercise: write the contract
pub fn all_zero_map(v: &mut [u32]) {
    // We could use `for_each` instead of `map` in theory,
    // but it's currently missing a specification in Creusot.
    v.iter_mut()
        .map(|x| {
            *x = 0;
        })
        .collect::<()>()
}

/// Add one
#[trusted] // Exercise: Write the contract
pub fn add_one(n: u32) -> u32 {
    n + 1
}

/// Sum of integers from 1 to n
#[trusted] // Exercise: write the contract
pub fn sum_first_n(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    while i < n {
        i += 1;
        sum += i;
    }
    sum
}

/// Sum of numbers in a slice
#[trusted] // Exercise: write the contract
// Hint: use `sum_seq(xs@)`
pub fn sum_slice(xs: &[u64]) -> u64 {
    let mut sum = 0;
    sum_slice_lemma(xs);
    let _ = xs
        .iter()
        .map_inv(|x, produced| {
            proof_assert! { sum@ + x@ == sum_seq(xs@[0..produced.len() + 1]) };
            sum += *x;
        })
        .collect::<()>();
    sum
}

/// Sum of numbers in a sequence, as a logic function
#[logic(open)]
#[variant(xs.len())]
pub fn sum_seq(xs: Seq<u64>) -> Int {
    pearlite! {
        if xs.len() == 0 {
            0
        } else {
            sum_seq(xs[0..xs.len() - 1]) + xs[xs.len() - 1]@
        }
    }
}

/// Lemmas for `sum_slice`
// These lemmas are already proved, and used in `sum_slices`. No need to do anything.
#[requires(sum_seq(xs@) <= u64::MAX@)]
#[ensures(forall<i> 0 <= i && i <= xs@.len() ==> sum_seq(xs@[0..i]) <= u64::MAX@)]
#[ensures(forall<i> 0 <= i && i < xs@.len() ==> xs@[0..i+1][0..i] == xs@[0..i])]
#[ensures(xs@[0..xs@.len()] == xs@)]
pub fn sum_slice_lemma(xs: &[u64]) {
    let _ = snapshot! { sum_seq_sub(xs@) };
}

/// "Proof" of `sum_slice_lemma`.
#[logic]
#[variant(xs.len())]
#[ensures(forall<i> 0 <= i && i <= xs.len() ==> sum_seq(xs[0..i]) <= sum_seq(xs))]
pub fn sum_seq_sub(xs: Seq<u64>) {
    pearlite! {
        if xs.len() != 0 {
            proof_assert! { xs[0..xs.len()] == xs };
            proof_assert! { forall<i> 0 <= i && i < xs.len() ==> xs[0..xs.len() - 1][0..i] == xs[0..i] };
            sum_seq_sub(xs[0..xs.len() - 1])
        }
    }
}

/// Shuffle the elements of a slice
#[trusted] // Exercise: write the contract, that the final slice is a permutation of the initial slice, and write the invariant.
// Hint: use `Seq::is_permutation`.
pub fn shuffle<T>(slice: &mut [T]) {
    let _old_slice = snapshot! {slice};
    // Hint: use the above snapshot in the invariant
    for i in 1..slice.len() {
        // Hint: the specs of `random` and `swap_slice` are already written.
        swap_slice(slice, i, random(i))
    }
}

/// Helper for `shuffle`.
/// A random number between `0` and `i`.
#[ensures(0 <= result@ && result@ <= i@)]
pub fn random(i: usize) -> usize {
    let _ = i;
    // a random number :)
    0
}

/// Helper for `shuffle`.
/// Swap two elements of a slice.
#[trusted] // Currently unsupported (Not an exercise!)
#[ensures((^slice)@.permutation_of((*slice)@))]
pub fn swap_slice<T>(slice: &mut [T], i: usize, j: usize) {
    if i != j {
        let [x, y] = slice.get_disjoint_mut([i, j]).unwrap();
        std::mem::swap(x, y)
    }
}

/// Equality test
#[trusted] // Exercise: write the contract
pub fn equal<T: Eq + DeepModel>(x: T, y: T) -> bool {
    x == y
}

/// Ordering test
#[trusted] // Exercise: write the contract
pub fn greater<T>(x: T, y: T) -> bool
where
    T: Ord + DeepModel,
    T::DeepModelTy: OrdLogic,
{
    x > y
}

/// As a simple example of type invariant, the following type
/// is intended to contain pairs of elements that sum up to 10.
pub struct SumTo10(i32, i32);

// Exercise: fill out the type invariant, saying that
// the two components of `self` sum up to 10.
impl Invariant for SumTo10 {
    #[logic]
    fn invariant(self) -> bool {
        pearlite! {
            true /* TODO */
        }
    }
}

impl SumTo10 {
    #[trusted] // Exercise: write the contract
    pub fn new(x: i32, y: i32) -> Self {
        SumTo10(x, y)
    }

    #[trusted] // Exercise: write the contract guaranteeing that the result is 10
    pub fn sum(self) -> i32 {
        self.0 + self.1
    }
}

#[allow(unused)] // Remove this after completing the exercises below
use creusot_std::{cell::PermCell, ghost::perm::Perm};

/// Minimal example of interior mutability
#[trusted] // Exercise: replace `UnsafeCell` with `PermCell`
// Then use the associated permission to write and read the cell.
// No contract for this function.
pub fn interior_mut() {
    use std::cell::UnsafeCell;
    // SAFETY: To be proved by Creusot
    unsafe {
        let cell = UnsafeCell::new(0); // `PermCell::new` will return a cell and a permission
        let (b1, b2) = (&cell, &cell); // Share the cell (this line won't change)
        *&mut *b1.get() = 1; // Replace this with `PermCell::set` or `PermCell::borrow_mut` to write to it
        let result = *&*b2.get(); // Replace this with `PermCell::get` or `PermCell::borrow` to read from it
        proof_assert! { result == 1i32 };
    }
}

/// Write `x` to `ptr`, given a suitable permission `perm`.
#[trusted]
// Exercise: Rewrite the pointer cast with `Perm::as_mut` (making use of the permission)
// Then write the contract of `write_ptr`, "`x` is written into `ptr`"
// Hint: Take inspiration from the contract of `Perm::as_mut`.
#[allow(unused)] // Remove this
pub unsafe fn write_ptr<T>(ptr: *const T, x: T, perm: Ghost<&mut Perm<*const T>>) {
    // SAFETY: To be proved by Creusot
    let r = unsafe { &mut *(ptr as *mut T) }; // Replace this cast with `Perm::as_mut`
    *r = x;
}
