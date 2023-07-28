struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("cool-user"),
        email: String::from("cool-user@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another-email@example.com");

    let _new_user = build_user(String::from("rustacean@example.com"), String::from("rustacean"));

    // This initiates a move for username - so user1 is no longer valid
    // If we had only copied active and sign_in_count, user1 would still be valid
    let _user2 = User {
        email: String::from("another-rustacean@example.com"),
        ..user1 // struct update - copies remaining fields that are not explicitly set
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // tuple structs are similar to normal tuples in that you can destructure them into their individual pieces
    // and you can use `.` followed by the index to access an individual value
    println!("{}", black.0);
    println!("{}", origin.2);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email, // params have the same names as the attributes
        sign_in_count: 1
    }
}
