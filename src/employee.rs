use std::{collections::HashMap, io};

pub fn add(hash_map: &mut HashMap<String, Vec<String>>) {
    let mut department_name = String::new();
    println!("Type department:");
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to read the department name");
    let department_name = department_name.trim().to_string();

    let mut employee_name = String::new();
    println!("Type employee:");
    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read the employee name");
    let employee_name = employee_name.trim().to_string();

    let department_vector = hash_map.entry(department_name).or_insert(vec![]);
    department_vector.push(employee_name);
}

pub fn display_all(hash_map: &HashMap<String, Vec<String>>) {
    for (department, employees) in hash_map.iter() {
        println!("Department: {department}");

        let mut employee_names = String::new();
        let last_idx = employees.len() - 1;
        for (idx, employee_name) in employees.iter().enumerate() {
            let employee_info = if idx == last_idx {
                String::from(employee_name) + "."
            } else {
                String::from(employee_name) + ", "
            };

            employee_names.push_str(&employee_info);
        }
        println!("  Employees: {employee_names}")
    }
}
