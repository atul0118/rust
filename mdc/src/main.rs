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
    io::stdout().flush();
    io::stdin().read_line(&mut choice).expect("failed to read user input");

    println!("\nFile name: ");
    print!("> ");
    io::stdout().flush();
    io::stdin().read_line(&mut file).expect("failed to read user input");


    println!("file name given by user: {}", file);
    // trim() works on &str, so we need to convert our String to &str
    let match_ret = match choice.as_str().trim() {

        "1" => {
            println!("User wants to Compress a file");
            opr = Operations::Encryption;
        }

        "2" => {
            println!("User wants to Decompress a file");
            opr = Operations::Decryption;
        }

        _ => {
            println!("Invalid choice {}", choice);
            opr = Operations::Invalid;
            ()
        }

    };

    if match_ret == () {
        ();
    }

    println!("Output for user {}", opr.start(file));

}
