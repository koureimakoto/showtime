

pub mod madonna;


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
mod first;

#[cfg(test)]
mod other;