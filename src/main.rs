use std::io::prelude::*;
use std::env;
use std::fs;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() {
        panic!("Usage: fcap [pcap]")
    }
    let filename = &args[1];
    let fltwrt = get_bytes(filename.to_string());
    create_data(fltwrt);
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
