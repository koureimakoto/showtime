pub mod aggregator {
    use std::fmt::{Display, Debug};


    pub trait Summary {
        fn summarize(&self)        -> String;
        fn summarize_author(&self) -> String {
            format!("(Read more from {}...)", self.summarize())
        }
    }

    /**
     * PRimeira Estrutura
     */
    pub struct 
    NewsArticle {
        pub headline: String, 
        pub location: String, 
        pub author  : String,
        pub content : String
    }

    impl Summary
    for  NewsArticle {
        fn 
        summarize(&self) 
        -> String {
            format!("{}, bys {} ({})"
            , self.headline
            , self.author
            , self.location
            )
        }

    }

    /**
     * Segunda Estrutura
     */
    pub struct 
    Tweet {
        pub uname  : String,
        pub content: String,
        pub reply : bool,
        pub retweet: bool
    }

    impl Summary
    for Tweet {
        fn
        summarize(&self)
        -> String {
            format!("{}: {}", self.uname, self.content)
        }


        fn
        summarize_author(&self)
        -> String {
            format!("@{}", self.uname)
        }
    
    }

    pub fn
    notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn 
    some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        5
    }

    pub fn
    some_function2<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone, U: Clone + Debug {
        6
    }


    /*
     * Demonstra que retornar um implementação só pode ser feita com
     * um unico tipo de implementação de uma trait por função.
     */
    // pub fn
    // returns_summarizable(switch: bool) -> impl Summary {
    //     if switch {
    //         Tweet {
    //             uname  : String::from("horse_ebooks"),
    //             content: String::from("Falo nada manolo, essi bagulho eh loKo"),
    //             reply  : false,
    //             retweet: false
    //         }
    //     } else {
    //         NewsArticle {
    //             headline: String::from("Ahhh, mano, pinguizada muito loKa"),
    //             location: String::from("Pittsburger, Pará, Usa ma*"),
    //             author  : String::from("Iceburger"),
    //             content : String::from("Falou de que ganharam no Rock do gelu")
    //         }
    //     }
    // }

}


pub mod
strager_thinks {
    use std::fmt::Display;

    struct
    Pair<T> {
        x: T,
        y: T
    }

    impl<T> Pair<T> {
        fn 
        new(x: T, y: T) -> Self {
            Self {x, y}
            
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn 
        cmp_display(&self) {
            if self.x >= self.y {
                println!("O maior número é x = {}", self.x);
            } else {
                println!("O maior número é y = {}", self.y)
            }
        }
    }
}

pub mod refer {
    // fn
    // longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
}