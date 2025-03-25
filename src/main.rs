use std::fs::{self, File}; // File handling modules
use std::env; // Environment module to get current working directory
use std::path::Path; // Path module to handle file paths
use std::process; // Process module to allow program exits
use colored::*; // For colored text in CLI output

// Define a fixed XOR key used for encryption and decryption
const XOR_KEY: u32 = 0xdeadbeef;

// Reads the content of a file and returns it as a vector of bytes.
// If the file cannot be read, it prints an error message and exits the program.
fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        eprintln!("Failed to read input file '{}': {}\n\
                   Current working directory: {}\n\
                   Ensure the file exists and the path is correct.",
                  path, err, env::current_dir().unwrap().display());
        process::exit(1); // Exit with non-zero status code to indicate failure
    })
}

// Encrypts or decrypts the data using XOR cipher.
// The same function is used for both encryption and decryption since XOR is symmetric.
fn xor_encrypt_decrypt(data: &[u8], key: u32) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ ((key >> (8 * (i % 4))) as u8))
        .collect()
}

// Writes the encrypted (or decrypted) data to the specified file.
// If the parent directory doesn't exist, it creates the directory.
// Provides detailed error messages if writing fails.
fn write_file(path: &str, data: &[u8]) {
    let parent_dir = Path::new(path).parent(); // Get parent directory path
    let file_stem = Path::new(path).file_stem(); // Get the file name without extension

    // Create the parent directory if it doesn't exist
    if let Some(inner) = parent_dir {
        if !inner.exists() {
            if Path::new(path).is_relative() {
                // Create directory relative to current working directory
                match fs::create_dir_all(inner) {
                    Ok(_) => println!("Adding directory/file to current working directory: {}", env::current_dir().unwrap().display()),
                    Err(err) => eprintln!("Failed to create directory: {}", err),
                }
            } else {
                // Create absolute directory path
                match fs::create_dir_all(inner) {
                    Ok(_) => println!("Created directory: {}", inner.display()),
                    Err(err) => eprintln!("Failed to create directory: {}", err),
                }
            }
        }
    }

    // Create file if it doesn't exist
    if file_stem.is_some() && !Path::new(path).exists() {
        match File::create(path) {
            Ok(_) => println!("Created file: {}", path),
            Err(err) => eprintln!("Failed to create file: {}", err),
        }
    }

    // Write data to file
    fs::write(path, data).unwrap_or_else(|err| {
        eprintln!("Failed to write file: {}\nCurrent working directory: {}\n\
                   Ensure the file exists and the path is correct.",
                  err, env::current_dir().unwrap().display());
        process::exit(1); // Exit with non-zero status code to indicate failure
    })
}

// Prints a brief summary of how to use the CLI application.
fn print_help_summary() {
    println!("{} encryptor <input_file> <output_file>", "Usage:".green().bold());
}

// Main function: handles argument parsing and program flow.
fn main() {
    // Collect command line arguments into a vector
    let args: Vec<String> = env::args().collect();
    // If there are fewer than 3 arguments, display help and exit
    if args.len() < 3 {
        print_help_summary();
        process::exit(1);
    }

    // First argument = input file, second argument = output file
    let input_file = &args[1];
    let output_file = &args[2];

    // Read the content of the input file
    let data = read_file(input_file);
    // Encrypt or decrypt the file content using the XOR key
    let encrypted_data = xor_encrypt_decrypt(&data, XOR_KEY);
    // Write the encrypted content to the output file
    write_file(output_file, &encrypted_data);

    // Print success message
    println!(
        "{}\nInput file: {}\nOutput file: {}\nXOR key: {:#x}",
        "Encryption successful!".green().bold(),
        input_file, output_file, XOR_KEY
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the encryption and decryption functions
    #[test]
    fn test_xor_encrypt_decrypt() {
        let data = b"Hello, Rust!"; // Sample data
        let encrypted = xor_encrypt_decrypt(data, XOR_KEY); // Encrypt data
        let decrypted = xor_encrypt_decrypt(&encrypted, XOR_KEY); // Decrypt data
        assert_eq!(data.to_vec(), decrypted); // Ensure original matches decrypted
    }
}
