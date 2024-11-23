use std::io;
use std::io::Write; // for flush()

/*
 * My code in other files
 */
mod encryption_algos;
mod decryption_algos;

enum Operations {
    Encryption,
    Decryption,
    Invalid
}

impl Operations {

    fn name(&self) -> &str {
        match self {
            &Operations::Encryption => "Compression",
            &Operations::Decryption => "Decompression",
            _ => "Invalid"
        }
    }

    fn start(&self, file:String) -> i32 {

        match self {
            &Operations::Encryption => encryption_algos::start_encryption(file),
            &Operations::Decryption => decryption_algos::start_decryption(file),
            _ => {
                println!("This should not print!");
                -1
            }
        }
    }

}

fn main() {

    // We'll read user input as string.
    // If we want we can convert it to int using:
    // input.trim().parse().expect("error");
    // when we use read_line(), it reads until \n and also
    // stores it in the input var. So we trim it before matching
    let mut choice = String::new();
    let mut file = String::new();
    let opr:Operations; // We are not initializing here, so no need to define it as mut

    println!("Choose one: ");
    println!("1. Compress a file");
    println!("2. Decompress a file");
    print!("> ");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut choice).expect("failed to read user input");

    // trim() works on &str, so we need to convert our String to &str
    let match_ret = match choice.as_str().trim() {

        "1" => {
            print!("\n> Enter file to compress: ");
            opr = Operations::Encryption;
        }

        "2" => {
            print!("\n> Enter file to decompress: ");
            opr = Operations::Decryption;
        }

        _ => {
            println!("Invalid choice {}", choice);
            opr = Operations::Invalid;
            ()
        }

    };

    let _ = io::stdout().flush(); // assigning it just to avoid warning during compilation
    io::stdin().read_line(&mut file).expect("failed to read user input");
    // read_line() will contain \n also at the end of file name, so trim it
    file = file.as_str().trim().to_string();

    if match_ret == () {
        ();
    }

    match opr.start(file) {
        0 => println!("{} completed successfully!", opr.name()),
        _ => println!("{} failed!", opr.name())
    }

}
