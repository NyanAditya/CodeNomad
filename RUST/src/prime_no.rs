use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num: u64 = input.trim().parse().expect("Please enter a valid number.");

    if is_prime(num) {
        println!("{} is a prime number.", num);
    } else {
        println!("{} is not a prime number.", num);
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        false
    } else if n <= 3 {
        true
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }
}
