use std::collections::HashMap;
//use std::fs::File;

use std::io::Write as IoWrite; // This is a trait, so File type will get all these methods. We
                               // don't need to call IoWrite explicitly, File implements it
                               // implicitly, hence
                               // File object can use methods in these traits directly.
use std::fs::OpenOptions;
use std::fmt::Write;
use std::fs::write as FsWrite;
use std::io;

pub fn encrypt_file(data:String, unique_chars:HashMap<char,i32>) -> i32
{
    let mut num_char = 1;
    let mut multi_chars:u8 = 0; // TODO: change this to char
    let mut compressed_data:String = String::new();

    /*
     * File where key used for compression will be stored
     */
    let mut key_file = String::new();
    let mut compressed_file_name = String::new();

    print!("> Enter file name where compression key will be stored: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut key_file).expect("Failed to read user input");
    key_file = key_file.trim().to_string();
    // TODO: what if file with the keyname already exists?
    // Check if write! will overwrite the file if it already exist

    print!("> Enter file name to store compressed data: ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut compressed_file_name).expect("Failed to read user input");
    compressed_file_name = compressed_file_name.trim().to_string();

    /*
     * File where compressed data will be stored
     */
    let compressed_file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .truncate(true)
                                    .open(compressed_file_name);

    /*
     * Result enum contains 'Ok' and 'Err'
     */
    let mut compressed_file = match compressed_file {

        Ok(fp) => fp,
        Err(err) => panic!("File opened failed: {}", err)

    };

    mydbg!("4bit encryption starts for file");

    /* Store key in 'key_file'. I'll store it as:
     * <char><index>,<char><index>
     */
    let mut keys:String = String::new();
    for (key,value) in unique_chars.iter() {
        write!(&mut keys, "{}:{},", key,value).unwrap();
    }

    _ = FsWrite(&key_file, keys);

    /*
     * We have the data now, we have to do 4 bit compression, hence 1 byte will not contain two
     * characters
     */
    mydbg!("size of data: {}", data.len());
    // Last char in data will be EOF

    /* Iterate through data and get index of char from unique_chars*/
    /* NOTE: String type is not an iterable in itself. Use .chars() or .bytes() */
//    let mut dbg_val = 0;
    for ch in data.chars() {

        let mut idx:i32;
        idx = *unique_chars.get(&ch).unwrap();
        idx = idx & 0x0F;

        mydbg!("Encoding character: {} that has idx:{}", ch, idx);
        // We are at odd number of character in the file
        if num_char & 1 != 0 {
            multi_chars = 0x0F & idx as u8;
        }

        // We are at even number of character in the file
        if num_char != 0 && (num_char & 1 == 0) {
            idx = idx << 4;
            multi_chars |= idx as u8;
            write!(&mut compressed_data, "{}", multi_chars as char).unwrap();
//            mydbg!("\tDEBUG: multi_chars: {} compressd data: {}", multi_chars, compressed_data);
//            mydbg!("\tcompressd data LEN: {}", compressed_data.len());
//            dbg_val += 1;
        }
        num_char += 1;

    }

    if num_char & 1 == 0 {
            write!(&mut compressed_data, "{}", multi_chars as char).unwrap();
//            dbg_val += 1;
//            mydbg!("DEBUG: compressd data: {}", compressed_data);
//            mydbg!("compressd data LEN: {}", compressed_data.len());
    }
//    mydbg!("num chars: {}\ncompressd data len: {}", num_char, dbg_val);
//    mydbg!("compressd data: {}", compressed_data);
//    mydbg!("compressd data LEN: {}", compressed_data.len());
    //write this char to file
    compressed_file.write_all(compressed_data.as_bytes()).expect("write failed!");

    return 0;
}
