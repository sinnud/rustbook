//mod chap2;
// mod chap4;
// mod chap5;
// mod chap6;
// mod chap7;
mod chap8;
fn main() {
    //chap2::main();
    // chap4::main11();
    // chap4::main12();
    // chap4::main13();
    // chap4::main21();
    // chap4::main22();
    // chap4::main31();
    // chap5::main21();
    // chap5::main22();
    // chap5::main23();
    // chap5::main24();
    // chap5::main31();
    // chap5::main32();
    // chap5::main33();
    // chap6::main11();
    // chap6::main12();
    // chap6::main31();
    // chap7::main();
    // chap8::main11();
    // chap8::main21();
    // chap8::main31();
    // c8ex1();
    // c8ex2();
    c8ex3();
}
/*
fn c8ex1(){
    // Given a list of integers, use a vector and return
    // the mean (the average value),
    // median (when sorted, the value in the middle position),
    // and mode (the value that occurs most often; a hash map will be helpful here)
    // of the list.
    let mut v = vec![1i32, 12, 2, 10, 3, 10, 11, 12, 13, 10, 7, 8, 3, 105, 108, 109, 105, 108];
    println!("The original vector is {:?}", v);
    let (mean, median, mode) = chap8::c8ex1(&mut v);
    println!("The vector is {:?}", v);
    println!("The mean is {}, median is {}, and mode is {}.", mean, median, mode);
}
fn c8ex2(){
    // Convert strings to pig latin. The first consonant of each word
    // is moved to the end of the word and “ay” is added, so “first” 
    // becomes “irst-fay.” Words that start with a vowel have “hay”
    // added to the end instead (“apple” becomes “apple-hay”). Keep 
    // in mind the details about UTF-8 encoding!
    // let s = "  initial  contents ".to_string();
    let s = "Convert strings to pig latin. The first consonant of each word".to_string();
    println!("The original string is '{}'", s);
    let t = chap8::c8ex2(s);
    println!("The resultant string is '{}'", t);
}
*/
fn c8ex3(){
    // Using a hash map and vectors, create a text interface
    // to allow a user to add employee names to a department
    // in a company. For example, “Add Sally to Engineering”
    // or “Add Amir to Sales.” Then let the user retrieve a
    // list of all people in a department or all people in
    // the company by department, sorted alphabetically.
    chap8::c8ex3();
}
