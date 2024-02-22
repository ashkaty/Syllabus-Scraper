use std::path::Path;
use std::env;
use std::fs::File;
use std::process;

fn readFile(filePath:&Path) -> File{
    match File::open(filePath){
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
        let filePath = Path::new("C:/Users/ashka/Documents/Coding/Guitar_Tab_Generator/A2_Flac.mp3");     
        let fileType = filePath.extension().unwrap().to_str().unwrap();

        let file = readFile(filePath);
    }

    //this chunk of code attempts to retrieve the file path from arguments. If the user doesn't put in a file path as an argument, it exits the program. If the file path is valid, it sends the path to the readFile function
}
