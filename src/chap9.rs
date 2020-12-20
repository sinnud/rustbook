/*
use std::fs::File;
use std::io;
use std::io::Read;

pub fn main21(){
    let rst=read_username_from_file();
    println!("return structure:{:#?}", rst);
    let content=match rst{
        Ok(v) => v,
        Err(e) => {println!("EXIT:{}",e);std::process::exit(1);},
    };
    println!("File contents:{}",content.trim());
}
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn main22(){
    let rst=read_username_from_file2();
    println!("return structure:{:#?}", rst);
    let content=match rst{
        Ok(v) => v,
        Err(e) => {println!("EXIT:{}",e);std::process::exit(1);},
    };
    println!("File contents:{}",content.trim());
}
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn main23(){
    let rst=read_username_from_file3();
    println!("return structure:{:#?}", rst);
    let content=match rst{
        Ok(v) => v,
        Err(e) => {println!("EXIT:{}",e);std::process::exit(1);},
    };
    println!("File contents:{}",content.trim());
}
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
*/
pub fn main24(){
    let rst=read_username_from_file4();
    println!("return structure:{:#?}", rst);
    let content=match rst{
        Ok(v) => v,
        Err(e) => {println!("EXIT:{}",e);std::process::exit(1);},
    };
    println!("File contents:{}",content.trim());
}
fn read_username_from_file4() -> Result<String, std::io::Error> {
    std::fs::read_to_string("hello.txt")
}