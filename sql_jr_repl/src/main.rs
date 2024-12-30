use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use std::fs::{OpenOptions, rename};
use std::io::prelude::*;
use std::io::BufReader;
use std::usize;
// use sql_jr_execution;
extern crate sql_jr_repl;
use sql_jr_repl::{writer,eraser_line};

const HISTORY_FILE: &str = "./history.txt";

fn main() -> Result<()> {
    let mut rl = Editor::<(),rustyline::history::FileHistory>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }


    // // Open and read the file
    // let file = OpenOptions::new().read(true).write(true).create(true).open("file.txt").unwrap();   
    // let reader = BufReader::new(&file);
    // let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // // Create output file
    // let mut file2 = OpenOptions::new().read(true).write(true).create(true).open("file_NEW.txt").unwrap();
    // // let mut writer: BufWriter::new(file2);




    loop {
        // Open and read the file
        let file1_path = "file.txt";
        let file = OpenOptions::new().read(true).write(true).create(true).open(file1_path).unwrap();   
        let reader = BufReader::new(&file);
        let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
        

        // Create output file
        // let file2_path = "file_NEW.txt";
        // let file2 = OpenOptions::new().read(true).write(true).create(true).open(file2_path).unwrap();
        // let mut writer: BufWriter::new(file2);

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
                    "size" => {
                        println!("Size of file: {}", lines.len());
                    }
                    "Make a sandwich" => {
                        println!("Making a sandwich...");
                    }

                    "read" => {
                        for line in lines {
                            println!("{}", line);
                        }
                        // print!("{}", lines[x as usize]);
                    },
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

                        let indx = inputs[1].parse::<i32>().unwrap();
                        if indx >= lines.len() as i32 {
                            println!("Index out of range! List has {} lines", lines.len());
                            continue
                        }

                        match (inputs[0],inputs[1].parse::<i32>().unwrap()) {
                            ("read",x) if x>=0 && x<lines.len() as i32 => {
                                println!("Reading content... \n{}", lines[x as usize]);
                                // print!("{}", lines[x as usize]);
                            },
                            ("erase",x) if x>=0 => {
                                // file.write_all(b"Hello, world!\n")?;
                                println!("Erasing content...");
                                // Close original file
                                drop(file);
                                eraser_line(file1_path, x as usize, &mut lines)?;


                            },
                            ("write",x) if x>=0 => {
                                
                                // Close original file
                                drop(file);
                                // Overwrite original file
                                writer(file1_path, x as usize, &lines)?;
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

        read-<integer> - Read content on line <integer>
        write-<integer> - Write content on line <integer>
            - <String> Provide content to be written. 

    Commands

    size   - Check the no. of lines
    help   - Display this help message
    exit   - Exit the program
    read   - Display content 
    write  - Write content
    CTRL-C - Skip line
    CTRL-D - Exit the program
    ";
    println!("{}", help);
}

