// 1) Check that `cargo test` passes with this file unchanged.
//
// 2) Delete the #[cfg(skip)] line on the second test function, and then make
//    count's definition generic, so `cargo test` passes again.
//
// 3) Finally, get the third test function working.

pub fn count(vec: Vec<i32>, val: i32) -> usize {
    let mut count = 0;
    for elt in vec {
        if elt == val {
            count += 1;
        }
    }
    count
}


#[test]
fn test_count_i32() {
    assert_eq!(count(vec![0, 0, 1, 5, 0], 0), 3);
    assert_eq!(count(vec![], 5), 0);
    assert_eq!(count(vec![0, 0, 1, 5, 0], 20), 0);
}

#[cfg(skip)] // For 2), delete this line to stop skipping this test function
#[test]
fn test_count_other_types() {
    assert_eq!(count(vec![-128i8, 0, 127], 127), 1);
    assert_eq!(count(vec!["iichigo", "kiichigo", "momo"], "iichigo"), 1);
}

#[cfg(skip)] // Delete for 3)
#[test]
fn test_count_floats() {
    use std::f64::consts::{PI, E};

    // IEEE floats are tricky: NaN != NaN, so f32 and f64 are not Eq! There is a
    // PartialEq trait, though. (Exact equality is not very meaningful on
    // floats; I know.)
    assert_eq!(count(vec![-0.25f32, 4.0, 6.022e+23, 4.0], 4.0), 2);
    assert_eq!(count(vec![PI, E, E, PI], std::f64::consts::E),
               2);
}
