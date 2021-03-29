/*! encrypt_decrypt.rs : test for the XOR random encryption and decryption
 * 
 * Given one flat text file, without special symbols (ASCII only)
 * The `split` function will generate random text file with same length of the given text file
 * Then encrypt the given text file using XOR operator with the randon text file
 * 
 * If we have both random text file and encrypt file
 * The `combine_decrypt` function will generate original text file by
 * XOR operation to the random text file and the encrypt file.
 * 
 * See source code on [GitHub](https://github.com/sinnud/rustbook/tree/encrypt_decrypt)
 */
use std::env;
use rand::prelude::*;

/** # encrypt decrypt example
 * code from [web](https://aml3.github.io/RustTutorial/html/02.html)
 * 
 * Code from web can not be compiled. Need re-write.
 * Call `split` to encrypt file into two files with suffix *.share1* and *.share2*
 * Call `combine_decrypt` to decrypt two files into original file
 */
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <inputfile>", args[0]);
        return;
    } 
    let fname = &args[1];
    split(fname.to_owned());
    let f1 = format!("{}.share1", fname);
    let f2 = format!("{}.share2", fname);
    let out_file = format!("{}.origin", fname);
    combine_decrypt(f1, f2, out_file);
}
/** # decryption
 * arguments: input two file names and output file name
 * 
 * Get contents of two input files
 * XOR of the two contentes
 * write to output file
 */
fn combine_decrypt(in_fn1: String, in_fn2: String, out_fn: String) {
    let rand_str = std::fs::read_to_string(&in_fn1).unwrap();
    let encrypted_str = std::fs::read_to_string(&in_fn2).unwrap();
    // println!("Random string length: {}; contents\n{}", rand_str.len(), rand_str);
    // println!("Encrypted string length: {}; contents\n{}", encrypted_str.len(), encrypted_str);
    let original_str = xor(&encrypted_str, &rand_str);
    // println!("original string length: {}; contents\n{}", original_str.len(), original_str);
    std::fs::write(out_fn, original_str).expect("Unable to write file");
}

/** # decryption
 * arguments: input file name
 * 
 * Get contents of the input file
 * Generate random string with same length of the contents of the input file
 * XOR to encrypt the input contents
 * Write to two share files as outputs
 */
fn split(in_fn: String) {
    let contents = std::fs::read_to_string(&in_fn).unwrap();
    // println!("File length: {}; contents:\n{}", contents.len(), contents);
    let f1 = format!("{}.share1", in_fn);
    let f2 = format!("{}.share2", in_fn);
    // println!("output file names: {} and {}", f1, f2);
    let mut rand_vec: Vec<char> = contents.chars().collect();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    for c in &mut rand_vec {
        let r = random::<u8>();
        let idx = r as u32 % CHARSET.len() as u32;
        // println!("This random value: {} and mod to {}.", r, idx);
     *c = CHARSET[idx as usize] as char;
    }
    let rand_str : String = rand_vec.into_iter().collect();
    let encrypted_str = xor(&contents, &rand_str);
    // println!("Random string length: {}; contents\n{}", rand_str.len(), rand_str);
    // println!("Encrypted string length: {}; contents\n{}", encrypted_str.len(), encrypted_str);
    std::fs::write(f1, rand_str).expect("Unable to write file");
    std::fs::write(f2, encrypted_str).expect("Unable to write file");
}

/** # XOR
 * For given two strings with same length
 * XOR each paired character
 * Collect output
 */
fn xor(a: &str, b: &str) -> String {
    let mut char_vec: Vec<char> = b.chars().collect();
    for i in 0..char_vec.len() {
        let c = char_vec[i];
	    char_vec[i] = ((a.as_bytes())[i] ^ (c as u8)) as char;
        // println!("'{}' xor '{}' to '{}'", a.as_bytes()[i] as char, c, char_vec[i]);
    }
    char_vec.into_iter().collect()
}

/* Looks that code from web does not work. Need re-write
use std::rand::random;
use std::os;
use std::io::File;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
        let fname = args[1];
        let path = Path::new(fname.clone());
        let msg_file = File::open(&path);

        match (msg_file) {
            Some(mut msg) => {
                let msg_bytes: Vec<u8> = msg.read_to_end();
                let share1_file 
                       = File::create(&Path::new(fname + ".share1"));
                let share2_file 
                       = File::create(&Path::new(fname + ".share2"));
                
                match (share1_file, share2_file) {
                    (Some(share1), Some(share2)) => { 
                        split(msg_bytes, share1, share2); 
                        } ,
                    (_, _) => fail!("Error opening output files!"),
                }
            } ,
            None => fail!("Error opening message file: {:s}", fname)
        }
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut ret = Vec<u8>;
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn split(msg_bytes: &[u8], mut share1: File, mut share2: File) {
    let mut random_bytes: ~[u8] = ~[];
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    for _ in range(0, msg_bytes.len()) {
	let random_byte = random();
	random_bytes.push(random_byte);
    }
    
    let encrypted_bytes = xor(msg_bytes, random_bytes);
    share1.write(random_bytes);
    share2.write(encrypted_bytes);
}
*/