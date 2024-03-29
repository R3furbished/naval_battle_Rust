use std::io;

fn main() {
    //main needs a loop to check user input against possible commands
    //TODO add possible commands.

    loop {
        let mut raw_in = String::new();
        io::stdin()
            .read_line(&mut raw_in)
            .expect("Failed to get input from usr.");

        match raw_in.trim() {
            "quit" => break,
            _ => println!("Continue"),
        }
    }
}
