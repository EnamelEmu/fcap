use std::io::prelude::*;
use std::env;
use std::fs;
use std::process::exit;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: fcap [file_to_read]");
        exit(1);
    }
    let filename = &args[1];
    let fltwrt = get_bytes(filename.to_string());
    let corrupt_file = corrupt_byte(fltwrt);
    create_data(corrupt_file, filename);

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
fn create_data(data: Vec<u8>, file_destination: &String)  {
    let formated_file_dest = format!("mutated{}",file_destination);
    let mut buffer = File::create(formated_file_dest).expect("error creating destination file");
    buffer.write_all(&data).expect("Error writing to destination file");

}

fn corrupt_byte(mut data: Vec<u8>) -> Vec<u8> {
    // this kinda sucks but basically it will get a precentage (of at minumin one) so it can overwrite the bytes over
    let data_length: f32 = data.len() as f32;
    let n = f32::max(1.0 ,data_length * 0.01); // if the file is big corrupt (datalenght * 0.1)% of it, otherwise, just corrupt 1%
    let mut n_uint: u32 = n as u32;
    while n_uint != 0 {
        let mut start_point: usize = rand::random::<usize>() % data_length as usize;
        for byte in create_magic().into_iter() {
            match data.get(start_point) {
                Some(_) => { data[start_point] = byte; }
                None => break,
            }
            start_point += 1;
        }
        n_uint -= 1

    }
   data

}

fn get_bytes(filename: String) -> Vec<u8> {
    fs::read(filename).expect("Error reading filename")
}
