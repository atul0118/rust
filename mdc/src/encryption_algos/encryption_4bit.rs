use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub fn encrypt_file(data:String, unique_chars:HashMap<char,i32>) -> i32
{
    let mut num_char:u8 = 0;
    let mut multi_chars:u8 = 0;
    let mut compressed_chars:char = 0 as char;
    let mut key_file:File = File::create("key_4bit_compression").expect("Cannot create file to store key");
    let mut compressed_file:File = File::create("4bit_compressed_file").expect("Cannot create compressed file");

    println!("4bit encryption starts for file");

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
        idx = *unique_chars.get(&ch).unwrap(); // TODO: Change this to check for case if key is not
                                            // present
        if num_char & 1 as u8 != 0 {
           idx = &idx & 0x0F;
           multi_chars = 0x0F & idx as u8;
        }

        if num_char != 0 && (num_char & 1 == 0) {
            idx = idx << 4;
            multi_chars |= idx as u8;
            compressed_chars = multi_chars as char;
            //write this char to file
            key_file.write(&compressed_chars).expect("Failed to write in key file");
        }
    }

    /*
     * We'll use char + index from unique_chars to encode data.
     * This info of char+index will have to be stored as well, which will later be used for
     * decompression
     */
    return 0;
}
