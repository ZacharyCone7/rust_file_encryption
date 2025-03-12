use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process;

const XOR_KEY: u32 = 0xDEADBEEF;

fn xor_cipher(data: &[u8], key: u32) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ ((key >> ((i % 4) * 8)) as u8))
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let input_data = match fs::read(input_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read input file '{}': {}", input_path, e);
            process::exit(1);
        }
    };

    let encrypted_data = xor_cipher(&input_data, XOR_KEY);

    let mut output_file = match File::create(output_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create output file '{}': {}", output_path, e);
            process::exit(1);
        }
    };

    if let Err(e) = output_file.write_all(&encrypted_data) {
        eprintln!("Failed to write to output file '{}': {}", output_path, e);
        process::exit(1);
    }

    println!(
        "Encryption successful! Input: '{}', Output: '{}', Key: 0x{:X}",
        input_path, output_path, XOR_KEY
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_cipher() {
        let data = b"hello world";
        let key = XOR_KEY;
        let encrypted = xor_cipher(data, key);
        let decrypted = xor_cipher(&encrypted, key);
        assert_eq!(decrypted, data);
    }
}