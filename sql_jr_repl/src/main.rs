use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::usize;
// use sql_jr_execution;

const HISTORY_FILE: &str = "./history.txt";

fn main() -> Result<()> {
    let mut rl = Editor::<(),rustyline::history::FileHistory>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }


    // Open and read the file
    let file = OpenOptions::new().read(true).write(true).create(true).open("file.txt").unwrap();   
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Create output file
    let mut file2 = OpenOptions::new().read(true).write(true).create(true).open("file_NEW.txt").unwrap();
    // let mut writer: BufWriter::new(file2);




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
                            ("erase",x) if x>=0 => {
                                // file.write_all(b"Hello, world!\n")?;
                                println!("Erasing content...");
                                // write!(file, "{}\n", inputs[1]).expect("Unable to write to file!");
                            },
                            ("write",x) if x>=0 => {
                                // file.write_all(b"Hello, world!\n")?;
                                println!("Provide content to be written:");
                                let mut buffer = String::new();
                                std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
                                // write!(file, "{}\n", inputs[1]).expect("Unable to write to file!");

                                // println!("Writing content...");
                                for i in 0..x {
                                    write!(file2, "{}\n", lines[i as usize]).expect("Unable to write to file!");
                                }
                                // write!(file2, "{}\n", inputs[1]).expect("Unable to write to file!");
                                write!(file2, "{}", buffer).expect("Unable to write to file!");

                                for i in x as usize..(lines.len()) {
                                    write!(file2, "{}\n", lines[i as usize]).expect("Unable to write to file!");
                                }
                                // write!(file2, "{}\n", lines[x]).expect("Unable to write to file!");
                                // write!(file, "{}\n", inputs[1]).expect("Unable to write to file!");
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

