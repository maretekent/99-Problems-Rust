// The author of this work hereby waives all claim of copyright (economic and
// moral) in this work and immediately places it in the public domain; it may
// be used, distorted or destroyed in any manner whatsoever without further
// attribution or notice to the creator.

//! Problem 03: Vectors: kth
//!
//! Find the `k`th element of a vector.
//! Your function could have this signature:
//! `fn kth<'a, T>(k: uint, vec: &'a [T]) -> Option<&'a T>`
//!

fn kth<'a, T>(k: uint, vec: &'a [T]) -> Option<&'a T> {
    if vec.len() >= k && k > 0{
        Some(&vec[k-1])
    } else {
        None
    }
}

#[test]
fn kth_outofrange() {
    let vec = ~['a'];
    assert_eq!(kth(3, vec), None)
}

#[test]
fn kth_inrange() {
    let vec = ~['a', 'b', 'c', 'd', 'e'];
    assert_eq!(kth(3, vec), Some(&'c'));
}

