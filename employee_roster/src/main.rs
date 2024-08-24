use std::collections::HashMap;
use std::io;
use std::io::Write;
use itertools::Itertools;

enum Command {
  Exit,
  Help,
  List(String),
  AddEmployee(Employee),
  AddDepartment(String),
  Invalid,
}

#[derive(Debug)]
struct Employee {
  name: String,
  department: String,
}

fn main() {

    let mut roster: HashMap<String, Vec<String>>  = HashMap::new();

    println!("Entering command mode...");
    println!("Welcome to department management!");

    // While in the REPL, continually accept input until user types "exit".
     loop {

        let mut raw_input  = String::new();
  
        // Print leading > character.
        print!("> ");
        io::stdout().flush().unwrap();
   
        // Acquire user input.
        io::stdin()
            .read_line(&mut raw_input)
            .expect("Failed to read line");

        // Split up words in command syntax to find matches.
        let input: Vec<&str> = raw_input.trim().split(" ").collect();
    
        // Consume input and transform it into a command type.
        let command: Command = command_factory(input);

        // Distribute commands to relevant functions.
        match command {
            Command::List(list) => show_list(list, &mut roster),
            Command::AddEmployee(employee) => {
                add_employee(employee, &mut roster);
            }
            Command::AddDepartment(department) => add_department(department, &mut roster),
            Command::Invalid => {
                println!("Invalid command, please try again or type \"help\" for assistance");
            }
            Command::Help => print_help(),
            Command::Exit => {
                println!("Bye!");
                break;
            }
        } 
    }
}

// Add an employee to the roster. If the employee's department doesn't exist, create it.
fn add_employee (employee: Employee, roster: &mut HashMap<String, Vec<String>>) {
    println!("Adding {} to {}", &employee.name, &employee.department);
    roster.entry(employee.department).or_insert_with(Vec::new).push(employee.name);
}


// Add a new department to the roster;
fn add_department (department: String, roster: &mut HashMap<String, Vec<String>>) {
    println!("Adding the {} department", &department);
    roster.entry(department).or_insert_with(Vec::new);
}

fn show_list (list_type: String, roster: &mut HashMap<String, Vec<String>>) {
  
    // List all departments.
    if list_type == "departments" {
      for department in roster.keys().sorted() {
        println!("{}", department);  
      }
    }
    
    // List all employees by department
    if list_type == "employees" {
      for department in roster.keys().sorted() {
          let employees = roster[department].clone();
        for employee in employees {
          println!("{}, {}", employee, department);
        }
      }
    }
}



fn print_help () {
 println!("Usage: employee_roster <command> [arguments]");
 println!("Commands: list, add, exit, help\n");
 println!("list employees  List the current employees by department");
 println!("list departments  List the current departments");
 println!("add employee <name> <department>  Add and employee to a department. Department is created if it doesn't exist");
 println!("add department <name>  Add a new department");
 println!("help  Prints this message");
 println!("exit  Exits the program");
}


// Create Commands from user input and return them as options to be checked. Warning: takes ownership of
// input variable.
fn command_factory (input: Vec<&str>) -> Command {

    let valid_arg_stems = ["department","departments", "employee", "employees"];
 
    // The add command must always have at least 3 arguments, the second of which must be either department(s)
    // or employee(s).
    if input[0] == "add" {
        if input.len() < 3 || !valid_arg_stems.contains(&input[1]) { 
            return Command::Invalid;
        }
        
        // Handle departments.
        if input[1].contains("department") {
            return Command::AddDepartment(String::from(input[2]));
        }

        // Handle employees.
        if input[1].contains("employee") {
            
            // Make sure we are looking for 4 arguments.
            if input.len() != 4 {
                return Command::Invalid;
            }  else {
                let employee = Employee {
                    name: String::from(input[2]),
                    department: String::from(input[3]),
                };
                return Command::AddEmployee(employee);
             }
        }
            
        // If we're at this point, the second arg was probably invalid.
        return Command::Invalid;
    }

    // The list command must always have 2 arguments, the second of which must be either
    // department(s) or employee(s).
    if input[0] == "list" {
        if input.len() != 2 || !valid_arg_stems.contains(&input[1]) {
          return Command::Invalid;
        }
        return Command::List(String::from(input[1]));
    }

    // The exit command simply exits the program and has no arguments.
    if input[0] == "exit" {
      return Command::Exit;
    }

    if input[0] == "help" {
      return Command::Help;
    }

    return Command::Invalid;

}
