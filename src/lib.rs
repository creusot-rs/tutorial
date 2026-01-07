//! # Creusot tutorial
//!
//! Some warm-up examples.
//! See the other files for more substantial exercises.

use creusot_contracts::prelude::*;

#[requires(a@ < i64::MAX@)]
#[ensures(result@ == a@ + 1)]
pub fn add_one(a: i64) -> i64 {
    a + 1
}

pub mod ex1_gnome_sort;
pub mod ex2_linked_list;

#[cfg(feature = "solutions")]
pub mod solutions {
    pub mod ex1_gnome_sort;
    pub mod ex2_linked_list;
}
