extern crate sdl2;
mod font;
mod processor;
use processor::Processor;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

fn main() {
    println!("start");
    let mut processor = Processor::new();
    println!("created");
    let args: Vec<_> = env::args().collect();
    let instructions = get_file_as_byte_vec(&args[1]);
    processor.load(&instructions);
    while (true) {
        processor.tick();
    }
}
