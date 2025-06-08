fn main() {
    // Print a welcome message when the program starts
    println!("Welcome to the Bootcamp Fizz Buzz program!");
    // Call the fizz_buzz function to run the Fizz Buzz logic
    fizz_buzz();
}

// This function runs the Fizz Buzz logic
fn fizz_buzz() {
    // This variable will count how many times we print "fizz buzz"
    let mut fizz_buzz_count = 0;
    // Loop from 1 to 301 (inclusive)
    for i in 1..=301 {
        // If the number is divisible by both 3 and 5
        if i % 3 == 0 && i % 5 == 0 {
            println!("fizz buzz"); // Print "fizz buzz"
            fizz_buzz_count += 1;  // Add 1 to our counter
        } else if i % 3 == 0 {
            // If the number is only divisible by 3
            println!("fizz"); // Print "fizz"
        } else if i % 5 == 0 {
            // If the number is only divisible by 5
            println!("buzz"); // Print "buzz"
        } else {
            // If the number is not divisible by 3 or 5
            println!("{}", i); // Just print the number
        }
    }
    // After the loop, print how many times we saw "fizz buzz"
    println!("'fizz buzz' occurred {} times.", fizz_buzz_count);
}
