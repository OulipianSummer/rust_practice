use std::env;
use std::process;

// Converts temperatures from Celcisus to Farenheit and vise-versa. Parses command line arguments
// to decide which mode to use.
fn main() {
    
    // Collect user args.
    let args: Vec<String> = env::args().collect();
    
    // Check that we received two args, exit the program with an error if not.
    if args.len() < 2 {
        print_usage_error();
        process::exit(1);
    }

    let command = &args[1];

    // Check the command arg.
    match command.as_str() {
        "c2f" => "c2f",
        "f2c" => "f2c",
        _ => {
            eprintln!("Error invalid command: {}", command);
            print_usage_error();
            process::exit(2);
        },
    }; 

    // Parse and check the command arg into a float value.
    let temp: f32 = match &args[2].trim().parse() {
        Ok(num) => *num,
        Err(_) => { 
            eprintln!("Error: temperature must be a valid number, either a decimnal or an integer.");
            print_usage_error();
            process::exit(2);
        }
    };

    // Convert our temps and print a result.
    convert_temp(command.to_string(), temp);
}

// Prints a standard error message indicating how to use this program.
fn print_usage_error () {
    eprintln!("Usage: $ temperature_converter <command> <temp>");
    eprintln!("Commands: c2f, f2c");
    eprintln!("Temp: a valid temperature in either Celsiuse or Farenheit");
}

// Given a command and a temperature, convert that temperature to a different format.
fn convert_temp(command: String, temp: f32) {

    if command == "c2f" { // Convert from Celsius to Farenheit.
        let result = (temp * 1.8) + 32.0;
        println!("Result: {temp:.2}°C = {result:.2}°F");
    } else if command == "f2c" { // Convert from Farenheit to Celcius.
       let result = (temp - 32.0) / 1.8;
       println!("Result: {temp:.2}°F = {result:.2}°C");
    } else { // Print the error message since we didn't get the correct inputs.
        print_usage_error();
    }
}
