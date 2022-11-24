use std::{slice, ops::{Add, Sub}, fmt::Display};


extern "C"{
    fn abs(input: i32) -> i32;
}

static OLA_MUNDO: &str = "hello, mundo";

fn main() {
    let mut num: i32 = 5;

    let raw_01: *const i32 = &num as *const i32;
    let raw_02: *mut i32   = &mut num as *mut i32;

    println!("*Const: {:?}", raw_01);
    println!("*Mut  : {:?}", raw_02);

    unsafe{
        println!("*Mut  : {:?}", *raw_02);
    }

    // --

    let address: usize = 0x012345;
    let raw_03 : *const i32 = address as *const i32;

    unsafe {
        println!("*Mut  : {:?}", raw_03);
    }

    unsafe {
        dangerous();
    }



    // --- Creating a safe abstraction over unsafe code

    let mut v1 = [1, 2, 3, 4, 5, 6];
    println!("{}", &v1[0] << 2);

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6, 7, 8, 9, 10]);
    
    let r1 = v1.split_at_mut(2);
    let r2 = split_at_mut(r, 4);

    println!("vec: {:?}", r1.0[0] );
    println!("vec: {:?}", r2.0[2] );


    // FINALMENTE, USANDO C DENTRO DO RUST

    unsafe {
        println!("Valor absolute -3, em C: {}", abs(-3));
    }

    let add_a = Point {x: 5 , y: 2 };
    let add_b = Point {x: 17, y: 22};
    let mut add_c = add_a + add_b;
    println!("{}\n", add_c);

    add_c = add_c - add_a;
    assert_eq!(add_c, add_b);
    println!("{}\n", add_c);


    println!("Metro (5) + Milimetro (360) : {}", Milimetros(360) + Metros(5));


}
struct Milimetros(u32);
struct Metros(u32);

impl Add<Metros> for Milimetros {
    type Output = Milimetros;
    
    fn add(self, rhs: Metros) -> Self::Output {
        const MILIMETRO: u32 = 1000;
        Milimetros(self.0 + (rhs.0 * MILIMETRO))
    }
}


impl Display for Milimetros {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}




trait qualquer<T = Self> {
    type X;
}

impl qualquer for Milimetros {
    type X = Milimetros;
}

impl qualquer<Metros> for Milimetros {
    type X = Milimetros;
}




#[derive(Debug, Clone, Copy, PartialEq)]
struct
Point {
    x: i32, 
    y: i32
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_axis = self.x;
        let y_axis = self.y;
        write!(f, "({}: {} - {}: {})",
        stringify!(x_axis), self.x,
        stringify!(y_axis), self.y)
    }
}



unsafe fn
dangerous() -> i32 {
    5
}

/**
 * Cuidar com essa aplicação de slice.
 * Em duas camadas do código indicava safe, mas internamente ao 
 * from_raw_parts_mut no slice_raw_parts_mut que devolve um 
 * elemento unstable da std::ptr que também utiliza um 
 * elemento da metadata, no core que é unstable também.
 * 
 * Pode ser a melhor solução aparente, mas ou gerir por conta
 * esses slices ou arriscar a std, que tem um código muito aninhado
 * e mais complicado de buscar o erro caso esteja no fonde deles.
 */
fn
split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}