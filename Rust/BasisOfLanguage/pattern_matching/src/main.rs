fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday: bool = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Usando sua cor favorita, {color} ... ");
    }
    else if is_tuesday {
        println!("A cor muda para verde nas terças");
    }
    else if let Ok(age) = age {
        if age > 30 {
            println!("A cor que mostra a idade chegando, roxo");
        } else {
            println!("Ahh cor laranja da juventude");
        }
    } else {
        println!("Usando azul para o fundo, que misera");
    }

    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    let mut other_stack: Vec<i32> = Vec::new();

    while let Some(top) = stack.pop() {
        println!("Valor do vetor: {}", top);
        other_stack.push(top);
    }

    for (index, item) in other_stack.iter().enumerate() {
        println!("Esse entra e sai deu nisso {index} e {item}")
    }

    // -- Aprendendo o Let

    let (
        casa,
        _,
        marmota,
    ) = (
        123,
        456,
        789
    );

    println!("Quero apenas a marmota {marmota}");

    let coord_point: (i32, i32) = (3,6);
    print_coord(&coord_point);
    print_coord(&(5, 8));

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Achou 50"),
        Some(y) => println!("Encontrado, y = {y}"),
        _ => println!("Padrãozin, x = {:?}", x),
    }

    println!("Fechou como: x = {:?}, y = {y}", x);

    match y {
        1 | 10 => println!("entrou bonitinho"),
        _ => println!("Achou algo, so não sei o que é")
    }

    let y = 9;

    match y {
        1..=10 => println!("entrou no range"),
        _ => println!("Achou algo, so não sei o que é")
    }

    // Destructuring

    let e = el {x: 1, y: 2};
    let el {x: a, y: _b} = e;

    assert_eq!(a, 1);

    let p = el{x: 5, y: 6};

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Que bizarro: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Nao creio")
        }
        Message::Hello { id } => println!("Sera que vou usar?: {}", id),
    }


}

struct el {
    x: i32, 
    y: i32
}


fn print_coord(&(x_axis, y_axis): &(i32, i32)) {
    println!("x: {x_axis} - y: {y_axis}")
}