//! Examples, including ones from the tutorial slides
//!
//! A couple of simple exercises.
//! Goal: remove all `#[trusted]` with an "Exercise" comment.
//!
//! See the tutorial slides or [`crate::solutions::ex0_examples`] for the solutions.

use creusot_std::{cell::PermCell, ghost::perm::Perm, prelude::*};

#[trusted] // Exercise: write the contract
pub fn swap<T>(x: &mut T, y: &mut T) {
    // The naive definition would require some constraints on T.
    // Here we use the general swap (which uses some unsafe primitives internally).
    std::mem::swap(x, y)
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

/// Pairs of elements that sum up to 10.
/// This property is formalized by its type invariant below.
pub struct SumTo10(i32, i32);

impl Invariant for SumTo10 {
    #[logic]
    fn invariant(self) -> bool {
        pearlite! { self.0@ + self.1@ == 10 }
    }
}

impl SumTo10 {
    #[requires(x@ + y@ == 10)]
    // Implicit: #[ensures(invariant(result))]
    pub fn new(x: i32, y: i32) -> Self {
        SumTo10(x, y)
    }

    // Implicit: #[requires(invariant(self))]
    #[ensures(result@ == 10)]
    pub fn sum(self) -> i32 {
        self.0 + self.1
    }
}

/// Minimal example of interior mutability
pub fn interior_mut() {
    // SAFETY: Proved by Creusot
    unsafe {
        let (cell, mut perm) = PermCell::new(0);
        let (b1, b2) = (&cell, &cell);
        b1.set(ghost! { &mut **perm }, 1);
        let result = b2.take(ghost! { &mut **perm });
        proof_assert! { result == 1i32 };
    }
}

/// Write `x` to `ptr`, given a suitable permission `perm`.
#[requires(ptr == *(*perm).ward())]
#[ensures(x == *(^perm).val())]
#[ensures((*perm).ward() == (^perm).ward())]
pub unsafe fn write_ptr<T>(ptr: *const T, x: T, perm: Ghost<&mut Perm<*const T>>) {
    // SAFETY: Proved by Creusot
    let r = unsafe { Perm::as_mut(ptr as *mut T, perm) };
    *r = x;
}
