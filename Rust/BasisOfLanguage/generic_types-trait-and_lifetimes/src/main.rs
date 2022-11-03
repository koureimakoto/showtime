use generic_types_trait_and_lifetimes::aggregator::{Summary, Tweet, NewsArticle, notify};
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






}
