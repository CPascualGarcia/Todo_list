use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};


use std::usize;
// use sql_jr_execution;
extern crate sql_jr_repl;
use sql_jr_repl::*;

// XXX To do -- Add history to the repl
// XXX To do -- Cleanup
// XXX To do -- Add done status
// XXX To do -- Add dates to tasks?


const HISTORY_FILE: &str = "./history.txt";

fn main() -> Result<()> {

    let mut rl = Editor::<(),rustyline::history::FileHistory>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    loop {
        
        //////////////////////////////////////////////////////////////////////////
        // Prepare database
        let db_path: &str = "TodoList.db";
        db_setup(db_path).unwrap();
        //////////////////////////////////////////////////////////////////////////
        

        let readline = rl.readline(">> ");
        match readline{
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {}", err);
                break
            },
            Ok(line_in) => {
                let inputs = parser_input(&line_in); 
                
                // let (command, inputs) = line_in.split_once(' ').unwrap();
                match &inputs[0] as &str {
                    "" => {continue},
                    "help" => {display_help()},
                    "exit" => {print!("Goodbye!"); break},
                    "size" => {println!("Size of database: {}", db_size(db_path).unwrap());
                        rl.add_history_entry(line_in.as_str())?;},
                    "read" => {
                        if inputs.len() == 1 {
                            read_all(db_path).unwrap();
                            rl.add_history_entry(line_in.as_str())?;
                            continue;
                        }
                        match inputs[1].parse::<usize>() {
                            Ok(x) => println!("{}", db_reader(db_path, x as usize).unwrap()),
                            Err(_) => {
                                println!("Invalid input: please enter a valid number");
                                continue}
                        };
                        rl.add_history_entry(line_in.as_str())?;
                    },
                    "write" => {
                        if inputs.len() <= 2 {
                            println!("Invalid input");
                            continue;
                        };
                        // Check contents of input line
                        let (_, rest) = line_in.trim().split_once(' ').unwrap();
                        let (input_line, content) = rest.split_once(' ').unwrap();

                        match (input_line.parse::<usize>(), content.parse::<String>()) {
                            (Ok(x),Ok(buffer)) => {

                                if db_verify(db_path, x) == true {
                                    println!("Entry id already defined in database: {}\nStill overwrite? [y/n]", db_reader(db_path, x as usize).unwrap());
                                    loop {
                                        let mut ask = String::new();
                                        std::io::stdin().read_line(&mut ask).expect("Failed to read line");
                                        if ask.trim() == "y" {
                                            db_writer(db_path, &buffer, x).unwrap();
                                            rl.add_history_entry(line_in.as_str())?;
                                            break;
                                        }
                                        else if ask.trim() == "n" {rl.add_history_entry(line_in.as_str())?;break;}
                                        else {println!("Still overwrite? [y/n]");}
                                    }
                                } else {
                                    db_writer(db_path, &buffer, x).unwrap();
                                    rl.add_history_entry(line_in.as_str())?;
                                }
                            
                            },
                            _ => {println!("Invalid input");continue}
                        };
                    },
                    "remove" => {
                        match inputs[1].parse::<usize>() {
                            Ok(x) => db_remove(db_path, x)?,
                            Err(_) => {
                                println!("Invalid input: please enter a valid number");
                                continue}
                        };
                        rl.add_history_entry(line_in.as_str())?;
                    },

                    _ => println!("Invalid input")           
                };
            }
        };
        
    }
    rl.save_history(HISTORY_FILE)
}


fn display_help(){
    let help: &str = " 
    Commands

    size   - Check the no. of lines
    help   - Display this help message
    exit   - Exit the program
    read   - Display content 
    write  - Write content
    CTRL-C - Skip line
    CTRL-D - Exit the program

    <String> - String input

        read-<integer>  - Read content on line <integer>
        write-<integer> - Write content on line <integer>
            <String>    - Provide content to be written.
    ";
    println!("{}", help);
}

