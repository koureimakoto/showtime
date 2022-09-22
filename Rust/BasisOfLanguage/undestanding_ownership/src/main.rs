use std::io;

fn
main() {

    /*
    --
    Test Push String
    */
    let mut s = String::from("hello");
    let s1 = String::from(" World");

    s.push_str( &s1 );
    
    /*
    --
    Testing the Slice Text Function
    */
    let (i, txt) = first_word(&s);
    
    println!( "Size: {} :: Text: {}", i, txt );
    println!( "Complete text: {}", s ); 

    // Set Empty 
    s.clear();

    // Test Empty
    println!("After s.clear(): {}", s);

    /*
    --
    Numerical Tuple Test
    */
    let (x , y) = tup();
    println!( "Tup: x :: {}", x ); // Outside variable
    println!( "Tup: y :: {y}" );   // Inside  variable


    /*
    ---
    Test dynamic insertion in pointer position
    */
    let fake: String = String::from("Makoto World");
    let mut opt : String = String::new();

    io::stdin()
        .read_line( &mut opt )
        .expect( "Xii Erro ao pegar o bagulho." );

    #[allow(unused_parens)]
    let opt  : usize = (
        opt
        .trim()
        .parse()
        .expect( "Xii no numero deu pau" )
    );

    let range = &fake[0..opt - 1];
    println!( "Print dyna range: {}", range );
    println!( "Print stat range: {}", &fake[0..5] );

    assert_eq!( range, &fake[..5]);
}


/**
 * Simple Returned Tuple
 */
fn
tup() -> ( i32, i32 ) {
    ( 4, 5 )
}

/**
 * Function of Rust Tutorial modified
 * 
 * Original return only num and I modified it to return a tuple for testing 
 * 
 */
fn
first_word( s: &str ) -> ( usize, &str ) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        
        if item == b' ' {
            return ( i, &s[..i] );
        }
    }

    ( s.len(), &s[..] )
}
