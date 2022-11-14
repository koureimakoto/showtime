use add_one;
use add_two;

fn main() {
    let num = 8;
    println!("Será que ele vai transpassar? > {} !", add_one::add_one_and_rand(num));
    println!("Seguindo o tutorial: {}", add_two::add_two(5));
}
