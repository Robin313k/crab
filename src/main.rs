use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file_path: &String = &args[1];

        let contents = fs::read_to_string(file_path)
            .expect("Crab couldn't find the specified file!");

        print!("{contents}");
    } else if args.len() > 2 {
        for i in 1..args.len() {
            let file_path: &String = &args[i];

            let contents = fs::read_to_string(file_path)
                .expect("Crab couldn't find the specified file!");
            
            println!("###########");
            println!("{file_path}");
            println!("###########");
            print!("{contents}");
        }
    } else {
        println!("Oh no! No input was given, therefore crab can't present Anything :(\nDid you mean to specify a file?")
    }
}
