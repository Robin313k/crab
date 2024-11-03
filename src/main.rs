use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path: &String = &args[1];

        let contents = fs::read_to_string(file_path)
            .expect("Crab couldn't find the specified file!");

        print!("{contents}");
    } else {
        println!("Oh noes! No input was given, therefore crab can't present Anything :(\nDid you mean to specify a file?")
    }
}
