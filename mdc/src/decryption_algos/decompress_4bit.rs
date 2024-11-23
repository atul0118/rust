use std::collections::HashMap;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn decompress(compressed_file:String, keys:HashMap<i32,char>) -> i32
{
    mydbg!("4bit decompress called");

    let mut compressed_data = String::new();
    let mut uncompressed_data = String::new();
    let mut fp = File::open(compressed_file).unwrap();

    // File where decompressed data will be written
    let mut uncompressed_fp = OpenOptions::new()
                            .create(true)
                            .write(true)
                            .truncate(true)
                            .open("uncompressed").unwrap();

    // Read data of compressed file
    _ = fp.read_to_string(&mut compressed_data);

    for chars in compressed_data.chars() {

        let mut idx:i32;
        let mut ch:char;

        // this will be first character
        idx = chars as i32 & 0x0000000F;
        ch = *keys.get(&idx).unwrap();
        uncompressed_data = format!("{}{}", uncompressed_data, ch); // conacatenat String + String
        mydbg!("decompressed char at idx {} is {}", idx, ch);

        // this will be second character
        idx = chars as i32 & 0x000000F0;
        idx = idx >> 4;
        ch = *keys.get(&idx).unwrap();
        uncompressed_data = format!("{}{}", uncompressed_data, ch);
        mydbg!("decompressed char at idx {} is {}", idx, ch);
    }

    // Write uncompressed_data to uncompressed_fp
    uncompressed_fp.write_all(uncompressed_data.as_bytes()).expect("Failed to write uncompressed data to a file");
    println!("Uncompressed data written to file: \"uncompressed\"");
    0

}
