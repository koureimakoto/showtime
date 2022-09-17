use std::io;
use rand::Rng;

fn
main() {
    let mut guess : String = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");

    println!("Thre secret number is: {secret_number}");

    println!("Please input your guess;");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    println!("Your Guessed: {guess}");

}

/*
for guess
match secret_number
    less    <<  putln( Too Small! )
    greater <<  putln( Too Big! )
    equal   <<  putln( Too Big! )
    -
-


*/