
use std::io::{self, Write};
use std::io::Cursor;
use murmur3::murmur3_32;

const M : usize = 1024;
const K : u32 = 35;

type BloomFilter = [u8; M];

fn get_input(msg : String) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input_string : String = String::from("");
    io::stdin().read_line(&mut input_string).unwrap();
    input_string = String::from(input_string.trim());
    return input_string;
}

fn hash_string(x: String) -> [u32; K as usize] {
    let mut hashes : [u32; K as usize] = [0; K as usize];
    for k in 0..K {
        let hash = murmur3_32(&mut Cursor::new(x.clone()), k).unwrap(); 
        hashes[k as usize] = hash % (M as u32);
    }
    return hashes;
}

fn insert(x: String, bloom_filter: &mut BloomFilter) {
    let hashes = hash_string(x);
    for i in hashes {
        bloom_filter[i as usize] = 1;
    }
}

fn lookup(x: String, bloom_filter: &mut BloomFilter) -> bool {
    let hashes = hash_string(x);
    for i in hashes {
        if bloom_filter[i as usize] == 0 {
            return false;
        }
    }
    return true;
}

fn handle_insert(mut bloom_filter: &mut BloomFilter) {
    let user_input = get_input(String::from("Value to insert: "));
    insert(user_input.clone(), &mut bloom_filter);
    println!("=== Inserted {} ===", user_input);
}

fn handle_lookup(mut bloom_filter: &mut BloomFilter) {
    let user_input = get_input(String::from("Value to lookup: "));
    match lookup(user_input.clone(), &mut bloom_filter) {
        true => println!("=== Value {} found! ===", user_input),
        false => println!("=== Value {} not found! ===", user_input)
    }
}

fn main() {
    println!("
============== Bloom Filter ===============
insert [string] - insert [string] to filter
lookup [string] - lookup [string] in filter
exit            - exit program
===========================================");

    let mut bloom_filter : BloomFilter = [0; M];
   
    let mut user_input : String = String::from("");
    while user_input != "exit" {
        user_input = get_input(String::from("Command: ")); 

        match user_input.clone().to_lowercase().as_str() {
            "insert" => handle_insert(&mut bloom_filter),
            "lookup" => handle_lookup(&mut bloom_filter),
            "exit" => (),
            _ => println!("Unrecognized command :(")
        }

    }

    println!("Exiting...");
}
