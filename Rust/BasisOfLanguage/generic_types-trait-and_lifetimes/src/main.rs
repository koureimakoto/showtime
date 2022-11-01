
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

fn
main() {
    // Studying generic types
    let p1 = Point { x:10, y:10.4};
    let p2 = Point { x:"Makoto", y:'W'};
    let p3 = p1.mixup(p2);

    println!("Legal -> p3.X = {}, p3.Y = {}", p3.x, p3.y);
}
