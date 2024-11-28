use std::io;

fn main() {
    let mut user_input = String::new();

    loop {
        user_input.clear();

        println!("Hi! Input the index of Fibonacci row and I will show you this number.");
        println!("to quit type 'quit'");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");

        if user_input.trim() == "quit" {
            break;
        }

        let user_input: u128 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input the valid order number");
                continue;
            }
        };

        let mut previous_fibonacci_number: u128 = 0;
        let mut current_fibonacci_number: u128 = 1;

        if user_input == 1 {
            println!(
                "The {}-nth Fibonacci number is {}",
                user_input, previous_fibonacci_number
            );
            continue;
        } else if user_input == 2 {
            println!(
                "The {}-nth Fibonacci number is {}",
                user_input, current_fibonacci_number
            );
            continue;
        }

        let mut index: u128 = 2;

        while index < user_input {
            let next_fibonacci_number = previous_fibonacci_number + current_fibonacci_number;
            previous_fibonacci_number = current_fibonacci_number;
            current_fibonacci_number = next_fibonacci_number;
            index += 1;
        }

        println!(
            "The {}-nth Fibonacci number is {}",
            user_input, current_fibonacci_number
        );
        continue;
    }
}
