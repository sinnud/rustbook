pub fn main11() {

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
            println!("Current message is: {:#?}", self);
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

// learn more about Option enum from 
// https://www.ameyalokare.com/rust/2017/10/23/rust-options.html
//
// cool function as_ref()
//
#[derive(Debug)]
struct FullName {
    first: String,
    middle: Option<String>,
    last: String,
}
pub fn main12(){
    let alice = FullName {
        first: String::from("Alice"),
        middle: Some(String::from("Bob")), // Alice has a middle name
        last: String::from("Smith")
    };
    
    let jon = FullName {
        first: String::from("Jon"),
        middle: None, // Jon has no middle name
        last: String::from("Snow")
    };
    let am=alice.middle.as_ref().unwrap();
    println!("Alice's middle name is {}", am); // prints Bob
    //println!("Jon's middle name is {}", jon.middle.unwrap()); // panics
    println!("Jon's middle name is {}",
        match jon.middle {
            None => "No middle name!",
            Some(ref x) => x,
        }
    );
    println!("Alice's middle name is {}",
        alice.middle.as_ref().unwrap_or(&"No middle name!".to_owned()));
    println!("Jon's middle name is {}",
        jon.middle.as_ref().unwrap_or(&"No middle name!".to_owned()));
    println!("Alice's name is {:#?}", alice); // prints full name
    println!("Jon's name is {:#?}", jon); // prints full name
    println!(
        "Alice's full name is {} {} {}",
        alice.first,
        alice.middle.as_ref().map(|m| &m[0..1]).unwrap_or(""), // as_ref() converts Option<String> to Option<&String>
        alice.last
    );
    println!(
        "Jon's full name is {} {} {}",
        jon.first,
        jon.middle.as_ref().map(|m| &m[0..1]).unwrap_or(""), // as_ref() converts Option<String> to Option<&String>
        jon.last
    );
    let optional_nickname = alice.middle.as_ref().and_then(|m| get_nickname(&m));
    println!("Alice's middle name's nickname is {}",
        optional_nickname.unwrap_or("(none found)")); // prints "The Builder"
    let optional_nickname = jon.middle.as_ref().and_then(|m| get_nickname(&m));
    println!("Jon's middle name's nickname is {}",
        optional_nickname.unwrap_or("(none found)")); // prints "(none found)"
}
fn get_nickname(name: &str) -> Option<&str> {
    match name {
        "Bob" => Some("The Builder"),
        "Elvis" => Some("The King"),
        _ => None,
    }
}
pub fn main31(){
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    if some_u8_value == Some(3) {println!("It is three.")}
    if let Some(3) = some_u8_value {
        println!("three");
    }
}