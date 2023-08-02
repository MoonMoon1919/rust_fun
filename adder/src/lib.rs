pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(PartialEq, Debug)]
enum States {
    Filling,
    Active,
    Draining,
}

#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Intentionally make a test that fails
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        let larger = Rectangle {
            width: 8,
            height: 8,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn two_rectangles_are_the_same_size() {
        let one = Rectangle {
            width: 2,
            height: 2,
        };

        let two = Rectangle {
            width: 2,
            height: 2,
        };

        assert_eq!(one, two);
    }

    #[test]
    fn two_states_are_the_same() {
        let one = States::Draining;
        let two = States::Draining;

        assert_eq!(one, two);
    }

    #[test]
    fn different_states_are_different() {
        let one = States::Draining;
        let two = States::Active;

        assert_ne!(one, two);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
