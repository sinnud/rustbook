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
