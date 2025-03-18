use std::fs::{self, File};
//use std::io::{Read, Write};
use std::env;
use std::path::Path;
use std::process;
use colored::*;

const XOR_KEY: u32 = 0xdeadbeef;

// Function to read the file content
fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        eprintln!("Failed to read input file '{}': {}
                   Current working directory: {}
                   Ensure the file exists and the path is correct.",
                  path, err, env::current_dir().unwrap().display());
        process::exit(1);
    })
}

// XOR encryption function
fn xor_encrypt_decrypt(data: &[u8], key: u32) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ ((key >> (8 * (i % 4))) as u8))
        .collect()
}

// Function to write the encrypted file content
fn write_file(path: &str, data: &[u8]) {
    let parent_dir = Path::new(path).parent();
    let file_stem = Path::new(path).file_stem();

    if let Some(inner) = parent_dir {
        if !inner.exists() {
            if Path::new(path).is_relative() {
                match fs::create_dir_all(inner) {
                    Ok(_) => println!("Adding directory/file to current working directory: {}", env::current_dir().unwrap().display()),
                    Err(err) => eprintln!("Failed to create directory: {}", err),
                }
            } else {
                match fs::create_dir_all(inner) {
                    Ok(_) => println!("Created directory: {}", inner.display()),
                    Err(err) => eprintln!("Failed to create directory: {}", err),
                }
            }
        }
    }

    if file_stem.is_some() && !Path::new(path).exists() {
        match File::create(path) {
            Ok(_) => println!("Created file: {}", path),
            Err(err) => eprintln!("Failed to create file: {}", err),
        }
    }

    fs::write(path, data).unwrap_or_else(|err| {
        eprintln!("Failed to write file: {}\nCurrent working directory: {}
                   Ensure the file exists and the path is correct.",
                  err, env::current_dir().unwrap().display());
        process::exit(1);
    })
}

// Print CLI Application usage
fn print_help_summary() {
    println!("{} encryptor [OPTIONS] --input <input_file> --output <output_file>", "Usage:".green().bold());

    println!(
        "\n{}:
        -c, --cipher <encrypt|e|decrypt|d> Set the cipher type [default: encrypt]
        -i, --input  <input_file> Set the input file
        -o, --output <output_file> Set the output file
        -h, --help    Print this help message (-h for summary)"
        , "Options".bold());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_help_summary();
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let data = read_file(input_file);
    let encrypted_data = xor_encrypt_decrypt(&data, XOR_KEY);
    write_file(output_file, &encrypted_data);

    println!(
        "{}\nInput file: {}\nOutput file: {}\nXOR key: {:#x}",
        "Encryption successful!".green().bold(),
        input_file, output_file, XOR_KEY
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor_encrypt_decrypt() {
        let data = b"Hello, Rust!";
        let encrypted = xor_encrypt_decrypt(data, XOR_KEY);
        let decrypted = xor_encrypt_decrypt(&encrypted, XOR_KEY);
        assert_eq!(data.to_vec(), decrypted);
    }
}
