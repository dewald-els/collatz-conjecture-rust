use std::io;

fn main() {
    println!("----------------------------------------------------");
    println!("| Welcome to Collatz Conjecture.                   |");
    println!("| ------------------------------------------------ |");
    println!("| The rules:                                       |");
    println!("| Odd Number: Multiply by 3 add                    |");
    println!("| Even Number: Divide by 2                         |");
    println!("---------------------------------------------------\n");
    println!("What number do you want to start with?");

    let mut seed_number = String::new();

    io::stdin()
        .read_line(&mut seed_number)
        .expect("Failed to read line.");

    let seed_number: u32 = seed_number.trim().parse().expect("Please type a number!");

    println!("------------------------------");
    println!("Starting with {seed_number}");

    calculate_next_number(seed_number);

    io::stdin().read_line(&mut String::new()).unwrap();
}

fn calculate_next_number(current_number: u32) {
    println!("{current_number}");

    if current_number == 1 {
        println!("Reached 1. Ending.");
        return;
    }
    if is_odd(current_number) {
        calculate_next_number(current_number * 3 + 1)
    } else {
        calculate_next_number(current_number / 2);
    }
}

fn is_odd(number: u32) -> bool {
    number % 2 != 0
}
