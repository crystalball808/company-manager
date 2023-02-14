use std::{collections::HashMap, io};
mod employee;

fn clear_terminal() {
    clearscreen::clear().expect("Failed to clear the terminal");
}

fn main() {
    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

    clear_terminal();
    println!("Welcome to Company Manager!");

    loop {
        println!(
            "\nType number of command:\n1. Add an employee\n2. Display the list of all employees\n3. Exit"
        );
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read the command");
        let command_number: i32 = match command.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Wrong input!");
                continue;
            }
        };

        if !(1..=3).contains(&command_number) {
            println!("Wrong command");
            continue;
        }
        clear_terminal();
        if command_number == 1 {
            employee::add(&mut hash_map);
            continue;
        }

        if command_number == 2 {
            employee::display_all(&hash_map);
            continue;
        }

        if command_number == 3 {
            break;
        }
    }
}
