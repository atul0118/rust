// macro to print to stdout if DEBUG=1 is set from command line
// Eg. DEBUG=1 cargo run
macro_rules! mydbg {

    ($($arg:tt)*) => {

        match std::env::var("DEBUG") {
            Ok(v) => {
                if v == "1" {
                    println!("[DEBUG]: {}",format!($($arg)*));
                }
            },
            Err(_) => {}
        }
    };

}
