//! # My Kratos
//!
//! `my_kratos` is a collection of madness


use more_about_cargo_and_crateio::{mix, PrimaryColor};




///
///  Add one  , sub one, add 2   , sub one
///  add three, sub two, add four, sub five.
/// 
///  # Complex Formula created by Alan Turing
/// 
/// ```
/// 
/// let crazy_num = 99;
/// let result    = anycratename::crazy_add_one(
///     crazy_num
/// )
/// 
/// print("Crazy Frog: Don DOn Dondoron don don DOn ...  ");
/// assert_eq!(100, result);
/// 
/// ```
///
pub fn
crazy_add_one(crazy_num: i32) -> i32 {
    crazy_num + 1 - 1 + 2 - 1 + 3 - 2 + 4 - 5
}

fn main() {
    println!("CRAZY?? {}", crazy_add_one(99));

    let red   : PrimaryColor = PrimaryColor::Red;
    let yellow: PrimaryColor = PrimaryColor::Yellow;

    mix(red, yellow);
}
