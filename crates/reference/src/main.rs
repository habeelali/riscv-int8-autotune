//! Host-side quantization checker used by tests/test_reference.py.
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("stdin");
        let v: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("integer"))
            .collect();
        assert_eq!(v.len(), 3, "expected x multiplier shift");
        println!("{}", reference::requantize(v[0], v[1], v[2]));
    }
}
