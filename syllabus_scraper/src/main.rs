use std::path::Path;
use std::env;
use std::fs::File;
use std::process;

fn read_file(file_path:&Path) -> File{
    match File::open(file_path){
        Ok(f) => return f,
        Err(e) => {
            println!("Error: File Path Not Valid");
            process::exit(1);
        },
    };
}
//description: This function takes in a file path and if it can find the file, returns the file type, otherwise it exits the program

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("Error: Not Enough Arguments. Enter a file path");
        process::exit(1);
    }

    else{
        let file_path_str = &args[1];
        let file_path = Path::new(file_path_str);     
        let file_type = file_path.extension().unwrap().to_str().unwrap();
        
        println!("File Type: {}", file_path_str);
        let file = read_file(file_path);
    }

    //this chunk of code attempts to retrieve the file path from arguments. If the user doesn't put in a file path as an argument, it exits the program. If the file path is valid, it sends the path to the readFile function
}

//to run, add a file path as an argument after cargo run
//example: cargo run "C:\Users\ashka\Downloads\Research_On_Cat_Face_Recognition.pdf"
