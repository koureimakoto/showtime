use std::ops::{Deref, RangeFrom, RangeTo ,Range};

enum
List<T>{ // Como uma estrutura tem o tamanho indefinido
    Cons(i32, Box<T>),
    Nil,
}

// use crate::List::{Cons, Nil};

enum
Msg { // Exemplo de como um extrutura tem um tamanho definido
   Quit,
   Move { x: i32, y: i32 },
   Write(String),
   ChangeColor(i32, i32, i32) 
}

struct
MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn
    deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn
    new(element: T) -> MyBox<T> {
        MyBox(element)
    }
}

fn
print_hello(name: &str) {
    println!("Fuck you, {}", name)
}

fn main() {
    let b = Box::new(5); //Deref
    println!("Teste de acess: {}", b);

    let list = List::Cons(1, Box::new(
        List::Cons(2, Box::new(
            List::Cons(3, Box::new(0))
        ))
    ));


    let sttc = 5;
    let deref = &sttc;
    let deref2 = MyBox::new(*deref);

    assert_eq!(5, *(deref2.deref()));
    // assert_eq!(*deref2, *deref);
    // assert_eq!(&deref2, &deref)

    let name = MyBox::new(String::from("Rust"));
    print_hello("Carlo");
    print_hello(&name);
    print_hello(name.deref());
    print_hello(&(*name)[2..]);
    // Testing RUST from the real implementation
    print_hello(&(*name)[Range{ start:1, end: 3 }]); // Range is equal to(x..y)
    print_hello(&(*name)[RangeTo{ end: 2 }       ]); // RangeTo is equal to (x..)
    print_hello(&(*name)[RangeFrom{ start: 2 }   ]); // RangeFrom is equal to (x..)

}
