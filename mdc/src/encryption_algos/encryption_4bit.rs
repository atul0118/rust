use std::collections::HashMap;
use std::fs::File;
use std::io::Write as IoWrite; // This is a trait, so File type will get all these methods. We
                               // don't need to call IoWrite explicitly, File implements it
                               // implicitly, hence
                               // File object can use methods in these traits directly.
use std::fs::OpenOptions;
use std::fmt::Write;
use std::fs::write as FsWrite;

pub fn encrypt_file(data:String, unique_chars:HashMap<char,i32>) -> i32
{
    let mut num_char:u8 = 1;
    let mut multi_chars:u8 = 0; // TODO: change this to char
    let mut compressed_data:String = String::new();

    /*
     * File where key used for compression will be stored
     */
    let mut key_file = "key_4bit_compression";

    /*
     * File where compressed data will be stored
     */
    let mut compressed_file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .truncate(true)
                                    .open("compressed_file");

    /*
     * Result enum contains 'Ok' and 'Err'
     */
    let mut compressed_file = match compressed_file {

        Ok(fp) => fp,
        Err(err) => panic!("File opened failed: {}", err)

    };


//    let mut file2 = OpenOptions::new()
//            .create(true) // create file if it doesn't exist
//            .append(true)
//            .open("atul_file");

    println!("4bit encryption starts for file");

    /* Store key in 'key_file'. I'll store it as:
     * <char><index>,<char><index>
     */
    let mut keys:String = String::new();
    for (key,value) in unique_chars.iter() {
        write!(&mut keys, "{}{},", key,value).unwrap();
    }
    FsWrite(&key_file, keys);

    /*
     * We have the data now, we have to do 4 bit compression, hence 1 byte will not contain two
     * characters
     */
    println!("size of data: {}", data.len());
    // Last char in data will be EOF

    /* Iterate through data and get index of char from unique_chars*/
    /* NOTE: String type is not an iterable in itself. Use .chars() or .bytes() */
    for ch in data.chars() {
        let mut idx:i32;
        idx = *unique_chars.get(&ch).unwrap(); // TODO: Change this to check for case if key is not present

        if num_char & 1 as u8 != 0 {
           idx = &idx & 0x0F;
           multi_chars = 0x0F & idx as u8;
        }

        if num_char != 0 && (num_char & 1 == 0) {
            idx = idx << 4;
            multi_chars |= idx as u8;
            write!(&mut compressed_data, "{}", multi_chars).unwrap();
        }
        num_char += 1;
    }
    println!("compressd data: {}", compressed_data);
    //write this char to file
    compressed_file.write_all(compressed_data.as_bytes()).expect("write failed!");

    /*
     * We'll use char + index from unique_chars to encode data.
     * This info of char+index will have to be stored as well, which will later be used for
     * decompression
     */
    return 0;
}
