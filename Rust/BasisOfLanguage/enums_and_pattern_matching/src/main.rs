

/*
// Old

enum 
IpAddrKind {
    V4,
    v6
}

struct 
IpAddr {
    kind: IpAddrKind,
    addr: String
}*/

enum 
IpAddr {
    TV4( u8, u8, u8, u8 ),
    V4(String),
    V6(String)
}


impl IpAddr {
    fn 
    splited_printer( self ) {
        match self {
            IpAddr::TV4( x, y,z ,w ) => {
                println!("[FF:{}](1)", x );
                println!("[FF:{}](2)", y );
                println!("[FF:{}](3)", z );
                println!("[FF:{}](4)", w );
            }, 
            _ => { }
        }
        
    }
}


/*
---
Coin Exemple
*/
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}



enum 
Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


impl Coin {
    fn 
    coin( &self ) -> u8 {
        match self {
            Coin::Penny   =>  1,
            Coin::Nickel  =>  5,
            Coin::Dime    => 10,
            Coin::Quarter(state) => {
                println!( "State Quarter from {:?}", state);
                25
            }
        }
    }
}

// -- Options<T>

fn 
plus_one( value: Option<i32> ) -> Option<i32> {
    match value {
        Some(n) => Some( n + 1 ),
        None => None
    }
}


/*

set :
    Penny,
    Nickel,
    Dime,
    Quarter
-

impl
Coin :

    u8
    coin(&) :
        match & :
            Coin<<Penny   :  1
            Coin<<Nickel  :
                io<<out( '{} {} {}', x , y) )
            -
            Coin<<Dime    : 10, Coin<<Quarter : 25 
---

*/



fn main() {
    /*
    // Old
    
    let v4: IpAddrKind = IpAddrKind::V4;
    let v6: IpAddrKind = IpAddrKind::v6;

    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from( "127.0.0.1" )
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::v6,
        addr: String::from( "::1" )
    };
    */

    IpAddr::TV4( 127, 0, 0, 1 ).splited_printer(); 
    let _home : IpAddr = IpAddr::V4( String::from( "127.0.01") );
    let _lb   : IpAddr = IpAddr::V6( String::from( "::1")      );


    println!( "Penny value   :: {}", Coin::Penny.coin()   );
    println!( "Nickel value  :: {}", Coin::Nickel.coin()  );
    println!( "Dime value    :: {}", Coin::Dime.coin()    );
    println!( "Quarter value :: {}", Coin::Quarter( UsState::Alabama ).coin() );


    // -- Options<T>

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}
