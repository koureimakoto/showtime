use  std::io;
use  std::cmp::Ordering;
use rand::Rng;

fn
main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number");

    loop {
        let mut guess : String = String::new();
        println!("Please input your guess;");
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        

        /*
        
        let 
        guess: u32 = match guess
            .trim()
            .parse() {//.expect( "Please type a  number!" ) match?
                Ok(num) => num,
                Err(_)  => continue,
            };
        
        
        */    

        let 
        guess: u32 = guess
            .trim()
            .parse()
            .expect( "Please type a  number!" );
        
        println!("Your Guessed: {guess}");
        match guess.cmp( &secret_number ) {
            Ordering::Less    => println!( "Too small!" ),
            Ordering::Greater => println!( "Too big!"   ),
            Ordering::Equal   => {
                println!( "YOU WIN!" ); 
                break;
            }
        }
    }
    println!("Thre secret number is: {secret_number}");

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