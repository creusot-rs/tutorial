//! # Creusot tutorial
//!
//! Some warm-up examples.
//! See the other files for more substantial exercises.

use creusot_contracts::{ghost::perm::Perm, prelude::*};

#[requires(n@ * (n@ + 1) / 2 <= u32::MAX@)]
#[ensures(result@ == n@ * (n@ + 1) / 2)]
pub fn sum_first_n(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    #[invariant(sum@ == i@ * (i@ + 1) / 2)]
    #[invariant(i@ <= n@)]
    while i < n {
        i += 1;
        sum += i;
    }
    sum
}

#[ensures(if b {
    ^x == 0u32 && ^y == *y
} else {
    ^x == *x && ^y == 0u32
})]
pub fn choose(b: bool, x: &mut u32, y: &mut u32) {
    if b {
        *x = 0;
    } else {
        *y = 0;
    }
}

#[ensures((^v)@.len() == v@.len())]
#[ensures(forall<i> 0 <= i && i < v@.len() ==> (^v)@[i]@ == 0)]
pub fn all_zero(v: &mut [u32]) {
    #[invariant(forall<i> 0 <= i && i < produced.len() ==> (^produced[i])@ == 0)]
    for x in v.iter_mut() {
        *x = 0;
    }
}

#[ensures((^slice)@.permutation_of((*slice)@))]
pub fn shuffle<T>(slice: &mut [T]) {
    let _old_slice = snapshot! {slice};
    #[invariant((*slice)@.permutation_of((**_old_slice)@))]
    for i in 1..slice.len() {
        swap_slice(slice, i, random(i))
    }
}

#[ensures(0 <= result@ && result@ <= i@)]
pub fn random(i: usize) -> usize {
    // a random number :)
    0
}

#[trusted] // Currently unsupported
#[ensures((^slice)@.permutation_of((*slice)@))]
pub fn swap_slice<T>(slice: &mut [T], i: usize, j: usize) {
    let [x, y] = slice.get_disjoint_mut([i, j]).unwrap();
    std::mem::swap(x, y)
}

#[ensures(result == (x.deep_model() == y.deep_model()))]
pub fn equal<T: Eq + DeepModel>(x: T, y: T) -> bool {
    x == y
}

#[ensures(result == (x.deep_model() > y.deep_model()))]
pub fn greater<T>(x: T, y: T) -> bool
where
    T: Ord + DeepModel,
    T::DeepModelTy: OrdLogic,
{
    x > y
}

pub struct SumTo10(i32, i32);

impl Invariant for SumTo10 {
    #[logic]
    fn invariant(self) -> bool {
        pearlite! { self.0@ + self.1@ == 10 }
    }
}

impl SumTo10 {
    #[requires(x@ + y@ == 10)]
    pub fn new(x: i32, y: i32) -> Self {
        SumTo10(x, y)
    }

    #[ensures(result@ == 10)]
    pub fn sum(self) -> i32 {
        self.0 + self.1
    }
}

#[requires(ptr == *(*perm).ward())]
#[ensures(x == *(^perm).val())]
#[ensures((*perm).ward() == (^perm).ward())]
pub unsafe fn write_ptr<T>(ptr: *const T, x: T, perm: Ghost<&mut Perm<*const T>>) {
    let r = unsafe { Perm::as_mut(ptr as *mut T, perm) };
    *r = x;
}

pub mod ex1_gnome_sort;
pub mod ex2_linked_list;

#[cfg(feature = "solutions")]
pub mod solutions {
    pub mod ex1_gnome_sort;
    pub mod ex2_linked_list;
}
