use std::fmt::format;

fn
expo(num: i32) -> i32 {
    num * num
}

// --

#[derive(Debug)]
struct
Rect { w: u32, h: u32 }

impl
Rect {
    pub fn
    can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }
}

// --

fn
greeting(name: &str) -> String {
    format!("Olá manolo {}", name)
}

// --

struct
Guess {
    value: i32
}

impl
Guess {
    pub fn
    new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Precisa ser entre 1 e 100, recebemos {}", value);
        }

        Guess { value: value }
    }    
}



#[cfg(test)]
mod tests {
    use crate::Guess;
    use crate::expo;
    use crate::Rect;
    use crate::greeting;

    /*
     * How to Write Tests
     */
    #[test]
    fn _expo(){
        let result: i32 = 25;
        assert_eq!(result, expo(5))
    }

    // #[test]
    // fn _force_panic() {
    //     panic!("Ai ai ai ai ai")
    // }

    #[test]
    fn _can_hold_larger() {
        let larger : Rect = Rect { w: 10, h: 09};
        let smaller: Rect = Rect { w: 09, h: 08};

        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn _can_hold_smaller() {
        let larger : Rect = Rect { w: 10, h: 09};
        let smaller: Rect = Rect { w: 09, h: 08};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn _greeting(){
        let result: String = greeting("Suvisco");
        assert!(
            !result.contains("Suv2isc"),
            "Vixe não é o seu nome mesmo '{}'",
            result
        );
    }


    #[test]
    #[should_panic]
    fn _guess__new() {
        Guess::new(200);
    }


}
