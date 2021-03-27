/*! collatz.rs : test for the Collatz conjecture
 * HISTORY: The Collatz conjecture is a conjecture in mathematics that concerns a sequence defined as follows: 
 * start with any positive integer n. Then each term is obtained from the previous term as follows: 
 * if the previous term is even, the next term is one half of the previous term. 
 * If the previous term is odd, the next term is 3 times the previous term plus 1. 
 * The conjecture is that no matter what value of n, the sequence will always reach 1.
 * The conjecture is named after Lothar Collatz, who introduced the idea in 1937, two years after receiving his doctorate.
 * 
 * See source code on [GitHub](https://github.com/sinnud/rustbook/tree/collatz)
 */
use std::env;

/** # Collatz example
 * code from [web](https://aml3.github.io/RustTutorial/html/01.html)
 * Get number from stdin
 * Call function `collatz` to
 * Compute Collatz steps for this number
 */
#[allow(dead_code)]
pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Please provide a number as argument.");
        return;
    }

    let i : u128 = args[1].parse().ok().expect("Please input integer.");
    println!("{:?} has {:?} Collatz steps", i, collatz(i));
}

/** # Collatz function
 * code from [web](https://aml3.github.io/RustTutorial/html/01.html)
 * called by function `main` and `reverse_collatz`
 * Recursively compute Collatz steps
 */
fn collatz(n: u128) -> u32 {
    if n == 1 { return 0; }
    match n % 2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(n*3+1) }
    }
}

/** # Collatz : reverse problem
 * Given Collatz steps
 * Compute least number with exactly this Collatz steps
 * Just enumerate integers start from 1
 * Call function `collatz` each time
 * May have quicker algorithm
 */
#[allow(dead_code)]
pub fn reverse_collatz() {
    let max_default : u128 = 100000000;
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Please provide a number as argument.");
        return;
    }

    let i : u32 = args[1].parse().ok().expect("Please input integer...");
    let max : u128 = if args.len() < 3 {max_default} 
    else {args[2].parse().ok().expect("Please input integer for max!.")};

    let mut n : u128 = 1;

    // Loop while `n` is less than given number
    while n < max {
        if collatz(n) == i {
            println!("Found {:?} as the first integer with exact {:?} Collatz steps", n, i);
            return;
        } 
        // Increment counter
        n += 1;
    }
    println!("The max number {} exceed but didn't find answer.", max);
}
/* // u32 is not large enough for collatz problem
pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: Please provide a number as argument.");
        return;
    }

    let i : u32 = args[1].parse().ok().expect("Please input integer.");
    println!("{:?} has {:?} Collatz steps", i, collatz(i));
}

fn collatz(n: u32) -> u32 {
    if n == 1 { return 0; }
    match n % 2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(n*3+1) }
    }
}
*/

/*
The longest progression for any initial starting number

less than 10 is 9, which has 19 steps,
less than 100 is 97, which has 118 steps,
less than 1000 is 871, which has 178 steps,
less than 104 is 6171, which has 261 steps,
less than 105 is 77031, which has 350 steps,
less than 106 is 837799, which has 524 steps,
less than 107 is 8400511, which has 685 steps,
less than 108 is 63728127, which has 949 steps,
less than 109 is 670617279, which has 986 steps,
less than 1010 is 9780657630, which has 1132 steps,[11]
less than 1011 is 75128138247, which has 1228 steps,
less than 1012 is 989345275647, which has 1348 steps,
less than 1013 is 7887663552367, which has 1563 steps,
less than 1014 is 80867137596217, which has 1662 steps,
less than 1015 is 942488749153153, which has 1862 steps,
less than 1016 is 7579309213675935, which has 1958 steps,
less than 1017 is 93571393692802302, which has 2091 steps and
less than 1018 is 931386509544713451, which has 2283 steps.[12]

Final Exercises:
To finish off this section, we have a small programming problem for you to solve. 
Starting with the above code (also available in a file here ), make a program that 
takes as command-line input a single number, representing a number of Collatz steps 
(steps required to reach 1 by following the Collatz procedure), and computes the 
lowest number (starting from 1) which requires this number of Collatz steps. 
For example, if the number input was 949, your program should output 63,728,127; 
similarly, if you input 1132, it should output 9,780,657,630 as the lowest number 
requiring 1132 Collatz steps. Since these are fairly large numbers, and it might 
take your code a very long time to reach them (unless you use a more advanced 
technique, such as in some manner memoizing previous results and efficiently 
checking to see if you've already found the number of steps remaining from a given 
number - but I digress) you can use the following smaller test cases: For an input 
of 6, your code should output 10. For an input of 45, it should output 361. Finally, 
for an input of 260, it should print 18514. Bonus points if you can do it using each 
type of loop in Rust, as well as if you can do it recursively. N.B.: Trying to call 
collatz(0) will result in a stack overflow, as the Collatz sequence is only defined 
for positive integers.
*/