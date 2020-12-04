/*
pub fn main11() {
    let mut v = vec![1, 2, 3];
    println!("The vector v is {:#?}", v);
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("The vector v is {:#?}", v);
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let first = &v[0];
    // let first = v.get(0).cloned();
    let first = v.last().cloned();

    v.push(6);

    println!("The first element is: {:?}", &first);
    for i in &v {
        println!("{}", i);
    }
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("now {}", i);
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for i in &row {
        println!("{:#?}", i);
    }
}
pub fn main21() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("Now string is {}", s);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("Now s3 is {}", s3);
    let char3: Vec<char> = s3.chars().collect::<Vec<_>>();
    println!("char3 is {:#?}", char3);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Now s is {}", s);

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("Now s is {}", s);

    let s = format!("{}-{}", s2, s3);
    println!("Now s is {}", s);

    let hello = "Здравствуйте";

    let s = &hello[0..6];
    println!("Now hello is {} (length {}) and s is {} (length {})", hello, hello.len(), s, s.len());
}
pub fn main31() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("dictionary scores is {:#?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    scores.insert(String::from("Green"), 30);
    println!("dictionary scores is {:#?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("dictionary map is {:#?}", map);
    println!("now field_name is {:#?} field_value is {:?}", field_name, field_value);


    println!("get dictionary element value for Blue: {:#?}", scores.get(&String::from("Blue")));
    for (key, value) in &scores {
        println!("scores 1 -> {}: {}", key, value);
    }
    scores.insert(String::from("Blue"), 25);
    for (key, value) in &scores {
        println!("scores 2 -> {}: {}", key, value);
    }
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    for (key, value) in &scores {
        println!("scores 3 -> {}: {}", key, value);
    }
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("now map is {:?}", map);

}

pub fn c8ex1(v1: &mut Vec<i32>)
-> (f64, f64, i32)
{
    // Given a list of integers, use a vector and return
    // the mean (the average value),
    // median (when sorted, the value in the middle position),
    // and mode (the value that occurs most often; a hash map will be helpful here)
    // of the list.
    //println!("In chap8::c8ex1, Vector length is {}.", v1.len());
    //v1.push(42);
    let mut sum=0i32;
    for x in v1.iter() {
        sum += x;
    }
    //println!("In chap8::c8ex1, Vector sum is {}.", sum);
    let mean=sum as f64/v1.len() as f64;
    extern crate quickersort;
    // need register on Cargo.toml
    quickersort::sort(&mut v1[..]);
    let med_idx:usize=v1.len() as usize / 2;
    //println!("In chap8::c8ex1, Vector length is {}, med_idx={}.", v1.len(), med_idx);
    let median; // make it in scope
    if med_idx * 2 + 1 == v1.len() as usize{
        median=v1[med_idx] as f64;
    }
    else{
        median=(v1[med_idx-1]+v1[med_idx]) as f64 / 2 as f64;
    }
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for word in v1 {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    //println!("In chap8::c8ex1, map is {:?}.", map);

    let maxtuple = map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let mode=**maxtuple.0;
    //println!("In chap8::c8ex1, {:?} and ({}, {})", maxtuple, mode, freq);
    (mean, median, mode)
}
pub fn c8ex2(v: String)
-> String
{
    // Convert strings to pig latin. The first consonant of each word
    // is moved to the end of the word and “ay” is added, so “first”
    // becomes “irst-fay.” Words that start with a vowel have “hay”
    // added to the end instead (“apple” becomes “apple-hay”). Keep
    // in mind the details about UTF-8 encoding!
    let mut word_list: Vec<&str> = v.split(' ').collect();
    // println!("In chap8::c8ex2, {:#?}", word_list);
    word_list.retain(|&i|i.len() > 0);
    // println!("In chap8::c8ex2, {:#?}", word_list);
    let mut out_list = vec![];
    let ay:&str="ay";
    let dash:&str="-";
    let hay:&str="-hay";
    for word in word_list{
        // println!("In chap8::c8ex2, {} with first char {} and rest {}.",
        //           word, &word[0..1], &word[1..]);
        if "aeiou".contains(&word[0..1]) {
            let word=format!("{}{}", &word, hay);
            out_list.push(word);
        }
        else {
            let firstchar=&word[0..1];
            let word=format!("{}{}{}{}", &word[1..], dash, firstchar, ay);
            out_list.push(word);
        }
    }
    // println!("In chap8::c8ex2, {:#?}", out_list);
    out_list.join(" ")
}
*/
pub fn c8ex3(){
    // Using a hash map and vectors, create a text interface
    // to allow a user to add employee names to a department
    // in a company. For example, “Add Sally to Engineering”
    // or “Add Amir to Sales.” Then let the user retrieve a
    // list of all people in a department or all people in
    // the company by department, sorted alphabetically.
    use std::collections::HashMap;
    let mut emp_dpt = HashMap::new();

    use std::path::Path;
    use std::fs;
    use std::env;
    let filename="artifact/c8ex3.txt";
    let f = Path::new(filename);
    if f.is_file(){
        let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
        let mut line_list: Vec<&str> = contents.split('\n').collect();
        line_list.retain(|&i|i.len() > 0);
        for line in line_list {
            let word_list: Vec<&str> =line.split(":").collect();
            let (key, value)=(word_list[0], word_list[1]);
            emp_dpt.insert(key.to_owned(), value.to_owned());
        }
        println!("From file {} loaded {} records."
        , filename
        , emp_dpt.keys().len());
    }
    else {
        println!("File {:?}/{} does not exist!", env::current_dir(), filename);
    }
    use std::io;

    loop {
        println!("Please input employee name (Ctrl+c exit; STOP or LIST):");
        let mut stdtmp = String::new();
        io::stdin()
            .read_line(&mut stdtmp)
            .expect("Failed to read line");
        let employee=stdtmp.trim().to_owned(); // remove new line symbol
        // println!("You input employee: '{}'", employee);
        if employee == "STOP" {
            break;
        }
        else if employee == "LIST" {
            println!("Current list: {:#?}", emp_dpt);
            continue;
        }
        println!("Please input department name (Ctrl+c exit; STOP or QUIT):");
        stdtmp=String::from("");
        io::stdin()
            .read_line(&mut stdtmp)
            .expect("Failed to read line");
        let department=stdtmp.trim().to_owned();
        // println!("You input employee: '{}', department: '{}'", employee, department);
        if department == "STOP" || department == "QUIT" {
            break;
        }
        emp_dpt.insert(employee.to_owned(), department.to_owned());

    }
    if emp_dpt.keys().len() > 0 {
        println!("Write to file...");
        let mut stdtmp = String::new();
        for (key, value) in emp_dpt {
            stdtmp=format!("{}{}:{}\n", stdtmp, key, value);
        }
        fs::write(filename, stdtmp).expect("Unable to write file");
    }
}