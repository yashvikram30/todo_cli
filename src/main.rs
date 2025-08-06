use std::io;
use std::fs;
use std::io::Read;


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool
}

fn save_tasks(list: &Vec<Task>){

    let json_file = serde_json::to_string(&list)
        .expect("Error occurred while converting to string");

    match fs::write("tasks.json", json_file){
        Ok(_) => println!("Completed"),
        Err(_) => println!("Error occurred"),
    }
}

fn load_tasks()-> Vec<Task>{
   match fs::read_to_string("tasks.json") {
    Ok(data) => {
        let tasks : Vec<Task> = serde_json::from_str(&data).unwrap_or_else(|_| Vec::new());
        tasks
    }
    Err(_) => Vec::new(),
   }
}

fn display_tasks(list: &Vec<Task>) {
    for (index, task) in list.iter().enumerate() {
        let completed = if task.completed { '✅' } else { '❌' };
        println!("{}. Task: {}, Completed: {}", index, task.description, completed);
    }
}


fn create_todo() -> Vec<Task> {
    let mut list = load_tasks();

    // CREATE: creating new tasks and pushing
    loop{
        println!("Enter: \n1. Press 1 to exit. \n2. Press 2 to create todos.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 1
        };

        if input == 1 { break;}
        else if input != 2 {
            println!("You entered wrong input");
            continue;
        }

        println!("Write your description:");
        let mut description = String::new();

        io::stdin()
            .read_line(&mut description)
            .expect("Error reading line");

        println!("Write whether it's completed (true/false):");
        let mut completed = String::new();

        io::stdin()
            .read_line(&mut completed)
            .expect("Error reading line");
        
        let completed: bool = match completed.trim().parse() {
            Ok(true) => true,
            Ok(false)=> false,
            Err(_) => false,
        };

        let task = Task{
            description,
            completed
        };
        
        list.push(task);
        println!("Todo created! \n");

    }

    save_tasks(&list);

    list
}

fn update_todo(list:&mut Vec<Task>){
    
    println!("Enter: \n1. Press 1 to update description \n2. Press 2 to create completion");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };

        // display all the tasks to user
        display_tasks(list);

        println!("Enter the index to be updated:");
            let mut index = String::new();

            io::stdin()
                .read_line(&mut index)
                .expect("Error reading line");

            let index = match index.trim().parse() {
                Ok(num) => num,
                Err(_) => 0
            };

            if index>=list.len(){
                println!("Enter valid index");
                return;
            }

        if input == 0 {
            println!("Wrong input entered");
        }

        if input == 1 {

            println!("Enter your description:");
            let mut description = String::new();

            io::stdin()
                .read_line(&mut description)
                .expect("Error reading line");

            list[index].description = description.trim().to_string();

        }

        if input == 2 {
            println!("Enter your completion (true/false):");
            let mut completion = String::new();

            io::stdin()
                .read_line(&mut completion)
                .expect("Error reading line");

            let completion:bool = match completion.trim().parse(){
                Ok(res) => res,
                Err(_) => false
            };
            list[index].completed = completion;
        }

    save_tasks(&list);
    
}

fn delete_todo(list:&mut Vec<Task>){

    display_tasks(list);

    println!("Enter your number:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading string");

    let guess: usize = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    if guess< list.len() {
        list.remove(guess);
    }

    save_tasks(&list);
}

fn main() {
    loop {
        println!("\nMenu:\nChoose your operation:\n1. Create Todos\n2. Display Todos\n3. Update Todos\n4. Delete Todos\n5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error while reading choices");

        let choice = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => 0
        };

        let mut list: Vec<Task>;

        match choice {
            1 => { list = create_todo(); }
            2 => { list = load_tasks(); display_tasks(&list); }
            3 => { list = load_tasks(); update_todo(&mut list); }
            4 => { list = load_tasks(); delete_todo(&mut list); }
            5 => break,
            _ => println!("Invalid choice entered."),
        }
    }
}
