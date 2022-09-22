struct 
User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


struct
Edge( i32, i32 );


#[derive(Debug)]
struct
Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn
    width( &self ) -> bool {
        self.width > 0
    }

    fn 
    square( size: u32 ) -> Self {
        Self { 
            width: size,
            height: size
        }
    }


    fn 
    can_hold( &self, rect: &Rect ) -> bool {
        ( self.width > rect.width  ) 
        && 
        ( self.height > rect.height)
    }

    fn 
    area( self: &Self ) -> u32 {
        self.width * self.height
    }

}

fn main() {
    
    // Immut User
    let user1 = User {
        email: String::from("makoto@email.com"),
        username: String::from("makotouser123"),
        active: true,
        sign_in_count: 1,
    };
    
    // Print Immut User
    printUser(&user1);

    // Mut user
    let mut user2 = User {
        email: String::from("karinho@email.com"),
        username: String::from("karinhouser123"),
        active: true,
        sign_in_count: 1,
    };

    // Modifying the Struct
    user2.active = false;
    user2.email  = String::from("anotherkarinhoeemail@email.com");

    // Creating user by function
    let user3 = buildUser( 
        "ext_email@email.com".to_string(),
        "extuser".to_string() 
    );

    printUser( &user3 );

    // Inject data of other user by syntax [..] 
    let user4 = User {
        email: String::from("injectotheruser@email.com"),
        ..user2
    };

    printUser( &user4 );


    // Struct Withou Name
    println!( "\n\n -- Struct without named field -- \n");
    println!( "Struct Edge( i32, i32 )" );

    let edge: Edge = Edge(52, 600);
    println!("Edge vertex position 01: {}", edge.0);
    println!("Edge vertex position 02: {}", edge.1);


    // Example using named field struct
    let rect: Rect = Rect { width: 30, height: 50 };
    let rect2:Rect = Rect { width: 10, height: 40 };
    let rect3:Rect = Rect { width: 60, height: 45 };

    println!( "Rectangle area: [{},{}] :: {}", rect.width, rect.height, rect.area() );
    dbg!( &rect );

    println!( "Can rect hold ract2? {}", rect.can_hold( &rect2 ) );
    println!( "Can rect hold ract3? {}", rect.can_hold( &rect3 ) );

    let sq = Rect::square(10);

    println!( "Rectangle area: [{},{}] :: {}", sq.width, sq.height, sq.area() );
    dbg!( &sq );

    if sq.width() {
        println!("Yupp!")
    }

}


/**
 * Create a new User
 */
#[allow(non_snake_case)]
fn
buildUser( email: String, username: String ) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

/**
 * Print Formated User Structure
 */
#[allow(non_snake_case)]
fn 
printUser( user: &User ) {

    println!("---");
    println!( "User name : {}", user.username );
    println!( "User email: {}", user.email    );
    println!( "state: {} :: sign count: {}",
        { if user.active {"on"} else {"off"} },
        user.sign_in_count
    );
    
}

