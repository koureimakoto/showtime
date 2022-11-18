use std::{thread, ops::Range, time::Duration, sync::mpsc};

fn main() {
    let handle = thread::spawn(|| {
        for i in (Range{start: 1, end: 10})  {
            println!("Ola número {i} da thread local");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in (Range{start: 1, end: 5}) {
        println!("Ola número {i} da thread global (principal)");
        thread::sleep(Duration::from_millis(2));
    }

    // --

    let v = vec![1, 2, 3];
    
    let handle_b = thread::spawn(create_a_thread_fn(v));

    // handle_b.join().unwrap();


    // Using msg passing to transfer data between threads


    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(
            String::from("Hi main thread!")
        )
    });

    let s = match rx.recv() {
       Ok(x) => x,
       Err(e)=> String::from(format!("Algum problema? Encontrei este erro {}", e))
    };
    println!("{}", s);


    // Channel and Ownership Transference

    let (o_tx, o_rx) = mpsc::channel();
    let o_tx_1 = o_tx.clone();
    thread::spawn(
        move || {
            let spam = vec![
                String::from("Sentar"),
                String::from("Spam"),
                String::from("em"),
                String::from("você"),
                String::from("vagabundo")
            ];
            for item in spam {
                o_tx_1.send(item).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        }
    );


    thread::spawn(
        move || {
            let spam = vec![
                String::from("Muita"),
                String::from("thread"),
                String::from("para"),
                String::from("pouco"),
                String::from("espaço")
            ];
            for item in spam {
                o_tx.send(item).unwrap();
                thread::sleep(Duration::from_secs(1))
            }
        }
    );


    for item in o_rx {
        println!("Pegay um pokemonn: {}", item)
    }

    // Shared-State Concurrency

    

}

fn
create_a_thread_fn(v: Vec<i32>) -> impl Fn() {
    move || { println!("Aqui é um vetor: {:?}", v); }
}