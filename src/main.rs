use std::io::{BufReader, BufWriter, Read, Write};
use rand::Rng;

// 0 is a useless key in terms of encrypting data with XOR operation
const USELESS_ENCRYPTING_KEY: u8 = 0;

const ENCRYPT_USER_OPTION: &str = "e";
const DECRYPT_USER_OPTION: &str = "d";

// This function needed to make getting user input more convenient
// query parameter is a text we printing to console when asking user for input
fn get_input(query: &str) -> std::io::Result<String> {
    print!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    // returning String without redundant spaces and transitions to a new line
    Ok(buffer.trim().to_owned())
}

// This function takes a string with keys string entered by user
// Function processing this string, handles errors and returns u8's vector
// If some key is invalid function replaces it with other random key
fn process_key_string(key_string: String) -> Vec<u8> {
    key_string.split_whitespace().map(|num_string| {
        match num_string.parse::<u8>() {
            Ok(num) => if num != USELESS_ENCRYPTING_KEY {
                num
            } else {
                let random_u8_key: u8 = rand::thread_rng().gen();
                println!(
                    "Key {} is useless for encrypting/decrypting and set to {}",
                    num,
                    random_u8_key
                );

                random_u8_key
            },
            Err(_) => {
                let random_u8_key: u8 = rand::thread_rng().gen();
                println!("Key {} is invalid and set to {}", num_string, random_u8_key);
                random_u8_key
            }
        }
    }).collect()
}

// This function encrypting/decrypting a given data with specified key
// data parameter is a vector of u8 numbers we will process
// key parameter is a encrypting/decrypting key, also u8 number
fn process_file_data(data: Vec<u8>, keys: Vec<u8>) -> Vec<u8> {
    // creating a vector with capacity to avoid memory reallocation
    let mut processed_data = Vec::with_capacity(data.len());

    // encrypting data with XOR operation
    for mut byte in data {
        for key in &keys {
            byte ^= key;
        }

        processed_data.push(byte);
    }

    processed_data
}

// Program entry point
fn main() {
    // Main console program loop
    loop {
        println!("# # # # # # #");

        let input_file_name = match get_input("Enter file name to process: ") {
            Ok(input_file_name) => input_file_name,
            Err(err) => {
                println!("Error getting user input, try again: {}\n", err);
                continue;
            }
        };

        // File to encrypt/decrypt
        let input_file = match std::fs::File::open(&input_file_name) {
            Ok(file) => file,
            Err(err) => {
                println!("Can't open file \"{input_file_name}\": {err}\n");
                continue;
            }
        };

        // String that contains keys list to encrypt/decrypt a file
        let key_string = match get_input("Enter a key for file encryption/decryption: ") {
            Ok(key_string) => key_string,
            Err(err) => {
                println!("Error getting user input, try again: {}\n", err);
                continue;
            }
        };

        // Encrypting/decrypting key
        let mut keys = process_key_string(key_string);

        // Key may be changed so we should show it to the user
        println!("Your key is \"{}\"", keys
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join(" ")
        );

        let mut reader = BufReader::new(input_file);

        // Temporary storage for file data in bytes
        let mut input_data = Vec::new();

        // Getting input file data to process sooner
        if let Err(err) = reader.read_to_end(&mut input_data) {
            println!("Failed to read input file: {}\n", err);
            continue;
        }

        // Encrypted/decrypted file data, ready to write to another file
        let processed_data = match get_input(
            format!("\nWhat to do with file encrypt/decrypt? ({}, {}) ",
                    ENCRYPT_USER_OPTION,
                    DECRYPT_USER_OPTION
            ).as_str()
        ) {
            Ok(mut option) => {
                option = option.to_lowercase();

                if option == ENCRYPT_USER_OPTION {
                    process_file_data(input_data, keys)
                } else if option == DECRYPT_USER_OPTION {
                    keys.reverse();
                    process_file_data(input_data, keys)
                } else {
                    println!("Unknown option, try again");
                    continue;
                }
            }
            Err(err) => {
                println!("Error getting user input, try again: {}\n", err);
                continue;
            }
        };

        // File to store processed data in
        let output_file_name = match get_input("Enter file name to output: ") {
            Ok(output_file_name) => output_file_name,
            Err(err) => {
                println!("Error getting user input, try again: {}\n", err);
                continue;
            }
        };

        // Encrypted/decrypted file
        let output_file = match std::fs::File::create(&output_file_name) {
            Ok(file) => file,
            Err(err) => {
                println!("Can't create file \"{}\": {}\n", output_file_name, err);
                continue;
            }
        };

        let mut writer = BufWriter::new(output_file);

        // Writing encrypted/decrypted data to output file
        if let Err(err) = writer.write_all(&processed_data) {
            println!("Failed to write to output file: {}\n", err);
            continue;
        }

        // New line for better console UI
        println!();
    }
}
