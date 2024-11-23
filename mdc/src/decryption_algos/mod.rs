//use std::fs::metadata as Metadata;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io;

include!("../helpers.rs");

mod decompress_4bit;

pub fn start_decryption(file:String) -> i32
{
    mydbg!("Decrypt function called for file: {}", file);

    /*
     * We can know type of decompression by reading the file where key is stored
     */
    let mut key_file = String::new();
    print!("> Enter the file name that contains compression key: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut key_file).expect("Failed to read user input");
    key_file = key_file.trim().to_string();

    //let key_file_metadata = Metadata(&key_file).unwrap();
    //mydbg!("key file \'{}\' has length={}", key_file, key_file_metadata.len());

    // Read the content of key_file in a HashMap.
    let mut keys:HashMap<i32,char> = HashMap::new();
    let mut key_file_fp = File::open(key_file).unwrap();
    let mut key_file_data = String::new();
    _ = key_file_fp.read_to_string(&mut key_file_data);

    for values in key_file_data.split(',') {

        let char_and_index:Vec<_> = values.split(":").collect();
        let key:char = match char_and_index[0].chars().next() {
            Some(c) => c,
            None => continue,
        };

        // use of if-let
        //if let Some(c) = char_and_index[0].chars().next() {
        //    // key should be mutable
        //    key = c;
        //}
        //else {
        //    continue;
        //}

        keys.insert(char_and_index[1].parse().unwrap(), key);

    }

    mydbg!("keys HashMap size: {}", keys.len());

    match keys.len() {

        9..=16 => {
            decompress_4bit::decompress(file, keys)
        }

        _ => {
            panic!("Unsupported decompression type {}-bit", keys.len());
        }

    }

}
