//! # Exercise 3: Parallel add
//!
//! This tutorial is work-in-progress. There is no explanation yet.

// You will need these imports
#[allow(unused)] // TODO: Remove this
use creusot_std::{
    ghost::{
        invariant::{AtomicInvariant, Protocol, Tokens, declare_namespace},
        perm::Perm,
        resource::{Authority, Fragment},
    },
    logic::{Id, ra::excl::Excl},
    prelude::*,
};

use std::sync::atomic::AtomicI32;
// TODO: Replace with the creusot_std version of AtomicI32 (you will also need the committer ☺)
// use creusot_std::std::sync::{AtomicI32, UpdateCommitter};

use std::thread;
// TODO: Replace with the creusot_std version of thread
// use creusot_std::std::thread::{self, JoinHandleExt};

// Spawn two threads that add 2 to a shared atomic variable.
// Prove that the final value is 4.
#[trusted]
#[ensures(result == 4i32)]
pub fn parallel_add() -> i32 {
    let atomic = AtomicI32::new(0);

    thread::scope(|s| {
        let atomic = &atomic;

        let t1 = s.spawn(|| {
            atomic.fetch_add(2, std::sync::atomic::Ordering::SeqCst);
        });

        let t2 = s.spawn(|| {
            atomic.fetch_add(2, std::sync::atomic::Ordering::SeqCst);
        });

        let _ = t1.join().unwrap();
        let _ = t2.join().unwrap();
    });

    atomic.into_inner() // Non-atomically read the atomic
}

#[test]
fn test() {
    assert! { parallel_add() == 4 }
}
