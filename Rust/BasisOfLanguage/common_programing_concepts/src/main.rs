use std::io;

fn
main() {

    let x: u32 = 5;
    println!( "The value of x is: {x}" );

    let x = 6;
    println!( "The value of x is: {x}" );

    let mut x: String;

    x = "Hello".to_string();

    println!( "The new value of x is: {}", x );


    // --- 
    let mut crash: String = String::new();

    io::stdin()
        .read_line( &mut crash )
        .expect( "Failed to read this input" );

    if  crash.eq( &String::from(&crash) ) {
        x = 6.to_string();//"Xii".to_string();
        println!("entrou");   
    }

    println!( "Result: {x}" );

    // ---

    println!( "Float Test" );

    let quotient = 50 / 60 ;
    println!( "Quotient without floating typing representation: {quotient}" );


    let quotient = 50.0 / 60.0 ;
    println!( "Quotient without floating typing representation: {quotient}" );


    let quotient: f32 = 50.0 / 60.0 ;
    println!( "Quotient without floating typing representation: {quotient}" );

    println!( "Remainder: {}", quotient / 4.0 );

    // --- Tuples
    println!( "Tuples Test" );
    
    let tup: (u8, f64, &str) = ( 1, 5.5, "Hello" );

    let (x, y, z) = tup; 

    println!( "Tuples values: {} {} {} ", x, y, z );
    println!( "Tuples values of \n tup.1: {}\n tup.2: {}", tup.1, tup.2 );

    // --- Array
    println!( "Array Test" );

    let arr: [char; 3] = ['o','i','\n'];
 
    println!( 
        "Array value of the \n first position: {} \n second position: {}",
        arr[0],
        arr[1]
    );

    myFirstRustFunction( 5 );
    mySecondRustFunction( 4 );

    // ---

    let expr = {
        let y = 5;
        y + 2
    };
    println!( "Rust EXPR: {expr}" );


    // ---
    println!( "Rust Functions Concept");

    println!( "No literal return: {}", noLiteralReturn() );
    println!( "Using Text now: {}", textWithouLiteralReturn());



    // --


    let inj = 2;
    for number in inj..4 {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

#[allow(non_snake_case)]
fn myFirstRustFunction( a: i32 ) {
    println!( "HelloW {a}" );
}

#[allow(non_snake_case)]
fn mySecondRustFunction( a: i32 ) {
    println!( "HelloW {a}" );
}

#[allow(non_snake_case)]
fn noLiteralReturn() -> i32 {
    5
}

#[allow(non_snake_case)]
fn textWithouLiteralReturn() -> String {
    "Sem memo em!".to_string()
}