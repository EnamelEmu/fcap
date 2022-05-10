use std::io::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use rand::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        panic!("Usage: fcap [pcap]")
    }
    let filename = &args[1];
    let fltwrt = get_bytes(filename.to_string());
    create_data(fltwrt);
    create_magic();
}

fn create_magic() {
    let magic_bytes: Vec<u32> =
        vec![0xFF,
             0x7F,
             0x00,
             0xFFFF,
             0x0000,
             0xFFFFFFFF,
             0x00000000,
             0x80000000,
             0x40000000,
             0x7FFFFFFF];
    let testvalue: u8 = rand::random::<u8>() % 11;
    println!("{}", testvalue)

}
fn create_data(data: Vec<u8>)  {
    let mut buffer = File::create("mutated.pcap").expect("error creating mutated.pcap");
    buffer.write_all(&data).expect("Error writing to mutated");

}

fn get_bytes(filename: String) -> Vec<u8>
{
    let contents = fs::read(filename).expect("Error reading filename");
    println!("{:?}", contents);
    contents
}
