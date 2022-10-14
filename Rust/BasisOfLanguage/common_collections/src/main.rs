
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


}
