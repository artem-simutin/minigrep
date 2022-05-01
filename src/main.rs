use core::arch;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let requested_path = &args[0];
    let filename = &args[1];

    println!("Path: {:?}", requested_path);
    println!("Filename: {:?}", filename);
}
