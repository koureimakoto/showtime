use std::collections::HashMap;

use crate::lib::my;

use crate::garden::{ 
    vegetables::Asparagus,
    fruits::Apple
};

pub mod lib;
pub mod garden;


fn main() {
    let plant: Asparagus = Asparagus {};
    let fruit: Apple     = Apple { color: (5, 5)};
    println!("Hello my plant {:?}!", plant);
    println!("Hello my plant {:?}!", fruit);

    my::test::deu();

    let mut map = HashMap::new();
    map.insert(1, 2);
}
