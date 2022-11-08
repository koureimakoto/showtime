use crate::Guess;

#[test]
#[should_panic]
fn _guess__new() {
    Guess::new(200);
}
