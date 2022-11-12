pub mod first {
    
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum
    ShirtColor {
        Green,
        Yellow
    }

    pub struct
    Inventory {
        pub shirts: Vec<ShirtColor>
    }

    impl
    Inventory {
        pub fn
        giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(
                || self.most_stocked()
            )
        }

        fn
        most_stocked(&self) -> ShirtColor {
            let mut num_green : u32 = 0;
            let mut num_yellow: u32 = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Green  => num_green  += 1,
                    ShirtColor::Yellow => num_yellow += 1
                }
            }

            if num_green > num_yellow {
                return ShirtColor::Green
            }
            ShirtColor::Yellow
        }

    }
}



#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn
shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(
        |s| s.size == shoe_size
    ).collect()
}

#[cfg(test)]
mod tests {
    use crate::{Shoe, shoes_in_size};

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal")  },
            Shoe { size: 10, style: String::from("boot")    }
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size, 
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot")    }
            ]
        );

    }
}