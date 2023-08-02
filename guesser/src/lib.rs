pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value <= 1 {
            panic!("Guess value should be greater than or equal to 1, got {}", value);
        } else if value >= 100 {
            panic!("Guess value should be less than or equal to 100, got {}", value);
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
