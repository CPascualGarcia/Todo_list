
use std::{process, sync::atomic::{self, AtomicU64}};

pub fn run(args: Vec<&str>, todo: &mut Vec<Task>) {
    parse_arguments(args,todo);
}
    

// Data struct for each task
#[derive(Debug)]
pub struct Task {
    task: String,
    done_status: bool,
    id: u64,
}


impl Task{
    fn update_status(&mut self) {
        self.done_status = true;
    }

    fn update_task(&mut self, new_name: String) {
        self.task = new_name;
    }
}

fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    let command = args[0];

    match command {
        "add" => {
            if let Some(value) = args.get(1){
                let new_task = *value;
                add_new_task(todo_list, new_task);
                display_todo(todo_list);                
            }   else {
                println!("Unrecognized name for the task"); 
            }
        },

        "show" => {
            display_todo(todo_list);
        },

        "delete" => {
            match &args[1].parse::<u64>(){
                Ok(value) => { remove_task(todo_list, *value) },
                Err(message) => { println!("{}", message.to_string())}
            }
        },

        "update" => {

            //  If id parsing error
            match &args[1].parse::<u64>() {
                Ok(value) => {
                    // Task getting error
                    if let Ok(task) = get_task(todo_list,*value){

                        if let Some(value) = args.get(2){
                            let new_task = *value;
                            task.update_task(new_task.into());
                        } else {
                            println!("Unrecognized name for the task");
                        } 
                    } else {
                        println!("Task id not found in the list");
                    }
                },
                Err(message) => {
                    println!("{}", message);
                }
            }
        },
        
        "done" => {
            match &args[1].parse::<u64>() {
                Ok(value) => {
                    if let Ok(task) = get_task(todo_list,*value){
                        task.update_status();
                    }else{
                        println!("task id not found in the list");
                    }
                },
                Err(message) => {
                    println!("{}", message.to_string());
                }
            }
        },
        "exit" => {process::exit(0);},
        "help"| _ => {display_help();}
    }
}


static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);
fn add_new_task(todo_list: &mut Vec<Task>, task_string: &str){

    // Here we assign an id to the task. This is done by taking
    // the global, static variable UNIQUE_ID and incrementing it
    let id_no = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);

    // Here we create a new task using the struct Task
    let task: Task = Task{
        task : task_string.into(), // Converts the &str to the expected type (String here)
        done_status: false,
        id: id_no
    };
    
    todo_list.push(task);
    println!("{} added to the todo list: ", task_string); 
}

fn display_todo(todo_list: &Vec<Task>){

    if todo_list.len()==0{
        println!("Todo list is empty");
        return;
    }

    for item in todo_list{
        println!("id: {} \n
        name: {} \n
        done: {}", item.id, item.task, item.done_status);
    }

}

fn remove_task(todo_list: &mut Vec<Task>, id_no: u64){

    todo_list.retain(|task| task.id != id_no);
}


fn get_task(todo_list: &mut Vec<Task>, task_id: u64) -> Result<&mut Task, &str>{ 

    for task in todo_list{
        if task.id == task_id{
            return Ok(task);
        }else{
            continue;
        }
    };
    return Err("Task not found in todo list");
}


fn display_help(){
    let help: &str = "
    Welcome to the todo list app
    structure of the query:
        command [arguments]
    
    supported commands:
        add <task_name> - Add a new task
        show - Show all tasks
        done <task_id> - Mark a task as done
        delete <task_id> - Delete a task
        update <task_id> <task_name> - Update a task
        exit - Exit the program
    arguments:
        <task_name> - The name of the task
        <task_id> - The id of the task
    ";
    println!("{}", help);
}