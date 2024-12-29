use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::io::BufReader;
// use sql_jr_execution;

const HISTORY_FILE: &str = "./history.txt";

fn main() -> Result<()> {
    let mut rl = Editor::<(),rustyline::history::FileHistory>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }


    let mut file = OpenOptions::new().read(true).write(true).create(true).open("file.txt").unwrap();
    // let mut file = OpenOptions::new().write(true).create(true).open("file.txt").unwrap();
    
    // THIS BELOW PREVENTS THE ERROR BUT ONLY WORKS AS READER!
    // let mut file = File::open("file.txt")?;
    
    // let file = OpenOptions::new()
    //     .write(true)
    //     .create(true)
    //     .open("file.txt")?; // File::open("List.csv")?;
    
    
    // let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    
    
    let reader = BufReader::new(&file);

    for line in reader.lines().take(10){
        match line { 
            Ok(line) => {lines.push(line)},
            Err(error) => {println!("Error reading line: {}", error)}
        }
    };


    // drop(reader); // Close the file

    // let mut exec = sql_jr_execution::Execution::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match line.as_str() {
                    "" => (),
                    "help" => display_help(),
                    "exit" => {
                        println!("Goodbye!");
                        break
                    },
                    "Make a sandwich" => {
                        println!("Making a sandwich...");
                    }
                    //  _ => {
                    //     rl.add_history_entry(line.as_str())?;
                    //     // 
                    //     let lineRef: &str = line.as_ref();
                    //     let res = exec.parse_and_run(lineRef);

                    //     match res {
                    //         Ok(exec_res) => println!("Added content: {}", exec_res),
                    //         Err(err) => println!("{}", err)
                    //     }
                    //     // 
                    //     // println!("Added line: {}", line);
                    // },

                    _ => {
                        let inputs: Vec<&str> = line.split_whitespace().collect();
                        match (inputs[0],inputs[1].parse::<i32>().unwrap()) {
                            ("read",x) if x>=0 => {
                                println!("Reading content... \n{}", lines[x as usize]);
                                // print!("{}", lines[x as usize]);
                            },
                            ("write",x) if x>=0 => {
                                // file.write_all(b"Hello, world!\n")?;
                                println!("Writing content...");
                                write!(file, "{}\n", inputs[1]).expect("Unable to write to file!");
                            },
                            _ => {
                                println!("Invalid command");
                            }
                        }

                    }
                    
                }
                
            },
            Err(ReadlineError::Interrupted) => {
                // CTRL-C so just skip
            },
            Err(ReadlineError::Eof) => {
                // CTRL-D so exit
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(HISTORY_FILE)
}


fn display_help(){
    let help: &str = "
    <String> - String input

    read-<integer> - Read content
    write-<string> - Write content

    Commands

    help   - Display this help message
    exit   - Exit the program
    read   - Display content 
    write  - Write content
    CTRL-C - Skip line
    CTRL-D - Exit the program
    ";
    println!("{}", help);
}

