use core::fmt;
use std::{fmt::Formatter, io::ErrorKind};



trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;


impl Pilot for Human {
    fn fly(&self) {
        println!("Sou Piloto Malokeiro senhores passeiros. Vou raspar rabeta aqui na nuvem, se cair nois levanta.")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Voo na ponta da vassoura... e não me interessa se você não gosta, só senta eu.")
    }
}

impl Human {
    fn fly(&self) {
        println!("Usei Alucinogeno do brabo e agora saltei do prédio, estou voando e minha visão está tão boa que cada vez vejo melhor o solo")
    }
}



fn main() {

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("O nome do dogachorro é: {}", Dog::baby_name());
    println!("O nome do dogachorro é: {}", <Dog as Animal>::baby_name());
    
    // OutlinePrint::outline_print(&Dog(5));

    Dog{x: 5}.outline_print();

    let w = Wrapper(vec![String::from("Hello"), String::from("Cabron")]);
    println!("w= {}", w);


    let num: i32 = 5;
    let km : Kilometro = 5;

    println!("Soma de tipos {num} + {km} = {}", num + km);

    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("Oi"));
    let f2: Reduzido = Box::new(|| println!("Reduzido Oi"));

    f();
    f2();

    // Teve explicações sobre alocações dinamicas e o uso de
    // T: Sized e ?Sized. Além de funções que nunca retornam
    // com  -> ! [panic]. Mas não realizei nenhum teste além 
    // do never returned.

} // -------------------------------------------

type Reduzido = Box<dyn Fn() + Send + 'static>;
type _Casa<T> = Result<T, ErrorKind>;


fn never_return() -> ! {
    println!("Oi");
    panic!();
}


trait Animal {
    fn baby_name() -> String;
}

struct Dog{
    x:i32
}


impl Dog {
    fn baby_name() -> String {
        String::from("Massaranduba")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Frederico BomRisMarcos")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}"    , "*".repeat(len + 4));
        println!("*{}*"  , " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*"  , " ".repeat(len + 2));
        println!("{}"    , "*".repeat(len + 4));
    }
}

impl OutlinePrint for Dog{}
impl fmt::Display for Dog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.x))
        // write!(f, "{}", self.x)
    }
}


struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[{}]", self.0.join(", ")))
    }
}

// --- TYPES

type Kilometro = i32;