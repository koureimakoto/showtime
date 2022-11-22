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
}


unsafe fn
dangerous() -> i32 {
    5
}