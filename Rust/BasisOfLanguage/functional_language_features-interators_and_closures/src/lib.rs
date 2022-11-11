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