use rand::Rng;

pub fn add_one_and_rand(left: usize) -> usize {
    left + 1 + rand::thread_rng().gen_range(1..=100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one_and_rand(2);
        assert_ne!(result, 4);
    }
}
