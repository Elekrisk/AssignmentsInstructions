/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get input lines as strings
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    // or read single line
    let mut line = String::new();
    input.read_line(&mut line).expect("IO Error");

    /* add code here ... */

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}
