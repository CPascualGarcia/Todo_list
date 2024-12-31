use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};


use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::usize;
// use sql_jr_execution;
extern crate sql_jr_repl;
use sql_jr_repl::*;//{writer_line,eraser_line};


use rusqlite::Connection;
use rusqlite::OpenFlags;

const HISTORY_FILE: &str = "./history.txt";

fn main() -> Result<()> {
    let mut rl = Editor::<(),rustyline::history::FileHistory>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }

    loop {
        // Open and read the file
        let file1_path = "file.txt";
        let file = OpenOptions::new().read(true).write(true).create(true).open(file1_path).unwrap();   
        let reader = BufReader::new(&file);
        let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

        //////////////////////////////////////////////////////////////////////////
        // Open database
        let conn = Connection::open_with_flags("TodoList.db",
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();

        // Create basic table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS 
            tasks (id INTEGER PRIMARY KEY, task TEXT NOT NULL)", 
            ()).unwrap();

        // Insert rows into the table
        let mut stmt = conn.prepare(
            "INSERT INTO tasks (id, task) VALUES (?1, ?2)").unwrap();
        stmt.execute((13, "sandwich")).unwrap();
            
        conn.execute(
            "INSERT INTO tasks (id, task) VALUES (?1, ?2)",
            (14, "mustard"),
        ).unwrap();

        
        // Prepare a query statement
        let mut stmt = conn.prepare("SELECT * FROM tasks WHERE id = ?1").unwrap();
        // Query the database
        // let mut rows = stmt.query(&[(":name", "one")]).unwrap();
        let mut rows = stmt.query(&[&13]).unwrap();
        // Iterate over the results
        while let Some(row) = rows.next().unwrap() {
            let id: i32 = row.get(0).unwrap();
            let task: String = row.get(1).unwrap();
        
            println!("id: {}, task: {}", id, task);
        }
        
        //////////////////////////////////////////////////////////////////////////
        

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

                        match (inputs[0],indx) {
                            ("read",x) if x>=0 && x<lines.len() as i32 => {
                                println!("Reading content... \n{}", lines[x as usize]);
                            },
                            ("erase",x) if x>=0 => {
                                println!("Erasing content...");
                                // Close original file
                                drop(file);
                                // Erase line
                                eraser_line(file1_path, x as usize, &mut lines)?;
                            },
                            ("write",x) if x>=0 => {
                                
                                // Close original file
                                drop(file);
                                // Overwrite original file
                                writer_line(file1_path, x as usize, &lines)?;
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

        read-<integer>  - Read content on line <integer>
        write-<integer> - Write content on line <integer>
            <String>    - Provide content to be written. 

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

