use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

include!("../helpers.rs");

const SUCCESS:i32 = 0;
const FAILED:i32 = -1;

/*
 * My code for all the supported encryption types
 */
mod encryption_4bit;

pub fn start_encryption(file_name: String) -> i32
{
    mydbg!("Compression function called for file: {}", file_name);

    let mut data:String = String::new();

    // Better than a HashSet, I can use a simple vector of size 26.
    // Though HashSet servers the purpose but the order of elements in it are not fixed.
    // Instead we can store unique chars in Hashmap with the index
    let mut unique_chars:HashMap<char,i32> = HashMap::new();
    let mut unique_char_len = 0;

    let mut file = match File::open(&file_name) {

        Err(error) => {
            println!("Cannot open file {}: {}", file_name, error);
            return -1;
        },
        Ok(fp) => fp,

    };

    // Read content of file
    match file.read_to_string(&mut data) {

        Err(error) => panic!("Cannot read file {}: {}", file_name, error),
        Ok(_) => mydbg!("Successfully read file {}", file_name),

    }

    // Iterate data in the file and store chars in HashSet, so that we have number of unique
    // characters in the file
    for ch in data.chars() {
        mydbg!("char in file: {} ascii:{}", ch, ch as i32);
        if !unique_chars.contains_key(&ch) {
            unique_chars.insert(ch, unique_char_len);
            unique_char_len += 1;
        }
    }

    mydbg!("Number of unique chars in file(including new line) are: {}", unique_chars.len());

    // set type of compression based on unique chars in the file
    match unique_chars.len() {

        9 ..=16 => {
            mydbg!("4-bit encryption");
            encryption_4bit::encrypt_file(data, unique_chars);
        }

        _ => {
            println!("Unsupported encryption");
            return FAILED;
        }

    }

    SUCCESS
}
