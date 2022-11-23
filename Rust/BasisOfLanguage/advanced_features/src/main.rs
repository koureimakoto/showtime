use std::slice;


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