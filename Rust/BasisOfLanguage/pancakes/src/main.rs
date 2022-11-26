use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    // Muito legal, certamente usarei
    Pancakes::hello_macro();
}


fn incr(n: &mut i32) {

    *n += 1;
  
  }
  
  