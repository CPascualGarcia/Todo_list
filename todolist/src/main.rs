extern crate todolist;
use todolist::Task;

use std::io::Write;
use std::io::stdin;
use std::io::stdout;


fn runprompt(todo: &mut Vec<Task>) {
    loop {
        let mut stdout = stdout(); // stdout is the data we input at the terminal
        print!("(todo list) -> "); // Print the prompt
        stdout.flush().expect("Flush failed"); // Remove the buffered information

        let mut buffer = String::new(); // Str variable where we store the user input
        // Here below, read the input, write it at the terminal via stdin() and use
        // expect to handle any error 
        stdin().read_line(&mut buffer).expect("Failed to read line");

        let args: Vec<&str> = buffer.split_whitespace().collect();

        todolist::run(args,todo);
    }
}


fn main() {
    let mut todo: Vec<Task> = Vec::new();

    runprompt(&mut todo);

}