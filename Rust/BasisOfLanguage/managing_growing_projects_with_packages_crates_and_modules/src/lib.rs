//use crate::front_of_house::hosting;



#[path = "./my/root.rs"]
pub mod my;


//mod 
//front_of_house;

fn 
deliver_order() {}

mod
back_of_house {

    pub const TEST: u32 = 5;

    pub enum 
    Appetizer {
        Soup,
        Salad
    }

    pub struct
    Breakfast{
        pub 
        toast         : String,
        seasonal_fruit: String
    }


    impl 
    Breakfast {
        pub fn 
        summer( toast: &str ) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from( "peaches" )
            }
        }
        
    }


    fn 
    fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }


    fn cook_order() {}

}

// mod 
// customer {
//     use crate::front_of_house::hosting;
//     use crate::back_of_house;

//     pub fn     
//     eat_at_restaurant() {
//         let mut meal = back_of_house::Breakfast::summer("Rye");
        
//         let order1 = super::back_of_house::Appetizer::Salad;
//         let order1 = super::back_of_house::Appetizer::Soup;
        
//         hosting::add_to_waitlist();
        
//         meal.toast = String::from( "Wheat" );
//         println!( "I'd like {} toast please", meal.toast);
//         println!( "my const: {}", back_of_house::TEST )
//     }
// }



// --
use std::fmt::Result;
use std::io::Result as IoResult;

pub fn 
func1() -> Result {
    Ok(())
}
fn 
func2() -> IoResult<()> {
    Ok(())
}

