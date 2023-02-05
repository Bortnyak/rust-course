struct User {
    active: bool,
    username: String,
    sign_in_count: i32,
}

#[derive(Debug)]
struct Coordinates(i32, i32, i32);

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Vova"),
        sign_in_count: 0,
    };

    println!("User name = {}", user1.username);
    println!("User active = {}", user1.active);
    println!("User sign in count = {}", user1.sign_in_count);

    let coords = Coordinates(1, 2, 3);
    println!("Coordinates = {:?}", coords);
}
