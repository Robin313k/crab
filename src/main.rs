use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path: &String = &args[1];

        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        print!("{contents}");
    } else {
        println!("No input was given, therefore Rusty cannot present it :(")
    }
}
