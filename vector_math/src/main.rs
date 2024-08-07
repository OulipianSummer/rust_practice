use std::env;
use std::process;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Make sure we receive at least two args.
    if args.len() <= 2 {
        print_error_msg();
        process::exit(1);
    }
        
    // Check to make sure we have a valid command here. 
    let command = &args[1];
    match command.as_str() {
        "median" => (),
        "mean" => (),
        _ => {
            eprintln!("Invalid command!");
            print_error_msg();
            process::exit(1);
        }
    };
    
    // Perform median math.
    if command == "median" {
  
        // Parse our CLI arguments into a vector. Note, we cast all values as floats to do floating
        // point math.
        let mut values: Vec<f32> = Vec::new();
        for arg in &args[2..] {
            let value: f32 = arg.parse().expect("Failed to parse user input into a vaid number");
            values.push(value);
        }

        // Sort a vector of floats.
        // See: https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let len = values.len();
        
        // An even number means we have to take the middle two values and get an average.
        if len % 2 == 0 {
          let low = values.get( len / 2 - 1 ).expect("Could not locate median low end");
          let high = values.get( len / 2 ).expect("Could not locate median hight end");
          let total = low + high;
          let median =  total / 2.0;

          println!("The median is: {}", median);

        } else { // Return the middle number
            let median: f32 = *values.get( ( len - 1 ) / 2 ).expect("Could not determine median");
            println!("The median is: {}", median);
        }  
    
    // Perform mean math.
    } else if command == "mean" {

        // Parse our CLI arguments into a vector. Note, we cast all values as floats to do floating
        // point math.
        let mut values: Vec<u32> = Vec::new();
        for arg in &args[2..] {
            let value: u32 = arg.parse().expect("Failed to parse user input into a vaid number");
            values.push(value);
        }

        let mut map: HashMap<String, u32> = HashMap::new();

        // Derive a hash map with a count of the most common entries.
        // See: https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value
        for value in values {
            let count = map.entry( value.to_string() ).or_insert(0);
            *count += 1;
        }

        // Sort a hashmap by converting it into a vector of tuples.
        // See: https://stackoverflow.com/questions/63950197/sort-a-hashmap-by-values-in-rust
        let mut hash_vec: Vec<(&String, &u32)> = map.iter().collect();
        hash_vec.sort_by(|a, b| b.1.cmp(a.1)); // Sort the vector by caount value (tup[1]).
        let (mean, count) = hash_vec[0];

        println!("The mean is: {}, it appeared {} time(s)", mean, count);
    }

}

fn print_error_msg() {
    eprintln!("Usage: vector_math <command> <integer 1> <ingeger 2> ...");
    eprintln!("Commands: median, mean");
}
