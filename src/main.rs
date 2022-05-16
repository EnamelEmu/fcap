use std::io::prelude::*;
use std::env;
use std::fs;
use std::process::exit;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: fcap [pcap]");
        exit(1);
    }
    let filename = &args[1];
    let fltwrt = get_bytes(filename.to_string());
    corrupt_byte(fltwrt, 0.01);
}

fn create_magic() -> Vec<u8> {
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
    let rand_num: usize = rand::random::<usize>() % 10;
    let magic_value: u32 = magic_bytes[rand_num];
    println!("{}", magic_value);
    if magic_value == 0 {
        let rand_num: u8 = rand::random::<u8>() % 3;
        match rand_num {
            0 => return vec![0; 1],
            1 => return vec![0; 2],
            2 => return vec![0; 4],
            _ => panic!("over"), //I think this is unreacheable
        }
    }
    else {
        match magic_value {
            1..=255 => magic_value.to_be_bytes().to_vec().drain(3..).as_slice().to_vec(), // this is terrible
            256..=65535 => magic_value.to_be_bytes().to_vec().drain(2..).as_slice().to_vec(), // this is terrible
            _ => magic_value.to_be_bytes().to_vec()
        }
    }
}
fn create_data(data: Vec<u8>)  {
    let mut buffer = File::create("mutated.pcap").expect("error creating mutated.pcap");
    buffer.write_all(&data).expect("Error writing to mutated");

}

fn corrupt_byte(data: Vec<u8>, percentage: f32) {
    // this kinda sucks but basically it will get a precentage (of at minumin one) so it can overwrite the bytes over
    let magic_to_u8_vec: Vec<u8> = create_magic();
    println!("{:?}",magic_to_u8_vec);
    let n = f32::max(1.0, data.len() as f32 * percentage);
}

fn get_bytes(filename: String) -> Vec<u8>
{
    let contents = fs::read(filename).expect("Error reading filename");
    println!("{:?}", contents);
    contents
}
