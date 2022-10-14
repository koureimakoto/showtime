use std::ptr;


enum Multype {
    Int(i32),
    Float(f64),
    Utf8String(String)
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[2];
    println!( "The Third Elmenent is {}", third );

    let el: Option<&i32> = v.get(2);
    match 
    el {
        Some(third) => println!( "The Third Element is {}.", third ),
        None              => println!( "There is no Element Founded."    )    
    }

    let el: Option<&i32> = v.get(3);
    match 
    el {
        Some(third) => println!( "The Third Element is {}.", third ),
        None              => println!( "There is no Element Founded."    )    
    }

    let el: Option<&i32> = v.get(3);
    println!(" O que sai aqui? {:?}", el);
 
    let v = vec![100, 32, 57];
    for i in &v {
        println!( "{}", i );
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!( "{}", i );
    }

    // ---
    
    let row = vec![
        Multype::Int(3),
        Multype::Float(13.13),
        Multype::Utf8String(String::from("Color"))
    ];

    let opt = &row[0];
    match opt {
        Multype::Int(el) => println!("Element {}", el),
        _  => println!("None")
    }

    let opt = row.get(3);
    match opt {
        Some(el)=> {
            match el {
                Multype::Float(intern_el) => println!("Element {}", intern_el),
                _ => println!("Não foi hoje")
            }
        },
        _  => println!("Qualquer coisa")
    }

    // ---
    // Storing UTF8 Encoded Text with String

    let mut s: String = String::new();
    let data : &str = "Um exemplo de Texto qualquer";
     
    let s_ptr = &mut *s;
    let data_ptr = data.as_ptr();
    println!("String - new : {:p}", s_ptr);
    println!("Str ptr   : {:p}", data);
    println!("Str ptr2  : {:p}", data_ptr);

    let s = data.to_string();
    let s_ptr2 = s.as_ptr();
    println!("String    - new : {}", s);
    println!("String ptr- new : {:p}", s_ptr2);


    let s = data.to_string();

    let mut hello = String::from("السلام عليكم");
    println!("Hello: {}", hello);
    let hello = String::from("Dobrý den");
    println!("Hello: {}", hello);
    let hello = String::from("Hello");
    println!("Hello: {}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("Hello: {}", hello);
    let hello = String::from("नमस्ते");
    println!("Hello: {}", hello);
    let hello = String::from("こんにちは");
    println!("Hello: {}", hello);
    let hello = String::from("안녕하세요");
    println!("Hello: {}", hello);
    let hello = String::from("你好");
    println!("Hello: {}", hello);
    let hello = String::from("Olá");
    println!("Hello: {}", hello);
    let hello = String::from("Здравствуйте");
    println!("Hello: {}", hello);
    let hello = String::from("Hola");
    println!("Hello: {}", hello);

}
