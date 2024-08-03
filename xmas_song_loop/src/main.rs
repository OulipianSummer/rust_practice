use ordinal::Ordinal;

// Print the lyrcis to the 12 days of christmas using a loop.
fn main() {
    
    let mut birds = Vec::new();

    // Loop 12 times for the 12 days of Christmas.
    for count in 1..=12  {
       
        // Format our current round as an orginal number.
        let ordinal = Ordinal(count).to_string();
        
        // Print the prelude lines with a formatted ordinal.
        println!("On the {} day of Christmas", ordinal);
        println!("my true love sent to me");
    
        // Add a new line for each round.
        match count {
            1 => birds.push("A partridge in a pear tree!\n"),
            2 => birds.push("Two turtle doves"),
            3 => birds.push("Three french hens"),
            4 => birds.push("Four calling birds"),
            5 => birds.push("Five golden rings!"),
            6 => birds.push("Six geese a layin'"),
            7 => birds.push("Seven swans a swimin'"),
            8 => birds.push("Eight maids a milkin'"),
            9 => birds.push("Nine ladies dancing"),
            10 => birds.push("Ten lords a leapin'"),
            11 => birds.push("Eleven pipers pipin"),
            12 => birds.push("Twelve drummers drummin'"),
            _ => continue,
        }
        
        // Print the current list.
        for bird in birds.clone().into_iter().rev() {
            println!("{}", bird);
        }
    
        // Change all future instances of the partridge line to have "and".
        if count == 1 {
            birds[0] = "And a partridge in a pear tree!\n";
        }

    }

}
