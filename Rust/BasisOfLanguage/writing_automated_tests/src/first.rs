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
#[ignore = "OBSOLETA"]
fn _greeting(){
    let result: String = greeting("Suvisco");
    println!("ESCAPEI!!");
    assert!(
        !result.contains("Suv2isc"),
        "Vixe não é o seu nome mesmo '{}'",
        result
    );
}
