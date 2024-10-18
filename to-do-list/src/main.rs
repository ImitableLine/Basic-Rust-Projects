// To Do List app
// A basic CLI program requirements below:
// 1- List current To-Do's,
// 2- Add To-Do,
// 3- Delete To-Do,
// 4- Edit Specific To-Do,
// 5- Delete all To-Do,
use std::io;

fn main() {
    let mut todos: Vec<String> = Vec::new();

    println!("Welcome to the To-Do app! Here are your current to-do's: ");
    loop {
        list_todo(&todos);
        list_choices(&mut todos);
    }
}

// List Choices ================================
fn list_choices(todos: &mut Vec<String>) {
    println!(
        "1: List To-Do's | 2: Add To-Do | 3: Delete a To-Do | 4: Edit a To-Do | 5: Delete all To-Do's"
    );

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line properly");

    let parsed_input = input.trim().parse::<i32>();

    match parsed_input {
        Ok(choice) => match choice {
            // Match against Ok(choice)
            1 => list_todo(&todos),
            2 => add_todo(todos),
            3 => delete_todo(todos),
            4 => edit_todo(todos),
            5 => delete_all_todo(todos),
            _ => println!("Invalid choice, please try again."),
        },
        Err(_) => println!("Invalid input, please enter a number."),
    }
}
// Operation functions ========================
fn list_todo(todos: &Vec<String>) {
    println!("-----------------------------------------------------------------------------");
    let mut count = 1;
    for todo in todos {
        println!("{}: {}", count, todo);
        count += 1;
    }
    println!("-----------------------------------------------------------------------------");
}
fn add_todo(todos: &mut Vec<String>) {
    println!("Please enter to-do: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read to-do correctly");

    let trimmed_input = input.trim().to_string();
    todos.push(trimmed_input);
}
fn delete_todo(todos: &mut Vec<String>) {
    println!("Please select a to-do to delete");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read choice correctly");
    match input.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= todos.len() => {
            todos.remove(index - 1);
        }
        _ => println!("Invalid choice, try again"),
    }
}
fn edit_todo(todos: &mut Vec<String>) {
    println!("Please select a to-do to edit (1-{}):", todos.len());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read choice correctly");

    match input.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= todos.len() => {
            let index = index - 1;
            println!("Current to-do: {}", todos[index]);
            println!("Enter the new value:");

            let mut new_value = String::new();
            io::stdin()
                .read_line(&mut new_value)
                .expect("Failed to read the new value");

            // Update the to-do item
            todos[index] = new_value.trim().to_string();
            println!("To-do updated successfully!");
        }
        _ => println!("Invalid choice, try again"),
    }
}
fn delete_all_todo(todos: &mut Vec<String>) {
    todos.clear();
}
