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

fn main() {
    let b = Box::new(5); //Deref
    println!("Teste de acess: {}", b);

    let list = List::Cons(1, Box::new(
        List::Cons(2, Box::new(
            List::Cons(3, Box::new(0))
        ))
    ));
}
