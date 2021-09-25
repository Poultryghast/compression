use compression::huffman;
use std::io::{self, Write, Read};
use std::env;

fn main() {
    //let text = include_bytes!("../corpus/alice.txt");
    //test(text);

    let mut buffer: Vec<u8> = Vec::new();
    io::stdin().read_to_end(&mut buffer).ok();
    let args: Vec<String> = env::args().collect();

    match &args[1][..] {
        "c" => compress(&buffer),
        "d" => decompress(buffer),
        _ => println!("Not an option.")
    }
}

fn compress(bytes: &[u8]) {
    let bits = huffman::compress::compress(bytes);
    let file = compression::wrap_bits(bits);
    io::stdout().write_all(&file).ok();
}

fn decompress(bytes: Vec<u8>) {
    let bits = compression::unwrap_bytes(bytes);
    let bytes = huffman::decompress::decompress(bits);
    io::stdout().write_all(&bytes).ok();
}

