use std::env;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use filebuffer::FileBuffer;

extern crate crypto;
extern crate filebuffer;

fn main() {
    for fname in env::args().skip(1) {
        let fbuffer = FileBuffer::open(&fname).expect("failed to open file");
        let mut hasher = Sha256::new();
        hasher.input(&fbuffer);
        println!("{}  {}", hasher.result_str(), fname);
    }
}
    

