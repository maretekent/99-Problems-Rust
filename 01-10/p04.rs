//
//

//! Problem 04: Vectors: length
//!
//! Compute the number of elements of a vector.  There is a method for it in
//! the standard library, you can use it or reimplement it.
//!
//! Your function must have this signature:
//! `fn length<T>(vector: &[T]) -> uint`
//!

fn length<T>(vector: &[T]) -> uint {
    vector.len()
}

#[test]
fn test04_length() {
    let vector = ~['a', 'b', 'c'];
    assert!(length(vector) == 3);

    let vector: ~[uint] = ~[];
    assert!(length(vector) == 0);
}

