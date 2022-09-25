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

}
