use std::{vec, thread, time::Duration};

use functional_language_features_interators_and_closures::first::{Inventory, ShirtColor};


fn main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Yellow,
            ShirtColor::Green,
            ShirtColor::Yellow
        ]
    };

    let user_01 : Option<ShirtColor> = Some(ShirtColor::Green);
    let gaway_01: ShirtColor         = store.giveaway(user_01);

    println!(
        "A preferência do usuário: {:?} são {:?}",
        user_01, gaway_01
    );

    let user_02 : Option<ShirtColor> = Some(ShirtColor::Yellow);
    let gaway_02: ShirtColor         = store.giveaway(user_02);

    println!(
        "A preferência do usuário: {:?} são {:?}",
        user_02, gaway_02
    );

    // Teste de closure com anotação explicita
    let expensive_closure = |num: u32| -> u32 {
        println!("Lerda pra caralho");
        thread::sleep(Duration::from_secs(1));
        num
    };

    println!("{}", expensive_closure(5));


    // Capturando referencias ou movendo de proprietário

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    only_borrows();
    println!("After calling closure: {:?}", list);


    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // let mut borrows_mutably = || list.push(7);

    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();



    // Processamento uma série de itens com iterador

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    
    for item in &v1 {
        println!("V1: {}", item);
    }
    
    
    for item in v1_iter {
        println!("V1 ITERATOR: {}", item);
    }
    
    // -- 
    let v2: Vec<i32> = vec![1, 3, 5];

    let v3: Vec<_> = v2.iter().map(|x| x + 1 ).collect();
    
    for item in v3 {
        print!("Complicado: {} -> ", item);
    }
    println!();

}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

