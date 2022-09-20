
fn
main() {

    let x: u32 = 5;
    println!( "The value of x is: {x}" );

    let x = 6;
    println!( "The value of x is: {x}" );

    let x: String;

    x = "Hello".to_string();

    println!( "The new value of x is: {}", x );

    const Y: &str = "AdeusMut";
    println!( "The new value of x is: {}",  &&&Y );

}



