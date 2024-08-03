use std::env;
use std::process;

fn main() {

    // Collect CLI args.
    let args: Vec<String> = env::args().collect();
        
    // Assign the user-provided arg to an integer. Allow negatives at first so parse doesn't panic.
    let n: i32  = match &args[1].parse() {
        Ok(num) => *num,
        Err(_) => {
            print_error_msg();
            process::exit(1);
        }
    };

    // Don't allow negative numbers.
    if n < 0 {
        print_error_msg();
        process::exit(2);
    }

    // Create the starter sequence.
    let mut sequence  = vec![0, 1, 1, 2];
   
    // If the user asked for a number we already have, just return that number.
    if n < 4 {
        let n = n - 1;
        println!("{}", sequence[n as usize]);
        return;
    }

    // Loop to generate the nth Fibonacci number;
    let mut step = 4;
    while step < n {
        let next = sequence[ sequence.len() - 1 ] + sequence[ sequence.len() - 2];
        sequence.push(next);
        step += 1;
    }

    // Print the last item in the sequence.
    println!("{}", sequence.last().unwrap());

}

// Print a generic error message telling users how to call this program.
fn print_error_msg () {
    eprintln!("Usage: fibonacci <number>",);
    eprintln!("<number> must be a positive integer");
}
