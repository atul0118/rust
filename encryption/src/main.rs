use std::io;
use std::io::Write; // for flush()

fn main() {

    // We'll read user input as string.
    // If we want we can convert it to int using:
    // input.trim().parse().expect("error");
    // when we use read_line(), it reads until \n and also
    // stores it in the input var. So we trim it before matching
    let mut choice = String::new();
    println!("Choose one: ");
    println!("1. Compress a file");
    println!("2. Decompress a file");
    print!("> ");
    io::stdout().flush();

    io::stdin().read_line(&mut choice).expect("failed to read user input");
    //println!("User chose: {}", choice);

    // trim() works on &str, so we need to convert our String to &str
    match choice.as_str().trim() {
        "1" => println!("User wants to Compress a file"),
        "2" => println!("User wants to Decompress a file"),
        _ => println!("Invalid choice {}", choice)
    };
}
