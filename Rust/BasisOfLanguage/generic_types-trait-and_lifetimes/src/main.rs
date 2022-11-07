use generic_types_trait_and_lifetimes::aggregator::{Summary, Tweet, NewsArticle, notify};
use generic_types_trait_and_lifetimes::refer::{self, longest, ImportantExcerpt, longest_with_an_announcement};
// use generic_types_trait_and_lifetimes::strager_thinks::{Pa}

/* Without Generic */
fn 
largest_i32(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn 
largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// -----------------------------------

/* With Generic */
// fn 
// largest<T>(list: &[T]) -> &T{
//     let mut largest: &T = &list[0];
    
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }

//     largest
// }

struct 
Point<X1, Y1> {
    x: X1, y:Y1 
}

impl <X1, Y1>
Point<X1, Y1> {
    fn
    mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: other.y }
    }
}


enum 
Options_i32 {
    Some(i32),
    None
}

enum 
Options_f64 {
    Some(f64),
    None
}


fn
main() {
    
    /*
     * Generic Data Types
     */
    let p1   = Point { x:10, y:10.4};
    let p2 = Point { x:"Makoto", y:'W'};
    let p3  = p1.mixup(p2);

    println!("Legal -> p3.X = {}, p3.Y = {}", p3.x, p3.y);

    let integer: Options_i32 = Options_i32::Some(5);
    let float  : Options_f64 = Options_f64::Some(0.5);


    /*
     * Traits: Defining Shared Behavior
     */


    let tweet: Tweet = Tweet {
        uname  : String::from("horse_ebooks"),
        content: String::from("Falo nada manolo, essi bagulho eh loKo"),
        reply  : false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article: NewsArticle = NewsArticle {
        headline: String::from("Ahhh, mano, pinguizada muito loKa"),
        location: String::from("Pittsburger, Pará, Usa ma*"),
        author  : String::from("Iceburger"),
        content : String::from("Falou de que ganharam no Rock do gelu")
    };

    println!("New articune available! {}", article.summarize());

    notify(&tweet  );
    notify(&article);

    /*
     * Trait as Parameters
     */


    // Valid referneces with lifetime
    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r : {} ??", r);


    let x = 5;
    let r = &x;
    println!("r : {} ??", r);


    let s1 = String::from("abcd");
    let s2 = "xyz";

    println!("longest: {}", refer::longest(&s1, s2));
    println!("{}", s1);
    println!("{}", s2);

    {
        let s3 = "ghjuio";
        let result = longest(&s1, s3);
        println!("'b: {}", result); 
    }

    let outer_result;
    {
        let s3 = "ghjuio";
        outer_result = longest(&s1, s3);
        println!("'b: {}", outer_result); 
    }
    println!("'a: {}", outer_result); 


    // Lifetime annations in stuct definitions
    let novel: String = String::from("Me chamo XaKira. A alguns ânus atrás ...");
    let split: Vec<&str> = novel.split('.').collect();
    let first_sentence : &str = novel.split('.').next().expect("Naum axei essa disgrasa");
    let important_e: ImportantExcerpt = ImportantExcerpt {
        part: first_sentence
    };
    println!("Vamos ver esse split 01: {}", novel);
    println!("Vamos ver esse split 02: {}", split[1]);
    println!("Vamos ver esse lifetime na struct: {}", important_e.part);

    // Lifetime Elision

    
    // Lifetime Annotations in method definitions
    println!("Test ImportantExcerpt.level(): {}", important_e.level());
    println!("Test ImportantExcerpt.announce_and_return_part(): {}", important_e.announce_and_return_part("Conquistinha de Ouro"));
    
    println!("VALUERA: {}",
        longest_with_an_announcement("CATACUREMINHAALMA", "LOVVY", important_e.announce_and_return_part("Carinhokinha Rabugentus"))
    );


}

