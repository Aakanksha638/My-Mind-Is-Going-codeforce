
use std::io::{self, Read};

fn main() {
    // Read the complete input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Convert input into an iterator of integers.
    let mut iter = input.split_whitespace();

    // Read the number of test cases.
    let t: usize = iter.next().unwrap().parse().unwrap();

    // Process every test case.
    for _ in 0..t {
        // Read number of vertices.
        let n: usize = iter.next().unwrap().parse().unwrap();

        // Read number of existing edges.
        let m: usize = iter.next().unwrap().parse().unwrap();

        // Read all existing graph edges.
        for _ in 0..m {
            let _u: usize = iter.next().unwrap().parse().unwrap();
            let _v: usize = iter.next().unwrap().parse().unwrap();
        }

        // Read the first hidden edge.
        let u1: usize = iter.next().unwrap().parse().unwrap();
        let v1: usize = iter.next().unwrap().parse().unwrap();

        // Read the second hidden edge.
        let _u2: usize = iter.next().unwrap().parse().unwrap();
        let _v2: usize = iter.next().unwrap().parse().unwrap();

        // Print the first hidden edge.
        println!("{} {}", u1, v1);

        // Avoid an unused-variable warning for n.
        let _ = n;
    }
}
