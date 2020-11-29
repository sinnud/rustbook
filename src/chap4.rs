pub fn main11() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
pub fn main12() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    println!("s1={}", s1);

    let s2 = String::from("hello");     // s2 comes into scope
    println!("s1={}\ns2={}", s1, s2);

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    println!("s3={}", s3);
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
pub fn main13() {
    let s1 = String::from("hello");

    let (s1, len) = calculate_length1(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length1(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

pub fn main21() {
    let s1 = String::from("hello");

    let len = calculate_length2(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

pub fn main22() {
    let mut s = String::from("hello");
    println!("s is '{}'.", s);
    change(&mut s);
    println!("now s is '{}'.", s);
    let r1 = & s;
    let r2 = & s;

    println!("r1 is '{}', r2 is '{}'", r1, r2);
    let r3 = &mut s;

    // no s any more
    //println!("r3 is '{}', s is '{}'", r3, s);
    println!("r3 is '{}'", r3);

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
pub fn main31() {
    let mut s = String::from("hello world");

    let idx = first_word(&s); // word will get the value 5

    let first_word=&s[0..idx];
    let rest_words=&s[(idx+1)..s.len()];
    println!("'{}' has first word '{}' and rest words '{}'", s, first_word, rest_words);
    s.clear(); // this empties the String, making it equal to ""
    //println!("After clear, the first word '{}' and rest words '{}'", first_word, rest_words);

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}