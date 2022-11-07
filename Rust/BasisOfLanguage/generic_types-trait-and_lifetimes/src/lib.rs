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

    use std::fmt::Display;

    pub fn
    longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            return x
        }          y
    }

    // pub fn
    // longest_2<'a, 'b>(s: &) {
        
    // }


    pub struct
    ImportantExcerpt<'a>{  
        pub part: &'a str
    }

    impl<'a> ImportantExcerpt<'a> {
        pub fn
        level(&self) -> i32 {
            3
        }

        pub fn
        announce_and_return_part(&'a self, announcement: &str) -> &str {
            println!("OH ATENTCAO POHRA {}", announcement);
            self.part
        }
    }


    pub fn
    first_word<'a>(s: &'a str) -> &'a str {
        let bytes: &[u8] = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    

    pub fn
    longest_with_an_announcement<'a, T>(
        x  : &'a str,
        y  : &'a str, 
        ann: T
    ) -> &'a str where T: Display {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

}